#[macro_export]
macro_rules! hash_test (
    ($inp:expr, $out_b:expr, $out_s:expr) => (
        HashTestCase {
            input: $inp,
            output: $out_b,
            output_str: $out_s
        }
    )
);
