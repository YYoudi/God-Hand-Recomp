//! Explicit source-color experiment; not inferred per-draw game material state.
use bevy::{
    asset::{AssetPath, embedded_asset, embedded_path},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::render_resource::{
        AsBindGroup, Face, RenderPipelineDescriptor, SpecializedMeshPipelineError,
    },
    shader::ShaderRef,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
#[bind_group_data(SourceColorKey)]
pub struct SourceColorMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    pub backface_culling: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SourceColorKey {
    backface_culling: bool,
}
impl From<&SourceColorMaterial> for SourceColorKey {
    fn from(material: &SourceColorMaterial) -> Self {
        Self {
            backface_culling: material.backface_culling,
        }
    }
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
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.bind_group_data.backface_culling.then_some(Face::Back);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn culling_is_in_pipeline_key_without_changing_shader_bindings() {
        let texture = Handle::default();
        let off = SourceColorMaterial {
            texture,
            backface_culling: false,
        };
        let on = SourceColorMaterial {
            backface_culling: true,
            ..off.clone()
        };
        let off_key = SourceColorKey::from(&off);
        let on_key = SourceColorKey::from(&on);
        assert_ne!(off_key, on_key);
        assert!(!off_key.backface_culling);
        assert!(on_key.backface_culling);
        assert_eq!(off.texture, on.texture);
    }
}
