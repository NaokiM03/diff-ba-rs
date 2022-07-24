pub mod _diff_ba_impl {
    trait Paint {
        fn red(&self) -> String;
        fn green(&self) -> String;
    }
    impl Paint for str {
        fn red(&self) -> String {
            format!("\u{1b}[31m{}\u{1b}[0m", self)
        }
        fn green(&self) -> String {
            format!("\u{1b}[32m{}\u{1b}[0m", self)
        }
    }

    pub fn _dbg(before: &str, after: &str) {
        for diff in diff::lines(before, after) {
            match diff {
                diff::Result::Left(l) => println!("{}", format!("- {l}").red()),
                diff::Result::Both(l, _) => println!("{}", format!("  {l}")),
                diff::Result::Right(r) => println!("{}", format!("+ {r}").green()),
            }
        }
    }
}

pub mod diff_ba {
    #[macro_export]
    macro_rules! dbg_ {
        ($val:expr, $b:block) => {{
            let before = format!("{:#?}", $val);
            let value = $b;
            let after = format!("{:#?}", $val);
            _diff_ba_impl::_dbg(&before, &after);
            value
        }};
    }
    pub use dbg_ as dbg;
}
pub mod prelude {
    pub use crate::{_diff_ba_impl, diff_ba};
}
