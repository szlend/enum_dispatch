#![feature(test)]
extern crate test;

mod common_structs;
use crate::common_structs::{EnumDispatched, DynamicDispatched, ReturnsValue, Zero, One};

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;
    extern crate rand;
    use self::rand::Rng;

    const VEC_SIZE: usize = 1024;

    #[bench]
    fn enumdispatch_homogeneous_vec(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let mut dispatches = vec![];
        for _ in 0..VEC_SIZE {
            if rng.gen() {
                dispatches.push(EnumDispatched::from(Zero));
            }
            else {
                dispatches.push(EnumDispatched::from(One));
            }
        }

        b.iter(|| {
            for i in 0..1000000 {
                test::black_box(dispatches[i % VEC_SIZE].return_value());
            }
        })
    }

    #[bench]
    fn customderive_homogeneous_vec(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let mut dispatches = vec![];
        for _ in 0..VEC_SIZE {
            if rng.gen() {
                dispatches.push(DynamicDispatched::from(Zero));
            }
            else {
                dispatches.push(DynamicDispatched::from(One));
            }
        }

        b.iter(|| {
            for i in 0..1000000 {
                test::black_box(dispatches[i % VEC_SIZE].inner().return_value());
            }
        })
    }

    #[bench]
    fn boxdyn_homogeneous_vec(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let mut dispatches: Vec<Box<ReturnsValue>> = vec![];
        for _ in 0..VEC_SIZE {
            if rng.gen() {
                dispatches.push(Box::new(Zero));
            }
            else {
                dispatches.push(Box::new(One));
            }
        }

        b.iter(|| {
            for i in 0..1000000 {
                test::black_box(dispatches[i % VEC_SIZE].return_value());
            }
        })
    }

    #[bench]
    fn refdyn_homogeneous_vec(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let t0 = Zero;
        let t1 = One;

        let mut dispatches: Vec<&dyn ReturnsValue> = vec![];
        for _ in 0..VEC_SIZE {
            if rng.gen() {
                dispatches.push(&t0);
            }
            else {
                dispatches.push(&t1);
            }
        }

        b.iter(|| {
            for i in 0..1000000 {
                test::black_box(dispatches[i % VEC_SIZE].return_value());
            }
        })
    }
}

fn main() {}
