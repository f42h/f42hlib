#[macro_export]
macro_rules! get_os {
    () => {
        if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else {
            "Failed to detect OS!"
        }
    };
}

#[macro_export]
macro_rules! tern {
    ($con:expr, $t:expr, $f:expr) => {
        if $con { $t } else { $f }
    };
}