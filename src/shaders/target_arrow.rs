use bevy::{
    app::Plugin,
    prelude::{
        App,
        Commands,
        Component,
        Query,
        Entity,
        Handle,
        Mesh,
        Res, ResMut, Assets,
        shape::Quad,
        Transform,
        Vec3,
    },
    render::{
        render_resource::{
            AsBindGroup,
            ShaderType, OwnedBindingResource, encase
        },
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        Extract,
        renderer::RenderQueue, RenderApp, RenderStage
    },
    reflect::TypeUuid,
    sprite::{Material2d, RenderMaterials2d, MaterialMesh2dBundle, Material2dPlugin},
    time::Time
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

#[derive(Component, Debug, Clone, Copy)]
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
pub struct ArrowSparkleMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(0)]
    last_time: f32,
    #[uniform(0)]
    points: f32,
}

impl Material2d for ArrowSparkleMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/target_arrows.frag".into()
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/target_arrows.vert".into()
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
    arrow_sparkle_material_query: Extract<Query<(Entity, &TimeSinceCorrect, &Handle<ArrowSparkleMaterial>)>>
) {
    for (entity, time_since_correct, handle) in arrow_sparkle_material_query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(*time_since_correct)
            .insert(handle.clone());
    }
}

pub fn prepare_arrow_sparkle_material(
    materials: Res<RenderMaterials2d<ArrowSparkleMaterial>>,
    arrow_sparkle_query: Query<(&TimeSinceCorrect, &Handle<ArrowSparkleMaterial>)>,
    time: Res<ExtractedTime>,
    render_queue: Res<RenderQueue>,
) {
    for (time_since_correct, handle) in arrow_sparkle_query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[0];
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

fn setup_target_arrows_sparkle(
    mut commands: Commands,
    mut my_material_assets: ResMut<Assets<ArrowSparkleMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    use Directions::*;
    let directions = [Up, Down, Left, Right];

    for direction in directions.iter() {
        let z = match direction {
            Up => 0.3,
            Down => 0.4,
            Left => 0.5,
            Right => 0.6,
        };

        let mut transform = Transform::from_translation(Vec3::new(TARGET_POSITION, direction.y(), z));
        transform.scale = Vec3::new(300., 300., 1.);
        commands.spawn_bundle(MaterialMesh2dBundle {
            material: my_material_assets.add(ArrowSparkleMaterial { time: 0., last_time: 1., points: 0.5 }),
            mesh: mesh_assets.add(Mesh::from(Quad::default())).into(),
            transform,
            ..Default::default()
        })
        .insert(TimeSinceCorrect {
            last_time: 3.,
            points: 0.5
        });
    }
}

pub struct ArrowSparkleMaterialPlugin;
impl Plugin for ArrowSparkleMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<ArrowSparkleMaterial>::default())
           .add_plugin(ExtractResourcePlugin::<ExtractedTime>::default())
           .add_startup_system(setup_target_arrows_sparkle);
        app.sub_app_mut(RenderApp)
           .add_system_to_stage(RenderStage::Extract, extract_time_since_correct)
           .add_system_to_stage(RenderStage::Prepare, prepare_arrow_sparkle_material);
    }
}
