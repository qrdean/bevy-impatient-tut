pub mod game_state;
pub mod loading;
pub mod pause;

use crate::characters::config::CharacterList;
use crate::characters::spawn::{CharactersListResource, initialize_player_character};
use bevy::prelude::*;

pub use game_state::GameState;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), loading::spawn_loading_screen)
            .add_systems(
                Update,
                (check_asset_loading, loading::animate_loading)
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                OnExit(GameState::Loading),
                (loading::despawn_loading_screen, initialize_player_character),
            )
            .add_systems(OnEnter(GameState::Paused), pause::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), pause::despawn_pause_menu)
            .add_systems(
                Update,
                toggle_pause.run_if(in_state(GameState::Playing).or(in_state(GameState::Paused))),
            );
    }
}

fn check_asset_loading(
    characters_list_res: Option<Res<CharactersListResource>>,
    characters_lists: Res<Assets<CharacterList>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(res) = characters_list_res else {
        return;
    };

    if characters_lists.get(&res.handle).is_some() {
        info!("assets loaded, transition to playing!");
        next_state.set(GameState::Playing);
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                info!("Game Paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                info!("Game resumed");
                next_state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}
