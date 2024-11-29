# workspace-workflow

When creating an initial workspace it's nnnecessary to create workspace.members
but you should create the workspaces Cargo.toml with workspace.resolver = "2" to
suppress some warnings. If you do that you can just use `cargo new abin`
or `cargo new --lib alib` in the workspace root. So initially the workspace
only contains one file Cargo.tom with this:
```toml
cat Cargo.toml
[workspace]
resolver = "2"
```
You can then `cargo new abin` or `cargo new --bin abin` and lets add
`cargo new --lib alib`, you'll then have:
```
$ tree -r .
.
├── Cargo.toml
├── Cargo.lock
├── alib
│   ├── src
│   │   └── lib.rs
│   └── Cargo.toml
└── abin
    ├── src
    │   └── main.rs
    └── Cargo.toml

5 directories, 6 files
```
Then `cd abin` and do `cargo add alib --path ../alib` to have the `abin`
depend on `../alib`. I then modified `abin/src/main.rs` to use `alib::add`:
```
$ cat -n abin/Cargo.toml
     1  [package]
     2  name = "abin"
     3  version = "0.1.0"
     4  edition = "2021"
     5
     6  [dependencies]
     7  alib = { version = "0.1.0", path = "../alib" }

$ cat -n abin/src/main.rs 
     1  use alib;
     2
     3  fn main() {
     4      println!("Hello, world! invoke alib::add");
     5
     6      let sum = alib::add(1, 2);
     7      println!("sum = {sum}");
     8      assert!(sum == 3);
     9  }
``
And left alib unchanged from the default:
```
$ cat -n alib/Cargo.toml 
     1  [package]
     2  name = "alib"
     3  version = "0.1.0"
     4  edition = "2021"
     5
     6  [dependencies]

$ cat -n alib/src/lib.rs 
     1  pub fn add(left: u64, right: u64) -> u64 {
     2      left + right
     3  }
     4
     5  #[cfg(test)]
     6  mod tests {
     7      use super::*;
     8
     9      #[test]
    10      fn it_works() {
    11          let result = add(2, 2);
    12          assert_eq!(result, 4);
    13      }
    14  }
```
At that point I could run abin and test alib:
```
$ cargo run abin
   Compiling alib v0.1.0 (/home/wink/prgs/rust/myrepos/workspace-workflow/alib)
   Compiling abin v0.1.0 (/home/wink/prgs/rust/myrepos/workspace-workflow/abin)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/abin abin`
Hello, world! invoke alib::add
sum = 3

$ cargo test alib
   Compiling abin v0.1.0 (/home/wink/prgs/rust/myrepos/workspace-workflow/abin)
   Compiling alib v0.1.0 (/home/wink/prgs/rust/myrepos/workspace-workflow/alib)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/main.rs (target/debug/deps/abin-b673de96d010ba96)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/alib-a39b3ae1f292624f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
