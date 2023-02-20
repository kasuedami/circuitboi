use bevy::prelude::{ResMut, Plugin, App, Resource, SystemSet};
use bevy_egui::{egui, EguiContext};

mod run_criteria;

#[derive(Default)]
pub struct UiPlugin {
    panels: ShowPanels,
}

impl UiPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all() -> Self {
        Self {
            panels: ShowPanels {
                left: true,
                right: true,
                top: true,
                bottom: true
            }
        }
    }

    pub fn left(mut self) -> Self {
        self.panels.left = true;
        self
    }

    pub fn right(mut self) -> Self {
        self.panels.right = true;
        self
    }

    pub fn top(mut self) -> Self {
        self.panels.top = true;
        self
    }

    pub fn bottom(mut self) -> Self {
        self.panels.bottom = true;
        self
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OccupiedScreenSpace>()
            .insert_resource(self.panels.clone())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_left_panel)
                    .with_system(left_panel_system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_right_panel)
                    .with_system(right_panel_system)
                    .after(left_panel_system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_top_panel)
                    .with_system(top_panel_system)
                    .after(right_panel_system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_bottom_panel)
                    .with_system(bottom_panel_system)
                    .after(top_panel_system)
            );
    }
}

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Default, Resource, Clone)]
struct ShowPanels {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

fn left_panel_system(
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

fn right_panel_system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}

fn top_panel_system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}

fn bottom_panel_system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}