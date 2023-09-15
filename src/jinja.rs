mod context;
mod filters;
mod system;

pub use context::*;
use filters::*;
use system::*;

use minijinja::{Environment, Value};

#[derive(Debug, Clone)]
pub struct Generator<'a> {
    pub engine: Environment<'a>,
}

impl Generator<'_> {
    pub const NIX_EXPRESSION: (&str, &str) = (
        "nix_expression",
        include_str!("./jinja/template/nix_expression.nix.j2"),
    );
    pub const CODELLDB: (&str, &str) = ("codelldb", include_str!("./jinja/template/codelldb.j2"));
}

impl<'a> Generator<'a> {
    pub fn new(sys_ctx: Option<&SystemContext>) -> Self {
        let mut engine = Environment::new();

        engine
            .add_template(Self::NIX_EXPRESSION.0, Self::NIX_EXPRESSION.1)
            .unwrap();

        engine.add_filter("nixfmt", nixfmt);

        match sys_ctx {
            Some(ctx) => {
                engine.add_global("system", Value::from_serializable(ctx));
            }
            None => {
                engine.add_global(
                    "system",
                    Value::from_serializable(&SystemContext::default()),
                );
            }
        }

        Self { engine }
    }

    pub fn render_asset_url(&self, context: &str, ctx: &AssetUrlContext) -> String {
        let tpl = self.engine.template_from_str(context).unwrap();
        tpl.render(minijinja::Value::from_serializable(ctx))
            .unwrap()
    }

    pub fn render(&mut self, ctx: &GeneratorContext) -> anyhow::Result<String> {
        Ok(self
            .engine
            .get_template(Self::NIX_EXPRESSION.0)?
            .render(minijinja::Value::from_serializable(ctx))?)
    }
}
