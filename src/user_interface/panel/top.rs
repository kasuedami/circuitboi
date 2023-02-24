use bevy::prelude::ResMut;
use bevy_egui::{EguiContext, egui};

use crate::{user_interface::{OccupiedScreenSpace, popup::{PopupState, PopupWindow}}, editor::{state::EditorState}};

pub(in crate::user_interface) fn system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut editor_state: ResMut<EditorState>,
    mut popup_state: ResMut<PopupState>,
) {
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        editor_state.reset();
                    }
                    if ui.button("Save").clicked() {
                        editor_state.save();
                    }
                    if ui.button("Save as").clicked() {
                        editor_state.save_as();
                    }
                });
                ui.menu_button("Logic", |ui| {
                    if ui.button("Solve equation").clicked() {
                        popup_state.show(PopupWindow::EquationSolver);
                    }
                });
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}