#![feature(test)]
extern crate test;

mod common_structs;
use crate::common_structs::{EnumDispatched, DynamicDispatched, ReturnsValue, Zero, One};

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn enumdispatch_blackbox(b: &mut Bencher) {
        let dis0 = EnumDispatched::from(Zero);
        let dis1 = EnumDispatched::from(One);

        b.iter(|| {
            for _ in 0..1000000 {
                test::black_box(dis0.return_value());
                test::black_box(dis1.return_value());
            }
        })
    }

    #[bench]
    fn customderive_blackbox(b: &mut Bencher) {
        let dis0 = DynamicDispatched::from(Zero);
        let dis1 = DynamicDispatched::from(One);

        b.iter(|| {
            for _ in 0..1000000 {
                test::black_box(dis0.inner().return_value());
                test::black_box(dis1.inner().return_value());
            }
        })
    }

    #[bench]

    fn boxdyn_blackbox(b: &mut Bencher) {
        let dis0: Box<dyn ReturnsValue> = Box::new(Zero);
        let dis1: Box<dyn ReturnsValue> = Box::new(One);

        b.iter(|| {
            for _ in 0..1000000 {
                test::black_box(dis0.return_value());
                test::black_box(dis1.return_value());
            }
        })
    }

    #[bench]
    fn refdyn_blackbox(b: &mut Bencher) {
        let dis0: &dyn ReturnsValue = &Zero;
        let dis1: &dyn ReturnsValue = &One;

        b.iter(|| {
            for _ in 0..1000000 {
                test::black_box(dis0.return_value());
                test::black_box(dis1.return_value());
            }
        })
    }
}

fn main() {}
