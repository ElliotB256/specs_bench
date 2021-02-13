extern crate nalgebra;
use nalgebra::Vector3;
use rand::distributions::{Distribution, Normal};
use specs::{
    Builder, Component, DispatcherBuilder, ReadStorage, System, VecStorage, World, WriteStorage,
};
use std::time::Instant;

pub struct ComponentA {
    x: f64,
    y: f64,
    z: f64,
}
impl Component for ComponentA {
    type Storage = VecStorage<Self>;
}

pub struct ComponentB {
    pub value: f64,
}
impl Component for ComponentB {
    type Storage = VecStorage<Self>;
}

pub struct ComponentC {
    pub value: f64,
}
impl Component for ComponentC {
    type Storage = VecStorage<Self>;
}

pub struct SimpleSystem;
impl<'a> System<'a> for SimpleSystem {
    type SystemData = (
        ReadStorage<'a, ComponentA>,
        ReadStorage<'a, ComponentB>,
        WriteStorage<'a, ComponentC>,
    );

    fn run(&mut self, (a, b, mut c): Self::SystemData) {
        use rayon::prelude::*;
        use specs::ParJoin;

        (&a, &b, &mut c).par_join().for_each(|(a, b, mut c)| {
            c.value = a.x * b.value + a.y;
        });
    }
}

fn main() {
    let mut world = World::new();
    let mut builder = DispatcherBuilder::new();

    builder.add(SimpleSystem, "simple_system", &[]);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build()
        .unwrap();

    builder.add_pool(::std::sync::Arc::new(pool));
    let mut dispatcher = builder.build();
    dispatcher.setup(&mut world.res);

    // Add test entities
    let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0);

    for _ in 0..1000000 {
        world
            .create_entity()
            .with(ComponentA {
                x: 1.0,
                y: 0.0,
                z: dist.sample(&mut rng),
            })
            .with(ComponentB {
                value: dist.sample(&mut rng),
            })
            .with(ComponentC { value: 0.0 })
            .build();
    }

    let loop_start = Instant::now();

    for _ in 0..10000 {
        dispatcher.dispatch(&mut world.res);
        world.maintain();
    }

    println!(
        "Simulation loop completed in {} ms.",
        loop_start.elapsed().as_millis()
    );
}
