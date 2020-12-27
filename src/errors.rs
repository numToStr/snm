#[macro_export]
macro_rules! pretty_error_msg {
    ($($l:expr),*) => {
        anyhow::Error::msg(format!($($l,)*));
    };
}

#[macro_export]
macro_rules! pretty_error {
    ($($l:expr),*) => {
        Err(anyhow::Error::msg(format!($($l,)*)));
    };
}
