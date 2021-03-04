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
