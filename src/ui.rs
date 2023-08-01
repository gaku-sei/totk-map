use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts};

use crate::resources::{DisplayedMarkers, FocusedMarkers, MapType, Markers, Options};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiHoverStatus>()
            .add_systems(PreUpdate, update_egui_mouse_check)
            .add_systems(Update, filters_ui);
    }
}

#[derive(Resource, Default)]
pub struct EguiHoverStatus {
    is_hovered: bool,
}

#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn egui_is_hovered(egui_checker: Res<EguiHoverStatus>) -> bool {
    egui_checker.is_hovered
}

#[allow(clippy::needless_pass_by_value)]
fn update_egui_mouse_check(
    mut contexts: bevy_egui::EguiContexts,
    mut egui_checker: ResMut<EguiHoverStatus>,
) {
    egui_checker.is_hovered = contexts.ctx_mut().is_pointer_over_area();
}

#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
fn filters_ui(
    mut contexts: EguiContexts,
    markers: Res<Markers>,
    mut displayed_markers: ResMut<DisplayedMarkers>,
    focused_markers: Res<FocusedMarkers>,
    diagnostics: Res<DiagnosticsStore>,
    mut map_type: ResMut<MapType>,
    options: Res<Options>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    egui::Window::new("Levels").show(contexts.ctx_mut(), |ui| {
        for map in MapType::iter() {
            if ui.button(map.as_str()).clicked() {
                *map_type = *map;
                displayed_markers.markers_mut().clear();
                let locations = markers
                    .locations(*map_type)
                    .iter()
                    .map(|location| location.name.clone());
                displayed_markers.add_missing_from(locations);
            }
        }
    });

    egui::Window::new("Locations").show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Show all").clicked() {
                let locations = markers
                    .locations(*map_type)
                    .iter()
                    .map(|location| location.name.clone());
                displayed_markers.add_missing_from(locations);
            }

            if ui.button("Hide all").clicked() {
                let locations = markers
                    .locations(*map_type)
                    .iter()
                    .map(|location| &location.name);
                displayed_markers.remove_from(locations);
            }

            for location in markers.locations(*map_type) {
                let mut checked = displayed_markers.markers().contains(&location.name);
                if ui.checkbox(&mut checked, &location.name).changed() {
                    displayed_markers.toggle(location.name.clone());
                }
            }
        });
    });

    egui::Window::new("Materials").show(contexts.ctx_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if ui.button("Hide all").clicked() {
                let materials = markers
                    .materials(*map_type)
                    .iter()
                    .map(|material| &material.name);
                displayed_markers.remove_from(materials);
            }

            for resource in markers.materials(*map_type) {
                let mut checked = displayed_markers.markers().contains(&resource.name);
                if ui.checkbox(&mut checked, &resource.name).changed() {
                    displayed_markers.toggle(resource.name.clone());
                }
            }
        });
    });

    if let Some(focused_marker) = focused_markers.markers().first() {
        if let Some(cursor_position) = primary_window.single().cursor_position() {
            let pos = cursor_position + 8.0;
            egui::Window::new("")
                .fixed_pos((pos.x, pos.y))
                .title_bar(false)
                .resizable(false)
                .show(contexts.ctx_mut(), |ui| {
                    ui.label(focused_marker);
                });
        }
    }

    if options.debug_display {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                egui::Window::new("fps").show(contexts.ctx_mut(), |ui| {
                    ui.label(format!("{value:.2}"));
                });
            }
        }
    }
}
