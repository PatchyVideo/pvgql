
#[derive(Debug, Clone)]
pub struct Context {
    pub session: Option<String>
}

impl juniper::Context for Context {}
