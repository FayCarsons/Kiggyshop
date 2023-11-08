#[macro_export]
macro_rules! log_debug {
    ($($e:expr), *) => {
        {

            #[cfg(debug)]
            #[cfg(target_arch = "wasm32")]
            {
                gloo::console::log!($($e), +)
            }

            #[cfg(debug)]
            #[cfg(not(target_arch = "wasm32"))]
            {
                println!($($e), +);
            }
        }
    }
}
