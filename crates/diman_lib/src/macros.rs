#[macro_export]
macro_rules! cfg_any_fastnum_decimal_type {
    ($($body:tt)*) => {
        #[cfg(any(
            feature = "fastnum-d64",
            feature = "fastnum-d128",
            feature = "fastnum-d256",
            feature = "fastnum-d512",
            feature = "fastnum-d1024",
        ))]
        $($body)*
    };
}

#[macro_export]
macro_rules! for_each_fastnum_decimal_type {
    ($body:ident) => {
        $body!("fastnum-d64", fastnum::D64, fastnum_d64);
        $body!("fastnum-d128", fastnum::D128, fastnum_d128);
        $body!("fastnum-d256", fastnum::D256, fastnum_d256);
        $body!("fastnum-d512", fastnum::D512, fastnum_d512);
        $body!("fastnum-d1024", fastnum::D1024, fastnum_d1024);
    };
}
