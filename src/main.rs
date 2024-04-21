mod arguments;
mod parser;
mod sim800;

use arguments::Arguments;
use clap::Parser;
use log::error;
use parser::{NetworkRegistrationStatus, Response};
use sim800::Sim800;
use std::error::Error;
use std::time::Duration;
use time::format_description::BorrowedFormatItem;
use time::macros::format_description;

use crate::parser::SmsMessageStatus;

const DATE_FORMAT: &[BorrowedFormatItem<'_>] =
    format_description!("[year].[month].[day] [hour]:[minute]:[second]");

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let arguments = Arguments::parse();
    let port = serialport::new(arguments.serial_port(), arguments.baud_rate())
        .timeout(Duration::from_secs(10))
        .open()?;
    let mut sim800 = Sim800::new(port)?;
    sim800.send(r#"AT"#)?;

    match sim800.send_one(r#"AT+CREG?"#)? {
        Some(Response::NetworkRegistration {
            status: NetworkRegistrationStatus::Registered,
            ..
        })
        | Some(Response::NetworkRegistration {
            status: NetworkRegistrationStatus::RegisteredRoaming,
            ..
        }) => println!("Module ready and registered in network."),
        _ => {
            error!("Not registered in network.");

            return Ok(());
        }
    }

    sim800.send(r#"AT+CMEE=2"#)?;
    sim800.send(r#"AT+CMGF=1"#)?;
    sim800.send(r#"AT+CSCS="UCS2""#)?;

    if arguments.list_messages() {
        for message in sim800.send_list(r#"AT+CMGL="ALL""#)? {
            match message {
                Response::ListSmsMessage {
                    status,
                    address,
                    timestamp,
                    text,
                    ..
                } => {
                    let mark = match status {
                        SmsMessageStatus::ReceivedUnread => "<-",
                        SmsMessageStatus::ReceivedRead => "--",
                        SmsMessageStatus::StoredUnsent => "--",
                        SmsMessageStatus::StoredSent => "->",
                    };

                    println!(
                        "{}: {} {} {}",
                        timestamp.format(DATE_FORMAT)?,
                        address,
                        mark,
                        text,
                    );
                }
                _ => {}
            }
        }
    }

    if arguments.delete_messages() {
        for message in sim800.send_list(r#"AT+CMGL="ALL""#)? {
            match message {
                Response::ListSmsMessage { index, .. } => {
                    sim800.send(&format!(r#"AT+CMGD={},0"#, index))?
                }
                _ => {}
            }
        }
    }

    Ok(())
}
