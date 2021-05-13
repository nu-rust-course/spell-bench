#[macro_export]
macro_rules! spell_bench_default_benches {
    ($model:ty) => {
        pub fn model_creation(c: &mut Criterion) {
            $model::from_corpus_bench(corpus::SMALL, c);
            $model::from_corpus_bench(corpus::HAMLET, c);
        }

        pub fn hamlet_corrections(c: &mut Criterion) {
            $model::check_hamlet_bench(100, "identity",
                Edit::I, c);
            $model::check_hamlet_bench(100, "delete 1",
                Edit::I.delete(1), c);
            $model::check_hamlet_bench(100, "transpose 2; insert 3 x",
                Edit::I.transpose(2).insert(3, 'x'),c);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __spell_bench_expand_bench {
    (
        fn $group:ident :: $name:ident()
        $(let { $($before:tt)* } in)?
        {$($body:tt)*}
    ) =>
    {
        fn $name(__crit: &mut $crate::criterion::Criterion) {
            let tag = __test_tag(stringify!($group), stringify!($name));
            __crit.bench_function(&tag, |b| {
                $($($before)*)?
                b.iter(|| {$($body)*});
            });
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __spell_bench_expand_group {
    (
        $group:ident {
            $(
                fn $name:ident()
                $(let {$($before:tt)*} in)?
                {$($body:tt)*}
            )*
        }
    ) => {
        pub fn $group() {
            let _c = &mut $crate::criterion::Criterion::default().configure_from_args();
            $($name(_c);)*
            $($crate::__spell_bench_expand_bench! {
                fn $group::$name()
                $(let {$($before)*} in)?
                {$($body)*}
            })*
        }
    };
}

#[macro_export]
macro_rules! spell_bench {
    (
        for $model:ty
        $(where $(
                mod $group:ident {$($body:tt)*}
        )*)?
    ) => {
        spell_bench! {
            mod spell_bench_module for $model
            $(where $(mod $group {$($body)*})*)?
        }
    };

    (
        mod $module:ident for $model:ty
        $(where $(mod $group:ident {$($body:tt)*})*)?
    ) => {
        mod $module {
            #[allow(unused)]
            use super::*;

            // use $crate::{
            //     Edit,
            //     CorrectorBench,
            //     corpus,
            //     criterion::{
            //         Criterion,
            //         criterion_main,
            //         criterion_group,
            //     },
            // };

            // spell_bench_benches!($model);

            fn __test_tag(group: &str, name: &str) -> String {
                format!("{}::{}", group, name)
            }

            pub mod groups {
                #[allow(unused)]
                use super::*;

                $($(
                    $crate::__spell_bench_expand_group! {
                        $group {$($body)*}
                    }
                )*)?
            }

            // criterion_group! {
            //     bench_group,
            //     model_creation,
            //     hamlet_corrections
            // }
        }

        $(
            $crate::criterion::criterion_main! {
                // $module::bench_group,
                $($module::groups::$group),*
            }
        )?
    };
}

// spell_bench! {
//     for correct::model::RomanDirectModel
//     where
//         mod meow {
//             fn generate_and_sum(){
//                 let v = (0 .. 1 << 20).collect::<Vec<u32>>();
//                 let u: u32 = v.iter().sum();
//                 u
//             }

//             fn generate(){
//                 let v = (0 .. 1 << 20).collect::<Vec<u32>>();
//                 v
//             }

//             fn sum()
//             let {
//                 let v = (0 .. 1 << 20).collect::<Vec<u32>>();
//             } in {
//                 let u: u32 = v.iter().sum();
//                 u
//             }
//         }

//         mod woof {
//             fn one() {
//                 "woof".len()
//             }
//         }
// }
