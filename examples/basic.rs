#![feature(fn_traits)]
#![feature(unboxed_closures)]

use service_levels::fn_wrapper;

mod example {
    pub fn lv0(n: u32) -> u32 {
        (0..=n).sum()
    }

    pub fn lv1(n: u32) -> u32 {
        ((n + 1) * n) / 2
    }
}

fn_wrapper!(Example, (u32) => u32, 0: example::lv0, 1: example::lv1);

fn main() {
    dbg!(Example::<0>(100));
    dbg!(Example::<1>(100));
}
