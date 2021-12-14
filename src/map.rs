use bevy::{prelude::*};

use std::collections::{hash_map::RandomState, HashMap};

#[derive(Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Default)]
pub struct MapPlugin {
    chunk_size: usize,
    tile_columns: usize,
    tile_rows: usize,
    tile_size: Vec2,
    chunks: HashMap<Point, Chunk>,
}

impl MapPlugin {
    pub fn new() -> Self {
        let s = RandomState::new();
        MapPlugin { chunk_size: 3, tile_columns: 2, tile_rows: 2, tile_size: Vec2::new(54.0, 24.0), chunks: HashMap::with_capacity_and_hasher(20, s) }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(MapPlugin::setup.system());
    }
}

impl MapPlugin {
    fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        // Todo(matt): we should be passing the asset name to the plugin or loading all of this from a json file?
        let texture_handle = asset_server.load("iso_tile_atlas.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 2, 2);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let map = Chunk::new(texture_atlas_handle.clone(), 3, Vec2::new(50.0, 24.0), Point::new(1, 1));
        map.spawn_tiles(&mut commands);
        commands.spawn().insert(Transform::default()).insert(map);

        let map2 = Chunk::new(texture_atlas_handle.clone(), 3, Vec2::new(50.0, 24.0), Point::new(-1, -1));
        map2.spawn_tiles(&mut commands);
        commands.spawn().insert(Transform::default()).insert(map2);
    }

    fn spawn_chunk() {

    }
}

pub struct Chunk {
    atlas: Handle<TextureAtlas>,
    heightmap: Vec<u32>,
    tile_columns: usize,
    tile_rows: usize,
    tile_size: Vec2,
    location: Point,
    size: u32,
}

impl Chunk {
    pub fn new(atlas: Handle<TextureAtlas>, size: u32, tile_size: Vec2, location: Point) -> Self {
        // Todo(matt): this may need to variable as the tilemap atlas may not be a power of 2
        let tile_columns = 2_usize.pow(size) + 1;
        let tile_rows = 2_usize.pow(size) + 1;
        let heightmap = vec![0; tile_columns * tile_rows];
        Self {
            atlas,
            heightmap,
            tile_columns,
            tile_rows,
            tile_size,
            location,
            size
        }
    }

    pub fn spawn_tiles(&self, commands: &mut Commands) {
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

        let chunk_offset_x = (self.location.x * self.size as i32) as f32 * self.tile_size.x;
        let chunk_offset_y = (self.location.y * self.size as i32) as f32 * self.tile_size.y;

        Vec3::new(iso_x * self.tile_size.x + chunk_offset_x, iso_y * self.tile_size.y + chunk_offset_y, draw_order as f32)
    }
}

