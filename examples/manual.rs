#![feature(fn_traits)]
#![feature(unboxed_closures)]

mod example {
    pub fn lv0(n: u32) -> u32 {
        (0..=n).sum()
    }

    pub fn lv1(n: u32) -> u32 {
        ((n + 1) * n) / 2
    }
}

struct Example<const LVL: u32>;

impl FnOnce<(u32,)> for Example<0> {
    type Output = u32;
    extern "rust-call" fn call_once(self, args: (u32,)) -> Self::Output {
        example::lv0(args.0)
    }
}

impl FnOnce<(u32,)> for Example<1> {
    type Output = u32;
    extern "rust-call" fn call_once(self, args: (u32,)) -> Self::Output {
        example::lv1(args.0)
    }
}

fn main() {
    dbg!(Example::<0>(100));
    dbg!(Example::<1>(100));
}
