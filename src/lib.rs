use bevy::prelude::*;

mod loading;
mod menu;

use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Restart,
    InGame,
    Loading,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_state(AppState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin);
    }
}
