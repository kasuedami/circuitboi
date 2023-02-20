use bevy::{prelude::{Res, ResMut}, ecs::schedule::ShouldRun};
use super::{OccupiedScreenSpace, ShowPanels};

pub(super) fn show_left_panel(
    show_panels: Res<ShowPanels>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) -> ShouldRun {
    if show_panels.left {
        ShouldRun::Yes
    } else {
        occupied_screen_space.as_mut().left = 0.0;
        ShouldRun::No
    }
}

pub(super) fn show_right_panel(
    show_panels: Res<ShowPanels>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) -> ShouldRun {
    if show_panels.right {
        ShouldRun::Yes
    } else {
        occupied_screen_space.as_mut().right = 0.0;
        ShouldRun::No
    }
}

pub(super) fn show_top_panel(
    show_panels: Res<ShowPanels>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) -> ShouldRun {
    if show_panels.top {
        ShouldRun::Yes
    } else {
        occupied_screen_space.as_mut().top = 0.0;
        ShouldRun::No
    }
}

pub(super) fn show_bottom_panel(
    show_panels: Res<ShowPanels>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) -> ShouldRun {
    if show_panels.bottom {
        ShouldRun::Yes
    } else {
        occupied_screen_space.as_mut().bottom = 0.0;
        ShouldRun::No
    }
}