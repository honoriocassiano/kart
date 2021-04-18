mod common;
mod crazypoints;

use crazypoints::generate;

fn main() {
    // let document = generate(800, 800, Some(50));
    let document = generate(800, 800, None);

    svg::save("image.svg", &document).unwrap();
}
