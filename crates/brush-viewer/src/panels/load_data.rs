use crate::{viewer::ViewerContext, ViewerPanel};
use brush_dataset::{LoadDatasetArgs, LoadInitArgs};
use egui::Slider;

pub(crate) struct LoadDataPanel {
    max_train_resolution: Option<u32>,
    max_frames: Option<usize>,
    eval_split_every: Option<usize>,
    sh_degree: u32,
}

impl LoadDataPanel {
    pub(crate) fn new() -> Self {
        Self {
            // High resolution performance just isn't great at the moment... limit this for now by default.
            max_train_resolution: None,
            max_frames: None,
            eval_split_every: Some(8),
            sh_degree: 3,
        }
    }
}

impl ViewerPanel for LoadDataPanel {
    fn title(&self) -> String {
        "Load data".to_owned()
    }

    fn ui(&mut self, ui: &mut egui::Ui, context: &mut ViewerContext) {
        ui.label("Select a .ply to visualize, or a .zip with training data.");

        if ui.button("Pick a file").clicked() {
            let load_data_args = LoadDatasetArgs {
                max_frames: self.max_frames,
                max_resolution: self.max_train_resolution,
                eval_split_every: self.eval_split_every,
            };
            let load_init_args = LoadInitArgs {
                sh_degree: self.sh_degree,
            };
            context.start_data_load(load_data_args, load_init_args);
        }

        ui.add_space(10.0);
        ui.heading("Train settings");

        ui.label("Spherical Harmonics Degree:");
        ui.add(Slider::new(&mut self.sh_degree, 1..=5));

        let mut limit_res = self.max_train_resolution.is_some();
        if ui
            .checkbox(&mut limit_res, "Limit training resolution")
            .clicked()
        {
            self.max_train_resolution = if limit_res { Some(800) } else { None };
        }

        if let Some(target_res) = self.max_train_resolution.as_mut() {
            ui.add(Slider::new(target_res, 32..=2048));
        }

        let mut limit_frames = self.max_frames.is_some();
        if ui.checkbox(&mut limit_frames, "Limit max frames").clicked() {
            self.max_frames = if limit_frames { Some(32) } else { None };
        }

        if let Some(max_frames) = self.max_frames.as_mut() {
            ui.add(Slider::new(max_frames, 1..=256));
        }

        let mut use_eval_split = self.eval_split_every.is_some();
        if ui
            .checkbox(&mut use_eval_split, "Split dataset for evaluation")
            .clicked()
        {
            self.eval_split_every = if use_eval_split { Some(8) } else { None };
        }

        if let Some(eval_split) = self.eval_split_every.as_mut() {
            ui.add(
                Slider::new(eval_split, 2..=32)
                    .prefix("1 out of ")
                    .suffix(" frames"),
            );
        }

        if ui.input(|r| r.key_pressed(egui::Key::Escape)) {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}
