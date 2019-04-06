#[macro_use]
extern crate quickcheck;
extern crate btree;
extern crate rand;

use self::quickcheck::{Arbitrary, Gen};
use rand::Rng;

const KEY_SPACE: u8 = 20;

#[derive(Clone, Debug)]
enum Op {
    Insert(u8, u8),
    Get(u8),
}
use Op::{Get, Insert};

impl Arbitrary for Op {
    fn arbitrary<G: Gen>(g: &mut G) -> Op {
        let k: u8 = g.gen_range(0, KEY_SPACE);

        if g.gen_bool(0.5) {
            Insert(k, g.gen())
        } else {
            Get(k)
        }
    }
}

quickcheck! {
    fn implementation_matches_model(ops: Vec<Op>) -> bool {
        let mut implementation = btree::Tree::default();
        let mut model = std::collections::BTreeMap::new();

        for op in ops {
            match op {
                Insert(k, v) => {
                    implementation.insert(k, v);
                    model.insert(k, v);
                }
                Get(k) => {
                    if implementation.get(&k) != model.get(&k) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
