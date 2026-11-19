use std::path::PathBuf;

use snake_render::View;

fn main() -> Result<(), String> {
    let directory = std::env::var_os("PRIMITIVE_APP_DIR")
        .map_or_else(|| PathBuf::from("/mnt/us/snake"), PathBuf::from);
    primitive_app::run(View::open(directory.join("snake-best.txt")), &directory)
}
