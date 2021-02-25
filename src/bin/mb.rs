use clap::Clap as _;
use my_boss::{
    args::{Args, Command},
    config::Config,
    contacts::Contacts,
};
use time::OffsetDateTime;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Init => {
            Config::init()?;
        }
        Command::Contacts(args) => {
            let config = Config::load()?;

            if args.all {
                // TASK: Print all contacts.
                return Err(anyhow::Error::msg(
                    "Printing all contacts is not supported yet.",
                ));
            }

            // TASK: Use local time instead. As of this writing, this is blocked
            //       by this issue: https://github.com/time-rs/time/issues/293
            let today = OffsetDateTime::now_utc().date();
            for contact in Contacts::load(config.contacts)?.due(today) {
                println!("{}", contact.summary()?);
            }
        }
    }

    Ok(())
}
