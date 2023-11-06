#[macro_export]
macro_rules! log_debug {
    ($($e:expr), *) => {
        {

            #[cfg(debug)]
            #[cfg(target = "wasm32-unknown-unknown")]
            {
                gloo::console::log!($($e), +)
            }

            #[cfg(debug)]
            #[cfg(not(target = "wasm32-unknown-unknown"))]
            {
                println!($($e), +);
            }
        }
    }
}
