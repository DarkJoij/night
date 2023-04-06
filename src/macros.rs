#[macro_export]
macro_rules! spawn_night_error {
    ($e_type:expr, $($arg:tt)*) => {{
        eprintln!("{}: {}", $e_type, format!($($arg)*));
        std::process::exit(0);
    }};
}

#[macro_export]
macro_rules! if_debug {
    ($($body:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $($body)*
        }
    }};
}

#[macro_export]
macro_rules! if_daily {
    ($($body:tt)*) => {{
        if cfg!(feature = "daily") {
            $($body)*
        }
    }};
}
