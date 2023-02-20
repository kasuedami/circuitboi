use bevy::prelude::ResMut;
use bevy_egui::{EguiContext, egui};

use crate::user_interface::OccupiedScreenSpace;

pub(in crate::user_interface) fn system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Elements");
            ui.separator();
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}