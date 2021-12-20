use bevy::prelude::*;

use crate::{map::Chunk};

pub(crate) fn click_system(
    mouse_button: Res<Input<MouseButton>>,
    q: Query<(&mut Transform, &Chunk)> )
{
    if mouse_button.just_pressed(MouseButton::Left) {
        let () = q.for_each_mut(|(_t, _)| {
            //t.translation.y += 0.5;
            //println!("{:?}", t.translation);
        });
    }
}