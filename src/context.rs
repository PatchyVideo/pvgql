
#[derive(Debug, Clone)]
pub struct Context {
	pub session: Option<String>,
	pub auth_header: Option<String>,
}

impl juniper::Context for Context {}
