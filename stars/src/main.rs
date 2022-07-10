use std::fmt::{Display, Formatter};
use nannou::prelude::*;
use nannou::{rand};
use nannou_egui::{self, egui, Egui};
use nannou::rand::Rng;

// Stores settings for egui
struct Settings {
    amount: usize,
    color: Srgba<f32>,
    min_decrease: f32,
    max_decrease: f32,
    radius: f32,
}

// Stores the coordinates and the alpha value, we could've used a Point2D too
struct Star {
    x: f32,
    y: f32,
    a: f32,
}

impl Star {
    // Create a new star
    fn new(bounds: &(u32, u32)) -> Star {
        let mut rng = rand::thread_rng();
        Star {
            // Since nannou handles coords from the center, this should be the way to get to generate the star at a random position
            // on the screen
            x: rng.gen_range(-((bounds.0/2) as i32)..((bounds.0/2) as i32)) as f32,
            y: rng.gen_range(-((bounds.1/2) as i32)..((bounds.1/2) as i32)) as f32,
            a: 1.,
        }
    }

    // Updates the "star" alpha, if the star doesn't exist anymore teleport it in another position, completely opaque
    fn update(&mut self, bounds: &(u32, u32), min_decrease: f32, max_decrease: f32) {
        let mut rng = rand::thread_rng();
        self.a = self.a - rng.gen_range(min_decrease..max_decrease);
        if self.a < 0. {
            self.a = 1.;
            self.x = rng.gen_range(-(bounds.0 as i32)..(bounds.0 as i32)) as f32;
            self.y = rng.gen_range(-(bounds.1 as i32)..(bounds.1 as i32)) as f32;
        }
    }
}

impl Display for Star {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Star x: {} y: {} alpha: {}", self.x, self.y, self.a)
    }
}

// We need to store the stars, settings and egui
struct Model {
    stars: Vec<Star>,
    settings: Settings,
    egui: Egui,
}

fn main() {
    nannou::app(model)
        .event(event)
        //.simple_window(view) // We don't need to have 2 windows
        .update(update)
        .size(1920,1080)
        .run();
}

fn model(_app: &App) -> Model {
    let stars = Vec::new();

    let window_id = _app
        .new_window() // I think we could've made the window in main and then use .main_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = _app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);
    // Default values
    Model {egui, settings: Settings{amount: 100, color: srgba(1.,1.,1.,1.), radius: 8., min_decrease: 0.001, max_decrease: 0.1 }, stars}
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    // println!("{:?}",_app.main_window().inner_size_pixels());
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let egui = &mut _model.egui;
    let settings = &mut _model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();

    // EGUI Menu
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Amount:");
        ui.add(egui::Slider::new(&mut settings.amount, 1..=10000)); // Value, Limit
        ui.label("Min Decrease amount:");
        ui.add(egui::Slider::new(&mut settings.min_decrease, 0. ..= settings.max_decrease-0.0000001));
        ui.label("Max Decrease amount:");
        ui.add(egui::Slider::new(&mut settings.max_decrease, settings.min_decrease+0.0000001 ..= 1.));
        ui.label("Radius:");
        ui.add(egui::Slider::new(&mut settings.radius, 0.5 ..= 64.));


        let clicked = ui.button("Random color").clicked();

        if clicked {
            settings.color = srgba(random(),random(),random(), 1.);
        }
    });

    // Get screen bounds to know where to draw the stars
    let bounds = _app.main_window().inner_size_pixels();

    // This is before the star amount check because we don't need to update the stars twice on the same frame
    // I.E. if we increase the star amount, we do not need to go over this part of the vector again
    for star in &mut _model.stars {
        star.update(&bounds, settings.min_decrease, settings.max_decrease);
    }

    // If there are less stars than needed
    if settings.amount > _model.stars.len() {
        for _ in _model.stars.len()..settings.amount {
            _model.stars.push(Star::new(&bounds))
        }
    } else if settings.amount < _model.stars.len() { // If there are more stars than needed
        _model.stars.drain(settings.amount.._model.stars.len());
    }
}


fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();

    // let p = Point2::new(50.,70.);
    // let p2 = Point2::new(500.,70.);
    // draw.line().start(p).end(p2).color(GRAY).stroke_weight(5.);

    draw.background().color(BLACK);

    // Draw each star
    for star in &_model.stars {
        let r = _model.settings.radius;
        let mut color = _model.settings.color;
        color.alpha = star.a; // Change alpha to create fade effect
        draw.ellipse().x(star.x).y(star.y).color(color).radius(r);
    }

    // Draw everything to the frame
    draw.to_frame(_app, &_frame).unwrap();
    // Overlay the gui
    _model.egui.draw_to_frame(&_frame).unwrap();
}