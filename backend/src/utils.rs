
pub fn print_red<T>(label: &str, object: &T) 
    where T: core::fmt::Debug + ?Sized {
    // ANSI escape code for red text
    let red_code = "\x1b[31m";

    // ANSI escape code for resetting color
    let reset_code = "\x1b[0m";

    println!("{red_code}{label}{:?}{reset_code}", object)
} 