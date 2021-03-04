pub fn remove(value: &mut toml::Value) {
    if let toml::Value::Table(table) = value {
        let mut to_remove = Vec::new();

        for (key, value) in table.iter_mut() {
            if let toml::Value::Array(array) = value {
                if array.is_empty() {
                    to_remove.push(key.clone());
                }
            }
            if let toml::Value::Table(table) = value {
                if table.is_empty() {
                    to_remove.push(key.clone());
                }

                // TASK: Step into table.
            }
        }

        for key in to_remove {
            table.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::remove;

    #[test]
    fn remove_should_remove_empty_arrays() {
        let mut table = toml::value::Table::new();
        table.insert(
            String::from("key"),
            toml::Value::Array(toml::value::Array::new()),
        );

        let mut value = toml::Value::Table(table);
        remove(&mut value);

        let empty_table = toml::Value::Table(toml::value::Table::new());
        assert_eq!(value, empty_table);
    }

    #[test]
    fn remove_should_remove_empty_tables() {
        let mut table = toml::value::Table::new();
        table.insert(
            String::from("key"),
            toml::Value::Table(toml::value::Table::new()),
        );

        let mut value = toml::Value::Table(table);
        remove(&mut value);

        let empty_table = toml::Value::Table(toml::value::Table::new());
        assert_eq!(value, empty_table);
    }
}
