use minijinja::Value;
use nixpkgs_fmt::reformat_string;

pub fn nixfmt(value: &str) -> String {
    reformat_string(value)
}

pub fn to_string(value: Value) -> String {
    format!(r#""{value}""#)
}
