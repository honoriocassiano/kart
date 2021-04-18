use rand::prelude::*;
use svg::node::element::Line;
use svg::Document;

use std::iter::Iterator;

use crate::common::Particle;
use crate::common::Vec2;

pub fn generate(width: u32, height: u32, max_it: u32) -> Document {
    let particle1 = Particle::new(Vec2(0.0, 300.0), Vec2(0.0, 10.0));
    let particle2 = Particle::new(Vec2(800.0, 500.0), Vec2(0.0, -10.0));

    let mut particles = vec![particle1, particle2];

    let mut document = Document::new().set("viewBox", (0, 0, width, height));

    let mut rng = rand::thread_rng();

    for _it in 0..max_it {
        particles.iter_mut().for_each(|p| {
            p.update(rng.gen::<f64>() * 3.0 + 1.0);
        });

        let p1 = particles[0].position();
        let p2 = particles[1].position();

        let line = Line::new()
            .set("x1", p1.x())
            .set("y1", p1.y())
            .set("x2", p2.x())
            .set("y2", p2.y())
            .set("stroke", "black")
            .set("stroke-width", 3);

        document = document.add(line);
    }

    document
}
