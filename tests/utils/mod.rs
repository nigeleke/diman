use diman_lib::for_each_fastnum_decimal_type;

#[cfg(feature = "f32")]
pub(crate) fn assert_is_close_f32<const U: crate::example_system::Dimension>(
    x: crate::example_system::Quantity<f32, U>,
    y: crate::example_system::Quantity<f32, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f32::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}

#[cfg(feature = "f64")]
pub(crate) fn assert_is_close_f64<const U: crate::example_system::Dimension>(
    x: crate::example_system::Quantity<f64, U>,
    y: crate::example_system::Quantity<f64, U>,
) {
    assert!(
        (x - y).abs().value_unchecked() < f64::EPSILON,
        "{} {}",
        x.value_unchecked(),
        y.value_unchecked()
    )
}

macro_rules! gen_assert_is_close_fastnum {
    ($feature:literal, $float_type: ty, $mod_name:ident) => {
        #[cfg(feature = $feature)]
        pub(crate) fn assert_is_close_fastnum<
            const N: usize,
            const U: crate::example_system::Dimension,
        >(
            x: crate::example_system::Quantity<fastnum::decimal::Decimal<N>, U>,
            y: crate::example_system::Quantity<fastnum::decimal::Decimal<N>, U>,
        ) {
            assert!(
                (x - y).abs().value_unchecked() < fastnum::decimal::EPSILON,
                "{} {}",
                x.value_unchecked(),
                y.value_unchecked()
            )
        }
    };
}

for_each_fastnum_decimal_type!(gen_assert_is_close_fastnum);

#[cfg(feature = "f32")]
pub(crate) fn assert_is_close_float_f32(x: f32, y: f32) {
    assert!((x - y).abs() < f32::EPSILON, "{} {}", x, y)
}

#[cfg(feature = "f64")]
pub(crate) fn assert_is_close_float_f64(x: f64, y: f64) {
    assert!((x - y).abs() < f64::EPSILON, "{} {}", x, y)
}

macro_rules! gen_assert_is_close_float_fastnum {
    ($feature:literal, $float_type:ty, $mod:ident) => {
        #[cfg(feature = $feature)]
        pub(crate) fn assert_is_close_float_fastnum(x: $float_type, y: $float_type) {
            assert!((x - y).abs() < fastnum::decimal::EPSILON, "{} {}", x, y)
        }
    };
}

for_each_fastnum_decimal_type!(gen_assert_is_close_float_fastnum);

// These help with what would otherwise require lots of
// type annotations for the float which are awkward to do
// because we can't write 1.0f64 or similar literals here

#[macro_export]
macro_rules! make_annotated_unit_constructor {
    ($unit: ident, $quantity: ty, $float_type: ty) => {
        fn $unit(x: $float_type) -> $quantity {
            x * units::$unit
        }
    };
}
