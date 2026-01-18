use std::mem::take;

use eframe::egui::{self, Color32, Frame, Margin, RichText, Vec2};

use crate::{
    app::{PredicateImprovApp, typed_object_input},
    story::{LogicExpr, Symbol},
};

#[derive(Debug, Default, Clone)]
pub struct ActionDraft {
    pub name: String,
    pub parameters: Vec<(String, String)>,
    pub precondition: LogicExpr,
    pub effect: LogicExpr,
}

#[derive(Debug, Default)]
pub struct ActionPanel {
    pub action_draft: ActionDraft,
    backup_action_draft: Option<ActionDraft>,
    pub add_action: bool,
    show_action_modal: bool,
}

impl ActionPanel {
    pub fn show(ui: &mut egui::Ui, app: &mut PredicateImprovApp) {
        ui.spacing_mut().item_spacing = [6., 12.].into();

        ui.label(RichText::new("Actions").text_style(egui::TextStyle::Name("Heading2".into())));

        ui.add_space(16.);

        if ui.button("New action").clicked() {
            app.action_panel.show_action_modal = true;
        }

        Frame::canvas(ui.style())
            .inner_margin(Margin::same(8))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 6.;

                        app.domain.actions.retain(|name, action| {
                            Frame::new()
                                .fill(Color32::from_rgb(36, 36, 36))
                                .corner_radius(4.0)
                                .inner_margin(Margin::same(4))
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(name.0.clone()).italics());

                                        ui.label("(");
                                        for var in &mut *action.parameters {
                                            Frame::new()
                                                .corner_radius(2.0)
                                                .stroke(ui.style().visuals.window_stroke())
                                                .show(ui, |ui| {
                                                    ui.label(format!(
                                                        " {}: {} ",
                                                        var.name.0, var.r#type.0
                                                    ));
                                                });
                                        }
                                        ui.label(")");

                                        ui.allocate_space(
                                            [(ui.available_width() - 40.).max(0.), 0.].into(),
                                        );

                                        ui.menu_button("…", |ui| {
                                            let mut retain = true;

                                            retain = retain
                                                && if ui.button("Edit").clicked() {
                                                    app.action_panel.show_action_modal = true;

                                                    app.action_panel.action_draft.name =
                                                        name.0.clone();
                                                    app.action_panel.action_draft.parameters =
                                                        action
                                                            .parameters
                                                            .iter_mut()
                                                            .map(|v| {
                                                                (
                                                                    take(&mut v.name.0),
                                                                    take(&mut v.r#type.0),
                                                                )
                                                            })
                                                            .collect();
                                                    app.action_panel.action_draft.precondition =
                                                        take(&mut action.precondition);
                                                    app.action_panel.action_draft.effect =
                                                        take(&mut action.effect);

                                                    app.action_panel.backup_action_draft =
                                                        Some(app.action_panel.action_draft.clone());

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

        if app.action_panel.show_action_modal {
            egui::Modal::new("action_modal".into()).show(ui.ctx(), |ui| action_modal(app, ui));
        }
    }
}

fn action_modal(app: &mut PredicateImprovApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("action:");
        ui.text_edit_singleline(&mut app.action_panel.action_draft.name);
    });

    {
        let mut i = 0;
        app.action_panel
            .action_draft
            .parameters
            .retain_mut(|(param, ty)| {
                i += 1;
                typed_object_input(&app.domain, ui, param, ty, format!("param {i}"))
            });
    }

    ui.horizontal(|ui| {
        ui.label("Add parameter");
        if ui.button(egui_material_icons::icons::ICON_ADD).clicked()
            && app.action_panel.action_draft.parameters.len() < 5
        {
            app.action_panel
                .action_draft
                .parameters
                .push((String::new(), String::new()));
        }
    });

    ui.horizontal(|ui| {
        ui.label("precondition:");
        let expr = logic_expr(app, app.action_panel.action_draft.precondition.clone(), ui);
        app.action_panel.action_draft.precondition = expr;
    });

    ui.horizontal(|ui| {
        ui.label("effect:");
        let expr = logic_expr(app, app.action_panel.action_draft.effect.clone(), ui);
        app.action_panel.action_draft.effect = expr;
    });

    ui.horizontal(|ui| {
        if ui
            .add(egui::Button::new("Submit").min_size(Vec2::new(ui.available_width() / 2., 0.)))
            .clicked()
        {
            app.action_panel.show_action_modal = false;
            app.action_panel.add_action = true;
        }

        if ui
            .add(egui::Button::new("Cancel").min_size(Vec2::new(ui.available_width(), 0.)))
            .clicked()
        {
            app.action_panel.show_action_modal = false;
        }
    });
}

fn logic_expr(app: &mut PredicateImprovApp, expr: LogicExpr, ui: &mut egui::Ui) -> LogicExpr {
    match expr {
        LogicExpr::True => {
            let mut out_expr = LogicExpr::True;

            ui.menu_button("none", |ui| {
                let next_signature = app.domain.predicates.keys().next();
                if ui
                    .add_enabled(next_signature.is_some(), egui::Button::new("Predicate"))
                    .clicked()
                {
                    let signature = next_signature.unwrap();
                    let variables = vec![Symbol::default(); signature.arity as usize];
                    out_expr = LogicExpr::Predicate(signature.clone(), variables);
                }

                if ui.button("Not").clicked() {
                    out_expr = LogicExpr::Not(Box::new(LogicExpr::True));
                }

                if ui.button("And").clicked() {
                    out_expr = LogicExpr::And(Box::new(LogicExpr::True), Box::new(LogicExpr::True));
                }

                if ui.button("Or").clicked() {
                    out_expr = LogicExpr::Or(Box::new(LogicExpr::True), Box::new(LogicExpr::True));
                }
            });

            out_expr
        }
        LogicExpr::Predicate(mut predicate_signature, mut variables) => {
            let retain = ui
                .menu_button(predicate_signature.function.0.clone(), |ui| {
                    for ps in app.domain.predicates.keys() {
                        if ui
                            .button(format!("{}/{}", ps.function.0, ps.arity))
                            .clicked()
                        {
                            predicate_signature = ps.clone();
                        }
                    }

                    !ui.button("Delete").clicked()
                })
                .inner
                .unwrap_or(true);

            for vc in &mut variables {
                egui::TextEdit::singleline(&mut vc.0)
                    .desired_width(20.0)
                    .show(ui);
            }

            if retain {
                LogicExpr::Predicate(predicate_signature, variables)
            } else {
                LogicExpr::True
            }
        }
        LogicExpr::Not(mut v) => {
            enum Out {
                Not,
                Unwrap,
                Delete,
            }

            let mut out = Out::Not;

            ui.menu_button("not", |ui| {
                if ui.button("Unwrap").clicked() {
                    out = Out::Unwrap;
                }
                if ui.button("Delete").clicked() {
                    out = Out::Delete;
                }
            });

            ui.label("(");
            *v = logic_expr(app, (*v).clone(), ui);
            ui.label(")");

            match out {
                Out::Not => LogicExpr::Not(v),
                Out::Unwrap => *v,
                Out::Delete => LogicExpr::True,
            }
        }
        LogicExpr::And(mut lhs, mut rhs) => {
            enum Out {
                And,
                Or,
                UnwrapLeft,
                UnwrapRight,
                Delete,
            }

            let mut out = Out::And;

            ui.label("(");
            *lhs = logic_expr(app, (*lhs).clone(), ui);

            ui.menu_button("and", |ui| {
                if ui.button("Convert to Or").clicked() {
                    out = Out::Or;
                }
                if ui.button("Unwrap left").clicked() {
                    out = Out::UnwrapLeft;
                }
                if ui.button("Unwrap right").clicked() {
                    out = Out::UnwrapRight;
                }
                if ui.button("Delete").clicked() {
                    out = Out::Delete;
                }
            });

            *rhs = logic_expr(app, (*rhs).clone(), ui);
            ui.label(")");

            match out {
                Out::And => LogicExpr::And(lhs, rhs),
                Out::Or => LogicExpr::Or(lhs, rhs),
                Out::UnwrapLeft => *lhs,
                Out::UnwrapRight => *rhs,
                Out::Delete => LogicExpr::True,
            }
        }
        LogicExpr::Or(lhs, rhs) => {
            ui.label("(");
            let new_lhs = Box::new(logic_expr(app, (*lhs).clone(), ui));

            let retain = ui
                .menu_button("or", |ui| !ui.button("Delete").clicked())
                .inner
                .unwrap_or(true);

            let new_rhs = Box::new(logic_expr(app, (*rhs).clone(), ui));
            ui.label(")");

            if retain {
                LogicExpr::Or(new_lhs, new_rhs)
            } else {
                LogicExpr::True
            }
        }
    }
}
