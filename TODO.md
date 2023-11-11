- DONE - FIX FRONTEND ERROR - likely cause by common dependencies that require 
    diesel/libsqlite which probably doesn't compile to wasm or whatever

- ASYNC STRIPE - implment stripe payments w Rust async stripe crate && stripe account
  
- DONE - Create cart && checkout page
  
- Emails to both cutsomer && owner whenever an order is placed
- 
- make pretty - tailwind? yes setup tailwind
- 
- finish admin page for viewing orders/editing products
- get real data in DBs - up to owner
- DONE ON BACKEND - Error handling: replace calls to unwrap with actual error handling
- Set up some form of exponential backoff on the frontend 

- DONE - Refactor Carts + Items, instead of HashMap of Item -> Quantity should be ItemId(i32)  -> quantity(u32)
- DONE - Fix cookies