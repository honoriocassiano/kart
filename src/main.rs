mod common;
mod crazypoints;

use common::Vec2;
use crazypoints::Simulation;

fn main() {
    let document = Simulation::new(Vec2(800.0, 800.0), None, false).generate();

    svg::save("image.svg", &document).unwrap();
}
