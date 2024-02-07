use nannou::prelude::*;
use nannou::rand::{rand, Rng};
use nannou::Event::WindowEvent;
use nannou_egui::{self, egui, Egui};
use std::iter;

struct Cell {
    coords: Point2,
    radius: f32,
    color: Srgba,
}

impl Cell {
    fn new(bounds: &(u32, u32), radius: f32, color: Srgba) -> Cell {
        let mut rng = rand::thread_rng();
        let coords = Point2::new(
            rng.gen_range(-((bounds.0 / 2) as i32)..((bounds.0 / 2) as i32)) as f32,
            rng.gen_range(-((bounds.1 / 2) as i32)..((bounds.1 / 2) as i32)) as f32,
        );
        Cell {
            coords,
            radius,
            color
            // color: srgba(1.,1.,1.,1.)
        }
    }

    fn update(&mut self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        let mut rng = rand::thread_rng();
        self.coords.x -= rng.gen_range(min_x..=max_x);
        self.coords.y -= rng.gen_range(min_y..=max_y);
    }

    fn split(&mut self) -> Cell {
        self.radius /= 2.;
        self.color.alpha -= 0.15;
        let mut rng = rand::thread_rng();
        self.color.blue = rng.gen_range(0.0 .. 1.0);
        self.color.red = rng.gen_range(0.0 .. 1.0);
        self.color.green = rng.gen_range(0.0 .. 1.0);
        let new_cell = Cell {
            coords: Point2::new(self.coords.x, self.coords.y),
            radius: self.radius,
            color: self.color,
        };
        self.coords.x -= rng.gen_range(-self.radius / 2.0..self.radius / 2.);
        self.coords.y -= rng.gen_range(-self.radius / 2.0..self.radius / 2.);
        new_cell
    }
}

struct Model {
    settings: Settings,
    cells: Vec<Cell>,
    egui: Egui,
}

struct Settings {
    min_move_x: f32,
    max_move_x: f32,
    min_move_y: f32,
    max_move_y: f32,
    new_cell_radius: f32,
}

fn main() {
    nannou::app(model)
        .event(event)
        // .simple_window(view)
        .update(update)
        .size(1000, 800)
        .run();
}

fn model(_app: &App) -> Model {
    let window_id = _app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = _app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    let bounds = _app.main_window().inner_size_pixels();
    let settings = Settings {
        min_move_x: -1.,
        max_move_x: 1.,
        min_move_y: -1.,
        max_move_y: 1.,
        new_cell_radius: 25.,
    };
    let cells: Vec<Cell> = iter::repeat_with(|| Cell::new(&bounds, settings.new_cell_radius, random_color())).take(5).collect();
    Model {
        settings,
        egui,
        cells,
    }
}

fn random_color() -> Srgba {
    let mut rng = rand::thread_rng();
    srgba(rng.gen_range(0.0 .. 1.0), rng.gen_range(0.0 .. 1.0), rng.gen_range(0.0 .. 1.0), 1.)
}

fn raw_window_event(_app: &App, _model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    _model.egui.handle_raw_event(event);
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        WindowEvent {
            simple: Some(MousePressed(mb)),
            ..
        } => {
            let mouse_pos = app.mouse.position();
            match mb {
                MouseButton::Left => {
                    if let Some(rev_index) = model.cells.iter().rev().position(|cell| is_inside_circle(&mouse_pos, cell)) {
                        let index = model.cells.len() - 1 - rev_index; // Apparently index returned by .iter().rev() is reversed...
                        let new_cell = model.cells[index].split();
                        model.cells.push(new_cell);
                    }
                }
                MouseButton::Right => {
                    if let Some(rev_index) = model
                        .cells
                        .iter()
                        .rev()
                        .position(|cell| is_inside_circle(&mouse_pos, cell))
                    {
                        let index = model.cells.len() - 1 - rev_index; // Apparently index returned by .iter().rev() is reversed...
                        model.cells.remove(index);
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn is_inside_circle(mouse_pos: &Point2, cell: &Cell) -> bool {
    mouse_pos.distance(cell.coords) <= cell.radius
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Boilerplate
    let egui = &mut _model.egui;
    let settings = &mut _model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();

    let bounds = _app.main_window().inner_size_pixels();

    for star in &mut _model.cells {
        star.update(
            settings.min_move_x,
            settings.max_move_x,
            settings.min_move_y,
            settings.max_move_y,
        );
    }

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label(format!("Amount: {}", _model.cells.len()));
        ui.label("Min move x:");
        ui.add(egui::Slider::new(&mut settings.max_move_x, 0. ..=5.));
        ui.label("Max move x:");
        ui.add(egui::Slider::new(&mut settings.min_move_x, -5. ..=0.));
        ui.label("Min move y:");
        ui.add(egui::Slider::new(&mut settings.max_move_y, 0. ..=5.));
        ui.label("Max move y:");
        ui.add(egui::Slider::new(&mut settings.min_move_y, -5. ..=0.));
        ui.label("New cell radius:");
        ui.add(egui::Slider::new(&mut settings.new_cell_radius,  1.0..=100.));

        let spawn_cell_clicked = ui.button("Spawn cell").clicked();
        let clear_clicked = ui.button("Clear cells").clicked();

        if spawn_cell_clicked {
            _model.cells.push(Cell::new(&bounds, settings.new_cell_radius, random_color()))
        } else if clear_clicked {
            _model.cells.clear();
        }
    });
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    for cell in _model.cells.iter() {
        draw.ellipse()
            .xy(cell.coords)
            .radius(cell.radius)
            .color(cell.color)
            .stroke_weight(1.);
    }

    draw.to_frame(_app, &_frame).unwrap();
    _model.egui.draw_to_frame(&_frame).unwrap();
}
