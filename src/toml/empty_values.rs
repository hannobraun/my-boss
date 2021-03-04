pub fn remove(value: &mut toml::Value) {
    if let toml::Value::Table(table) = value {
        let mut to_remove = Vec::new();

        for (key, value) in table.iter_mut() {
            if let toml::Value::Table(table) = value {
                if table.is_empty() {
                    to_remove.push(key.clone());
                }
            }
            if let toml::Value::Array(array) = value {
                if array.is_empty() {
                    to_remove.push(key.clone());
                }
            }
        }

        for key in to_remove {
            table.remove(&key);
        }
    }
    if let toml::Value::Array(array) = value {
        array.retain(|item| should_retain(item))
    }
}

fn should_retain(value: &toml::Value) -> bool {
    if let toml::Value::Table(table) = value {
        if table.is_empty() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::remove;

    #[test]
    fn remove_should_remove_empty_table_from_table() {
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

    #[test]
    fn remove_should_remove_empty_array_from_table() {
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
    fn remove_should_remove_empty_table_from_array() {
        let mut array = toml::value::Array::new();
        array.push(toml::Value::Table(toml::value::Table::new()));

        let mut value = toml::Value::Array(array);
        remove(&mut value);

        let empty_array = toml::Value::Array(toml::value::Array::new());
        assert_eq!(value, empty_array);
    }
}
