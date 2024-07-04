use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

#[derive(Resource, Default)]
pub struct UiState {
    label: String,
    is_dark_mode: bool,
}

use crate::sand::metadata::SandAmount;

pub struct ElementsPlugin;

impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>();
        app.add_systems(Update, setup_ui);
    }
}

fn setup_ui(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut metadata: ResMut<SandAmount>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });
            ui.vertical(|ui| {
                ui.label("Sand amount ");
                ui.label(metadata.grains.to_string());
                if ui.button("Add sand").clicked() {
                    metadata.increase()
                };
                if ui.button("Subtract sand").clicked() {
                    metadata.decrease()
                };
            });

            ui.checkbox(&mut ui_state.is_dark_mode, "Dark Mode");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
