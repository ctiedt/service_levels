#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(concat_idents)]

#[macro_export]
macro_rules! fn_wrapper {
    ($name:ident, ($($args:ty),+) => $returns:ty ,$($lv:literal: $fun:path),+) => {

        struct $name<const LVL: u32>;

        type Args = ($($args),+,);

        $(impl FnOnce<Args> for $name<$lv> {
            type Output = $returns;
            extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
                core::ops::Fn::call(&$fun, args)
            }
        })+

    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
