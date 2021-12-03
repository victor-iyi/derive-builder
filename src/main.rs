use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
  executable: String,
  #[builder(each = "arg")]
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

  println!("{:?}", command);
}

impl std::fmt::Debug for Command {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Command {{ executable: {:?}, args: {:?}, env: {:?}, current_dir: {:?} }}",
      self.executable, self.args, self.env, self.current_dir
    )
  }
}
