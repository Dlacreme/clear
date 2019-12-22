use std::fmt::Display;

static mut LOGS_ENABLED: bool = false;

pub fn enable_info_log(logs_enabled: bool) {
    unsafe {
        LOGS_ENABLED = logs_enabled;
    }
}

pub fn log<D>(message: D)
where D: Display {
    println!("Info: {}", message)
}

pub fn warn<D>(message: D)
where D: Display {
    println!("Warning! : {}", message)
}

pub fn error<D>(message: D)
where D: Display {
    println!("!Error! : {}", message)
}