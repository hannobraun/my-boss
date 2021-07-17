use std::{fs::File, io::prelude::*, path::Path};

use time::macros::format_description;

use crate::money::transactions::Transactions;

pub fn to_toml(
    transactions: &Transactions,
    path: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let path = path.as_ref();

    for transaction in transactions {
        let date = transaction
            .date
            .format(&format_description!("[year]-[month]-[day]"))?;

        let mut i = 0;
        loop {
            let file_name = format!("{}_{}.toml", date, i);
            let path = path.join(&file_name);

            if path.exists() {
                i += 1;
                continue;
            }

            let transaction = toml::to_vec(&transaction)?;
            File::create(path)?.write_all(&transaction)?;
            break;
        }
    }

    Ok(())
}
