use bevy::prelude::*;
use std::collections::{hash_map::RandomState, HashMap};
use rand::prelude::*;

use crate::map;

#[derive(Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Default)]
pub struct Tile {}

#[derive(Default)]
pub struct Chunk {
    location: Point,
}

#[derive(Default)]
pub struct Map {
}

pub fn setup_map_system(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    let texture_handle = asset_server.load("iso_tile_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let map_id = commands.spawn().id();
    commands.entity(map_id).insert(Map::default());
    commands.entity(map_id).insert(Transform::default());
    commands.entity(map_id).insert(GlobalTransform::default());

    let _chunk_id = spawn_chunk(&mut commands, map_id, &texture_atlas_handle, Vec2::new(0.0, 0.0));
    let _chunk2_id = spawn_chunk(&mut commands, map_id, &texture_atlas_handle, Vec2::new(-4.0, 0.0));
    let _chunk2_id = spawn_chunk(&mut commands, map_id, &texture_atlas_handle, Vec2::new(-8.0, 4.0));
    let _chunk2_id = spawn_chunk(&mut commands, map_id, &texture_atlas_handle, Vec2::new(-12.0, 8.0));

}

fn spawn_chunk(commands: &mut Commands, map_id: Entity, texture_atlas_handle: &Handle<TextureAtlas>, offset: Vec2) -> Entity {
    let chunk_id = commands.spawn().id();
    commands.entity(chunk_id).insert(Transform::default());
    commands.entity(chunk_id).insert(GlobalTransform::default());

    let mut rng = rand::thread_rng();

    let mut tiles = vec![];

    for y in (-2..2).rev() {
        for x in (-2..2).rev() {
            tiles.push(spawn_tile(commands, chunk_id, &texture_atlas_handle, 
                Vec3::new(x as f32 + offset.x, y as f32 + offset.y, rng.gen_range(0.0..5.0))));
        }
    }
    //commands.entity(chunk_id).push_children(&tiles);
    commands.entity(map_id).push_children(&[chunk_id]);

    chunk_id
}

fn spawn_tile(commands: &mut Commands, chunk_id: Entity, texture_atlas_handle: &Handle<TextureAtlas>, pos: Vec3) -> Entity {
    let tile = commands.spawn().id();
    commands.entity(tile).insert_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            color: Color::GREEN,
            index: 0,
            ..Default::default()
        },
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(iso_coords(pos.x as i32, pos.y as i32, pos.z)),
        ..Default::default()
    });
    commands.entity(chunk_id).push_children(&[tile]);
    tile
}

fn iso_coords(x: i32, y: i32, z: f32) -> Vec3 {
    let screen_x = (x - y) as f32 * (50.0 / 2.0);
    let screen_y = (x + y) as f32 * (24.0 / 2.0) + z;

    Vec3::new(screen_x, screen_y, 0.0)
}