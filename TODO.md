# ADMIN
- add order editing on admin page
- test auth for admin dashboard
- get real data in DBs - depends on owner
- more control over orders - add tracking #

# BACKEND
- Error handling: replace any calls to unwrap with propagation up call stack
- Implement nightly(3am) Backups for DB, save current stock and orders to JSON file
- Tracking URL
- refund policy 
- returns 
- sales tax
- improve test coverage: read, update, delete for DB, everything for Actix

# FRONTEND
- research benefits of and potentially set up exponential backoff for frontend API requests
- Finish Navbar 
- 

# Stripe 
- Collect phone, email, any relevant info in case of error
- Set up taxes
- Test order insertion in DB