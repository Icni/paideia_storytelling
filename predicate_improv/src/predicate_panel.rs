use std::mem::take;

use eframe::egui::{self, Color32, Frame, Margin, RichText, Vec2};

use crate::app::{PredicateImprovApp, typed_object_input};

#[derive(Debug, Default, Clone)]
pub struct PredicateDraft {
    pub function: String,
    pub variables: Vec<(String, String)>,
}

#[derive(Debug, Default)]
pub struct PredicatePanel {
    pub predicate_draft: PredicateDraft,
    backup_predicate_draft: Option<PredicateDraft>,
    pub add_predicate: bool,
    show_predicate_modal: bool,
}

impl PredicatePanel {
    pub fn show(ui: &mut egui::Ui, app: &mut PredicateImprovApp) {
        ui.spacing_mut().item_spacing = [6., 12.].into();

        ui.label(RichText::new("Predicates").text_style(egui::TextStyle::Name("Heading2".into())));

        ui.add_space(16.);

        if ui.button("New predicate").clicked() {
            app.predicate_panel.show_predicate_modal = true;
        }

        Frame::canvas(ui.style())
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 6.;

                        app.domain.predicates.retain(|predicate, variables| {
                            Frame::new()
                                .fill(Color32::from_rgb(36, 36, 36))
                                .corner_radius(4.0)
                                .inner_margin(Margin::same(4))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(&*predicate.function).italics());

                                        ui.label("(");
                                        for var in &mut *variables {
                                            Frame::new()
                                                .corner_radius(2.0)
                                                .stroke(ui.style().visuals.window_stroke())
                                                .show(ui, |ui| {
                                                    ui.label(format!(
                                                        " {}: {} ",
                                                        &var.name.0, &var.r#type.0
                                                    ));
                                                });
                                        }
                                        ui.label(")");

                                        ui.allocate_space(
                                            [(ui.available_width() - 40.).max(0.), 0.].into(),
                                        );

                                        ui.menu_button("…", |ui| {
                                            let mut retain = if ui.button("Edit").clicked() {
                                                app.predicate_panel.show_predicate_modal = true;

                                                app.predicate_panel.predicate_draft.function =
                                                    predicate.function.0.clone();
                                                app.predicate_panel.predicate_draft.variables =
                                                    variables
                                                        .iter_mut()
                                                        .map(|v| {
                                                            (
                                                                take(&mut v.name.0),
                                                                take(&mut v.r#type.0),
                                                            )
                                                        })
                                                        .collect();

                                                app.predicate_panel.backup_predicate_draft = Some(
                                                    app.predicate_panel.predicate_draft.clone(),
                                                );

                                                false
                                            } else {
                                                true
                                            };

                                            retain = retain && !ui.button("Delete").clicked();

                                            retain
                                        })
                                        .inner
                                        .unwrap_or(true)
                                    })
                                    .inner
                                })
                                .inner
                        });
                    });
            });

        if app.predicate_panel.show_predicate_modal {
            egui::Modal::new("predicate_modal".into())
                .show(ui.ctx(), |ui| predicate_modal(app, ui));
        }
    }
}

fn predicate_modal(app: &mut PredicateImprovApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("function:");
        ui.text_edit_singleline(&mut app.predicate_panel.predicate_draft.function);
    });

    {
        let mut i = 0;
        app.predicate_panel
            .predicate_draft
            .variables
            .retain_mut(|(var, ty)| {
                i += 1;
                typed_object_input(&app.domain, ui, var, ty, format!("var {i}"))
            });
    }

    ui.horizontal(|ui| {
        ui.label("Add variable");
        if ui.button(egui_material_icons::icons::ICON_ADD).clicked()
            && app.predicate_panel.predicate_draft.variables.len() < 5
        {
            app.predicate_panel
                .predicate_draft
                .variables
                .push((String::new(), String::new()));
        }
    });

    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new("Submit").min_size(Vec2::new(ui.available_width() / 2., 0.)))
            .clicked()
        {
            app.predicate_panel.show_predicate_modal = false;
            app.predicate_panel.add_predicate = true;
        }

        if ui
            .add(egui::Button::new("Cancel").min_size(Vec2::new(ui.available_width(), 0.)))
            .clicked()
        {
            app.predicate_panel.show_predicate_modal = false;

            if let Some(draft) = take(&mut app.predicate_panel.backup_predicate_draft) {
                app.predicate_panel.predicate_draft = draft;
                app.predicate_panel.add_predicate = true;
            }
        }
    });
}
