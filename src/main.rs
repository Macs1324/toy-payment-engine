use std::{error::Error, io::Write};

use csv::ReaderBuilder;

use crate::{error::PaymentError, event::Event, io::EventCsvRecord, state::State};

pub mod client;
pub mod error;
pub mod event;
pub mod io;
pub mod state;

pub type ClientId = u16;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if &args.len() - 1 != 1 {
        return Err(Box::new(PaymentError::InvalidCliArguments));
    }
    let filename = &args[1];

    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(filename)?;

    let mut state = State::new();

    for record in reader.deserialize() {
        let record: EventCsvRecord = record?;
        let event: Event = record.try_into()?;

        state.apply_event(&event)?;
    }

    let clients = state.dump_clients();

    std::io::stdout().write_all("client,available,held,total,locked\n".as_bytes())?;
    for client in clients {
        std::io::stdout().write_fmt(format_args!(
            "{},{},{},{},{}\n",
            client.id(),
            client.available_amount(),
            client.held_amount(),
            client.total_amount(),
            client.is_locked()
        ))?;
    }
    std::io::stdout().flush()?;

    Ok(())
}
