use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use primitive_app::{font_paths, frame, write_png, App, Change, HEIGHT, WIDTH};
use primitive_ui::{Canvas, Input};
use snake_core::{Point, Status};
use snake_render::View;

fn step(view: &mut View) {
    let timestamp = view.deadline().expect("scheduled next step");
    assert_eq!(view.tick(timestamp), Change::Partial);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(
        std::env::args_os()
            .nth(1)
            .unwrap_or_else(|| "build/previews".into()),
    );
    fs::create_dir_all(&output)?;
    let fonts =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../doannt-kindle-launcher/assets/fonts");
    let mut canvas = Canvas::new(WIDTH, HEIGHT, &font_paths(&fonts))?;
    let mut view = View::preview(2026);
    let mut capture = |view: &View, name: &str, dark| {
        write_png(
            &output.join(name),
            WIDTH,
            HEIGHT,
            &frame(view, &mut canvas, dark),
        )
    };
    let button = Input::Tap(632.0, 1570.0);
    let speed_button = Input::Tap(1025.0, 211.0);

    capture(&view, "snake.png", false)?;
    assert!(view.deadline().is_none());
    assert_eq!(view.input(speed_button), Change::Full);
    assert!(view.deadline().is_none());
    capture(&view, "snake-speed.png", false)?;
    assert_eq!(view.input(Input::Tap(1110.0, 90.0)), Change::Full);
    assert_eq!(view.game.speed(), 1);
    assert_eq!(view.input(speed_button), Change::Full);
    assert_eq!(view.input(Input::Tap(990.0, 540.0)), Change::Full);
    assert_eq!(view.game.speed(), 3);
    assert_eq!(view.game.interval(), Duration::from_millis(750));
    assert_eq!(view.game.score(), 0);
    assert_eq!(view.input(button), Change::Partial);
    assert_eq!(view.input(speed_button), Change::None);
    let first = view.deadline().unwrap();
    assert_eq!(view.tick(first - Duration::from_millis(1)), Change::None);
    for _ in 0..4 {
        step(&mut view);
    }
    assert_eq!(view.game.score(), 3);
    assert_eq!(view.best(), 3);
    assert_eq!(view.game.interval(), Duration::from_millis(750));
    assert_eq!(
        view.input(Input::Swipe {
            dx: 4.0,
            dy: -180.0,
            start_y: 830.0
        }),
        Change::Partial
    );
    step(&mut view);
    assert_eq!(view.game.body()[0], Point { x: 10, y: 7 });
    capture(&view, "snake-playing-dark.png", true)?;

    assert_eq!(view.input(button), Change::Partial);
    assert!(view.deadline().is_none());
    assert_eq!(view.tick(first + Duration::from_secs(30)), Change::None);
    capture(&view, "snake-paused.png", false)?;
    assert_eq!(view.input(button), Change::Partial);
    assert_eq!(view.input(Input::Tap(975.0, 1340.0)), Change::Partial);
    for _ in 10..16 {
        step(&mut view);
    }
    assert_eq!(view.game.status(), Status::HitWall);
    assert!(view.deadline().is_none());
    capture(&view, "snake-game-over.png", false)?;

    let best = view.best();
    assert_eq!(view.input(button), Change::Partial);
    assert_eq!(view.game.score(), 0);
    assert_eq!(view.game.speed(), 3);
    assert_eq!(view.best(), best);
    assert_eq!(view.input(Input::Tap(1110.0, 90.0)), Change::Exit);
    Ok(())
}
