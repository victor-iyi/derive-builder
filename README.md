# The Builder Design Pattern

This macro generates the boilerplate code involved in implementing the
[builder pattern] in Rust. Builders are a mechanism for instantiating structs,
especially structs with many fields, and especially if many of those fields are
optional or the set of fields may need to grow backward compatibly over time.

There are few different possibilities for espressing builders in Rust. To keep
things simple for this project, I used the example of the standard library's
[`std::process::Command`] builder in which the setter methods each receive and
return `&mut self` to allow chained method calls.

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern
[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

## Usage

The caller will invoke the macro as follows:

```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
  executable: String,
  #[builder(each = "arg")]
  args: Vec<String>,
  #[builder(each = "env")]
  env: Vec<String>,
  current_dir: Option<String>,
}

fn main() {
  let command = Command::builder()
      .executable("cargo".to_owned())
      .arg("build".to_owned())
      .arg("--release".to_owned())
      .build()
      .unwrap();

  assert_eq!(command.executable, "cargo");
  assert_eq!(command.args, &["cargo", "--release"]);
  assert!(command.env.is_empty());
  assert!(command.current_dir.is_none());


  let command = Command::builder()
      .executable("cargo".to_owned())
      .arg("build".to_owned())
      .arg("--release".to_owned())
      .current_dir("..".to_owned())
      .build()
      .unwrap();

  assert_eq!(command.executable, "cargo");
  assert_eq!(command.args, &["cargo", "--release"]);
  assert!(command.env.is_empty());
  assert!(command.current_dir.is_some());
}
```
## License (Apache 2.0 or MIT)

Licensed under either of [Apache License, Version 2.0] or [MIT license].
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[Apache License, Version 2.0]: ./LICENSE-APACHE
[MIT license]: ./LICENSE-MIT
