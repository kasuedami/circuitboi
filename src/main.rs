use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod editor;
pub mod logic;
mod user_interface;

#[cfg(test)]
mod tests;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(user_interface::UiPlugin::all())
        .add_plugin(editor::EditorPlugin)
        .run();
}