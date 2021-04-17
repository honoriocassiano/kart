use rand::prelude::*;
// use rand::seq::IteratorRandom;

use svg::Document;
// use svg::node::element::Path;
// use svg::node::element::path::Data;
use svg::node::element::Line;

fn main() {
    // let data = Data::new()
    //     .move_to((10, 10))
    //     .line_by((0, 50))
    //     .line_by((50, 0))
    //     .line_by((0, -50))
    //     .close();
    //
    // let path = Path::new()
    //     .set("fill", "none")
    //     .set("stroke", "black")
    //     .set("stroke-width", 3)
    //     .set("d", data);

    let mut document = Document::new().set("viewBox", (0, 0, 800, 800));

    let mut rng = thread_rng();

    let x2: i32 = 500;
    let y2: i32 = 500;
    let colors = vec!["black", "orange", "blue", "green", "brown"];

    for _ in 0..100 {
        let x1: i32 = rng.gen_range(0..400);
        let y1: i32 = rng.gen_range(0..400);

        let color = *colors.choose(&mut rng).unwrap();

        let line = Line::new()
            .set("x1", x1)
            .set("x2", x2)
            .set("y1", y1)
            .set("y2", y2)
            .set("stroke", color)
            .set("stroke-witdh", "2");

        document = document.add(line);
    }

    svg::save("image.svg", &document).unwrap();
}
