mod context;
mod filters;
mod system;

pub use context::*;
use filters::*;
pub use system::*;

use minijinja::Environment;

#[derive(Debug, Clone)]
pub struct Generator<'a> {
    pub engine: Environment<'a>,
}

impl Generator<'_> {
    pub const NIX_EXPRESSION: (&'static str, &'static str) = (
        "nix_expression",
        include_str!("./jinja/template/nix_expression.nix.j2"),
    );
    pub const CODELLDB: (&'static str, &'static str) =
        ("codelldb", include_str!("./jinja/template/codelldb.j2"));
}

macro_rules! add_filter {
    ($jinja:ident, $ty:expr) => {
        $jinja.add_filter(stringify!($ty), $ty);
    };
}

macro_rules! add_function {
    ($jinja:ident, $ty:expr) => {
        $jinja.add_function(stringify!($ty), $ty);
    };
}

impl<'a> Generator<'a> {
    pub fn new() -> Self {
        let mut engine = Environment::new();

        engine
            .add_template(Self::NIX_EXPRESSION.0, Self::NIX_EXPRESSION.1)
            .unwrap();

        add_filter!(engine, nixfmt);
        add_filter!(engine, to_string);
        add_function!(engine, is_universal);
        add_function!(engine, is_linux_x86);
        add_function!(engine, is_linux_arm);
        add_function!(engine, is_darwin_x86);
        add_function!(engine, is_darwin_arm);

        Self { engine }
    }

    pub fn render_asset_url(&self, tmpl: &str, ctx: &AssetUrlContext) -> String {
        let tpl = self.engine.template_from_str(tmpl).unwrap();
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

impl<'a> Default for Generator<'a> {
    fn default() -> Self {
        Self::new()
    }
}
