- DONE - FIX FRONTEND ERROR - likely cause by common dependencies that require 
    diesel/libsqlite which probably doesn't compile to wasm or whatever
- ASYNC STRIPE - implment stripe payments w Rust async stripe crate && stripe account
- Create cart && checkout page
- Emails to both cutsomer && kristen whenever an order is placed
- make pretty
- finish admin page for viewing orders/editing products
- get real data in DBs
- Error handling: replace calls to unwrap with actual error handling 
- Set up some form of exponential backoff on the frontend