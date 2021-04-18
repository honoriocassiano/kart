use rand::prelude::*;
use svg::node::element::Line;
use svg::Document;

use std::iter::Iterator;

use crate::common::Particle;
use crate::common::Vec2;

// TODO Use Vec2 instead of width and height
pub fn generate(width: u32, height: u32, max_it: Option<u32>) -> Document {
    let particle1 = Particle::new(Vec2(0.0, 0.0), Vec2(0.0, 10.0));
    let particle2 = Particle::new(Vec2(800.0, 800.0), Vec2(0.0, -10.0));

    let mut particles = vec![particle1, particle2];

    let mut document = Document::new().set("viewBox", (0, 0, width, height));

    let mut rng = rand::thread_rng();

    match max_it {
        Some(it) => run_n_iterations(&mut document, &mut particles, &mut rng, it),
        None => run_all(&mut document, &mut particles, &mut rng, height),
    }
}

fn run_all(
    document: &Document,
    particles: &mut Vec<Particle>,
    rng: &mut ThreadRng,
    height: u32, // TODO Use Vec2
) -> Document {
    let mut doc = document.clone();

    loop {
        update_all(particles, rng);

        let p1 = particles[0].position();
        let p2 = particles[1].position();

        if !inside_viewbox(height, p1) || !inside_viewbox(height, p2) {
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
) -> Document {
    let mut doc = document.clone();

    for _it in 0..max_it {
        update_all(particles, rng);

        let p1 = particles[0].position();
        let p2 = particles[1].position();

        let line = create_line(p1, p2);

        doc = doc.add(line);
    }

    doc
}

fn inside_viewbox(height: u32, point: Vec2) -> bool {
    // TODO Fix it
    let fheight = height as f64;

    point.y() >= 0.0 && point.y() <= fheight
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

fn update_all(particles: &mut Vec<Particle>, rng: &mut ThreadRng) -> Vec<Vec2> {
    particles
        .iter_mut()
        .map(|p| p.update(rng.gen::<f64>() * 3.0 + 1.0))
        .collect()
}
