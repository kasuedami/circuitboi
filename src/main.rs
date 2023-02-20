use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod user_interface;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(user_interface::UiPlugin::all())
        .run();
}