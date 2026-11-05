//! Original caller-ordered GPU alpha-stage adapter. No retail state inference.
//! Contracts: ordered-alpha-gpu-v1.md and byte-source-over-v1.md; not full GS replay.
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};
use godhand_engine::byte_blend::ByteBlend;
use godhand_engine::fragment_alpha::{AlphaComparison, AlphaStage, WritePermissions};
use std::{borrow::Cow, collections::BTreeMap, error::Error, sync::mpsc, time::Duration};

pub type GpuResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
fn invalid(message: &'static str) -> Box<dyn Error + Send + Sync> {
    std::io::Error::other(message).into()
}

#[derive(Clone, Copy, Debug)]
pub struct AlphaVertex {
    pub clip: [f32; 4],
    pub uv: [f32; 2],
    /// Raw byte-domain values, not normalized colors. Interpolated screen-linearly.
    pub color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct RawAlphaTexture {
    pub width: u32,
    pub height: u32,
    /// Raw RGBA bytes. Presentation-normalized retail alpha is NOT interchangeable.
    pub rgba: Vec<u8>,
}

impl From<godhand_disc::tim2::RawRgbaImage> for RawAlphaTexture {
    fn from(image: godhand_disc::tim2::RawRgbaImage) -> Self {
        Self {
            width: u32::from(image.width),
            height: u32::from(image.height),
            rgba: image.pixels,
        }
    }
}

/// Explicit host nearest-sampler addressing, not full GS region addressing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextureWrap {
    #[default]
    Clamp,
    Repeat,
}
impl std::str::FromStr for TextureWrap {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "clamp" => Ok(Self::Clamp),
            "repeat" => Ok(Self::Repeat),
            _ => Err("address must be clamp or repeat; region/mirror modes unsupported"),
        }
    }
}
impl TextureWrap {
    fn gpu(self) -> AddressMode {
        match self {
            Self::Clamp => AddressMode::ClampToEdge,
            Self::Repeat => AddressMode::Repeat,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlphaDraw {
    pub vertices: Vec<AlphaVertex>,
    pub texture: usize,
    pub stage: AlphaStage,
    pub blend: ByteBlend,
    pub cull_backfaces: bool,
    pub address: [TextureWrap; 2],
}

fn comparison_code(comparison: AlphaComparison) -> u32 {
    use AlphaComparison::*;
    match comparison {
        Never => 0,
        Always => 1,
        Less => 2,
        LessEqual => 3,
        Equal => 4,
        GreaterEqual => 5,
        Greater => 6,
        NotEqual => 7,
    }
}

pub fn validate_inputs(
    size: [u32; 2],
    textures: &[RawAlphaTexture],
    draws: &[AlphaDraw],
) -> GpuResult<()> {
    if size.iter().any(|v| !(64..=1024).contains(v)) {
        return Err(invalid("alpha target dimensions must be within64..1024"));
    }
    if textures.is_empty() || textures.len() > 256 || draws.len() > 4096 {
        return Err(invalid("alpha input count exceeds bounds"));
    }
    let mut total_bytes = 0usize;
    for image in textures {
        if image.width == 0 || image.height == 0 || image.width > 4096 || image.height > 4096 {
            return Err(invalid("invalid raw alpha texture dimensions"));
        }
        let expected = image.width as usize * image.height as usize * 4;
        if image.rgba.len() != expected {
            return Err(invalid("raw alpha texture length mismatch"));
        }
        total_bytes += expected;
        if total_bytes > 64 * 1024 * 1024 {
            return Err(invalid("raw alpha texture budget exceeded"));
        }
    }
    let mut total_vertices = 0usize;
    for draw in draws {
        if draw.texture >= textures.len()
            || draw.vertices.is_empty()
            || !draw.vertices.len().is_multiple_of(3)
        {
            return Err(invalid("invalid alpha triangle input"));
        }
        total_vertices += draw.vertices.len();
        if total_vertices > 1_048_576 {
            return Err(invalid("alpha vertex budget exceeded"));
        }
        for v in &draw.vertices {
            if !v
                .clip
                .iter()
                .chain(&v.uv)
                .chain(&v.color)
                .all(|x| x.is_finite())
                || v.color.iter().any(|x| !(0.0..=255.0).contains(x))
            {
                return Err(invalid("invalid finite alpha vertex or byte color"));
            }
        }
    }
    Ok(())
}

fn vertex_bytes(vertices: &[AlphaVertex]) -> Vec<u8> {
    debug_assert!(vertices.len().is_multiple_of(3));
    let mut bytes = Vec::with_capacity(vertices.len() * 64);
    for triangle in vertices.chunks_exact(3) {
        let reference = triangle[0].color;
        let mut mask = 0u32;
        for (channel, value) in reference.iter().enumerate() {
            if triangle
                .iter()
                .all(|vertex| vertex.color[channel] == *value)
            {
                mask |= 1 << channel;
            }
        }
        for vertex in triangle {
            for value in vertex.clip.iter().chain(&vertex.uv).chain(&vertex.color) {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            for value in reference {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            bytes.extend_from_slice(&mask.to_le_bytes());
            bytes.extend_from_slice(&0u32.to_le_bytes());
        }
    }
    bytes
}

struct PreparedDraw {
    vertices: Buffer,
    count: u32,
    branches: Vec<([bool; 4], BindGroup)>,
    blended: bool,
}

pub struct OrderedAlphaGpu {
    shader: ShaderModule,
    layout: BindGroupLayout,
    samplers: BTreeMap<[TextureWrap; 2], Sampler>,
    pipelines: BTreeMap<[bool; 4], RenderPipeline>,
}

pub struct AlphaFrame {
    color: Texture,
    framebuffer_alpha: Texture,
    depth: Texture,
    size: [u32; 2],
}

pub struct AlphaReadback {
    /// Presentable byte RGB. Its alpha MUST remain255, independent of data alpha.
    pub color: Vec<[u8; 4]>,
    pub framebuffer_alpha: Vec<u8>,
    pub depth: Vec<f32>,
}

impl OrderedAlphaGpu {
    pub fn new(device: &RenderDevice) -> Self {
        let shader = device
            .wgpu_device()
            .create_shader_module(ShaderModuleDescriptor {
                label: Some("original ordered alpha stage"),
                source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("alpha_stage.wgsl"))),
            });
        let layout = device.create_bind_group_layout(
            "ordered alpha input",
            &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(32),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        );
        let mut samplers = BTreeMap::new();
        for u in [TextureWrap::Clamp, TextureWrap::Repeat] {
            for v in [TextureWrap::Clamp, TextureWrap::Repeat] {
                samplers.insert(
                    [u, v],
                    device.create_sampler(&SamplerDescriptor {
                        label: Some("explicit nearest texture addressing"),
                        mag_filter: FilterMode::Nearest,
                        min_filter: FilterMode::Nearest,
                        address_mode_u: u.gpu(),
                        address_mode_v: v.gpu(),
                        ..Default::default()
                    }),
                );
            }
        }
        Self {
            shader,
            layout,
            samplers,
            pipelines: BTreeMap::new(),
        }
    }

    fn pipeline(
        &mut self,
        device: &RenderDevice,
        writes: WritePermissions,
        cull_backfaces: bool,
    ) -> [bool; 4] {
        let key = [
            writes.rgb,
            writes.framebuffer_alpha,
            writes.depth,
            cull_backfaces,
        ];
        if self.pipelines.contains_key(&key) {
            return key;
        }
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("ordered alpha pipeline layout"),
            bind_group_layouts: &[Some(&self.layout)],
            immediate_size: 0,
        });
        let targets = [
            Some(ColorTargetState {
                format: TextureFormat::Rgba8Unorm,
                blend: None,
                write_mask: if writes.rgb {
                    ColorWrites::RED | ColorWrites::GREEN | ColorWrites::BLUE
                } else {
                    ColorWrites::empty()
                },
            }),
            Some(ColorTargetState {
                format: TextureFormat::R8Unorm,
                blend: None,
                write_mask: if writes.framebuffer_alpha {
                    ColorWrites::RED
                } else {
                    ColorWrites::empty()
                },
            }),
        ];
        let attributes = [
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 16,
                shader_location: 1,
            },
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 24,
                shader_location: 2,
            },
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 40,
                shader_location: 3,
            },
            VertexAttribute {
                format: VertexFormat::Uint32,
                offset: 56,
                shader_location: 4,
            },
        ];
        let pipeline = device.create_render_pipeline(&RawRenderPipelineDescriptor {
            label: Some("ordered alpha permissions"),
            layout: Some(&layout),
            vertex: RawVertexState {
                module: &self.shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[Some(RawVertexBufferLayout {
                    array_stride: 64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &attributes,
                })],
            },
            primitive: PrimitiveState {
                cull_mode: if cull_backfaces {
                    Some(Face::Back)
                } else {
                    None
                },
                ..Default::default()
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: Some(writes.depth),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: Default::default(),
            fragment: Some(RawFragmentState {
                module: &self.shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &targets,
            }),
            multiview_mask: None,
            cache: None,
        });
        self.pipelines.insert(key, pipeline);
        key
    }

    pub fn render(
        &mut self,
        device: &RenderDevice,
        queue: &RenderQueue,
        size: [u32; 2],
        clear: [u8; 4],
        textures: &[RawAlphaTexture],
        draws: &[AlphaDraw],
    ) -> GpuResult<AlphaFrame> {
        validate_inputs(size, textures, draws)?;
        let extent = Extent3d {
            width: size[0],
            height: size[1],
            depth_or_array_layers: 1,
        };
        let target = |label, format| {
            device.create_texture(&TextureDescriptor {
                label: Some(label),
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format,
                usage: TextureUsages::RENDER_ATTACHMENT
                    | TextureUsages::COPY_SRC
                    | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
        };
        let frame = AlphaFrame {
            color: target("byte RGB/presentation alpha", TextureFormat::Rgba8Unorm),
            framebuffer_alpha: target("raw framebuffer alpha data", TextureFormat::R8Unorm),
            depth: target("ordered alpha depth", TextureFormat::Depth32Float),
            size,
        };
        let destination_snapshot = device.create_texture(&TextureDescriptor {
            label: Some("previous primitive byte RGB"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let destination_view = destination_snapshot.create_view(&Default::default());
        let gpu_textures: Vec<_> = textures
            .iter()
            .map(|image| {
                device.create_texture_with_data(
                    queue,
                    &TextureDescriptor {
                        label: Some("raw source RGBA"),
                        size: Extent3d {
                            width: image.width,
                            height: image.height,
                            depth_or_array_layers: 1,
                        },
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: TextureDimension::D2,
                        format: TextureFormat::Rgba8Unorm,
                        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
                        view_formats: &[],
                    },
                    TextureDataOrder::LayerMajor,
                    &image.rgba,
                )
            })
            .collect();
        let views: Vec<_> = gpu_textures
            .iter()
            .map(|image| image.create_view(&Default::default()))
            .collect();
        let mut prepared = Vec::with_capacity(draws.len());
        for draw in draws {
            let vertices = device.create_buffer_with_data(&BufferInitDescriptor {
                label: Some("ordered alpha vertices"),
                contents: &vertex_bytes(&draw.vertices),
                usage: BufferUsages::VERTEX,
            });
            let mut branches = Vec::new();
            for passed in [true, false] {
                if !passed && !draw.stage.enabled {
                    continue;
                }
                let permissions = draw.stage.branch_permissions(passed);
                if !permissions.rgb && !permissions.framebuffer_alpha && !permissions.depth {
                    continue;
                }
                let key = self.pipeline(device, permissions, draw.cull_backfaces);
                let params: Vec<u8> = [
                    u32::from(draw.stage.enabled),
                    comparison_code(draw.stage.comparison),
                    u32::from(draw.stage.reference),
                    u32::from(!passed),
                    u32::from(draw.blend == ByteBlend::SourceOver),
                    0,
                    0,
                    0,
                ]
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect();
                let uniform = device.create_buffer_with_data(&BufferInitDescriptor {
                    label: Some("explicit alpha-stage parameters"),
                    contents: &params,
                    usage: BufferUsages::UNIFORM,
                });
                let group = device.create_bind_group(
                    "ordered alpha branch",
                    &self.layout,
                    &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::TextureView(&views[draw.texture]),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Sampler(&self.samplers[&draw.address]),
                        },
                        BindGroupEntry {
                            binding: 2,
                            resource: uniform.as_entire_binding(),
                        },
                        BindGroupEntry {
                            binding: 3,
                            resource: BindingResource::TextureView(&destination_view),
                        },
                    ],
                );
                branches.push((key, group));
            }
            prepared.push(PreparedDraw {
                vertices,
                count: draw.vertices.len() as u32,
                branches,
                blended: draw.blend == ByteBlend::SourceOver,
            });
        }
        let color = frame.color.create_view(&Default::default());
        let alpha = frame.framebuffer_alpha.create_view(&Default::default());
        let depth = frame.depth.create_view(&Default::default());
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ordered alpha commands"),
        });
        let target_views = [&color, &alpha, &depth];
        if prepared.iter().all(|d| !d.blended) {
            render_targets(
                &mut encoder,
                target_views,
                Some(clear),
                &prepared,
                &self.pipelines,
                None,
            );
        } else {
            render_targets(
                &mut encoder,
                target_views,
                Some(clear),
                &[],
                &self.pipelines,
                None,
            );
            for draw in &prepared {
                if draw.branches.is_empty() {
                    continue;
                }
                if !draw.blended {
                    render_targets(
                        &mut encoder,
                        target_views,
                        None,
                        std::slice::from_ref(draw),
                        &self.pipelines,
                        None,
                    );
                    continue;
                }
                for first in (0..draw.count).step_by(3) {
                    encoder.copy_texture_to_texture(
                        TexelCopyTextureInfo {
                            texture: &frame.color,
                            mip_level: 0,
                            origin: Origin3d::ZERO,
                            aspect: TextureAspect::All,
                        },
                        TexelCopyTextureInfo {
                            texture: &destination_snapshot,
                            mip_level: 0,
                            origin: Origin3d::ZERO,
                            aspect: TextureAspect::All,
                        },
                        extent,
                    );
                    render_targets(
                        &mut encoder,
                        target_views,
                        None,
                        std::slice::from_ref(draw),
                        &self.pipelines,
                        Some(first),
                    );
                }
            }
        }
        queue.submit([encoder.finish()]);
        Ok(frame)
    }
}

fn render_targets(
    encoder: &mut CommandEncoder,
    views: [&TextureView; 3],
    clear: Option<[u8; 4]>,
    draws: &[PreparedDraw],
    pipelines: &BTreeMap<[bool; 4], RenderPipeline>,
    first_triangle: Option<u32>,
) {
    let [color, alpha, depth] = views;
    let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("caller-ordered alpha-stage draw stream"),
        color_attachments: &[
            Some(RenderPassColorAttachment {
                view: color,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: clear.map_or(LoadOp::Load, |c| {
                        LoadOp::Clear(
                            bevy::color::LinearRgba::new(
                                f32::from(c[0]) / 255.,
                                f32::from(c[1]) / 255.,
                                f32::from(c[2]) / 255.,
                                1.,
                            )
                            .into(),
                        )
                    }),
                    store: StoreOp::Store,
                },
            }),
            Some(RenderPassColorAttachment {
                view: alpha,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: clear.map_or(LoadOp::Load, |c| {
                        LoadOp::Clear(
                            bevy::color::LinearRgba::new(f32::from(c[3]) / 255., 0., 0., 1.).into(),
                        )
                    }),
                    store: StoreOp::Store,
                },
            }),
        ],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
            view: depth,
            depth_ops: Some(Operations {
                load: if clear.is_some() {
                    LoadOp::Clear(0.)
                } else {
                    LoadOp::Load
                },
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        }),
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    for draw in draws {
        pass.set_vertex_buffer(0, *draw.vertices.slice(..));
        let range = first_triangle.map_or(0..draw.count, |first| first..first + 3);
        for triangle in range.step_by(3) {
            for (key, group) in &draw.branches {
                pass.set_pipeline(&pipelines[key]);
                pass.set_bind_group(0, &**group, &[]);
                pass.draw(triangle..triangle + 3, 0..1);
            }
        }
    }
}

impl AlphaFrame {
    /// Byte-coded RGB, with opaque presentation alpha. Not an sRGB attachment.
    pub fn color_texture(&self) -> &Texture {
        &self.color
    }
    /// Separate framebuffer alpha DATA; never use as presentation transparency.
    pub fn framebuffer_alpha_texture(&self) -> &Texture {
        &self.framebuffer_alpha
    }
    pub fn depth_texture(&self) -> &Texture {
        &self.depth
    }
    pub fn size(&self) -> [u32; 2] {
        self.size
    }

    pub fn readback(&self, device: &RenderDevice, queue: &RenderQueue) -> GpuResult<AlphaReadback> {
        let copy = |texture: &Texture,
                    bytes_per_pixel: u32,
                    aspect: TextureAspect|
         -> GpuResult<Vec<u8>> {
            let row_bytes = self.size[0] * bytes_per_pixel;
            let pitch = row_bytes.div_ceil(256) * 256;
            let buffer = device.create_buffer(&BufferDescriptor {
                label: Some("bounded alpha-stage readback"),
                size: u64::from(pitch) * u64::from(self.size[1]),
                usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            let mut encoder = device.create_command_encoder(&Default::default());
            encoder.copy_texture_to_buffer(
                TexelCopyTextureInfo {
                    texture,
                    mip_level: 0,
                    origin: Origin3d::ZERO,
                    aspect,
                },
                TexelCopyBufferInfo {
                    buffer: &buffer,
                    layout: TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(pitch),
                        rows_per_image: Some(self.size[1]),
                    },
                },
                Extent3d {
                    width: self.size[0],
                    height: self.size[1],
                    depth_or_array_layers: 1,
                },
            );
            queue.submit([encoder.finish()]);
            let (tx, rx) = mpsc::sync_channel(1);
            buffer.slice(..).map_async(MapMode::Read, move |result| {
                let _ = tx.send(result);
            });
            device.poll(PollType::Wait {
                submission_index: None,
                timeout: Some(Duration::from_secs(15)),
            })?;
            rx.recv_timeout(Duration::from_secs(1))??;
            let mapped = buffer.slice(..).get_mapped_range()?;
            let mut bytes = Vec::with_capacity(row_bytes as usize * self.size[1] as usize);
            for row in mapped
                .chunks_exact(pitch as usize)
                .take(self.size[1] as usize)
            {
                bytes.extend_from_slice(&row[..row_bytes as usize]);
            }
            drop(mapped);
            buffer.unmap();
            if bytes.len() != row_bytes as usize * self.size[1] as usize {
                return Err(invalid("incomplete alpha readback"));
            }
            Ok(bytes)
        };
        let color = copy(&self.color, 4, TextureAspect::All)?
            .chunks_exact(4)
            .map(|v| v.try_into().unwrap())
            .collect();
        let framebuffer_alpha = copy(&self.framebuffer_alpha, 1, TextureAspect::All)?;
        let depth = copy(&self.depth, 4, TextureAspect::DepthOnly)?
            .chunks_exact(4)
            .map(|v| f32::from_le_bytes(v.try_into().unwrap()))
            .collect();
        Ok(AlphaReadback {
            color,
            framebuffer_alpha,
            depth,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use godhand_engine::fragment_alpha::{AlphaComparison::Always, AlphaFailure::Keep};
    fn valid() -> (RawAlphaTexture, AlphaDraw) {
        (
            RawAlphaTexture {
                width: 1,
                height: 1,
                rgba: vec![1, 2, 3, 128],
            },
            AlphaDraw {
                texture: 0,
                vertices: vec![
                    AlphaVertex {
                        clip: [0., 0., 0.5, 1.],
                        uv: [0., 0.],
                        color: [128.; 4]
                    };
                    3
                ],
                stage: AlphaStage {
                    enabled: true,
                    comparison: Always,
                    reference: 0,
                    failure: Keep,
                    depth_write_masked: false,
                },
                blend: ByteBlend::Overwrite,
                cull_backfaces: false,
                address: [TextureWrap::Clamp; 2],
            },
        )
    }
    #[test]
    fn addressing_is_explicit_bounded_and_default_compatible() {
        assert_eq!(TextureWrap::default(), TextureWrap::Clamp);
        assert_eq!(
            "repeat".parse::<TextureWrap>().unwrap().gpu(),
            AddressMode::Repeat
        );
        assert_eq!(
            "clamp".parse::<TextureWrap>().unwrap().gpu(),
            AddressMode::ClampToEdge
        );
        for bad in ["auto", "mirror", "region-repeat", "0", ""] {
            assert!(bad.parse::<TextureWrap>().is_err());
        }
        let modes = [TextureWrap::Clamp, TextureWrap::Repeat];
        let keys: std::collections::BTreeSet<_> = modes
            .into_iter()
            .flat_map(|u| modes.map(move |v| [u, v]))
            .collect();
        assert_eq!(keys.len(), 4);
    }
    #[test]
    fn rejects_malformed_and_nonfinite_inputs_before_gpu_use() {
        let (texture, draw) = valid();
        assert!(
            validate_inputs(
                [64, 1024],
                std::slice::from_ref(&texture),
                std::slice::from_ref(&draw)
            )
            .is_ok()
        );
        for size in [[0, 64], [63, 64], [64, 1025]] {
            assert!(
                validate_inputs(
                    size,
                    std::slice::from_ref(&texture),
                    std::slice::from_ref(&draw)
                )
                .is_err()
            );
        }
        for mut t in [texture.clone(), texture.clone(), texture.clone()]
            .into_iter()
            .enumerate()
        {
            match t.0 {
                0 => t.1.width = 0,
                1 => t.1.width = 4097,
                _ => t.1.rgba.pop().map(|_| ()).unwrap(),
            }
            assert!(validate_inputs([64, 64], &[t.1], std::slice::from_ref(&draw)).is_err());
        }
        for kind in 0..7 {
            let mut d = draw.clone();
            match kind {
                0 => d.texture = 1,
                1 => {
                    d.vertices.pop();
                }
                2 => d.vertices[0].clip[3] = f32::NAN,
                3 => d.vertices[0].clip[0] = f32::NAN,
                4 => d.vertices[0].uv[0] = f32::INFINITY,
                5 => d.vertices[0].color[3] = -1.,
                _ => d.vertices[0].color[3] = 256.,
            }
            assert!(validate_inputs([64, 64], std::slice::from_ref(&texture), &[d]).is_err());
        }
        assert!(
            validate_inputs([64, 64], std::slice::from_ref(&texture), &vec![draw; 4097]).is_err()
        );
    }
    #[test]
    fn serialized_vertex_has_explicit_little_endian_stride() {
        let (_, mut draw) = valid();
        draw.vertices[0] = AlphaVertex {
            clip: [1., 2., 3., 4.],
            uv: [5., 6.],
            color: [7., 8., 9., 10.],
        };
        let bytes = vertex_bytes(&draw.vertices);
        assert_eq!(bytes.len(), 3 * 64);
        let values: Vec<_> = bytes[..40]
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(values, [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
        let flat: Vec<_> = bytes[40..56]
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(flat, [7., 8., 9., 10.]);
        assert_eq!(u32::from_le_bytes(bytes[56..60].try_into().unwrap()), 0);
        assert_eq!(&bytes[60..64], &[0; 4]);
    }
    #[test]
    fn constant_masks_are_per_triangle_and_keep_subthreshold_floats() {
        let (_, mut draw) = valid();
        draw.vertices[0].color = [10., 20., 30., 128.];
        draw.vertices[1].color = [11., 21., 31., 128.];
        draw.vertices[2].color = [12., 22., 32., 128.];
        let below = f32::from_bits(128f32.to_bits() - 1);
        let mut second = draw.vertices.clone();
        for vertex in &mut second {
            vertex.color = [7.5, 8.25, 9.75, below];
        }
        draw.vertices.extend(second);
        let bytes = vertex_bytes(&draw.vertices);
        for i in 0..6 {
            let at = i * 64;
            assert_eq!(
                u32::from_le_bytes(bytes[at + 56..at + 60].try_into().unwrap()),
                if i < 3 { 8 } else { 15 }
            );
            assert_eq!(
                f32::from_le_bytes(bytes[at + 52..at + 56].try_into().unwrap()),
                if i < 3 { 128. } else { below }
            );
        }
    }
    #[test]
    fn comparison_uniform_codes_are_explicit_and_unique() {
        use AlphaComparison::*;
        let codes: Vec<_> = [
            Never,
            Always,
            Less,
            LessEqual,
            Equal,
            GreaterEqual,
            Greater,
            NotEqual,
        ]
        .into_iter()
        .map(comparison_code)
        .collect();
        assert_eq!(codes, [0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
