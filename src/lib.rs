use std::cell::RefCell;

pub const FN_COLOR: &str = "\x1b[3;33m";
pub const LOG_COLOR: &str = "\x1b[34m";
pub const RESET_COLOR: &str = "\x1b[0m";
pub const INDENT_COLOR: &str = "\x1b[2;37m";

thread_local! {
    pub static LOG_DEPTH: RefCell<usize> = const { RefCell::new(0) };
}

/// Prints indentation based on the current depth
#[macro_export]
macro_rules! indent {
    () => {{
        print!("{}", $crate::INDENT_COLOR);
        $crate::LOG_DEPTH.with(|d| {
            (0..*d.borrow()).for_each(|_| print!("⎸   "));
        });
        print!("{}", $crate::RESET_COLOR);
    }};
}

/// A guard that decreases the indentation level when dropped
pub struct LogGuard;

impl LogGuard {
    pub fn new() -> Self {
        LOG_DEPTH.with(|d| *d.borrow_mut() += 1);
        LogGuard
    }
}

impl Default for LogGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        LOG_DEPTH.with(|d| *d.borrow_mut() -= 1);
        indent!();
        println!("}}");
    }
}

/// Starts a new logging scope
#[macro_export]
macro_rules! func {
    ($($arg:tt)*) => {{
        $crate::indent!();
        print!("{}", $crate::FN_COLOR);
        print!($($arg)*);
        println!("{} {{", $crate::RESET_COLOR);
        $crate::LogGuard::new()
    }};
}

/// Logs a message within the current scope
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::indent!();
        print!("- {}", $crate::LOG_COLOR);
        print!($($arg)*);
        println!("{}", $crate::RESET_COLOR);
    }};
}
