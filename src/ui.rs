use macroquad::{color::hsl_to_rgb, prelude::*, ui::{ hash, root_ui, widgets  }};

const SPACING: f32 = 30.;

pub struct Ui {
    pub showing: bool,
    pub width: usize,
    pub height: usize,
    pub mine_percent: usize,
    pub hue: f32,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            showing: false,
            width: 20,
            height: 15,
            mine_percent: 20,
            hue: 175.,
        }
    }

    pub fn color(&self) -> Color {
        hsl_to_rgb(self.hue / 255., 90. / 255., 110. / 255.)
    }

    pub fn mine_count(&self) -> usize {
        self.width * self.height * self.mine_percent / 100
    }

    pub fn update(&mut self) -> bool {
        if !self.showing {
            return false;
        };

        let mut updated = false;
        widgets::Window::new(hash!(), vec2(10., 10.), vec2(200., 165.))
            .label("Options")
            .titlebar(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(Vec2::new(10., SPACING * 0.), &format!("Width: {}", self.width));
                if ui.button(Vec2::new(130., SPACING * 0. + 5.), " + ") {
                    self.width = clamp(self.width + 1, 4, 100);
                };
                if ui.button(Vec2::new(160., SPACING * 0. + 5.), " - ") {
                    self.width = clamp(self.width - 1, 4, 100);
                };

                ui.label(Vec2::new(10., SPACING * 1.), &format!("Height: {}", self.height));
                if ui.button(Vec2::new(130., SPACING * 1. + 5.), " + ") {
                    self.height = clamp(self.height + 1, 4, 100);
                };
                if ui.button(Vec2::new(160., SPACING * 1. + 5.), " - ") {
                    self.height = clamp(self.height - 1, 4, 100);
                };

                ui.label(Vec2::new(10., SPACING * 2.), &format!("Mine%: {}", self.mine_percent));
                if ui.button(Vec2::new(130., SPACING * 2. + 5.), " + ") {
                    self.mine_percent = clamp(self.mine_percent + 1, 0, 50);
                };
                if ui.button(Vec2::new(160., SPACING * 2. + 5.), " - ") {
                    self.mine_percent = clamp(self.mine_percent - 1, 0, 50);
                };

                widgets::Group::new(hash!(), Vec2::new(180., SPACING))
                    .position(Vec2::new(10., SPACING * 3.))
                    .ui(ui, |ui| {
                        ui.slider(hash!(), "Hue", 0.0..255., &mut self.hue);

                        if ui.button(Vec2::new(120., 5.), " + ") {
                            self.hue = clamp(self.hue + 1., 0., 255.);
                        };

                        if ui.button(Vec2::new(150., 5.), " - ") {
                            self.hue = clamp(self.hue - 1., 0., 255.);
                        };
                    });

                if ui.button(Vec2::new(5., SPACING * 4. + 5.), "Restart") {
                    self.showing = false;
                    updated = true;
                };

                ui.label(Vec2::new(65., SPACING * 4. + 5.), "Space to continue");
            });

        updated
    }
}
