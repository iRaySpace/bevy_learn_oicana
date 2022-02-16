use bevy::prelude::*;
use bevy_learn_oicana::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Bevy Learn Oicana".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
