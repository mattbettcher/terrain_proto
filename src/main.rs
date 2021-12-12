use bevy::{prelude::*, render::camera::Camera};

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.add_system(move_camera.system());
    app.add_startup_system(setup.system()).run();
}

const MAP_ORIGIN_X: f32 = 0.0;
const MAP_ORIGIN_Y: f32 = 0.0;
const TILE_SIZE_X: f32 = 50.0;
const TILE_SIZE_Y: f32 = 24.0;

fn move_camera(mut query: Query<(&Camera, &mut Transform)>)
{
    if let Ok((_cam, mut transform)) = query.single_mut() {
        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += 0.1;
        translation.y += 0.1;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("iso_tile_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    // create a layer of tiles
    for y in (0..32).rev() {
        for x in (0..30).rev() {
            
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::GREEN,
                    index: ((x as f32).sin() * 10.0 + (y as f32).cos() * 10.0) as u32 % 3,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::from((to_screen(x as f32, y as f32), 0.0))),
                texture_atlas: texture_atlas_handle.clone(),
                ..Default::default()
            });
        }
    }
}

fn to_screen(x: f32, y: f32) -> Vec2 {
    Vec2::new(MAP_ORIGIN_X * TILE_SIZE_X + (x - y) * TILE_SIZE_X / 2.0,
              MAP_ORIGIN_Y * TILE_SIZE_Y + (x + y) * TILE_SIZE_Y / 2.0)
}