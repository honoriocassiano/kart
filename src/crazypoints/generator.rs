use rand::prelude::*;
use svg::node::element::Line;
use svg::Document;

use std::iter::Iterator;

use crate::common::Particle;
use crate::common::Vec2;

pub fn generate(viewbox_size: Vec2, max_it: Option<u32>, sync: bool) -> Document {
    let particle1 = Particle::new(Vec2(0.0, 0.0), Vec2(0.0, 10.0));
    let particle2 = Particle::new(Vec2(800.0, 800.0), Vec2(0.0, -10.0));

    let mut particles = vec![particle1, particle2];

    let mut document = Document::new().set("viewBox", (0, 0, viewbox_size.x(), viewbox_size.y()));

    let mut rng = rand::thread_rng();

    match max_it {
        Some(it) => run_n_iterations(&mut document, &mut particles, &mut rng, it, sync),
        None => run_all(&mut document, &mut particles, &mut rng, viewbox_size, sync),
    }
}

fn run_all(
    document: &Document,
    particles: &mut Vec<Particle>,
    rng: &mut ThreadRng,
    viewbox_size: Vec2,
    sync: bool,
) -> Document {
    let mut doc = document.clone();

    loop {
        update_all(particles, rng, sync);

        let p1 = particles[0].position();
        let p2 = particles[1].position();

        if !inside_viewbox(viewbox_size, p1) || !inside_viewbox(viewbox_size, p2) {
            break;
        }

        let line = create_line(p1, p2);

        doc = doc.add(line);
    }

    doc
}

fn run_n_iterations(
    document: &Document,
    particles: &mut Vec<Particle>,
    rng: &mut ThreadRng,
    max_it: u32,
    sync: bool,
) -> Document {
    let mut doc = document.clone();

    for _it in 0..max_it {
        update_all(particles, rng, sync);

        let p1 = particles[0].position();
        let p2 = particles[1].position();

        let line = create_line(p1, p2);

        doc = doc.add(line);
    }

    doc
}

fn inside_viewbox(viewbox_size: Vec2, point: Vec2) -> bool {
    point.x() >= 0.0
        && point.x() <= viewbox_size.x()
        && point.y() >= 0.0
        && point.y() <= viewbox_size.y()
}

fn create_line(p1: Vec2, p2: Vec2) -> Line {
    Line::new()
        .set("x1", p1.x())
        .set("y1", p1.y())
        .set("x2", p2.x())
        .set("y2", p2.y())
        .set("stroke", "black")
        .set("stroke-width", 3)
}

fn update_all(particles: &mut Vec<Particle>, rng: &mut ThreadRng, sync: bool) -> Vec<Vec2> {
    let step = sync.then(|| random_step(rng));

    particles
        .iter_mut()
        .map(|p| p.update(step.unwrap_or(random_step(rng))))
        .collect()
}

fn random_step(rng: &mut ThreadRng) -> f64 {
    rng.gen::<f64>() * 3.0 + 1.0
}
