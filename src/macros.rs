macro_rules! run {
    ($command:expr$(,$arg:expr)*) => {
        println!(concat!("RUN>", $command)$(, $arg)*);
    };
}

// Dependencies: colored::Colorize
macro_rules! error {
    ($message:expr$(,$arg:expr)*) => {
        eprintln!(concat!("{} ", $message), "error:".red().bold()$(, $arg)*);
    };
}

// Dependencies: colored::Colorize
macro_rules! warn {
    ($message:expr$(,$arg:expr)*) => {
        println!(concat!("{} ", $message), "warning:".yellow().bold()$(, $arg)*);
    };
}

// Dependencies: VERBOSE: bool, colored::Colorize
macro_rules! log {
    ($message:expr$(,$arg:expr)*) => {
        unsafe {
            if VERBOSE {
                println!(concat!("{} ", $message), "info:".bright_blue().bold()$(, $arg)*);
            }
        }
    };
}

// Dependencies: log!
macro_rules! skip {
    ($condition:expr) => {
        if $condition {
            continue;
        }
    };
    ($condition:expr, $message:expr) => {
        if $condition {
            log!("{}", $message);
            continue;
        }
    };
}

// Dependencies: skip!
macro_rules! skip_err {
    ($result:expr) => {
        skip!($result.is_err(), $result.unwrap_err());
    };
}

// Dependencies: skip!
macro_rules! skip_none {
    ($option:expr) => {
        skip!($option.is_none());
    };
    ($option:expr, $message:expr) => {
        skip!($option.is_none(), $message);
    };
}

// Dependencies: std::io, warn!, Exit (src/exit.rs)
macro_rules! confirm {
    ($action:expr$(,$arg:expr)*) => {
        loop {
            warn!(concat!("Are you certain you wish to ", $action, "? [y/n]")$(,$arg)*);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .unwrap_or_exit("Could not read line");
            response = response.to_lowercase();
            let response: &str = response.trim();
            if response == "y" || response == "yes" {
                break;
            }
            if response == "n" || response == "no" {
                println!("Aborting");
                return;
            }
        }
    };
}
