[![CircleCI](https://circleci.com/gh/Licenser/simdjson-rs/tree/master.svg?style=svg)](https://circleci.com/gh/Licenser/simdjson-rs/tree/master)[![Build status](https://ci.appveyor.com/api/projects/status/0kf0v6hj5v2gite9?svg=true)](https://ci.appveyor.com/project/Licenser/simdjson-rs)

# simdjson-rs

Rust port of extremely fast [simdjson](https://github.com/lemire/simdjson) JSON parser with [serde](serde.rs) compatibility.

## readme (for real!)

### CPU target

For taking advantage of simdjson your system needs to be SIMD compatible. This means to compile with native cpu support and the given features. Look at [The cargo config in this repository](.cargo/config) to get an example.

### jemalloc

If you are writing perormance centric code, make sure to use jemalloc and not the system allocator (that has now become default in rust), it gives an very noticable boost imperformance.



## Other interesting things

There are also bindings for simdjson available [here](https://github.com/SunDoge/simdjson-rust)

## License

simdjson-rs itself is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

However it ports a lot of code from [simdjson](https://github.com/lemire/simdjson) so their work and copyright on that should be respected along side.

The [serde](serde.rs) integration is based on their example and serde-json so again, their copyright should as well be respected.
