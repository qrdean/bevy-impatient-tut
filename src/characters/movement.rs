use crate::characters::animation::*;
use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::prelude::*;


fn calculate_movement_speed(character: &CharacterEntry, is_running: bool) -> f32 {
    if is_running {
        character.base_move_speed * character.run_speed_multiplier
    } else {
        character.base_move_speed
    }
}

// FIXME: remove 
#[derive(Component)]
pub struct Player;

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        &mut AnimationController,
        &mut AnimationState,
        &CharacterEntry,
    )>,
) {
    let Ok((mut transform, mut animated, mut state, character)) = query.single_mut() else {
        return;
    };

    let direction = read_movement_input(&input);
    if input.just_pressed(KeyCode::Space) {
        state.is_jumping = true;
        animated.current_animation = AnimationType::Jump;
    }

    let is_running = input.pressed(KeyCode::ShiftLeft);

    if direction != Vec2::ZERO {
        let move_speed = calculate_movement_speed(character, is_running);
        let delta = direction.normalize() * move_speed * time.delta_secs();
        transform.translation += delta.extend(0.);

        animated.facing = Facing::from_direction(direction);

        if !state.is_jumping {
            state.is_moving = true;
            animated.current_animation = if is_running {
                AnimationType::Run
            } else {
                AnimationType::Walk
            };
        }
    } else if !state.is_jumping {
        state.is_moving = false;
        animated.current_animation = AnimationType::Walk;
    }
}

pub fn update_jump_state(
    mut query: Query<(
        &mut AnimationController,
        &mut AnimationState,
        &AnimationTimer,
        &Sprite,
        &CharacterEntry,
    )>,
) {
    for (mut animated, mut state, timer, sprite, character) in query.iter_mut() {
        if !state.is_jumping {
            continue;
        }

        let Some(atlas) = sprite.texture_atlas.as_ref() else {
            continue;
        };

        let Some(clip) = animated.get_clip(character) else {
            continue;
        };

        if clip.is_complete(atlas.index, timer.just_finished()) {
            state.is_jumping = false;
            animated.current_animation = AnimationType::Walk;
        }
    }
}
