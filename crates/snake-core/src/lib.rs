//! A continuously moving snake on a fixed board with solid walls.

use std::collections::VecDeque;
use std::time::Duration;

pub const SIDE: usize = 16;
const START_LENGTH: usize = 3;
pub const MAX_SPEED: usize = 9;
const BOARD_CLEAR_BONUS: usize = 100;
pub const MAX_SCORE: usize = (SIDE * SIDE - START_LENGTH) * MAX_SPEED + BOARD_CLEAR_BONUS;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Left, Self::Up, Self::Down, Self::Right];

    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    fn step(self, Point { x, y }: Point) -> Option<Point> {
        match self {
            Self::Left => x.checked_sub(1).map(|x| Point { x, y }),
            Self::Up => y.checked_sub(1).map(|y| Point { x, y }),
            Self::Right => (x + 1 < SIDE).then_some(Point { x: x + 1, y }),
            Self::Down => (y + 1 < SIDE).then_some(Point { x, y: y + 1 }),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Playing,
    HitWall,
    HitBody,
    BoardFull,
}

pub struct Game {
    body: VecDeque<Point>,
    food: Option<Point>,
    direction: Direction,
    next_direction: Option<Direction>,
    rng: u64,
    status: Status,
    speed: usize,
}

impl Game {
    pub fn new(seed: u64, speed: usize) -> Self {
        Self {
            body: VecDeque::from([
                Point { x: 6, y: 8 },
                Point { x: 5, y: 8 },
                Point { x: 4, y: 8 },
            ]),
            food: Some(Point { x: 10, y: 8 }),
            direction: Direction::Right,
            next_direction: None,
            rng: seed.max(1),
            status: Status::Playing,
            speed: speed.clamp(1, MAX_SPEED),
        }
    }

    pub fn restart(&mut self, speed: usize) {
        *self = Self::new(self.random(), speed);
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn body(&self) -> &VecDeque<Point> {
        &self.body
    }

    pub fn food(&self) -> Option<Point> {
        self.food
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn next_direction(&self) -> Option<Direction> {
        self.next_direction
    }

    pub fn score(&self) -> usize {
        let growth_score = (self.body.len() - START_LENGTH) * self.speed;
        if self.status == Status::BoardFull {
            growth_score + BOARD_CLEAR_BONUS
        } else {
            growth_score
        }
    }

    pub fn speed(&self) -> usize {
        self.speed
    }

    pub fn interval(&self) -> Duration {
        Duration::from_millis(850 - (self.speed as u64 - 1) * 50)
    }

    /// Buffer one valid turn per step, so a rapid double tap cannot reverse into the neck.
    pub fn steer(&mut self, direction: Direction) -> bool {
        if self.status != Status::Playing
            || self.next_direction.is_some()
            || direction == self.direction
            || direction == self.direction.opposite()
        {
            return false;
        }
        self.next_direction = Some(direction);
        true
    }

    pub fn advance(&mut self) {
        if self.status != Status::Playing {
            return;
        }
        if let Some(direction) = self.next_direction.take() {
            self.direction = direction;
        }
        let Some(next) = self.direction.step(self.body[0]) else {
            self.status = Status::HitWall;
            return;
        };

        let grew = self.food == Some(next);
        // A regular move can occupy the square its tail vacates.
        let occupied = self.body.len() - usize::from(!grew);
        if self.body.iter().take(occupied).any(|&cell| cell == next) {
            self.status = Status::HitBody;
            return;
        }
        if !grew {
            self.body.pop_back();
        }
        self.body.push_front(next);
        if grew {
            self.food = self.spawn_food();
            if self.food.is_none() {
                self.status = Status::BoardFull;
            }
        }
    }

    fn random(&mut self) -> u64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        self.rng
    }

    fn spawn_food(&mut self) -> Option<Point> {
        let empty = SIDE * SIDE - self.body.len();
        if empty == 0 {
            return None;
        }
        let choice = (self.random() % empty as u64) as usize;
        (0..SIDE * SIDE)
            .map(|i| Point {
                x: i % SIDE,
                y: i / SIDE,
            })
            .filter(|point| !self.body.contains(point))
            .nth(choice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_steps_move_the_tail_and_food_increases_length_and_score() {
        let mut game = Game::new(17, 5);
        let starting_interval = game.interval();
        assert_eq!(starting_interval, Duration::from_millis(650));
        game.advance();
        assert_eq!(game.body()[0], Point { x: 7, y: 8 });
        assert_eq!(game.body().back(), Some(&Point { x: 5, y: 8 }));
        assert_eq!(game.score(), 0);
        for _ in 0..3 {
            game.advance();
        }
        assert_eq!(game.score(), 5);
        assert_eq!(game.interval(), starting_interval);
        assert_eq!(game.body().len(), 4);
        let food = game.food().unwrap();
        assert!(!game.body().contains(&food));
    }

    #[test]
    fn reverse_and_extra_turns_are_ignored_until_the_next_step() {
        let mut game = Game::new(17, 1);
        assert!(!game.steer(Direction::Left));
        assert!(game.steer(Direction::Up));
        assert!(!game.steer(Direction::Left));
        assert_eq!(game.direction(), Direction::Right);
        game.advance();
        assert_eq!(game.body()[0], Point { x: 6, y: 7 });
        assert!(game.steer(Direction::Left));
        game.advance();
        assert_eq!(game.body()[0], Point { x: 5, y: 7 });
    }

    #[test]
    fn wall_ends_the_round_and_restart_clears_the_board_and_score() {
        let mut game = Game::new(11, 1);
        for _ in 6..SIDE {
            game.advance();
        }
        assert_eq!(game.status(), Status::HitWall);
        assert!(!game.steer(Direction::Down));
        game.restart(MAX_SPEED);
        assert_eq!(game.status(), Status::Playing);
        assert_eq!(game.score(), 0);
        assert_eq!(game.body()[0], Point { x: 6, y: 8 });
        assert_eq!(game.interval(), Duration::from_millis(450));
    }

    #[test]
    fn tail_will_vacate_but_hitting_another_body_segment_ends_the_round() {
        let mut game = Game::new(1, 1);
        game.food = Some(Point { x: 0, y: 0 });
        game.body = VecDeque::from([
            Point { x: 2, y: 2 },
            Point { x: 2, y: 3 },
            Point { x: 1, y: 3 },
            Point { x: 1, y: 2 },
        ]);
        game.direction = Direction::Up;
        assert!(game.steer(Direction::Left));
        game.advance();
        assert_eq!(game.status(), Status::Playing);
        assert_eq!(game.body()[0], Point { x: 1, y: 2 });
        game.body = VecDeque::from([
            Point { x: 2, y: 2 },
            Point { x: 2, y: 3 },
            Point { x: 1, y: 3 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 1 },
        ]);
        game.direction = Direction::Up;
        assert!(game.steer(Direction::Left));
        game.advance();
        assert_eq!(game.status(), Status::HitBody);
    }

    #[test]
    fn filling_the_last_square_completes_the_round() {
        let mut game = Game::new(1, MAX_SPEED);
        game.body = (0..SIDE * SIDE - 1)
            .rev()
            .map(|i| Point {
                x: i % SIDE,
                y: i / SIDE,
            })
            .collect();
        game.food = Some(Point {
            x: SIDE - 1,
            y: SIDE - 1,
        });
        game.direction = Direction::Right;
        game.advance();
        assert_eq!(game.status(), Status::BoardFull);
        assert_eq!(game.score(), MAX_SCORE);
        assert_eq!(game.food(), None);
        assert_eq!(game.interval(), Duration::from_millis(450));
    }
}
