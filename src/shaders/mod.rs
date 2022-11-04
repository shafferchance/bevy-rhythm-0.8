mod background;
mod material;

use bevy::{prelude::{App, Plugin}};

use self::background::BackgroundMaterialPlugin;

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BackgroundMaterialPlugin);
    }
}

