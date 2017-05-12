# porting Corrode to Rust

It'd be cool if [Corrode, the C to Rust translator written in Haskell](https://github.com/jameysharp/corrode) were also written in Rust. A large blocker is the lack of an extensive C parsing library like Haskell has.

This project has the goal of translating both libraries to Rust using a mix of automated translation and manual cleanup. Much like Corrode!

```
git clone http://github.com/tcr/corrode-but-in-rust --recursive
```

This project contains a proof-of-concept cross-compiler from Haskell to Rust which is not designed to be either correct or generalizable. Instead, I expect the conversion to go like this:

* [x] Automate bulk cross-compilation as modules (see the `out/` directory for current status)
* [x] Write a proof-of-concept parser for Haskell and compilation to Rust
* [x] Support functions, variables, literals, types
* [x] Support case statements and guards
* [ ] Convert instances and data structures correctly
* [ ] Detect pointfree code and convert it into pointwise
* [ ] Properly convert $ and . operators
* [ ] Convert rest of operators into Rust equivalents or fn wrappers
* [ ] Detect types better to switch between &str and String values, slice and Vecs
* [ ] Successfully parse all files (except lexer.x parser.y) (failures currently output as // ERROR)
* [ ] Parse flex-based lexer.x and parser.y files so they can be converted
* [ ] language-c test bench
* [ ] corrode test bench
* [ ] Preserve literate Haskell comments into Rust
* [ ] Feature-complete

## Status

See the current status by looking at the cross-compiled files in the out/ directory. These are equivalent to:

```
cargo run "./corrode/src/Language" > out/corrode.rs
cargo run "./language-c/src/Language/C" > out/language_c.rs
cargo run "./test" > out/test.rs
```

Look at the `test/input.hs` file and `out/test.rs` for an example of compilation you can (almost!) execute.

## References

* Great Haskell language reference: http://echo.rsmw.net/n00bfaq.html

## License

This project licensed as MIT or Apache-2, at your option.

Language-C licensed as BSD-3.

Corrode licensed as GPLv2.
