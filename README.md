Rust web frameworks (practical) comparison
==== 

Main goal of mine when starting working on this repo was to check the current state of Rust web-frameworks. 

There is a great list of all web frameworks as part of [are we web yet](https://www.arewewebyet.org/topics/frameworks).
You may also find [Rust web framework comparson](https://github.com/flosse/rust-web-framework-comparison) useful.

I decded to test only those that work on Rust stable (so no Rocket, unfortunatelly) as well as those providing something more than being just a thin wrapper over HTTP protocol.

So far this repo contains projects written using:
- [actix-web](https://github.com/actix/actix-web)
- ~~[gotham](https://gotham.rs/)~~
- [tower-web](https://github.com/carllerche/tower-web)
- [warp](https://github.com/seanmonstar/warp)

Examples:
----

### PESEL

For testing purposes I decided to use my other crate - [PESEL](https://github.com/MaciekTalaska/pesel) and wrap it around REST-ish HTTP interface. 

Each project should expose two different route(s):

#### PESEL validation

`http://localhost:8080/pesel/<pesel_number>`

#### PESEL generation

`http://localhost:8080/<year>/<month>/<day>/<biological_gender>`
