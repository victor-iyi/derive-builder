use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
  executable: String,
  args: Vec<String>,
  env: Vec<String>,
  current_dir: Option<String>,
}

fn main() {
  println!("Hello, World!");

  let command = Command::builder()
    .executable("cargo".to_owned())
    .args(vec!["build".to_owned(), "--release".to_owned()])
    .env(vec![])
    .build()
    .unwrap();

  assert_eq!(command.executable, "cargo");
  assert_eq!(
    command.args,
    vec!["build".to_string(), "--release".to_string()]
  );

  assert!(command.env.is_empty());
  assert!(command.current_dir.is_none());
}
