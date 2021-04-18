mod common;
mod crazypoints;

use crazypoints::generate;

fn main() {
    // let document = generate(800, 800, Some(50), false);
    let document = generate(800, 800, None, false);

    svg::save("image.svg", &document).unwrap();
}
