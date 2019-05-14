#[doc(hidden)]
#[macro_export]
macro_rules! __edit_hamlet {

    ( $name:ident : $e:expr, N = $n:expr ) => {

        #[bench]
        fn $name(bench: &mut $crate::Bencher) {
            use __detail::*;
            <T as CB>::check_hamlet_with_edits($n, &$e, bench);
        }

    };

    ( $name:ident : $e:expr ) => {
        $crate::__edit_hamlet!($name : $e, N = N);
    };

}

#[doc(hidden)]
#[macro_export]
macro_rules! __check_hamlet {
    ( $name:ident ) => {

        #[bench]
        fn $name(bench: &mut $crate::Bencher) {
            <T as CB>::read_hamlet(bench);
        }

    };

    ( $name:ident, $n:expr ) => {

        #[bench]
        fn $name(bench: &mut $crate::Bencher) {
            <T as CB>::check_hamlet($n, bench);
        }

    };

    ( $name:ident, $n:expr, $e:expr ) => {

        #[bench]
        fn $name(bench: &mut $crate::Bencher) {
            <T as CB>::check_hamlet_with_edits($n, $e, bench);
        }

    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __bench_parser {
    ( ) => {

        mod parse_hamlet {
            use super::__detail::*;

            hamlet!(read_all);
        }

    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __bench_corrector {
    ( ) => {

        mod words_from_hamlet {
            use super::__detail::*;

            hamlet!(_10, 10);
            hamlet!(_100, 100);
        }

        mod edit_hamlet {
            use $crate::edits::*;
            use super::__detail::*;

            hamlet!(first_1, N, Edit::delete(0));
//            hamlet!(first_2, N, LSkip(2));
//            hamlet!(first_3, N, LSkip(3));
//            hamlet!(last_1, N, RSkip(1));
//            hamlet!(last_2, N, RSkip(2));
//            hamlet!(first_1_last_1, N, LSkip(1).and(RSkip(1)));

//            hamlet!(pre_a, N, Pre("a"));
//            hamlet!(post_t, N, Post("t"));
//            hamlet!(replace_first_z, N, LSkip(1).and(Pre("z")));
//            hamlet!(transpose_0, N, Transpose(0));
//            hamlet!(transpose_1, N, Transpose(1));
//            hamlet!(transpose_2, N, Transpose(2));

        }

    };
}

#[macro_export]
macro_rules! __process_items {

    {
        $(#[$attr:meta])*
        $v:vis const N: $t:ty = $n:expr;
        $($rest:tt)*
    } => {
        $(#[$attr])*
        $v const N: $t = $n;
        $($rest)*
    };

    {
        $($rest:tt)*
    } => {
        const N: usize = $crate::N;
        $($rest)*
    };

}

#[macro_export]
macro_rules! spell_bench {
    {

        $(#[$outer:meta])*
        $v:vis mod $m:ident {
            $(#![$inner:meta])*

            $($i:tt)*
        }

    } => {

        $(#[$outer])*
        $v mod $m {
            $(#![$inner])*

            use $crate::__edit_hamlet as edit_hamlet;
            use $crate::__bench_corrector as bench_corrector;
            use $crate::__bench_parser as bench_parser;

            mod __detail {
                pub (super) use $crate::{
                    CorrectorBenches as CB,
                    __check_hamlet as hamlet,
                };

                pub (super) type T = super::Corrector;
                pub (super) const N: usize = super::N;
            }

            $crate::__process_items!($($i)*);

        }

    };
}
