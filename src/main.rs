pub mod particle;

use crate::particle::Particle;
use rand::Rng;
use std::sync::mpsc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{self, Color, DrawParams};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use threadpool::ThreadPool;

const WIDTH: f32 = 1280.0;

const HEIGHT: f32 = 720.0;
const AMOUNT: f32 = 50000.0;
// const TURN_FRACTION: f32 = 1.6180339;
const TURN_FRACTION: f32 = 0.001756;

#[derive(Debug)]
struct Message {
    x: f32,
    y: f32,
}

struct GameState {
    // timer: f32,
    counter: u128,
    rng: rand::prelude::ThreadRng,
    particles: Vec<Particle>,
    pool: ThreadPool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut rng = rand::thread_rng();

        let mut particles = Vec::<Particle>::with_capacity(AMOUNT as usize);

        particles.push(Particle::new(
            vec![HEIGHT / 2.0, WIDTH / 2.0],
            vec![rng.gen_range(-5.0..1.0), rng.gen_range(-5.0..1.0)],
            vec![0.0, 0.4],
            HEIGHT,
            WIDTH,
        ));

        window::set_mouse_visible(ctx, true).unwrap();
        window::set_title(ctx, "Particles");

        Ok(GameState {
            // timer: 0.0,
            counter: 0,
            particles,
            rng,
            pool: ThreadPool::new(1024),
        })
    }
}

impl State for GameState {
    fn update(&mut self, _: &mut Context) -> tetra::Result {
        self.counter = self.counter + 40;
        if self.counter % 8 == 0 {
            self.particles.push(particle::Particle::new(
                vec![HEIGHT / 2.0, WIDTH / 2.0],
                vec![self.rng.gen_range(-5.0..1.0), self.rng.gen_range(-5.0..1.0)],
                vec![0.0, 0.4],
                HEIGHT,
                WIDTH,
            ));
        }

        for i in (0..(self.particles.len() - 1)).rev() {
            if self.particles[i].clone().finished() {
                self.particles[i] = Particle::new(
                    vec![HEIGHT / 2.0, WIDTH / 2.0],
                    vec![self.rng.gen_range(-5.0..1.0), self.rng.gen_range(-5.0..1.0)],
                    vec![0.0, 0.4],
                    HEIGHT,
                    WIDTH,
                );

                // self.particles.remove(i);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        // for i in 0..AMOUNT as i128 {
        //     let dst = i as f32 / (AMOUNT - 1.0);
        //     let dst = dst.sqrt();

        //     // let angle = i as f32 * TURN_FRACTION * std::f32::consts::TAU;
        //     let angle = std::f32::consts::TAU * TURN_FRACTION * i as f32;

        //     let x = angle.cos() * dst * WIDTH + WIDTH / 2.0;
        //     let y = angle.sin() * dst * HEIGHT + HEIGHT / 2.0;

        //     // let x = dst * angle.cos() * angle.sin();
        //     // let y = dst * angle.sin() * angle.cos();

        //     // println!("x: {:?} y: {:?} dst: {:?} index: {:?}", x, y, dst, i);

        //     let mesh = Mesh::circle(ctx, ShapeStyle::Fill, Vec2::new(1., 1.), 1.)
        //         .expect("failed to create mesh");

        //     mesh.draw(
        //         ctx,
        //         DrawParams::new()
        //             .position(Vec2::new(x, y))
        //             .color(Color::rgb(0.255, 0.1, 0.255)),
        //     );
        // }

        for i in 0..self.particles.len() {
            println!("index: {:?}", i);
            let (tx, rx) = mpsc::channel::<Message>();

            self.pool.execute(move || {
                let dst = i as f32 / (AMOUNT - 1.0);
                let dst = dst.sqrt();
                //     // let angle = i as f32 * TURN_FRACTION * std::f32::consts::TAU;
                let angle = std::f32::consts::TAU * TURN_FRACTION * i as f32;
                let x = angle.cos() * dst * WIDTH + WIDTH / 2.0;
                let y = angle.sin() * dst * HEIGHT + HEIGHT / 2.0;
                let message = Message { x, y };
                tx.send(message).unwrap();
            });

            let message = rx.recv().unwrap();

            println!("message: {:?}", message);

            let mesh = Mesh::ellipse(
                ctx,
                ShapeStyle::Fill,
                Vec2::new(self.particles[i].size, self.particles[i].size),
                Vec2::new(self.particles[i].size, self.particles[i].size),
            )
            .expect("failed to create mesh");

            mesh.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        self.particles[i].pos[0],
                        self.particles[i].pos[1],
                    ))
                    .color(Color::rgba(
                        self.particles[i].colour[0],
                        self.particles[i].colour[1],
                        self.particles[i].colour[2],
                        self.particles[i].lifetime,
                    )),
            );

            self.particles[i].update(message.x, message.y);
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    let mut ctx = ContextBuilder::new("Custom Mesh", WIDTH as i32, HEIGHT as i32).build()?;

    ctx.run(GameState::new)
}
