use bevy::{
    prelude::{
        Commands,
        Component,
        Query,
        Entity,
        Handle,
        Res, ResMut, Assets, AssetServer,
        Transform,
        Vec3, Quat
    },
    render::{
        render_resource::{
            AsBindGroup,
            ShaderType, OwnedBindingResource, encase
        },
        extract_resource::ExtractResource,
        Extract, 
        mesh::Mesh,
        renderer::RenderQueue
    },
    reflect::TypeUuid,
    sprite::{Material2d, RenderMaterials2d, MaterialMesh2dBundle, SpriteBundle},
    time::Time, window::WindowDescriptor
};

use crate::{types::Directions, consts::TARGET_POSITION};

// Resources to Extract for use in shader
pub struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(source: &Self::Source) -> Self {
        ExtractedTime { seconds_since_startup: source.seconds_since_startup() as f32 }
    }
}

#[derive(Component)]
pub struct TargetArrowSparkle {
    direction: Directions
}

#[derive(Component, Clone, Copy)]
pub struct TimeSinceCorrect {
    pub last_time: f32,
    pub points: f32
}

#[derive(Clone, ShaderType)]
pub struct TargetArrowSparkleData {
    time: f32,
    last_time: f32,
    points: f32,
}

#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "c9400817-b3a3-4baa-8bfa-0320b9b87b17"]
pub struct ArrowSparkMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    last_time: f32,
    #[uniform(0)]
    points: f32,
}

impl Material2d for ArrowSparkMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/target_arrows.frag".into()
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

pub fn extract_time_since_correct(
    mut commands: Commands,
    time_since_correct_query: Extract<Query<(Entity, &TimeSinceCorrect, &Handle<ArrowSparkMaterial>)>>
) {
    for (entity, time_since_correct, handle) in time_since_correct_query.iter() {
        print!("Extracted: {:?}", time_since_correct.last_time);
        commands
            .get_or_spawn(entity)
            .insert(*time_since_correct)
            .insert(handle.clone());
    }
}

pub fn prepare_arrow_sparkle_material(
    materials: Res<RenderMaterials2d<ArrowSparkMaterial>>,
    arrow_sparkle_query: Query<(&TimeSinceCorrect, &Handle<ArrowSparkMaterial>)>,
    time: Res<ExtractedTime>,
    render_queue: Res<RenderQueue>,
) {
    for (time_since_correct, handle) in arrow_sparkle_query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[0];
            println!("{:?}", &material.bindings[1].get_binding());
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer
                    .write(&TargetArrowSparkleData {
                        time: time.seconds_since_startup,
                        last_time: time_since_correct.last_time,
                        points: time_since_correct.points,
                    })
                    .unwrap();
                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}

// fn setup_arrow_sparkle(
//     mut commands: Commands,
//     mut mesh_assets: ResMut<Assets<Mesh>>,
//     mut my_material_assets: ResMut<Assets<ArrowSparkMaterial>>,
//     assets: Res<AssetServer>,
// ) {
//     use Directions::*;
//     let directions = [Up, Down, Left, Right];

//     for direction in directions.iter() {
//         let mut transform = Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), 1.));
//         transform.rotate(Quat::from_rotation_z(direction.rotation()));
//         commands.spawn_bundle(SpriteBundle {
//             texture: materials
//         })
//     }
// }
