use super::{GameStatePlugin,GameState,cleanup,loading};
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
struct MenuElement;

#[derive(Resource)]
pub(crate) struct MenuResource<T> where T: GameState {
    pub(crate) menu_state: T,
    pub(crate) start_state: T,
    pub(crate) end_state: T,
}

pub fn setup_menus<T>(app: &mut App, state_plugin: &GameStatePlugin<T>) where T: GameState {
    let menu_resource = MenuResource {
        menu_state: state_plugin.menu_state,
        start_state: state_plugin.start_state,
        end_state: state_plugin.end_state,
    };

    app.add_plugins(bevy_egui::EguiPlugin { enable_multipass_for_primary_context: false });
    app.insert_resource(menu_resource);

    app.add_systems(OnEnter(state_plugin.menu_state), setup::<T>);
    app.add_systems(Update, run::<T>.run_if(in_state(state_plugin.menu_state)));
    app.add_systems(OnExit(state_plugin.menu_state), cleanup::<MenuElement>);

    app.add_systems(OnEnter(state_plugin.end_state), setup::<T>);
    app.add_systems(Update, run::<T>.run_if(in_state(state_plugin.end_state)));
    app.add_systems(OnExit(state_plugin.end_state), cleanup::<MenuElement>);

    app.add_systems(OnEnter(T::default()), loading::setup);
    app.add_systems(Update, loading::run::<T>.run_if(in_state(T::default())));
    app.add_systems(OnExit(T::default()), loading::exit);
}

fn setup<T>(
    state: Res<State<T>>,
    mut commands: Commands,
    menu_resource: Res<MenuResource<T>>,
    loaded_assets: crate::AssetResource,
    assets: Res<crate::AssetStore>
) where T: GameState {
    let current_state = state.get();
    let menu_graphic = {
        if menu_resource.menu_state == *current_state {
            assets.get_handle("main_menu", &loaded_assets).unwrap()
        } else if menu_resource.end_state == *current_state {
            assets.get_handle("game_over", &loaded_assets).unwrap()
        } else {
            panic!("Unknown menu state");
        }
    };

    commands.spawn((Camera2d, MenuElement));
    commands.spawn((
        Sprite { image: menu_graphic, ..default() },
        Transform::from_xyz(0.0,0.0,1.0),
        MenuElement
    ));
}

fn run<T>(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    current_state: Res<State<T>>,
    mut next_state: ResMut<NextState<T>>,
    menu_state: Res<MenuResource<T>>,
) where T: GameState {
    let current_state = current_state.get();
    if menu_state.menu_state == *current_state {
        if keyboard.just_pressed(KeyCode::KeyP) {
            next_state.set(menu_state.start_state);
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.write(AppExit::Success);
        }
    } else if menu_state.end_state == *current_state {
        if keyboard.just_pressed(KeyCode::KeyM) {
            next_state.set(menu_state.menu_state);
        } else if keyboard.just_pressed(KeyCode::KeyQ) {
            exit.write(AppExit::Success);
        }
    }
}
