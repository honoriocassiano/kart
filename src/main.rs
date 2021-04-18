mod common;
mod crazypoints;

use crazypoints::generate;

fn main() {
    let document = generate(800, 800, 50);

    svg::save("image.svg", &document).unwrap();
}
