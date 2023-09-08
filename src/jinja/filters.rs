use nixpkgs_fmt::reformat_string;

pub fn nixfmt(value: &str) -> String {
    reformat_string(value)
}
