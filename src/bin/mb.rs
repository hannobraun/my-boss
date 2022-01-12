use clap::Parser as _;
use my_boss::{
    args::{contacts, Args, Command},
    config::Config,
    contacts::{Contact, Contacts},
};
use time::OffsetDateTime;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    match args.command {
        Command::Init => {
            Config::init()?;
        }
        Command::Contacts(contacts::Command::Create(args)) => {
            let config = Config::load()?;
            let path = args.path.unwrap_or(config.contacts);
            let path = Contact::create(args.name, path)?;
            println!("Created {}", path.display());
        }
        Command::Contacts(contacts::Command::List(args)) => {
            let config = Config::load()?;
            let contacts = Contacts::load(config.contacts)?;

            if args.all {
                print_contacts(contacts.all())?;
                return Ok(());
            }

            // TASK: Use local time instead. As of this writing, this is blocked
            //       by this issue: https://github.com/time-rs/time/issues/293
            let today = OffsetDateTime::now_utc().date();
            print_contacts(contacts.due(today))?;
        }
    }

    Ok(())
}

fn print_contacts<'r>(
    contacts: impl Iterator<Item = &'r Contact>,
) -> anyhow::Result<()> {
    for contact in contacts {
        println!("{}", contact.summary()?);
    }

    Ok(())
}
