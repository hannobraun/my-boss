use log::trace;

pub fn check_value(
    from: &toml::Value,
    to: &toml::Value,
    invalid: &mut Vec<String>,
    prefix: String,
) {
    trace!("Checking value:\n\t{:?}\n\t{:?}", from, to);

    if let (toml::Value::Table(from), toml::Value::Table(to)) = (from, to) {
        check_table(from, to, invalid, prefix.clone());
    }
    if let (toml::Value::Array(from), toml::Value::Array(to)) = (from, to) {
        check_array(from, to, invalid, prefix);
    }
}

pub fn check_table(
    from: &toml::value::Table,
    to: &toml::value::Table,
    invalid: &mut Vec<String>,
    prefix: String,
) {
    trace!("Checking value:\n\t{:?}\n\t{:?}", from, to);

    for (key, from_value) in from.iter() {
        let prefix = format!("{}.{}", prefix, key);

        match to.get(key) {
            Some(to_value) => {
                check_value(from_value, to_value, invalid, prefix);
            }
            None => {
                invalid.push(prefix);
            }
        }
    }
}

pub fn check_array(
    from: &toml::value::Array,
    to: &toml::value::Array,
    invalid: &mut Vec<String>,
    prefix: String,
) {
    trace!("Checking value:\n\t{:?}\n\t{:?}", from, to);

    for (from_item, to_item) in from.iter().zip(to.iter()) {
        check_value(from_item, to_item, invalid, prefix.clone());
    }
}
