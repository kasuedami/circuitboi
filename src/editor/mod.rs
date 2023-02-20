use bevy::prelude::{Plugin, App};

use self::state::{EditorState, handle_editor_state};

pub mod state;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorState>()
            .add_system(handle_editor_state);
    }
}