use nannou::prelude::*;
use nannou::rand::{rand, Rng};
use nannou_egui::{self, egui, Egui};
use crate::egui::Color32;
use crate::egui::color_picker::Alpha;
use crate::wgpu::PolygonMode::Point;

struct Star {
    coords: Point2,
    prev_coords: Point2,
}

impl Star {
    fn new(bounds: &(u32, u32)) -> Star {
        let mut rng = rand::thread_rng();
        let coords = Point2::new(
            rng.gen_range(-((bounds.0/2) as i32)..((bounds.0/2) as i32)) as f32,
            rng.gen_range(-((bounds.1/2) as i32)..((bounds.1/2) as i32)) as f32);
        Star {coords, prev_coords: coords }
    }

    fn update(&mut self, bounds: &(u32, u32)) {

        if !(self.coords.x < (bounds.0/2) as f32 && self.coords.x > -((bounds.0 / 2) as f32)) &&
            !(self.coords.y < (bounds.1/2) as f32 && self.coords.y > -((bounds.0 / 2) as f32)) {
            let mut rng = rand::thread_rng();
            self.coords.x = rng.gen_range(-((bounds.0/2) as i32)..((bounds.0/2) as i32)) as f32;
            self.coords.y = rng.gen_range(-((bounds.1/2) as i32)..((bounds.1/2) as i32)) as f32;
            self.prev_coords = self.coords;
            return;
            }

        self.prev_coords = self.coords;
        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(1. .. 15.);
        if self.coords.x > 0. {
            // 1st quadrant
            if self.coords.y > 0. {
                self.coords.x += offset;
                self.coords.y += offset+1.; // TODO: make a slider for this
            } else { // 4th quadrant
                self.coords.x += offset;
                self.coords.y -= offset+1.;
            }
        } else {
            if self.coords.y > 0. { // 2nd quadrant
                self.coords.x -= offset;
                self.coords.y += offset+1.;
            } else { // 3rd quadrant
                self.coords.x -= offset;
                self.coords.y -= offset+1.;
            }
        }
    }
}

struct Model {
    settings: Settings,
    stars: Vec<Star>,
    egui: Egui,
}

struct Settings {
    amount: usize,
}

fn main() {
    nannou::app(model)
        // .event(event)
        // .simple_window(view)
        .update(update)
        .size(1920,1080)
        .run();
}

fn model(_app: &App) -> Model {
    let window_id = _app
        .new_window() // I think we could've made the window in main and then use .main_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let mut stars = Vec::new();

    let window = _app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    Model { settings: Settings { amount: 150 }, egui, stars }
}

fn raw_window_event(_app: &App, _model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    _model.egui.handle_raw_event(event);
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let egui = &mut _model.egui;
    let settings = &mut _model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();

    let bounds = _app.main_window().inner_size_pixels();
    if settings.amount > _model.stars.len() {
        for _ in _model.stars.len()..settings.amount {
            _model.stars.push(Star::new(&bounds))
        }
    } else if settings.amount < _model.stars.len() { // If there are more stars than needed
        _model.stars.drain(settings.amount.._model.stars.len());
    }

    for star in &mut _model.stars {
        star.update(&bounds);
    }
    let mut c = Color32::BLACK;

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Amount:");
        ui.add(egui::Slider::new(&mut settings.amount, 1..=10000)); // Value, Limit
    });

}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    for star in _model.stars.iter() {
        draw.line().start(star.prev_coords).end(star.coords).color(WHITE).stroke_weight(1.);
    }

    draw.to_frame(_app, &_frame).unwrap();
    _model.egui.draw_to_frame(&_frame).unwrap();
}