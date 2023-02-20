use std::fs::File;

use bevy::prelude::{Resource, ResMut, info};

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

pub(super) fn handle_editor_state(editor_state: ResMut<EditorState>) {
    match editor_state.action {
        EditorStateAction::Reset => reset(editor_state),
        EditorStateAction::Save => save(editor_state),
        EditorStateAction::SaveAs => save_as(editor_state),
        _ => (),
    }
}

fn reset(mut editor_state: ResMut<EditorState>) {
    editor_state.action = EditorStateAction::None;
    editor_state.editing_file = None;
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