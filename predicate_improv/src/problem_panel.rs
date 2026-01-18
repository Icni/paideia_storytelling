use std::mem::take;

use eframe::egui::{self, Color32, Frame, Margin, RichText, Stroke, Vec2};

use crate::{
    app::{PredicateImprovApp, type_button, untyped_object_input},
    story::{LogicExpr, Symbol, TypeName, TypedSymbol},
};

#[derive(Debug, Default, Clone)]
pub struct BoundPredicateDraft {
    pub predicate_name: String,
    pub bound_objects: Vec<String>,
}

#[derive(Debug, Default)]
pub struct ProblemPanel {
    pub bound_predicate_draft: BoundPredicateDraft,
    backup_bound_predicate_draft: Option<BoundPredicateDraft>,
    pub object_draft: (String, String),
    pub add_bound_predicate: bool,
    show_bound_predicate_modal: bool,
}

impl ProblemPanel {
    pub fn show(ui: &mut egui::Ui, app: &mut PredicateImprovApp) {
        ui.spacing_mut().item_spacing = [6., 12.].into();

        egui::TopBottomPanel::bottom("initial_state_panel")
            .frame(
                Frame::window(ui.style())
                    .stroke(Stroke::NONE)
                    .inner_margin(0),
            )
            .show_separator_line(false)
            .default_height(ui.available_height() / 2.)
            .show_inside(ui, |ui| {
                ui.add_space(16.);

                ui.label("Initial State");

                let next_signature = app.domain.predicates.keys().next();
                if ui
                    .add_enabled(
                        next_signature.is_some(),
                        egui::Button::new("Bind predicate"),
                    )
                    .clicked()
                {
                    app.problem_panel.show_bound_predicate_modal = true;
                }

                Frame::canvas(ui.style())
                    .inner_margin(Margin::same(8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 6.;

                                app.problem.initial_state.bound_predicates.retain(
                                    |signature, bindings| {
                                        Frame::new()
                                            .fill(Color32::from_rgb(36, 36, 36))
                                            .corner_radius(4.0)
                                            .inner_margin(Margin::same(4))
                                            .show(ui, |ui| {
                                                ui.horizontal(|ui| {
                                                    ui.label(&signature.function.0);

                                                    ui.label("(");
                                                    for var in &mut *bindings {
                                                        Frame::new()
                                                            .corner_radius(2.0)
                                                            .stroke(
                                                                ui.style().visuals.window_stroke(),
                                                            )
                                                            .show(ui, |ui| {
                                                                ui.label(format!(" {} ", &var.0));
                                                            });
                                                    }
                                                    ui.label(")");

                                                    ui.allocate_space(
                                                        [(ui.available_width() - 40.).max(0.), 0.]
                                                            .into(),
                                                    );

                                                    ui.menu_button("…", |ui| {
                                                        let mut retain = true;

                                                        retain = retain
                                                            && if ui.button("Edit").clicked() {
                                                                app.problem_panel
                                                                    .show_bound_predicate_modal =
                                                                    true;

                                                                app.problem_panel
                                                                    .bound_predicate_draft
                                                                    .predicate_name =
                                                                    signature.function.0.clone();
                                                                app.problem_panel
                                                                    .bound_predicate_draft
                                                                    .bound_objects = bindings
                                                                    .iter_mut()
                                                                    .map(|v| take(&mut v.0))
                                                                    .collect();

                                                                app.problem_panel
                                                                    .backup_bound_predicate_draft =
                                                                    Some(
                                                                        app.problem_panel
                                                                            .bound_predicate_draft
                                                                            .clone(),
                                                                    );

                                                                false
                                                            } else {
                                                                true
                                                            };

                                                        retain = retain
                                                            && !ui.button("Delete").clicked();

                                                        retain
                                                    })
                                                    .inner
                                                    .unwrap_or(true)
                                                })
                                                .inner
                                            })
                                            .inner
                                    },
                                );
                            });
                    });
            });

        ui.label(RichText::new("Problem").text_style(egui::TextStyle::Name("Heading2".into())));

        ui.add_space(16.);

        ui.label("Objects");
        ui.horizontal(|ui| {
            egui::TextEdit::singleline(&mut app.problem_panel.object_draft.0)
                .desired_width((ui.available_width() - 50.) / 2.)
                .show(ui);

            type_button(&app.domain, ui, &mut app.problem_panel.object_draft.1);

            if ui.button(egui_material_icons::icons::ICON_ADD).clicked()
                && !app.problem_panel.object_draft.0.is_empty()
                && !app.problem_panel.object_draft.1.is_empty()
            {
                app.problem.objects.push(TypedSymbol {
                    name: Symbol(take(&mut app.problem_panel.object_draft.0)),
                    r#type: TypeName(take(&mut app.problem_panel.object_draft.1)),
                });
            }
        });

        Frame::canvas(ui.style())
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 6.;

                        app.problem.objects.retain(|obj| {
                            Frame::new()
                                .fill(Color32::from_rgb(36, 36, 36))
                                .corner_radius(4.0)
                                .inner_margin(Margin::same(4))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}: {}", obj.name.0, obj.r#type.0));

                                        ui.allocate_space(
                                            [(ui.available_width() - 40.).max(0.), 0.].into(),
                                        );

                                        !ui.button(egui_material_icons::icons::ICON_REMOVE)
                                            .clicked()
                                    })
                                    .inner
                                })
                                .inner
                        });
                    });
            });

        if app.problem_panel.show_bound_predicate_modal {
            egui::Modal::new("bound_predicate_modal".into())
                .show(ui.ctx(), |ui| bound_predicate_modal(app, ui));
        }
    }
}

fn bound_predicate_modal(app: &mut PredicateImprovApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(format!(
            "predicate: {}",
            app.problem_panel.bound_predicate_draft.predicate_name
        ));

        ui.menu_button("Change", |ui| {
            for ps in app.domain.predicates.keys() {
                if ui
                    .button(format!("{}/{}", ps.function.0, ps.arity))
                    .clicked()
                {
                    app.problem_panel.bound_predicate_draft.predicate_name = ps.function.0.clone();

                    app.problem_panel.bound_predicate_draft.bound_objects =
                        vec![String::new(); ps.arity as usize];
                }
            }
        });
    });

    {
        let mut i = 0;
        app.problem_panel
            .bound_predicate_draft
            .bound_objects
            .retain_mut(|obj| {
                i += 1;
                untyped_object_input(ui, obj, format!("var {i}"))
            });
    }

    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new("Submit").min_size(Vec2::new(ui.available_width() / 2., 0.)))
            .clicked()
        {
            app.problem_panel.show_bound_predicate_modal = false;
            app.problem_panel.add_bound_predicate = true;
        }

        if ui
            .add(egui::Button::new("Cancel").min_size(Vec2::new(ui.available_width(), 0.)))
            .clicked()
        {
            app.problem_panel.show_bound_predicate_modal = false;

            if let Some(draft) = take(&mut app.problem_panel.backup_bound_predicate_draft) {
                app.problem_panel.bound_predicate_draft = draft;
                app.problem_panel.add_bound_predicate = true;
            }
        }
    });
}
