use diman::internal::for_each_fastnum_decimal_type;

// These just need to compile.
macro_rules! gen_tests_for_float {
    ($float_type: ty, $mod_name: ident) => {
        mod $mod_name {
            use crate::example_system::dimensions::Length;
            use crate::example_system::dimensions::Time;
            use diman::Product;
            use diman::Quotient;

            #[allow(unused)]
            fn product_1(
                length: Length<$float_type>,
                time: Time<$float_type>,
            ) -> Product<Length<$float_type>, Time<$float_type>> {
                length * time
            }

            #[allow(unused)]
            fn quotient_1(
                length: Length<$float_type>,
                time: Time<$float_type>,
            ) -> Quotient<Length<$float_type>, Time<$float_type>> {
                length / time
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(f32, f32);

#[cfg(feature = "f64")]
gen_tests_for_float!(f64, f64);

macro_rules! tests_for_fastnum {
    ($feature:literal, $float_type: ty, $mod_name:ident) => {
        #[cfg(feature = $feature)]
        gen_tests_for_float!($float_type, $mod_name);
    };
}

for_each_fastnum_decimal_type!(tests_for_fastnum);
