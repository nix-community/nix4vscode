use minijinja::Environment;
use serde::{Deserialize, Serialize};

use crate::data::NixContext;

#[derive(Debug, Deserialize, Serialize)]
struct Context {
    nixs: Vec<NixContext>,
}

// empty
#[derive(Debug)]
pub struct Generator<'a> {
    pub engine: Environment<'a>,
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        let mut engine = Environment::new();

        engine
            .add_template(
                "nix_expression",
                include_str!("./jinja/nix_expression.nix.j2"),
            )
            .unwrap();

        Self { engine }
    }

    pub fn render(&mut self, nixs: Vec<NixContext>) -> anyhow::Result<String> {
        Ok(self
            .engine
            .get_template("nix_expression")?
            .render(minijinja::Value::from_serializable(&Context { nixs }))?)
    }
}
