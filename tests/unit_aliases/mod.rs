use ::diman::unit_system;

unit_system!(
    quantity_type Quantity;
    dimension_type Dimension;
    dimension Length;
    #[base(Length)]
    #[alias(metres)]
    #[metric_prefixes]
    #[symbol(m)]
    unit meters: Length;

    #[prefix(kilo)]
    unit foo: Length = 0.25 * meters;
);

macro_rules! gen_tests_for_float {
    ($float_type: ty, $mod_name: ident, $assert_is_close: path, $assert_is_close_float: path) => {
        mod $mod_name {
            use super::dimensions::Length;
            use super::units;
            use crate::make_annotated_unit_constructor;
            make_annotated_unit_constructor!(meters, Length<$float_type>, $float_type);
            make_annotated_unit_constructor!(metres, Length<$float_type>, $float_type);
            make_annotated_unit_constructor!(centimeters, Length<$float_type>, $float_type);
            make_annotated_unit_constructor!(centimetres, Length<$float_type>, $float_type);
            make_annotated_unit_constructor!(kilometers, Length<$float_type>, $float_type);
            make_annotated_unit_constructor!(foo, Length<$float_type>, $float_type);
            make_annotated_unit_constructor!(kilofoo, Length<$float_type>, $float_type);

            #[test]
            fn unit_aliases() {
                assert_eq!(
                    meters(<$float_type>::from(100.0)),
                    metres(<$float_type>::from(100.0))
                );
                let x = meters(<$float_type>::from(100.0));
                assert_eq!(x.value_in(units::meters), <$float_type>::from(100.0));
                assert_eq!(x.value_in(units::metres), <$float_type>::from(100.0));
            }

            #[test]
            fn prefixed_aliases() {
                assert_eq!(
                    centimeters(<$float_type>::from(100.0)),
                    centimetres(<$float_type>::from(100.0))
                );
                let x = centimeters(<$float_type>::from(100.0));
                assert_eq!(x.value_in(units::meters), <$float_type>::from(1.0));
                assert_eq!(x.value_in(units::metres), <$float_type>::from(1.0));
                assert_eq!(x.value_in(units::centimeters), <$float_type>::from(100.0));
                assert_eq!(x.value_in(units::centimetres), <$float_type>::from(100.0));
            }

            #[test]
            fn explicit_prefix() {
                assert_eq!(
                    foo(<$float_type>::from(100.0)),
                    meters(<$float_type>::from(25.0))
                );
                assert_eq!(
                    kilofoo(<$float_type>::from(100.0)),
                    kilometers(<$float_type>::from(25.0))
                );
            }
        }
    };
}

#[cfg(feature = "f32")]
gen_tests_for_float!(
    f32,
    mod_f32,
    crate::utils::assert_is_close_f32,
    crate::utils::assert_is_close_float_f32
);

#[cfg(feature = "f64")]
gen_tests_for_float!(
    f64,
    mod_f64,
    crate::utils::assert_is_close_f64,
    crate::utils::assert_is_close_float_f64
);

macro_rules! gen_fastnum_tests {
    ($feature:literal, $ty:ty, $mod_name:ident) => {
        #[cfg(feature = $feature)]
        gen_tests_for_float!(
            $ty,
            $mod_name,
            crate::utils::assert_is_close_fastnum,
            crate::utils::assert_is_close_float_fastnum
        );
    };
}

gen_fastnum_tests!("fastnum-d64", fastnum::D64, fastnum_d64);
