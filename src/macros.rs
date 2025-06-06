#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("\x1b[32m[INFO]\x1b[0m {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        eprintln!("\x1b[31m[ERROR]\x1b[0m {}", format!($($arg)*));
    };
}
