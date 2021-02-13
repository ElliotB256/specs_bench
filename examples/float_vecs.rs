extern crate nalgebra;
use nalgebra::Vector3;
use rand::distributions::{Distribution, Normal};
use specs::{Builder, World, VecStorage, System, Component, DispatcherBuilder, ReadStorage, WriteStorage};
use std::time::Instant;

#[derive(Copy, Clone)]
pub struct ComponentA {
    x: f64,
    y: f64,
    z: f64
}
impl Default for ComponentA { 

    fn default() -> Self {
        let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0);
    ComponentA {
        x : dist.sample(&mut rng),
        y : dist.sample(&mut rng),
        z : dist.sample(&mut rng),
    }
    }
}

#[derive(Copy, Clone)]
pub struct ComponentB {
    pub value: f64,
}
impl Default for ComponentB { 

    fn default() -> Self {
        let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0);
    ComponentB {
        value : dist.sample(&mut rng),
    }
    }
}

#[derive(Copy, Clone)]
pub struct ComponentC {
    pub value: f64,
}
impl Default for ComponentC { 

    fn default() -> Self {
        let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0);
    ComponentC {
        value : dist.sample(&mut rng),
    }
    }
}

fn main() {

    let a_vec = [ComponentA::default(); 1000000];
    let b_vec = [ComponentB::default(); 1000000];
    let mut c_vec = [ComponentC::default(); 1000000];


    let loop_start = Instant::now();

    for _ in 0..10000 {

        for (mut c, (a,b)) in c_vec.iter_mut().zip(a_vec.iter().zip(b_vec.iter()))
        {
            c.value = a.x * b.value + a.y; 
        }
    }

    println!(
        "Simulation loop completed in {} ms.",
        loop_start.elapsed().as_millis()
    );
}
