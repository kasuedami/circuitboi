use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui_example_system)
        .run();
}

fn ui_example_system(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}