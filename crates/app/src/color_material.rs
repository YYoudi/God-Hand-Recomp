//! Explicit source-color experiment; not inferred per-draw game material state.
use bevy::{
    asset::{AssetPath, embedded_asset, embedded_path},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SourceColorMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
}

fn shader() -> ShaderRef {
    AssetPath::from_path_buf(embedded_path!("source_color.wesl"))
        .with_source("embedded")
        .into()
}

impl Material for SourceColorMaterial {
    fn vertex_shader() -> ShaderRef {
        shader()
    }
    fn fragment_shader() -> ShaderRef {
        shader()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

pub struct SourceColorPlugin;
impl Plugin for SourceColorPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "source_color.wesl");
        app.add_plugins(MaterialPlugin::<SourceColorMaterial>::default());
    }
}
