mod context;
mod filters;

pub use context::*;
use filters::*;

use minijinja::Environment;

#[derive(rust_embed::RustEmbed)]
#[folder = "src/jinja/template/"]
struct Asset;

#[derive(Debug, Clone)]
pub struct Generator<'a> {
    pub engine: Environment<'a>,
}

impl Generator<'_> {
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

impl Generator<'_> {
    pub fn new() -> Self {
        let mut engine = Environment::new();

        engine.set_loader(
            move |name| match Asset::get(format!("{name}.j2").as_str()) {
                Some(file) => match String::from_utf8(file.data.to_vec()) {
                    Ok(val) => Ok(Some(val)),
                    Err(_) => unreachable!(),
                },
                None => Ok(None),
            },
        );

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
        tpl.render(minijinja::Value::from_serialize(ctx)).unwrap()
    }

    pub fn render(&mut self, ctx: &GeneratorContext) -> anyhow::Result<String> {
        Ok(self
            .engine
            .get_template("nix_expression.nix")?
            .render(minijinja::Value::from_serialize(ctx))?)
    }
}

impl Default for Generator<'_> {
    fn default() -> Self {
        Self::new()
    }
}
