/// Create [`anyhow::Error`] with the content of "file_name:line_number: `msg`".
///
/// # Example
/// ```
/// fn f() -> anyhow::Result<bool> {
///     Err(error_with_location("Explode!"))
/// }
/// ```
/// Output: "full_file_path:line_number: Explode!"
macro_rules! error_with_location {
    ($($msg:tt)+) => {
        ::anyhow::anyhow!(concat!(file!(), ":", line!(), ": {}"), format_args!($($msg)+))
    };
}

/// Transform `res` into [`anyhow::Error`] and decorate "file_name:line_number" to context.
macro_rules! with_location {
    ($res:expr) => {
        $res.map_err(::anyhow::Error::from)
            .with_context(|| format!("{}:{}", file!(), line!()))
    };
}

/// Transform `res` into [`anyhow::Error`] and decorate "file_name:line_number: `msg`" to context.
macro_rules! with_location_msg {
    ($res:expr, $($msg:tt)+) => {
        $res.map_err(::anyhow::Error::from)
            .with_context(|| format!("{}:{}: {}", file!(), line!(), format_args!($($msg)+)))
    };
}

pub(crate) use error_with_location;
pub(crate) use with_location;
pub(crate) use with_location_msg;
