use rand::prelude::*;
use svg::node::element::Line;
use svg::Document;

use std::iter::Iterator;

use crate::common::Particle;
use crate::common::Vec2;

pub struct Simulation {
    viewbox_size: Vec2,
    max_it: Option<u32>,
    sync: bool,
    rng: ThreadRng,
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(viewbox_size: Vec2, max_it: Option<u32>, sync: bool) -> Self {
        Self {
            viewbox_size,
            max_it,
            sync,
            rng: rand::thread_rng(),
            particles: Self::generate_particles(),
        }
    }

    fn generate_particles() -> Vec<Particle> {
        let particle1 = Particle::new(Vec2(0.0, 0.0), Vec2(0.0, 10.0));
        let particle2 = Particle::new(Vec2(800.0, 800.0), Vec2(0.0, -10.0));

        vec![particle1, particle2]
    }

    pub fn generate(&mut self) -> Document {
        let mut document = Document::new().set(
            "viewBox",
            (0, 0, self.viewbox_size.x(), self.viewbox_size.y()),
        );

        match self.max_it {
            Some(it) => self.run_n_iterations(&mut document, it),
            None => self.run_all(&mut document),
        }
    }

    fn run_all(&mut self, document: &Document) -> Document {
        let mut doc = document.clone();

        loop {
            self.update_all();

            let p1 = self.particles[0].position();
            let p2 = self.particles[1].position();

            if !inside_viewbox(self.viewbox_size, p1) || !inside_viewbox(self.viewbox_size, p2) {
                break;
            }

            let line = create_line(p1, p2);

            doc = doc.add(line);
        }

        doc
    }

    fn run_n_iterations(&mut self, document: &Document, max_it: u32) -> Document {
        let mut doc = document.clone();

        for _it in 0..max_it {
            self.update_all();

            let p1 = self.particles[0].position();
            let p2 = self.particles[1].position();

            let line = create_line(p1, p2);

            doc = doc.add(line);
        }

        doc
    }

    fn update_all(&mut self) -> Vec<Vec2> {
        let step = self.sync.then(|| random_step(&mut self.rng));

        let mut rng = self.rng.clone();

        self.particles
            .iter_mut()
            .map(|p| p.update(step.unwrap_or(random_step(&mut rng))))
            .collect()
    }
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

fn random_step(rng: &mut ThreadRng) -> f64 {
    rng.gen::<f64>() * 3.0 + 1.0
}
