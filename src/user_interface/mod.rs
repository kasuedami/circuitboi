use bevy::prelude::{Plugin, App, Resource, SystemSet};

mod run_criteria;
mod panel;

#[derive(Default)]
pub struct UiPlugin {
    panels: ShowPanels,
}

impl UiPlugin {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn left(mut self) -> Self {
        self.panels.left = true;
        self
    }

    #[allow(dead_code)]
    pub fn right(mut self) -> Self {
        self.panels.right = true;
        self
    }

    #[allow(dead_code)]
    pub fn top(mut self) -> Self {
        self.panels.top = true;
        self
    }

    #[allow(dead_code)]
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
                    .with_system(panel::left::system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_right_panel)
                    .with_system(panel::right::system)
                    .after(panel::left::system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_top_panel)
                    .with_system(panel::top::system)
                    .before(panel::left::system)
                    .before(panel::right::system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_criteria::show_bottom_panel)
                    .with_system(panel::bottom::system)
                    .after(panel::left::system)
                    .after(panel::right::system)
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