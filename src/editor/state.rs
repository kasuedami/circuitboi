use std::fs::File;

use bevy::prelude::{Resource, ResMut, info};
use bevy_egui::{EguiContext, egui::{self, Align2, Layout, Align}};

#[derive(Default, Resource)]
pub(crate) struct EditorState {
    action: EditorStateAction,
    editing_file: Option<File>,
}

#[derive(Default)]
enum EditorStateAction {
    #[default]
    None,
    Reset,
    Save,
    SaveAs,
}

impl EditorState {
    pub fn reset(&mut self) {
        self.action = EditorStateAction::Reset;
    }

    pub fn save(&mut self) {
        self.action = EditorStateAction::Save;
    }

    pub fn save_as(&mut self) {
        self.action = EditorStateAction::SaveAs;
    }
}

pub(super) fn handle_editor_state(
    editor_state: ResMut<EditorState>,
    egui_context: ResMut<EguiContext>,
) {
    match editor_state.action {
        EditorStateAction::Reset => reset(editor_state, egui_context),
        EditorStateAction::Save => save(editor_state),
        EditorStateAction::SaveAs => save_as(editor_state),
        _ => (),
    }
}

fn reset(
    mut editor_state: ResMut<EditorState>,
    mut egui_context: ResMut<EguiContext>,
) {

    egui::Window::new("Confirm reset")
        .pivot(Align2::CENTER_CENTER)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .default_size(egui::vec2(100.0, 50.0))
        .resizable(false)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Yes").clicked() {
                    editor_state.editing_file = None;
                    editor_state.action = EditorStateAction::None;
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("No").clicked() {
                        editor_state.action = EditorStateAction::None;
                    }
                });
            });
    });
}

fn save(mut editor_state: ResMut<EditorState>) {
    info!("save");
    if editor_state.editing_file.is_some() {
        // do the saving
        editor_state.action = EditorStateAction::None;
    } else {
        save_as(editor_state)
    }
}

fn save_as(mut editor_state: ResMut<EditorState>) {
    info!("save as");
    // get file from dialog

    editor_state.action = EditorStateAction::None;
}