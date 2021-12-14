use bevy::{prelude::*};

mod camera;
mod map;

use camera::*;
use map::MapPlugin;


fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 });
    app.insert_resource(MouseDownPos::new());
    app.add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_plugin(MapPlugin {});
    app.add_system(save_start_pos.system());
    app.add_system(camera_mouse_system.system());
    app.add_startup_system(setup.system()).run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}