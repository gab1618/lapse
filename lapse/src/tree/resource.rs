pub enum Resource {
  Requests,
  Env,
  Scripts,
}

impl Into<&'static str> for Resource {
  fn into(self) -> &'static str {
    match self {
      Resource::Requests => "requests",
      Resource::Env => "env",
      Resource::Scripts => "scripts",
    }
  }
}
