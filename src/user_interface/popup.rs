use bevy::prelude::{Plugin, Resource, Res, ResMut};
use bevy_egui::{EguiContext, egui::{self, Align2}};

pub struct PopupPlugin;

impl Plugin for PopupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PopupState>()
            .add_system(popup_system);
    }
}

#[derive(Default, Resource)]
pub struct PopupState {
    show: Option<PopupWindow>,
}

impl PopupState {
    pub fn show(&mut self, popup: PopupWindow) {
        if self.show.is_none() {
            self.show = Some(popup);
        }
    }
}

pub enum PopupWindow {
    EquationSolver,
}

fn popup_system(
    popup_state: Res<PopupState>,
    egui_context: ResMut<EguiContext>,
) {
    if let Some(popup) = &popup_state.show {
        match popup {
            PopupWindow::EquationSolver => equation_solver(egui_context),
        }
    }
}

fn equation_solver(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Confirm reset")
        .pivot(Align2::CENTER_CENTER)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(false)
        .collapsible(false)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.input();
                ui.button("Solve");
            });

            ui.output();
    });
}