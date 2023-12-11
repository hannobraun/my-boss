pub fn remove(value: &mut toml::Value) {
    if let toml::Value::Table(table) = value {
        let mut to_remove = Vec::new();

        for (key, value) in table.iter_mut() {
            // Before we check if the value should be retained, remove empty
            // values from it recursively.
            remove(value);

            if !should_retain(value) {
                to_remove.push(key.clone());
            }
        }

        for key in to_remove {
            table.remove(&key);
        }
    }
    if let toml::Value::Array(array) = value {
        // Before we check which values should be retained, remove empty values
        // recursively.
        for value in array.iter_mut() {
            remove(value);
        }

        array.retain(should_retain);
    }
}

fn should_retain(value: &toml::Value) -> bool {
    if let toml::Value::Table(table) = value {
        if table.is_empty() {
            return false;
        }
    }
    if let toml::Value::Array(array) = value {
        if array.is_empty() {
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

    #[test]
    fn remove_should_remove_empty_value_nested_in_table() {
        let mut inner = toml::value::Table::new();
        inner.insert(
            String::from("key"),
            toml::Value::Table(toml::value::Table::new()),
        );

        let mut outer = toml::value::Table::new();
        outer.insert(String::from("inner"), toml::Value::Table(inner));

        let mut value = toml::Value::Table(outer);
        remove(&mut value);

        let empty_table = toml::Value::Table(toml::value::Table::new());
        assert_eq!(value, empty_table);
    }

    #[test]
    fn remove_should_remove_empty_value_nested_in_array() {
        let mut inner = toml::value::Table::new();
        inner.insert(
            String::from("key"),
            toml::Value::Table(toml::value::Table::new()),
        );

        let mut outer = toml::value::Array::new();
        outer.push(toml::Value::Table(inner));

        let mut value = toml::Value::Array(outer);
        remove(&mut value);

        let empty_array = toml::Value::Array(toml::value::Array::new());
        assert_eq!(value, empty_array);
    }
}
