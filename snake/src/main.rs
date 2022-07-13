use nannou::prelude::*;
use nannou::{rand};
use nannou::rand::Rng;
use nannou::text::FontSize;

struct Model {
    main_window: WindowId,
    food_pos: Point2, // Food position
    snake: Snake, // Player
    game_over: bool, // Game state
    // settings: Settings,
    // egui: Egui,
}

// These should be constant, and maybe add a multiplier
const BLOCK_SIZE: f32 = 8.;
const MOVEMENT_SPEED: f32 = 8.;

#[derive(Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Stores the player's snake
struct Snake {
    pos: Vec<Point2>,
    direction: Direction,
}

impl Snake {
    // Used to check for game over
    fn is_self_collision(&self) -> bool {
        for i in 1..self.pos.len() {
            if self.pos[0]==self.pos[i] {
                return true
            }
        }
        false
    }

    // Updates the positions of all the body segments in the pos vector
    fn update_position(&mut self) {
        // First we update the body
        let mut i = self.pos.len()-1;
        while i>0 {
            self.pos[i]=self.pos[i-1];
            i-=1;
        }

        // Then update the head
        let head = &mut self.pos[0];
        match self.direction {
            Direction::Up =>    { head.y += MOVEMENT_SPEED },
            Direction::Down =>  { head.y -= MOVEMENT_SPEED },
            Direction::Left =>  { head.x -= MOVEMENT_SPEED },
            Direction::Right => { head.x += MOVEMENT_SPEED },
        }
    }

    // Check if head is in the screen bounds
    fn is_head_in(&self, bounds: &(u32, u32)) -> bool {
        let x =  (self.pos[0].x.abs()*2.) as u32;
        let y = (self.pos[0].y.abs()*2.) as u32;
        // Not sure why the math above doesn't work perfectly
        if x+(x/10) >= bounds.0 || y+(y/10) >= bounds.1 {
            false
        } else {
            true
        }
    }

    // Check if head is on specific position, used for food checking
    fn is_head_on_pos(&self, pos: Point2) -> bool {
        if self.pos.len() > 0 && self.pos[0] == pos {
            true
        } else {
            false
        }
    }

    // Adds a segment at the end of the snake
    fn add_segment(&mut self) {
        let mut last = self.pos[self.pos.len()-1];
        match self.direction {
            Direction::Up =>    { last.y += MOVEMENT_SPEED },
            Direction::Down =>  { last.y -= MOVEMENT_SPEED },
            Direction::Left =>  { last.x -= MOVEMENT_SPEED },
            Direction::Right => { last.x += MOVEMENT_SPEED },
        }
        self.pos.push(last);
    }
}

impl Model {
    // Gets a random position in the bounds
    fn get_random_position(bounds: &(u32, u32)) -> Point2 {
        let bounds = (bounds.0 as i32 - 100, bounds.1 as i32 - 100);
        let mut rng = rand::thread_rng();
        let bls = BLOCK_SIZE as i32;
        // Maybe mapping the values would be a better way of doing this
        let mut pos = Point2::new(rng.gen_range(-(bounds.0-bls)/2..(bounds.0-bls)/2) as f32,rng.gen_range(-(bounds.1-bls)/2..(bounds.1-bls)/2) as f32 - 50.);
        let posx = pos.x as i32;
        let posy = pos.y as i32;
        if posx % bls !=0 {
           pos.x = (posx + bls - posx % bls) as f32;
        }
        if posy % bls !=0 {
            pos.y = (posy + bls - posy % bls) as f32;
        }
        pos
    }
    fn randomise_food_position(&mut self, bounds: &(u32, u32)) {
        self.food_pos = Model::get_random_position(bounds);
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

// Pretty clunky since this is at a different "framerate" than the game itself, the user can do stuff
// they shouldn't be able to. Maybe fix
fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {
    match _key {
        Key::Up => { if _model.snake.direction != Direction::Down {_model.snake.direction = Direction::Up } }
        Key::Down => { if _model.snake.direction != Direction::Up {_model.snake.direction = Direction::Down } }
        Key::Left => { if _model.snake.direction != Direction::Right {_model.snake.direction = Direction::Left } }
        Key::Right => { if _model.snake.direction != Direction::Left {_model.snake.direction = Direction::Right } }
        _ => {}
    }

}

fn model(_app: &App) -> Model {
    let main_window = _app.new_window()
        .size(800, 800)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut pos = Vec::<Point2>::new();
    pos.push(Point2::new(0.,0.));

    Model{ main_window, food_pos: Point2::new(32.,16.), snake: Snake { pos, direction: Direction::Right }, game_over: false }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    if _app.elapsed_frames() % 10 != 0 { // todo fix
        return;
    }

    // If the game is over, we shouldn't continue to do calculations in the "backend"
    if _model.game_over {
        return;
    }

    let bounds = _app.main_window().inner_size_pixels();
    // println!("bounds: {:?} pos: {}", bounds, _model.snake.pos[0] );

    _model.snake.update_position();
    if _model.snake.is_self_collision() || !_model.snake.is_head_in(&bounds) {
        _model.game_over = true;
        return;
    }
    if _model.snake.is_head_on_pos(_model.food_pos) {
        _model.snake.add_segment();
        _model.randomise_food_position(&bounds);
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    if _app.elapsed_frames() % 10 != 0 { // todo fix
        return;
    }

    let draw = _app.draw();
    draw.background().color(BLACK);

    if _model.game_over {
        draw.text("GAME OVER!").font_size(32 as FontSize);
        draw.to_frame(_app, &_frame).unwrap();
        return;
    }

    // Food
    draw.quad().xy(_model.food_pos).w_h(BLOCK_SIZE,BLOCK_SIZE).color(RED);

    // Snake
    for pos in _model.snake.pos.iter() {
        draw.quad().xy(*pos).w_h(BLOCK_SIZE,BLOCK_SIZE).color(WHITE);
    }

    draw.to_frame(_app, &_frame).unwrap();
}