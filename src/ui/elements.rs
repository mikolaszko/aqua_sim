use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum AquaElementsState {
    Sand,
    #[default]
    Rock,
    Plant,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub label: String,
    pub radio: AquaElementsState,
}

use crate::sand::metadata::SandAmount;

pub struct ElementsPlugin;

impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>();
        app.init_state::<AquaElementsState>();
        app.add_systems(Update, setup_ui);
    }
}

fn setup_ui(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut meta_state: ResMut<NextState<AquaElementsState>>,
    sand_metadata: Res<SandAmount>,
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
                ui.label(sand_metadata.grains.to_string());
            });

            ui.label("Select aquarium item to insert");
            ui.horizontal(|ui| {
                if ui
                    .radio_value(&mut ui_state.radio, AquaElementsState::Sand, "Sand")
                    .clicked()
                {
                    meta_state.set(AquaElementsState::Sand)
                };
                if ui
                    .radio_value(&mut ui_state.radio, AquaElementsState::Rock, "Rocks")
                    .clicked()
                {
                    meta_state.set(AquaElementsState::Rock)
                };
                if ui
                    .radio_value(&mut ui_state.radio, AquaElementsState::Plant, "Plants")
                    .clicked()
                {
                    meta_state.set(AquaElementsState::Plant)
                };
            });

            println!("{:?}", AquaElementsState::Sand);
            ui.end_row();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });
}
