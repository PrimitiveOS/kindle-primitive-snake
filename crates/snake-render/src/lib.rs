use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use primitive_app::{App, Change};
use primitive_ui::{
    back_button, header, screen_rect, Canvas, Icon, Input, Rect, TextStyle, BLACK, BODY, CAPTION,
    PAPER,
};
use snake_core::{Direction, Game, Point, Status, MAX_SCORE, MAX_SPEED, SIDE};

const BOARD_X: i32 = 152;
const BOARD_Y: i32 = 318;
const CELL: i32 = 60;
const BOARD_SIZE: i32 = CELL * SIDE as i32;

#[derive(Clone, Copy)]
enum Phase {
    Ready,
    SelectSpeed,
    Running(Instant),
    Paused,
    Finished,
}

pub struct View {
    pub game: Game,
    phase: Phase,
    best: usize,
    saved_best: usize,
    path: Option<PathBuf>,
    save_failed: bool,
}

impl View {
    pub fn open(path: PathBuf) -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(1, |elapsed| elapsed.as_nanos() as u64);
        let best = File::open(&path)
            .ok()
            .and_then(|file| {
                let mut text = String::new();
                file.take(32).read_to_string(&mut text).ok()?;
                text.trim().parse::<usize>().ok()
            })
            .filter(|&score| score <= MAX_SCORE)
            .unwrap_or(0);

        Self {
            game: Game::new(seed, 1),
            phase: Phase::Ready,
            best,
            saved_best: best,
            path: Some(path),
            save_failed: false,
        }
    }

    pub fn preview(seed: u64) -> Self {
        Self {
            game: Game::new(seed, 1),
            phase: Phase::Ready,
            best: 0,
            saved_best: 0,
            path: None,
            save_failed: false,
        }
    }

    pub fn best(&self) -> usize {
        self.best
    }

    fn persist(&mut self) {
        self.save_failed = self.save().is_err();
    }

    fn pause(&mut self) {
        if matches!(self.phase, Phase::Running(_)) {
            self.phase = Phase::Paused;
        }
    }

    fn action(&mut self, now: Instant) -> Change {
        match self.phase {
            Phase::Ready | Phase::Paused => {
                self.phase = Phase::Running(now + self.game.interval());
            }
            Phase::Running(_) => {
                self.pause();
                self.persist();
            }
            Phase::Finished => {
                self.game.restart(self.game.speed());
                self.phase = Phase::Ready;
            }
            Phase::SelectSpeed => return Change::None,
        }
        Change::Partial
    }

    fn steer(&mut self, direction: Direction) -> Change {
        if matches!(self.phase, Phase::Running(_)) && self.game.steer(direction) {
            Change::Partial
        } else {
            Change::None
        }
    }

    fn tap(&mut self, x: f32, y: f32) -> Change {
        let width = primitive_app::WIDTH as i32;
        if matches!(self.phase, Phase::SelectSpeed) {
            if back_button(width).contains(x, y) {
                self.phase = Phase::Ready;
                return Change::Full;
            }
            if let Some(speed) =
                (1..=MAX_SPEED).find(|&speed| speed_choice(width, speed).contains(x, y))
            {
                self.game.restart(speed);
                self.phase = Phase::Ready;
                return Change::Full;
            }
            return Change::None;
        }
        if back_button(width).contains(x, y) {
            return Change::Exit;
        }
        if matches!(self.phase, Phase::Ready) && speed_button(width).contains(x, y) {
            self.phase = Phase::SelectSpeed;
            return Change::Full;
        }
        if action_button(width).contains(x, y) {
            return self.action(Instant::now());
        }
        for (i, direction) in Direction::ALL.into_iter().enumerate() {
            if direction_button(width, i).contains(x, y) {
                return self.steer(direction);
            }
        }
        Change::None
    }

    fn draw_speed_choices(&self, canvas: &mut Canvas, width: i32) {
        header(canvas, "Round speed", "Back");
        canvas.text(
            screen_rect(width, 40, 248, 1184, 64),
            "1 is slowest. 9 is fastest.",
            BODY.center(),
        );
        for speed in 1..=MAX_SPEED {
            canvas.button(
                speed_choice(width, speed),
                &speed.to_string(),
                speed == self.game.speed(),
                false,
            );
        }
    }

    fn draw_board(&self, canvas: &mut Canvas, width: i32) {
        let field = board(width);
        canvas.panel(field, 22, 4, PAPER);
        let rail = canvas.px(4);
        let grid = canvas.px(1).max(1);
        for line in 1..SIDE as i32 {
            let offset = canvas.px(line * CELL);
            canvas.fill(
                Rect::new(field.x + offset, field.y + rail, grid, field.h - 2 * rail),
                200,
            );
            canvas.fill(
                Rect::new(field.x + rail, field.y + offset, field.w - 2 * rail, grid),
                200,
            );
        }
        let margin = canvas.px(4);
        let link_inset = canvas.px(15);
        let body = self.game.body();
        for (&first, &second) in body.iter().zip(body.iter().skip(1)) {
            let a = cell(width, first);
            let b = cell(width, second);
            let bridge = if first.y == second.y {
                let left = a.x.min(b.x) + margin;
                let right = a.right().max(b.right()) - margin;
                Rect::new(left, a.y + link_inset, right - left, a.h - 2 * link_inset)
            } else {
                let top = a.y.min(b.y) + margin;
                let bottom = a.bottom().max(b.bottom()) - margin;
                Rect::new(a.x + link_inset, top, a.w - 2 * link_inset, bottom - top)
            };
            canvas.fill(bridge, BLACK);
        }
        for &part in body {
            canvas.panel(cell(width, part).inset(margin), 12, 1, BLACK);
        }
        if let Some(&point) = body.front() {
            let head = cell(width, point);
            let dot = canvas.px(8);
            let forward = canvas.px(16);
            let apart = canvas.px(9);
            let center = (head.x + head.w / 2, head.y + head.h / 2);
            let (a, b) = match self.game.direction() {
                Direction::Up => ((-apart, -forward), (apart, -forward)),
                Direction::Down => ((-apart, forward), (apart, forward)),
                Direction::Left => ((-forward, -apart), (-forward, apart)),
                Direction::Right => ((forward, -apart), (forward, apart)),
            };
            for (dx, dy) in [a, b] {
                canvas.circle(
                    Rect::new(center.0 + dx - dot / 2, center.1 + dy - dot / 2, dot, dot),
                    PAPER,
                );
            }
        }
        if let Some(food) = self.game.food() {
            let bounds = cell(width, food).inset(canvas.px(9));
            canvas.icon(
                bounds,
                Icon {
                    stroke: &["M12 2L22 12L12 22L2 12Z"],
                    fill: &["M12 8L16 12L12 16L8 12Z"],
                },
                BLACK,
            );
        }
    }
}

impl App for View {
    fn draw(&self, canvas: &mut Canvas) {
        let w = canvas.bounds().w;
        if matches!(self.phase, Phase::SelectSpeed) {
            self.draw_speed_choices(canvas, w);
            return;
        }
        header(canvas, "Snake", "Exit");

        canvas.card(screen_rect(w, 40, 162, 1184, 100));
        let values = [
            ("Score", self.game.score()),
            ("Best", self.best),
            ("Speed", self.game.speed()),
        ];
        for (i, (label, value)) in values.into_iter().enumerate() {
            if i == 2 && matches!(self.phase, Phase::Ready) {
                canvas.button(speed_button(w), &format!("Speed {value}"), false, false);
                continue;
            }
            let left = 40 + i as i32 * 394;
            canvas.text(screen_rect(w, left, 172, 394, 37), label, CAPTION.center());
            canvas.text(
                screen_rect(w, left, 206, 394, 52),
                &value.to_string(),
                TextStyle::new(38).bold().center(),
            );
        }

        let (guidance, action) = match self.phase {
            Phase::SelectSpeed => unreachable!(),
            Phase::Ready => ("Press Start to play.", "Start"),
            Phase::Running(_) => ("Eat food to grow.", "Pause"),
            Phase::Paused => ("Paused", "Resume"),
            Phase::Finished => (
                match self.game.status() {
                    Status::HitWall => "You hit a wall.",
                    Status::HitBody => "You hit your own body.",
                    Status::BoardFull => "Board full. 100-point bonus.",
                    Status::Playing => "Round finished.",
                },
                "New round",
            ),
        };
        canvas.text(
            screen_rect(w, 40, 269, 1184, 47),
            guidance,
            BODY.bold().center(),
        );
        self.draw_board(canvas, w);

        let active = matches!(self.phase, Phase::Running(_));
        for (index, direction) in Direction::ALL.into_iter().enumerate() {
            let button = direction_button(w, index);
            // A tap reverses the button: a selected direction turns white while it is held.
            let pressed = active && canvas.control_pressed(button, false);
            let selected = self.game.next_direction() == Some(direction);
            let filled = active && (selected != pressed);
            canvas.panel(button, 16, 4, if filled { BLACK } else { PAPER });
            let size = canvas.px(54);
            let glyph = Rect::new(
                button.x + (button.w - size) / 2,
                button.y + (button.h - size) / 2,
                size,
                size,
            );
            let color = if filled {
                PAPER
            } else if active {
                BLACK
            } else {
                85
            };
            canvas.icon(glyph, arrow(direction), color);
        }

        let hint = if self.save_failed {
            "Could not save the best score."
        } else {
            "Swipe across the board or tap an arrow to steer."
        };
        canvas.text(screen_rect(w, 40, 1415, 1184, 60), hint, CAPTION.center());
        canvas.button(action_button(w), action, true, false);
    }

    fn input(&mut self, input: Input) -> Change {
        match input {
            Input::Tap(x, y) => self.tap(x, y),
            Input::Swipe { dx, dy, start_y }
                if (BOARD_Y as f32..(BOARD_Y + BOARD_SIZE) as f32).contains(&start_y)
                    && dx.abs().max(dy.abs()) >= 42.0 =>
            {
                let direction = if dx.abs() >= dy.abs() {
                    if dx > 0.0 {
                        Direction::Right
                    } else {
                        Direction::Left
                    }
                } else if dy > 0.0 {
                    Direction::Down
                } else {
                    Direction::Up
                };
                self.steer(direction)
            }
            Input::PageBack if matches!(self.phase, Phase::SelectSpeed) => {
                self.phase = Phase::Ready;
                Change::Full
            }
            Input::PageBack => Change::Exit,
            Input::Sleep => {
                self.pause();
                Change::None
            }
            _ => Change::None,
        }
    }

    fn deadline(&self) -> Option<Instant> {
        match self.phase {
            Phase::Running(next_step) => Some(next_step),
            _ => None,
        }
    }

    fn tick(&mut self, now: Instant) -> Change {
        if self.deadline().is_none_or(|next_step| now < next_step) {
            return Change::None;
        }
        self.game.advance();
        self.best = self.best.max(self.game.score());
        if self.game.status() == Status::Playing {
            // If refresh took longer than the interval, continue with one step, without catch-up.
            self.phase = Phase::Running(now + self.game.interval());
        } else {
            self.phase = Phase::Finished;
            self.persist();
        }
        Change::Partial
    }

    fn save(&mut self) -> io::Result<()> {
        if self.best > self.saved_best {
            if let Some(path) = &self.path {
                primitive_app::save(path, format!("{}\n", self.best).as_bytes())?;
            }
            self.saved_best = self.best;
        }
        Ok(())
    }

    fn storage_returned(&mut self) -> io::Result<()> {
        self.pause();
        Ok(())
    }
}

fn board(width: i32) -> Rect {
    screen_rect(width, BOARD_X, BOARD_Y, BOARD_SIZE, BOARD_SIZE)
}

fn cell(width: i32, point: Point) -> Rect {
    screen_rect(
        width,
        BOARD_X + point.x as i32 * CELL,
        BOARD_Y + point.y as i32 * CELL,
        CELL,
        CELL,
    )
}

fn direction_button(width: i32, index: usize) -> Rect {
    screen_rect(width, 152 + index as i32 * 243, 1296, 231, 106)
}

fn speed_button(width: i32) -> Rect {
    screen_rect(width, 846, 174, 360, 76)
}

fn speed_choice(width: i32, speed: usize) -> Rect {
    let index = speed as i32 - 1;
    screen_rect(
        width,
        102 + index % 3 * 360,
        450 + index / 3 * 230,
        340,
        190,
    )
}

fn action_button(width: i32) -> Rect {
    screen_rect(width, 349, 1510, 566, 110)
}

fn arrow(direction: Direction) -> Icon {
    let stroke: &[&str] = match direction {
        Direction::Left => &["M21 12H3 M10 5L3 12L10 19"],
        Direction::Right => &["M3 12H21 M14 5L21 12L14 19"],
        Direction::Up => &["M12 21V3 M5 10L12 3L19 10"],
        Direction::Down => &["M12 3V21 M5 14L12 21L19 14"],
    };
    Icon { stroke, fill: &[] }
}
