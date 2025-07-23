# quark


## How to Run

#### To run the REPL
```rust
cargo run quark
```

#### To run a program 
```rust
cargo run quark file_name
```

#### To enable debug logs
```rust
cargo run quark file_name --debug
```

*To run one of the test files*
```rust
cargo run quark test/test.quark --debug
```

## to-do
- [ ] add instructions to load long constants
- [ ] add testing
- [ ] add string interpolation
- [ ] add flags for 
- [ ] look into strum crate to iterate over enum variant instead of hardcoding their codes
- [ ] make a OpPopN instruction that takes operand for number of slots to pop and pops them all at once.
- [ ] replace all functions returning a value with Option if it is possible to return an invalid value.
- [ ] some sort of flag to toggle which parts of compiler need to be traced
- [ ] add const keyword and implement it
- [ ] change read_byte() return type to u8 instead of OpCode as not all bytes read will be OpCode (some might be operands)




