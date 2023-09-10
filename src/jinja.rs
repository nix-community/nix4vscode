mod context;
mod filters;
mod system;

pub use context::*;
use filters::*;
use system::*;

use minijinja::Environment;

#[derive(Debug, Clone)]
pub struct Generator<'a> {
    pub engine: Environment<'a>,
}

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator<'_> {
    pub const NIX_EXPRESSION: (&str, &str) = (
        "nix_expression",
        include_str!("./jinja/template/nix_expression.nix.j2"),
    );
    pub const CODELLDB: (&str, &str) = ("codelldb", include_str!("./jinja/template/codelldb.j2"));
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        let mut engine = Environment::new();

        engine
            .add_template(Self::NIX_EXPRESSION.0, Self::NIX_EXPRESSION.1)
            .unwrap();

        engine.add_filter("nixfmt", nixfmt);

        engine.add_global(
            "system",
            minijinja::Value::from_serializable(&SystemContext::default()),
        );

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
