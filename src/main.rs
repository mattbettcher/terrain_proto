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
    let map = Map::new(texture_atlas_handle, 2, Vec2::new(50.0, 24.0));
    map.spawn_tiles(&mut commands);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

struct Map {
    atlas: Handle<TextureAtlas>,
    heightmap: Vec<u32>,
    tile_columns: usize,
    tile_size: Vec2,
}

impl Map {
    fn new(atlas: Handle<TextureAtlas>, map_size: u32, tile_size: Vec2) -> Self {
        let tile_columns = 2usize.pow(map_size) + 1;
        let heightmap = vec![0; tile_columns * tile_columns];
        Self {
            atlas,
            heightmap,
            tile_columns,
            tile_size,
        }
    }

    fn spawn_tiles(&self, commands: &mut Commands) {
        let tile_sprites: Vec<SpriteSheetBundle> = self
            .heightmap
            .iter()
            .enumerate()
            .map(|(index, &height)| SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::GREEN,
                    index: height,
                    ..Default::default()
                },
                texture_atlas: self.atlas.clone(),
                transform: Transform::from_translation(self.iso_coords(index)),
                ..Default::default()
            })
            .collect();
        commands.spawn_batch(tile_sprites);
    }

    fn iso_coords(&self, tile_index: usize) -> Vec3 {
        let map_x = tile_index % self.tile_columns;
        let map_y = tile_index / self.tile_columns;
        let draw_order = map_x + map_y;

        let cart_x = map_x as f32 - (self.tile_columns as f32 / 2.0);
        let cart_y = (self.tile_columns as f32 / 2.0) - map_y as f32;

        let iso_x = (cart_x + cart_y) / -2.0;
        let iso_y = (cart_x - cart_y) / -2.0;

        Vec3::new(iso_x * self.tile_size.x, iso_y * self.tile_size.y, draw_order as f32)
    }
}