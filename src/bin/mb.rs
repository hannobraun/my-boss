use clap::Clap as _;
use my_boss::{
    args::{Args, Command, ContactsCmd},
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
        Command::Contacts(ContactsCmd::List(args)) => {
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

fn print_contacts(
    contacts: impl Iterator<Item = Contact>,
) -> anyhow::Result<()> {
    for contact in contacts {
        println!("{}", contact.summary()?);
    }

    Ok(())
}
