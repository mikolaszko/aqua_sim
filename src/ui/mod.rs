use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::elements::ElementsPlugin;
pub mod elements;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_plugins(ElementsPlugin);
    }
}
