use bevy::{
    prelude::*,
    reflect::TypeUuid,
    window::WindowResized,
    render::{
        render_resource::{AsBindGroup, ShaderType, OwnedBindingResource, encase }, extract_resource::{ExtractResource, ExtractResourcePlugin}, renderer::{RenderQueue}, RenderApp, RenderStage,
    }, sprite::{MaterialMesh2dBundle, Material2d, RenderMaterials2d, Material2dPlugin}
};

use super::target_arrow::{prepare_arrow_sparkle_material, extract_time_since_correct, TimeSinceCorrect};



#[derive(Component)]
pub struct Background;

#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid="4ee9c363-1124-4113-890e-199d81b00281"]
pub struct BackgroundMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    thing: f32
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/background.frag".into()
    }
    
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/background.vert".into()
    }

    fn specialize(
            descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
            _layout: &bevy::render::mesh::MeshVertexBufferLayout,
            _key: bevy::sprite::Material2dKey<Self>,
        ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

#[derive(Clone, ShaderType)]
struct BackgroundMaterialUniformData {
    time: f32,
}

pub struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedTime { seconds_since_startup: time.seconds_since_startup() as f32 }
    }
}

pub fn prepare_background_material(
    materials: Res<RenderMaterials2d<BackgroundMaterial>>,
    material_query: Query<&Handle<BackgroundMaterial>>,
    time: Res<ExtractedTime>,
    render_queue: Res<RenderQueue>,
) {
    for handle in material_query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[0];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding { 
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer
                    .write(&BackgroundMaterialUniformData {
                        time: time.seconds_since_startup,
                    })
                    .unwrap();
                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}

pub fn setup_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    window: Res<WindowDescriptor>,
    _asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(BackgroundMaterial {
            time: 0.0,
            thing: 0.0
        }),
        transform: Transform::from_scale(Vec3::new(window.width + 10., window.height + 10., 1.)),
        ..Default::default()
    })
    .insert(Background);
}

pub struct BackgroundMaterialPlugin;

impl Plugin for BackgroundMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<BackgroundMaterial>::default())
           .add_plugin(ExtractResourcePlugin::<ExtractedTime>::default())
           .add_startup_system(setup_background);
        app.sub_app_mut(RenderApp)
           .add_system_to_stage(RenderStage::Extract, extract_time_since_correct)
           .add_system_to_stage(RenderStage::Prepare, prepare_background_material);
    }
}