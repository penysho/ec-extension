#[macro_export]
macro_rules! log_error {
    ($message:expr) => {
        tracing::error!($message)
    };
    ($message:expr, $($key:expr => $value:expr),+) => {
        tracing::error!(message = $message, $($key = ?$value),+)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($message:expr) => {
        tracing::warn!($message)
    };
    ($message:expr, $($key:expr => $value:expr),+) => {
        tracing::warn!(message = $message, $($key = ?$value),+)
    };
}

#[macro_export]
macro_rules! log_info {
    ($message:expr) => {
        tracing::info!($message)
    };
    ($message:expr, $($key:expr => $value:expr),+) => {
        tracing::info!(message = $message, $($key = ?$value),+)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($message:expr) => {
        tracing::debug!($message)
    };
    ($message:expr, $($key:expr => $value:expr),+) => {
        tracing::debug!(message = $message, $($key = ?$value),+)
    };
}

#[macro_export]
macro_rules! log_trace {
    ($message:expr) => {
        tracing::trace!($message)
    };
    ($message:expr, $($key:expr => $value:expr),+) => {
        tracing::trace!(message = $message, $($key = ?$value),+)
    };
}
