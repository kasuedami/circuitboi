use std::fs::File;

use bevy::{prelude::{Resource, ResMut, info, Res}, window::Windows};
use bevy_egui::{EguiContext, egui::{self, Align2}};

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
    windows: Res<Windows>,
) {
    match editor_state.action {
        EditorStateAction::Reset => reset(editor_state, egui_context, windows),
        EditorStateAction::Save => save(editor_state),
        EditorStateAction::SaveAs => save_as(editor_state),
        _ => (),
    }
}

fn reset(
    mut editor_state: ResMut<EditorState>,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
) {

    let center_width = windows.get_primary().unwrap().width() / 2.0;
    let center_height = windows.get_primary().unwrap().height() / 2.0;

    egui::Window::new("Confirm reset")
        .pivot(Align2::CENTER_CENTER)
        .fixed_pos(egui::pos2(center_width, center_height))
        .show(egui_context.ctx_mut(), |ui| {
            if ui.button("Yes").clicked() {
                editor_state.editing_file = None;
                editor_state.action = EditorStateAction::None;
            }
            if ui.button("No").clicked() {
                editor_state.action = EditorStateAction::None;
            }
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