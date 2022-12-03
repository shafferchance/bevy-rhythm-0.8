use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_resource::{AsBindGroup, ShaderRef},
        renderer::RenderQueue,
    },
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin},
        render_resource::*,
        Extract, RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, RenderMaterials2d, SpriteBundle },
};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CoolMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    time: f32,
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

#[derive(Clone, ShaderType)]
struct CoolMaterialUniformData {
    color: Color,
    time: f32,
}

#[derive(Component, Clone, Copy)]
struct Health {
    value: f32,
}

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self,app: &mut App) {
        app.add_plugin(Material2dPlugin::<CoolMaterial>::default())
        .add_plugin(ExtractResourcePlugin::<ExtractedTime>::default())
        .add_startup_system(setup);
        // Add all render world systems/resources
        app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, extract_health)
        .add_system_to_stage(RenderStage::Prepare, prepare_my_material);
    }
}

#[derive(Component)]
struct Rect;

fn make_rect(x: f32, y: f32, z: f32) -> SpriteBundle {
    let sprite = Sprite {
        color: Color::rgb(255.0, 255.0, 255.0),
        flip_x: false,
        flip_y: false,
        custom_size: Some(Vec2::new(35.0, 35.0)),
        anchor: Default::default(),
    };

    SpriteBundle {
        sprite: sprite,
        transform: Transform::from_xyz(0.0 + x, 0.0 + y, 0.0 + z),
        ..Default::default()
    }
}

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<CoolMaterial>>,
    assets: Res<AssetServer>,
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2 { x: 200., y: 200. }))).into(),
            material: my_material_assets.add(CoolMaterial {
                color: Color::rgb(0.0, 1.0, 0.3),
                time: 0.0,
                image: assets.load("images/awesome.png"),
            }),
            transform: Transform::from_xyz(50., 0., 15.),    
            ..default()
        })
        .insert(Health { value: 0.2 });

    // commands
    //     .spawn_bundle(MaterialMesh2dBundle {
    //         mesh: mesh_assets.add(Mesh::from(shape::Quad::new(Vec2 { x: 200., y: 200. }))).into(),
    //         material: my_material_assets.add(CoolMaterial {
    //             color: Color::rgb(0.0, 1.0, 0.3),
    //             time: 0.0,
    //             image: assets.load("images/awesome.png"),
    //         }),
    //         transform: Transform::from_xyz(-200., 0., 15.),    
    //         ..default()
    //     })
    //     .insert(Health { value: 0.8 });
}

struct ExtractedTime {
    seconds_since_startup: f32,
}

impl ExtractResource for ExtractedTime {
    type Source = Time;

    fn extract_resource(time: &Self::Source) -> Self {
        ExtractedTime {
            seconds_since_startup: time.seconds_since_startup() as f32,
        }
    }
}

fn extract_health(
    mut commands: Commands,
    health_query: Extract<Query<(Entity, &Health, &Handle<CoolMaterial>)>>,
) {
    for (entity, health, handle) in health_query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(*health)
            .insert(handle.clone());
    }
}

fn prepare_my_material(
    materials: Res<RenderMaterials2d<CoolMaterial>>,
    health_query: Query<(&Health, &Handle<CoolMaterial>)>,
    time: Res<ExtractedTime>,
    render_queue: Res<RenderQueue>,
) {
    for (health, handle) in health_query.iter() {
        if let Some(material) = materials.get(handle) {
            let binding = &material.bindings[2];
            if let OwnedBindingResource::Buffer(cur_buffer) = binding {
                let mut buffer = encase::UniformBuffer::new(Vec::new());
                buffer
                    .write(&CoolMaterialUniformData {
                        color: Color::rgb(health.value, 0.0, 0.0),
                        time: time.seconds_since_startup % 1.0,
                    })
                    .unwrap();
                render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
            }
        }
    }
}