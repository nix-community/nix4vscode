use vscode_derive::api;

#[api(Default)]
pub struct ExtensionStatistics {
    statistic_name: String,
    value: usize,
}
