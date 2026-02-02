[![wakatime](https://wakatime.com/badge/user/9657b28a-a48a-4a67-baed-44a03722033d/project/975385a4-1144-4022-9602-0bbf7a0e7a3a.svg)](https://wakatime.com/badge/user/9657b28a-a48a-4a67-baed-44a03722033d/project/975385a4-1144-4022-9602-0bbf7a0e7a3a)


Page: 623/623  
(on the macbook) don't forget: docker build --build-arg ARCH_TARGET=aarch64 --tag ...

- add validation for the incoming token (sql injection!)
    - ???
- use a proper template solution for our emails (tera?)

- how on earth do you do angular with this there must be a way
    - the way is make this a backend *only* and send requests from the main page.

# Retrospective
 
I've never really written a retrospective for a project like this before, but I feel the need to set out my understanding of how the zero2prod project functions. In order: I'll go through the libraries and tools used, then authentication, domain, idempotency implementation, and the actual web pages. I'll probably fill this in as I go along.