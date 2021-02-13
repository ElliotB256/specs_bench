extern crate nalgebra;
use nalgebra::Vector3;
use rand::distributions::{Distribution, Normal};
use specs::{Builder, World, VecStorage, System, Component, DispatcherBuilder, ReadStorage, WriteStorage};
use std::time::Instant;

pub const ARRAY_SIZE: usize = 16;

#[derive(Copy, Clone)]
pub struct ComponentA {
    contents: [f64; ARRAY_SIZE],
}
impl Default for ComponentA { 

    fn default() -> Self {
        let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0);
    ComponentA {
        contents : [dist.sample(&mut rng); ARRAY_SIZE],
    }
    }
}

#[derive(Copy, Clone)]
pub struct ComponentB {
    pub contents: [f64; ARRAY_SIZE]
}
impl Default for ComponentB { 

    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let dist = Normal::new(0.0, 1.0);
    ComponentB {
        contents : [dist.sample(&mut rng); ARRAY_SIZE],
    }
    }
    
}

fn main() {

    let a_vec = [ComponentA::default(); 1000];
    let mut b_vec = [ComponentB::default(); 1000];

    let loop_start = Instant::now();

    for _ in 0..10000 {

        for (a, b) in a_vec.iter().zip(b_vec.iter_mut())
        {
            for (aval, bval) in a.contents.iter().zip(b.contents.iter_mut()) {
                *bval = aval * *bval + aval; 
            }
        }
    }

    println!(
        "Simulation loop completed in {} ms.",
        loop_start.elapsed().as_millis()
    );
}
