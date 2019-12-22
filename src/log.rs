use std::fmt::Display;

/** TODO
 * How to keep a flag available for this module only to enable/disable log without having to use an `unsafe` block?
 */
static mut LOGS_ENABLED: bool = false;

pub fn enable_info_log(logs_enabled: bool) {
    unsafe {
        LOGS_ENABLED = logs_enabled;
    }
}

pub fn log<D>(message: D)
where D: Display {
    unsafe {
        if LOGS_ENABLED {
            println!("Info: {}", message)
        }
    }
}

/* Not sure those will be useful one day */

// pub fn warn<D>(message: D)
// where D: Display {
//     println!("Warning! : {}", message)
// }

// pub fn error<D>(message: D)
// where D: Display {
//     println!("!Error! : {}", message)
// }
