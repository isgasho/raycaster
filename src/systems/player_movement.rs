use quicksilver::prelude::*;
use specs::prelude::*;

use crate::components::player::Player;
use crate::components::pose::Pose;
use crate::components::solid::Solid;
use crate::config::{PLAYER_RADIUS, STRAFE_SPEED, TURN_SPEED, WALK_SPEED};
use crate::resources::input::{Binding, Input};
use crate::resources::room::Room;

pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Solid>,
        WriteStorage<'a, Pose>,
        Read<'a, Input>,
        Read<'a, Room>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (players, solids, mut poses, input, room) = data;

        if let Some((_, player_pose)) = (&players, &mut poses).join().next() {
            handle_rotation(player_pose, &input);
        }

        let solids: Vec<Vector> = (&solids, &poses)
            .join()
            .map(|(_, pose)| pose.position)
            .collect();

        let move_to = (&players, &poses)
            .join()
            .next()
            .and_then(|(_, player_pose)| handle_movement(player_pose, &input, &room, &solids));

        if let Some(to) = move_to {
            if let Some((_, player_pose)) = (&players, &mut poses).join().next() {
                player_pose.position = to;
            }
        }
    }
}

fn handle_rotation(player_pose: &mut Pose, input: &Input) {
    if input.is_down(Binding::TurnLeft) {
        player_pose.turn_left(TURN_SPEED);
    } else if input.is_down(Binding::TurnRight) {
        player_pose.turn_right(TURN_SPEED);
    }
}

fn handle_movement(
    player_pose: &Pose,
    input: &Input,
    room: &Room,
    _solids: &[Vector],
) -> Option<Vector> {
    let dx = if input.is_down(Binding::MoveForward) {
        player_pose.move_forward(WALK_SPEED)
    } else if input.is_down(Binding::MoveBack) {
        player_pose.move_back(WALK_SPEED)
    } else {
        Vector::ZERO
    };

    let dy = if input.is_down(Binding::StrafeLeft) {
        player_pose.strafe_left(STRAFE_SPEED)
    } else if input.is_down(Binding::StrafeRight) {
        player_pose.strafe_right(STRAFE_SPEED)
    } else {
        Vector::ZERO
    };

    let dt = dx + dy;

    // Did the player move?
    if dt == Vector::ZERO {
        return None;
    }

    let from = player_pose.position;
    let mut to = from + dt;

    // Disallow out of bounds
    let width = room.width() as f32;
    let height = room.height() as f32;
    if to.x < 0. || to.y < 0. || to.x >= width || to.y >= height {
        return None;
    }

    // Anticipate collision with player radius
    let to_buf =
        Transform::translate(to) * Transform::rotate(dt.angle()) * Vector::new(PLAYER_RADIUS, 0);

    // Rollback x or y on collision with walls
    if room.get_tile_xy(to_buf.x as u32, from.y as u32) != 0 {
        to.x = from.x;
    }
    if room.get_tile_xy(from.x as u32, to_buf.y as u32) != 0 {
        to.y = from.y;
    }

    Some(to)
}
