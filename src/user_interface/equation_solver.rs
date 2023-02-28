use bevy::prelude::{Plugin, Resource, ResMut, Res, SystemSet};
use bevy_egui::{EguiContext, egui::{self, Align2}};

use crate::logic::{Binary, equation::Equation};

pub struct EquationSolverPlugin;

impl Plugin for EquationSolverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<EquationSolverOpen>()
            .init_resource::<EquationSolverState>()
            .add_system_set(
                SystemSet::new()
                    .with_system(reset_equation_solver_state_system)
                    .before(equation_solver_system)
            )
            .add_system(equation_solver_system);
    }
}

#[derive(Default, Resource)]
pub struct EquationSolverOpen {
    open: bool,
}

#[derive(Default, Resource)]
struct EquationSolverState {
    equation_string: String,
    equation: Equation,
    solution: Option<Binary>,
}

impl EquationSolverOpen {
    pub fn open(&mut self) {
        self.open = true;
    }
}

impl EquationSolverState {
    pub fn equation_string(&mut self) -> &mut String {
        &mut self.equation_string
    }

    pub fn try_solve(&mut self) {
        self.equation = Equation::from(self.equation_string.clone());
        self.solution = self.equation.solve();
    }

    pub fn is_solved(&self) -> bool {
        self.solution.is_some()
    }

    pub fn get_solution(&self) -> &Option<Binary> {
        &self.solution
    }
}

fn reset_equation_solver_state_system(
    open: Res<EquationSolverOpen>,
    mut state: ResMut<EquationSolverState>,
) {
    if open.is_changed() && !open.open {
        *state = EquationSolverState::default();
    }
}

fn equation_solver_system(
    mut open: ResMut<EquationSolverOpen>,
    mut state: ResMut<EquationSolverState>,
    mut egui_context: ResMut<EguiContext>,
) {
    egui::Window::new("Confirm reset")
        .pivot(Align2::CENTER_CENTER)
        .anchor(Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .resizable(false)
        .collapsible(false)
        .open(&mut open.open)
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(state.equation_string());
                if ui.button("Solve").clicked() {
                    state.try_solve();
                }
            });

            if state.is_solved() {
                ui.label("Solution: ".to_owned() + &state.get_solution().unwrap().to_string());
            }
    });
}
