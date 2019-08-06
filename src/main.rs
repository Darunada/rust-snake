//! The simplest possible example that does something.

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::graphics::{Color, Rect, DrawParam};


use rand;
use rand::Rng;
use std::collections::VecDeque;

const SQUARE_SIZE: i32 = 30; // pixels
const MAX_X: i32 = 20; // squares
const MAX_Y: i32 = 20;

pub enum Direction {
    LEFT,
    DOWN,
    RIGHT,
    UP,
}

trait Render {
    fn render(&self, ctx: &mut Context);
}

pub struct Square {
    x: i32,
    y: i32,
    color: Color
}

impl Render for Square {
    fn render(&self, ctx: &mut Context) {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new_i32(self.x * SQUARE_SIZE, self.y * SQUARE_SIZE, SQUARE_SIZE, SQUARE_SIZE),
            self.color,
        ).unwrap();
        graphics::draw(ctx, &square, DrawParam::default()).unwrap();
    }
}


pub struct Apple {
    location: Square
}

impl Apple {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Apple {
            location: Square {
                x: rng.gen::<i32>() % MAX_X,
                y: rng.gen::<i32>() % MAX_Y,
                color: Color {
                    r: (rng.gen::<f32>() % 255.0) / 255.0,
                    g: (rng.gen::<f32>() % 255.0) / 255.0,
                    b: (rng.gen::<f32>() % 255.0) / 255.0,
                    a: 1.0
                }
            }
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        self.location.render(ctx);
    }
}


pub struct Snake {
    segments: VecDeque<Square>,
}

impl Snake  {
    pub fn new() -> Snake {
        Snake {
            segments:  vec![
                Square {
                    x: MAX_X / 2,
                    y: MAX_Y / 2,
                    color: graphics::WHITE
                },
                Square {
                    x: MAX_X / 2 + 1,
                    y: MAX_Y / 2,
                    color: graphics::WHITE
                }
            ].into_iter().collect()
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        for s in self.segments.iter() {
            s.render(ctx);
        }
    }

    fn get_next_head(&self, direction: Direction) -> Square {
        let head = self.segments.front().unwrap();

        match direction {
            Direction::LEFT => {
                Square {
                    x: head.x - 1,
                    y: head.y,
                    color: head.color.clone()
                }
            },
            Direction::DOWN => {
                Square {
                    x: head.x,
                    y: head.y - 1,
                    color: head.color.clone()
                }
            },
            Direction::RIGHT => {
                Square {
                    x: head.x + 1,
                    y: head.y,
                    color: head.color.clone()
                }
            },
            Direction::UP => {
                Square {
                    x: head.x,
                    y: head.y + 1,
                    color: head.color.clone()
                }
            }
        }
    }

    pub fn head(&self) -> &Square {
        self.segments.front().unwrap()
    }

    pub fn move_snake(&mut self, direction: Direction) -> Square {
        let new_head = self.get_next_head(direction);
        self.segments.push_front(new_head);
        self.segments.pop_back().unwrap()
    }

    pub fn grow_snake(&mut self, direction: Direction) {
        let tail_end = self.move_snake(direction);
        self.segments.push_back(tail_end);
    }

    pub fn collides(&self, square: &Square) {

    }
}



struct GameState {
    snake: Snake,
    apples: Vec<Apple>,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState { snake: Snake::new(), apples: vec![Apple::new()] })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.snake.move_snake(Direction::LEFT);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.snake.render(ctx);
        for apple in self.apples.iter() {
            apple.render(ctx);
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("snake", "leaf");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}