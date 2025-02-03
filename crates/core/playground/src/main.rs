#![allow(dead_code, unused_variables, unused_imports, unused_macros)]
mod error;

struct Meat {
    bex: Box<[usize]>,
}

impl Meat {
    fn new() -> Meat {
        Meat {
            bex: vec![0, 1, 2].into_boxed_slice(),
        }
    }
}
fn main() {
    let v = &Meat::new();
    let b = &*v.bex;
}
