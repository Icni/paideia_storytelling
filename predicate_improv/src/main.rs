use eframe::{egui, NativeOptions};

use crate::app::PredicateImprovApp;

mod app;
mod action_panel;
mod predicate_panel;
mod problem_panel;
mod types_and_constants_panel;
mod story;

fn main() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder {
            title: Some("Predicate Improvizer".into()),
            inner_size: Some(egui::Vec2::new(1600., 900.)),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native("Predicate Improvizer", native_options, Box::new(|cc| Ok(Box::new(PredicateImprovApp::new(cc)))))
}
