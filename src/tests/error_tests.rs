#[cfg(feature = "std")]
#[test]
fn error_impl_std_error_error_test() {
    use crate::{error, test};
    test::compile_time_assert_std_error_error::<error::Unspecified>();
    test::compile_time_assert_std_error_error::<error::KeyRejected>();
}
