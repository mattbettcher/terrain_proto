use bevy::{prelude::*, render::camera::Camera};

use crate::map::{MapPlugin, Map};

#[derive(Default)]
pub struct MouseDownPos {
    pub mouse_pos_start: Vec2,
    pub map_pos_start: Vec3,
    pub saved: bool,
}

impl MouseDownPos {
    pub fn new() -> Self {
        MouseDownPos {
            mouse_pos_start: Vec2::ZERO,
            map_pos_start: Vec3::ZERO,
            saved: false,
        }
    }
}

pub(crate) fn camera_mouse_system(
    mouse_button: Res<Input<MouseButton>>, 
    windows: Res<Windows>,
    start_pos: ResMut<MouseDownPos>,
    mut q: Query<&mut Transform, With<Camera>> )
{
    if let Ok(mut transform) = q.single_mut() {
        let translation = &mut transform.translation;

        if mouse_button.pressed(MouseButton::Right) {
            let _cursor_position = if let Some(cursor_position) = windows
                .get_primary()
                .and_then(|window| window.cursor_position())
            {
                translation.x = start_pos.map_pos_start.x - (cursor_position.x - start_pos.mouse_pos_start.x);
                translation.y = start_pos.map_pos_start.y - (cursor_position.y - start_pos.mouse_pos_start.y);
            };
        }
    }
}

pub(crate) fn save_start_pos(
    mut start_pos: ResMut<MouseDownPos>,
    mouse_button: Res<Input<MouseButton>>, 
    windows: Res<Windows>,
    q: Query<(&Transform, &Map)>) 
{
    if mouse_button.just_released(MouseButton::Right) {
        start_pos.saved = false;
    }

    if !start_pos.saved {
        if mouse_button.pressed(MouseButton::Right) {
            start_pos.saved = true;
            let _cursor_position = if let Some(cursor_position) = windows
                .get_primary()
                .and_then(|window| window.cursor_position())
            {
                start_pos.mouse_pos_start = cursor_position;
                start_pos.map_pos_start = q.single().unwrap().0.translation;
            };
        }
    } 
}