use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Rect};
use ggez::{Context, GameResult};

use cgmath::num_traits::abs;
use ggez::conf::*;
use ggez::event::{quit, KeyCode, KeyMods};
use rand;
use rand::Rng;
use std::collections::VecDeque;
use std::thread::sleep;
use std::time;

const SQUARE_SIZE: i32 = 30; // pixels
const MAX_X: i32 = 20; // squares
const MAX_Y: i32 = 20;

#[derive(Debug)]
pub enum Direction {
    Left,
    Down,
    Right,
    Up,
}

trait Render {
    fn render(&self, ctx: &mut Context);
}

#[derive(Debug)]
pub struct Square {
    x: i32,
    y: i32,
    color: Color,
}

impl Render for Square {
    fn render(&self, ctx: &mut Context) {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new_i32(
                self.x * SQUARE_SIZE,
                self.y * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
            ),
            self.color,
        )
        .unwrap();
        graphics::draw(ctx, &square, DrawParam::default()).unwrap();
    }
}

#[derive(Debug)]
pub struct Apple {
    location: Square,
}

impl Apple {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let apple = Apple {
            location: Square {
                x: abs(rng.gen::<i32>() % MAX_X),
                y: abs(rng.gen::<i32>() % MAX_Y),
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            },
        };

        //        println!("{:?}", apple);
        apple
    }

    pub fn render(&self, ctx: &mut Context) {
        self.location.render(ctx);
    }
}

#[derive(Debug)]
pub struct Snake {
    segments: VecDeque<Square>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            segments: vec![
                Square {
                    x: MAX_X / 2,
                    y: MAX_Y / 2,
                    color: graphics::WHITE,
                },
                Square {
                    x: MAX_X / 2 + 1,
                    y: MAX_Y / 2,
                    color: graphics::WHITE,
                },
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        for s in self.segments.iter() {
            s.render(ctx);
        }
    }

    fn get_next_head(&self, direction: &Direction) -> Square {
        let head = self.segments.front().unwrap();

        let mut new_head = match direction {
            Direction::Left => Square {
                x: head.x - 1,
                y: head.y,
                color: head.color.clone(),
            },
            Direction::Down => Square {
                x: head.x,
                y: head.y + 1,
                color: head.color.clone(),
            },
            Direction::Right => Square {
                x: head.x + 1,
                y: head.y,
                color: head.color.clone(),
            },
            Direction::Up => Square {
                x: head.x,
                y: head.y - 1,
                color: head.color.clone(),
            },
        };

        new_head.x = if new_head.x < 0 {
            MAX_X
        } else {
            new_head.x % MAX_X
        };
        new_head.y = if new_head.y < 0 {
            MAX_Y
        } else {
            new_head.y % MAX_Y
        };
        new_head
    }

    pub fn head(&self) -> &Square {
        self.segments.front().unwrap()
    }

    pub fn overlaps(&self) -> bool {
        let head = self.head();
        let mut matches = 0;
        for s in self.segments.iter() {
            if head.x == s.x && head.y == s.y {
                matches += 1;
            }
        }

        return matches > 1;
    }

    pub fn move_snake(&mut self, direction: &Direction, grow: bool) {
        let new_head = self.get_next_head(direction);
        self.segments.push_front(new_head);
        let tail_end = self.segments.pop_back().unwrap();

        if grow {
            self.segments.push_back(tail_end);
        }
    }

    pub fn collides(&self, square: &Square) -> bool {
        for segment in self.segments.iter() {
            if segment.x == square.x && segment.y == square.y {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct GameState {
    snake: Snake,
    apple: Apple,
    direction: Direction,
    dead: bool,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        Ok(GameState {
            snake: Snake::new(),
            apple: Apple::new(),
            direction: Direction::Left,
            dead: false,
        })
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // collision detection w/ self
        self.dead = self.snake.overlaps();

        if !self.dead {
            // growing from apples?
            let mut grow = false;
            if self.snake.collides(&self.apple.location) {
                grow = true;
                self.apple = Apple::new();
            }

            // move and grow
            self.snake.move_snake(&self.direction, grow);
        }

        sleep(time::Duration::from_millis(150));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        self.apple.render(ctx);
        self.snake.render(ctx);

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let direction = match keycode {
            KeyCode::Escape => {
                quit(ctx);
                None
            }
            KeyCode::Return => {
                let game_state = GameState::new().unwrap();
                self.snake = game_state.snake;
                self.direction = game_state.direction;
                self.dead = game_state.dead;
                self.apple = game_state.apple;
                None
            }
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Down => Some(Direction::Down),
            _ => None,
        };

        if direction.is_some() {
            self.direction = direction.unwrap();
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("snake", "leaf")
        .window_mode(
            WindowMode::default()
                .dimensions((MAX_X * SQUARE_SIZE) as f32, (MAX_Y * SQUARE_SIZE) as f32)
                .resizable(false),
        )
        .window_setup(WindowSetup::default().title("Snake"));

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
