use std::collections::HashMap;

pub fn ignore_scripts(data: &HashMap<String, String>) -> bool {
    let default_value = super::default::scripts();
    data.keys()
        .all(|item| default_value.get(item) == data.get(item))
}
