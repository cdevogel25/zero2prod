Page: 497/623  
(on the macbook) don't forget: docker build --build-arg ARCH_TARGET=aarch64 --tag ...

- what happens if a user tries to subscribe twice?
    - check if subscription_active, and send a 409
- what happens if a user clicks the confirmation link twice?
    - same as above
- what happens if a sub token is well-formatted but non-existent
    - something like what happens if email is non-existent
- add validation for the incoming token (sql injection!)
    - ???
- use a proper template solution for our emails (tera?)

- how on earth do you do angular with this there must be a way
    - the way is make this a backend *only* and send requests from the main page.