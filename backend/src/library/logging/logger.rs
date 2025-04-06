use once_cell::sync::Lazy;
pub use slog::*;
use slog_async::Async;

pub static DEFAULT: Lazy<Logger> = Lazy::new(|| {
    let mk_json = || slog_json::Json::default(std::io::stdout()).fuse();
    let drain = Async::default(slog_envlogger::new(mk_json())).fuse();

    Logger::root(drain, o!())
});

#[macro_export]
macro_rules! log_error {
    ($($args:tt)+) => {
        $crate::library::logging::logger::error!($crate::library::logging::logger::DEFAULT, $($args)+)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($args:tt)+) => {
        $crate::library::logging::logger::warn!($crate::library::logging::logger::DEFAULT, $($args)+)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($args:tt)+) => {
        $crate::library::logging::logger::info!($crate::library::logging::logger::DEFAULT, $($args)+)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($args:tt)+) => {
        $crate::library::logging::logger::debug!($crate::library::logging::logger::DEFAULT, $($args)+)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($($args:tt)+) => {
        $crate::library::logging::logger::trace!($crate::library::logging::logger::DEFAULT, $($args)+)
    };
}
