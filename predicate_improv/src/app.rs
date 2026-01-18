use std::mem::take;

use eframe::egui::{self, Frame, Margin, Stroke, Vec2, WidgetText};

use crate::{
    action_panel::ActionPanel,
    predicate_panel::PredicatePanel,
    problem_panel::ProblemPanel,
    story::{
        Action, PredicateDomain, PredicateProblem, PredicateSignature, PredicateStory, Symbol,
        TypeName, TypedSymbol,
    },
    types_and_constants_panel::TypesAndConstantsPanel,
};

#[derive(Debug, Default)]
pub struct PredicateImprovApp {
    pub domain: PredicateDomain,
    pub problem: PredicateProblem,
    pub story: Option<PredicateStory>,
    pub action_panel: ActionPanel,
    pub predicate_panel: PredicatePanel,
    pub types_and_constants_panel: TypesAndConstantsPanel,
    pub problem_panel: ProblemPanel,
    pub viewing_story: bool,
}

impl PredicateImprovApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        egui_material_icons::initialize(&cc.egui_ctx);

        cc.egui_ctx.style_mut(|style| {
            use egui::{FontFamily, FontId, TextStyle};

            style.text_styles = [
                (
                    TextStyle::Heading,
                    FontId::new(36.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Name("Heading2".into()),
                    FontId::new(32.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Name("Context".into()),
                    FontId::new(23.0, FontFamily::Proportional),
                ),
                (TextStyle::Body, FontId::new(24.0, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new(24.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Button,
                    FontId::new(24.0, FontFamily::Proportional),
                ),
                (
                    TextStyle::Small,
                    FontId::new(10.0, FontFamily::Proportional),
                ),
            ]
            .into()
        });

        Self {
            domain: PredicateDomain::default(),
            problem: PredicateProblem::default(),
            action_panel: ActionPanel::default(),
            predicate_panel: PredicatePanel::default(),
            types_and_constants_panel: TypesAndConstantsPanel::default(),
            problem_panel: ProblemPanel::default(),
            story: None,
            viewing_story: false,
        }
    }
}

impl eframe::App for PredicateImprovApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let style = ctx.style();

        egui::SidePanel::left("predicate_panel")
            .frame(Frame::window(&style).stroke(Stroke::NONE).inner_margin(16))
            .default_width(ctx.viewport_rect().width() / 4.)
            .show(ctx, |ui| PredicatePanel::show(ui, self));

        egui::SidePanel::right("problem_panel")
            .frame(Frame::window(&style).stroke(Stroke::NONE).inner_margin(16))
            .default_width(ctx.viewport_rect().width() / 4.)
            .show(ctx, |ui| ProblemPanel::show(ui, self));

        egui::TopBottomPanel::bottom("action_panel")
            .frame(Frame::window(&style).stroke(Stroke::NONE).inner_margin(16))
            .default_height(ctx.viewport_rect().height() / 4.)
            .show(ctx, |ui| ActionPanel::show(ui, self));

        egui::TopBottomPanel::bottom("types_and_constants_panel")
            .frame(Frame::window(&style).stroke(Stroke::NONE).inner_margin(16))
            .default_height(ctx.viewport_rect().height() / 4.)
            .show(ctx, |ui| TypesAndConstantsPanel::show(ui, self));

        egui::CentralPanel::default()
            .frame(Frame::window(&style).stroke(Stroke::NONE).inner_margin(16))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = [6., 12.].into();

                ui.horizontal(|ui| {
                    ui.heading("Predicate Improvizer");

                    let generate_text =
                        format!("Generate {}", egui_material_icons::icons::ICON_CASINO);

                    let generate_response =
                        ui.add_visible(false, egui::Button::new(&generate_text));

                    ui.allocate_space(
                        [ui.available_width() - generate_response.rect.width(), 0.].into(),
                    );

                    if ui.button(generate_text).clicked() {
                        self.story = self.domain.generate_story();
                        self.viewing_story = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.spacing_mut().slider_width = 300.;
                    ui.add(egui::Slider::new(
                        &mut self.problem.max_story_length,
                        0..=10,
                    ));
                    ui.add_space(12.);
                    ui.label("Story length");
                });

                // ui.horizontal(|ui| {
                //     ui.add_sized(
                //         [ui.available_width() - 30., 20.],
                //         egui::TextEdit::singleline(&mut self.action_panel.action_draft.name),
                //     );
                //
                //     if ui.button(egui_material_icons::icons::ICON_ADD).clicked() {
                //         add_action = true;
                //     }
                // });

                Frame::canvas(&style)
                    .inner_margin(Margin::same(8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.spacing_mut().item_spacing.y = 6.;

                                // self.domain.predicates.retain(|predicate, variables| {
                                //     Frame::new()
                                //         .fill(Color32::from_rgb(36, 36, 36))
                                //         .corner_radius(4.0)
                                //         .inner_margin(Margin::same(4))
                                //         .show(ui, |ui| {
                                //             ui.horizontal(|ui| {
                                //                 ui.label(format!(
                                //                     "{}({:?})",
                                //                     *predicate.function, variables
                                //                 ));
                                //                 ui.allocate_space(
                                //                     [ui.available_width() - 40., 0.].into(),
                                //                 );
                                //
                                //                 !ui.button(egui_material_icons::icons::ICON_REMOVE)
                                //                     .clicked()
                                //             })
                                //             .inner
                                //         })
                                //         .inner
                                // });
                            });
                    });
            });

        egui::Window::new("My story")
            .collapsible(false)
            .fixed_size(ctx.viewport_rect().size() - Vec2::new(50., 100.))
            .resizable(false)
            .open(&mut self.viewing_story)
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());
                if let Some(story) = &self.story {
                    ui.label(&story.text);
                }
            });

        if self.predicate_panel.add_predicate {
            let signature = PredicateSignature {
                function: Symbol(take(&mut (self.predicate_panel.predicate_draft.function))),
                arity: self.predicate_panel.predicate_draft.variables.len() as u32,
            };

            let variables = take(&mut self.predicate_panel.predicate_draft.variables)
                .iter_mut()
                .map(|(var, ty)| TypedSymbol {
                    name: Symbol(take(var)),
                    r#type: TypeName(take(ty)),
                })
                .collect();

            self.domain.predicates.insert(signature, variables);

            self.predicate_panel.add_predicate = false;
        }

        if self.action_panel.add_action {
            let name = Symbol(take(&mut self.action_panel.action_draft.name));
            let parameters = self
                .action_panel
                .action_draft
                .parameters
                .iter_mut()
                .map(|(param, ty)| TypedSymbol {
                    name: Symbol(take(param)),
                    r#type: TypeName(take(ty)),
                })
                .collect();
            let action = Action {
                parameters,
                precondition: take(&mut self.action_panel.action_draft.precondition),
                effect: take(&mut self.action_panel.action_draft.effect),
            };

            self.domain.actions.insert(name, action);

            self.action_panel.add_action = false;
        }

        if self.types_and_constants_panel.add_type {
            let name = TypeName(take(&mut self.types_and_constants_panel.type_draft.name));
            let supertypes = self
                .types_and_constants_panel
                .type_draft
                .supertypes
                .iter_mut()
                .map(|ty| TypeName(take(ty)))
                .collect();

            self.domain.types.insert(name, supertypes);

            self.types_and_constants_panel.add_type = false;
        }

        if self.types_and_constants_panel.add_constant {
            let name = Symbol(take(
                &mut self.types_and_constants_panel.constant_draft.name,
            ));
            let r#type = TypeName(take(
                &mut self.types_and_constants_panel.constant_draft.r#type,
            ));

            self.domain.constants.insert(name, r#type);

            self.types_and_constants_panel.add_constant = false;
        }

        if self.problem_panel.add_bound_predicate {
            let signature = PredicateSignature {
                function: Symbol(take(
                    &mut self.problem_panel.bound_predicate_draft.predicate_name,
                )),
                arity: self.problem_panel.bound_predicate_draft.bound_objects.len() as u32,
            };
            let bindings = take(
                &mut self
                    .problem_panel
                    .bound_predicate_draft
                    .bound_objects
                    .iter_mut(),
            )
            .map(|b| Symbol(take(b)))
            .collect();

            self.problem
                .initial_state
                .bound_predicates
                .insert(signature, bindings);

            self.problem_panel.add_bound_predicate = false;
        }
    }
}

pub fn untyped_object_input(
    ui: &mut egui::Ui,
    var: &mut String,
    text: impl Into<WidgetText>,
) -> bool {
    ui.horizontal(|ui| {
        ui.label(text);

        let input_width = (ui.available_width() - 150.) / 2.;
        egui::TextEdit::singleline(var)
            .desired_width(input_width)
            .show(ui);

        !ui.button(egui_material_icons::icons::ICON_REMOVE).clicked()
    })
    .inner
}

pub fn type_button(domain: &PredicateDomain, ui: &mut egui::Ui, ty: &mut String) {
    let ty_text = if ty.is_empty() {
        String::from("<object>")
    } else {
        ty.clone()
    };

    let button_size = Vec2::new(
        ui.available_width() - 40.,
        ui.style().spacing.button_padding.y * 2. + 24.,
    );

    let button = egui::Button::new(ty_text);
    let response = ui.add_sized(button_size, button);

    egui::Popup::menu(&response)
        .width(button_size.x - 20.)
        .show(|ui| {
            for r#type in domain.types.keys() {
                if ui.button(&r#type.0).clicked() {
                    *ty = r#type.0.clone();
                }
            }
        });
}

pub fn typed_object_input(
    domain: &PredicateDomain,
    ui: &mut egui::Ui,
    var: &mut String,
    ty: &mut String,
    text: impl Into<WidgetText>,
) -> bool {
    ui.horizontal(|ui| {
        ui.label(text);

        let input_width = (ui.available_width() - 150.) / 2.;
        egui::TextEdit::singleline(var)
            .desired_width(input_width)
            .show(ui);

        ui.label(", type:");
        type_button(domain, ui, ty);

        !ui.button(egui_material_icons::icons::ICON_REMOVE).clicked()
    })
    .inner
}
