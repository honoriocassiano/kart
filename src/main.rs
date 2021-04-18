use rand::prelude::*;

use svg::node::element::Line;
use svg::Document;

fn main() {
    let mut rng = thread_rng();

    let x2: i32 = 500;
    let y2: i32 = 500;
    let colors = vec!["black", "orange", "blue", "green", "brown"];

    let mut document = Document::new().set("viewBox", (0, 0, 800, 800));

    for _ in 0..100 {
        let x1: i32 = rng.gen_range(0..400);
        let y1: i32 = rng.gen_range(0..400);

        let color = *colors.choose(&mut rng).unwrap();

        let line = Line::new()
            .set("x1", x1)
            .set("y1", y1)
            .set("x2", x2)
            .set("y2", y2)
            .set("stroke", color)
            .set("stroke-witdh", "2");

        document = document.add(line);
    }

    svg::save("image.svg", &document).unwrap();
}
