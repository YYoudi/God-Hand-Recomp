//! Original caller-ordered GPU alpha-stage adapter. No retail state inference.
//! Contract: specs/ordered-alpha-gpu-v1.md. RGB overwrite only (no blending yet).
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};
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

#[derive(Clone, Debug)]
pub struct AlphaDraw {
    pub vertices: Vec<AlphaVertex>,
    pub texture: usize,
    pub stage: AlphaStage,
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
    let mut bytes = Vec::with_capacity(vertices.len() * 40);
    for vertex in vertices {
        for value in vertex.clip.iter().chain(&vertex.uv).chain(&vertex.color) {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
    }
    bytes
}

struct PreparedDraw {
    vertices: Buffer,
    count: u32,
    branches: Vec<([bool; 3], BindGroup)>,
}

pub struct OrderedAlphaGpu {
    shader: ShaderModule,
    layout: BindGroupLayout,
    sampler: Sampler,
    pipelines: BTreeMap<[bool; 3], RenderPipeline>,
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
                        min_binding_size: BufferSize::new(16),
                    },
                    count: None,
                },
            ],
        );
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("raw alpha nearest clamp"),
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            ..Default::default()
        });
        Self {
            shader,
            layout,
            sampler,
            pipelines: BTreeMap::new(),
        }
    }

    fn pipeline(&mut self, device: &RenderDevice, writes: WritePermissions) -> [bool; 3] {
        let key = [writes.rgb, writes.framebuffer_alpha, writes.depth];
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
        ];
        let pipeline = device.create_render_pipeline(&RawRenderPipelineDescriptor {
            label: Some("ordered alpha permissions"),
            layout: Some(&layout),
            vertex: RawVertexState {
                module: &self.shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[Some(RawVertexBufferLayout {
                    array_stride: 40,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &attributes,
                })],
            },
            primitive: PrimitiveState {
                cull_mode: None,
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
                let key = self.pipeline(device, permissions);
                let params: Vec<u8> = [
                    u32::from(draw.stage.enabled),
                    comparison_code(draw.stage.comparison),
                    u32::from(draw.stage.reference),
                    u32::from(!passed),
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
                            resource: BindingResource::Sampler(&self.sampler),
                        },
                        BindGroupEntry {
                            binding: 2,
                            resource: uniform.as_entire_binding(),
                        },
                    ],
                );
                branches.push((key, group));
            }
            prepared.push(PreparedDraw {
                vertices,
                count: draw.vertices.len() as u32,
                branches,
            });
        }
        let color = frame.color.create_view(&Default::default());
        let alpha = frame.framebuffer_alpha.create_view(&Default::default());
        let depth = frame.depth.create_view(&Default::default());
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("ordered alpha commands"),
        });
        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("caller-ordered alpha-stage draw stream"),
                color_attachments: &[
                    Some(RenderPassColorAttachment {
                        view: &color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(
                                bevy::color::LinearRgba::new(
                                    f32::from(clear[0]) / 255.,
                                    f32::from(clear[1]) / 255.,
                                    f32::from(clear[2]) / 255.,
                                    1.,
                                )
                                .into(),
                            ),
                            store: StoreOp::Store,
                        },
                    }),
                    Some(RenderPassColorAttachment {
                        view: &alpha,
                        depth_slice: None,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(
                                bevy::color::LinearRgba::new(
                                    f32::from(clear[3]) / 255.,
                                    0.,
                                    0.,
                                    1.,
                                )
                                .into(),
                            ),
                            store: StoreOp::Store,
                        },
                    }),
                ],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(0.),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            for draw in &prepared {
                pass.set_vertex_buffer(0, *draw.vertices.slice(..));
                // Preserve primitive order even when overlapping triangles have
                // different alpha outcomes. Whole-draw branch batching is not equivalent.
                for first in (0..draw.count).step_by(3) {
                    for (key, group) in &draw.branches {
                        pass.set_pipeline(&self.pipelines[key]);
                        pass.set_bind_group(0, &**group, &[]);
                        pass.draw(first..first + 3, 0..1);
                    }
                }
            }
        }
        queue.submit([encoder.finish()]);
        Ok(frame)
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
            },
        )
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
        let bytes = vertex_bytes(&draw.vertices[..1]);
        assert_eq!(bytes.len(), 40);
        let values: Vec<_> = bytes
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(values, [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
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
