use bevy::prelude::ResMut;
use bevy_egui::{EguiContext, egui};
use egui_extras::{TableBuilder, Column};

use crate::user_interface::OccupiedScreenSpace;

pub(in crate::user_interface) fn system(
    mut egui_context: ResMut<EguiContext>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Inputs");
            ui.allocate_rect(half_height_available_rect_before_wrap(ui), egui::Sense::hover());
            ui.separator();

            TableBuilder::new(ui)
            .column(Column::auto()).resizable(true)
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Column");
                });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label("Moin");
                    });
                });
            });

            ui.heading("Outputs");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover())
        })
        .response
        .rect
        .width();
}

fn half_height_available_rect_before_wrap(ui: &egui::Ui) -> egui::Rect {
    let mut rectangle = ui.available_rect_before_wrap();
    let half_size = rectangle.bottom() / 2.0;
    rectangle.set_bottom(half_size);

    rectangle
}