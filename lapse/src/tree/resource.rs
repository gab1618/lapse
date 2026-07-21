pub enum Resource {
  Requests,
  Env,
  Scripts,
}

impl From<Resource> for &'static str {
  fn from(value: Resource) -> Self {
    match value {
      Resource::Requests => "requests",
      Resource::Env => "env",
      Resource::Scripts => "scripts",
    }
  }
}
