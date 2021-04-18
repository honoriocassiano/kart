mod common;
mod crazypoints;

use common::Vec2;
use crazypoints::generate;

fn main() {
    let document = generate(Vec2(800.0, 800.0), None, false);

    svg::save("image.svg", &document).unwrap();
}
