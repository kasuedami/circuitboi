use bevy::prelude::ResMut;
use bevy_egui::{EguiContext, egui};

use crate::user_interface::OccupiedScreenSpace;

pub(in crate::user_interface) fn system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .show(egui_context.ctx_mut(), |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New").clicked() {
                    // empty editor
                }
                if ui.button("Save").clicked() {
                    // save editor state
                }
                if ui.button("Save as").clicked() {
                    // save as editor state
                }
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}