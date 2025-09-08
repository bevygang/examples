mod menus;

use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use menus::setup_menus;

pub trait GameState: States+Copy+FromWorld+FreelyMutableState {}

impl<T> GameState for T where T:States+Copy+FromWorld+FreelyMutableState {}

pub struct GameStatePlugin<T> where T: GameState {
    menu_state: T,
    start_state: T,
    end_state: T,
}

impl<T> GameStatePlugin<T> where T: GameState {
    #[allow(clippy::new_without_default)]
    pub fn new(menu_state: T, start_state: T, end_state: T) -> Self {
        Self { menu_state, start_state, end_state }
    }
}

impl<T> Plugin for GameStatePlugin<T> where T: GameState {
    fn build(&self, app: &mut App) {
        app.init_state::<T>();
        setup_menus(app, self);
    }
}

pub fn cleanup<T>(query: Query<Entity, With<T>>, mut commands: Commands) where T: Component {
    query.iter().for_each(|entity| commands.entity(entity).despawn())
}

