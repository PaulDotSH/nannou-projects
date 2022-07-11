use nannou::prelude::*;
use nannou::rand::{rand, Rng};

struct Model {
    drops: Vec<Drop>,
}

struct Drop {
    pos: Point2,
    z: f32,
    len: f32,
    speed: f32,
}

impl Drop {
    fn update(&mut self, bounds: &(u32, u32)) {
        self.pos.y -= self.speed;

        let grav = map_range(self.z, 0., 20., 0., 0.2);
        self.speed += grav;

        if self.pos.y < -(bounds.1 as f32) {
            let mut rng = rand::thread_rng();
            self.pos.x = rng.gen_range(-(bounds.0 as i32)..(bounds.0 as i32)) as f32;
            self.pos.y = bounds.1 as f32 /2.;
            self.speed = map_range(self.z, 0., 20., 3., 8.);
            self.z = rng.gen_range(0. .. 20.);
            self.len = map_range(self.z, 0., 20., 1., 20.);
        }
        // println!("X: {} Y: {}",self.pos.x, self.pos.y);
    }

    fn new(bounds: &(u32, u32)) -> Drop {
        let mut rng = rand::thread_rng();

        // Start offscreen
        let x = rng.gen_range(-(bounds.0 as i32)..(bounds.0 as i32)) as f32;
        let y = bounds.1 as f32 / 2. + rng.gen_range(10. .. 250.);
        let z = rng.gen_range(0. .. 20.);
        let len = map_range(z, 0., 20., 1., 20.);
        let pos = Point2::new(x, y);
        Drop { pos, speed: 1., z, len }
    }
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .update(update)
        .size(1400,800)
        .run();
}

fn model(_app: &App) -> Model {
    let mut drops = Vec::new();
    let bounds = _app.main_window().inner_size_pixels();
    println!("Screen size: {}x{}", bounds.0,bounds.1);
    for _ in 0..2000 {
        drops.push(Drop::new(&bounds))
    }
    Model { drops }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    // println!("{:?}",_app.main_window().inner_size_pixels());
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    let bounds = _app.main_window().inner_size_pixels();
    for drop in _model.drops.iter_mut() {
        drop.update(&bounds);
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(BLACK);

    for drop in &_model.drops {
        let mut pos2 = drop.pos;
        pos2.y += drop.len;
        let weight = map_range(drop.z,0.,20.,1.,3.);
        draw.line().start(drop.pos).end(pos2).color(PURPLE).stroke_weight(weight);
    }

    draw.to_frame(_app, &_frame).unwrap();
}