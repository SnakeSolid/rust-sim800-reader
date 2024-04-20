mod error;

use error::Sim800Error;
use log::info;
use log::warn;
use serialport::SerialPort;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SendError;
use std::sync::mpsc::Sender;
use std::thread::Builder;
use std::thread::JoinHandle;

use crate::parser::parse;
use crate::parser::ParseResult;
use crate::parser::Response;

#[derive(Debug)]
pub struct Sim800 {
    port: Box<dyn SerialPort>,
    command_sender: Sender<String>,
    response_receiver: Receiver<Response>,
    command_write: JoinHandle<Result<(), Sim800Error>>,
    response_read: JoinHandle<Result<(), Sim800Error>>,
}

impl Sim800 {
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self, Sim800Error> {
        let (command_sender, command_receiver) = mpsc::channel::<String>();
        let (response_sender, response_receiver) = mpsc::channel();
        let mut other = port.try_clone().map_err(Sim800Error::from)?;
        let command_write = Builder::new().spawn(move || {
            for command in command_receiver {
                info!(">> {}", command);

                other
                    .write_all(command.as_bytes())
                    .map_err(Sim800Error::from)?;
                other
                    .write_all("\r\n".as_bytes())
                    .map_err(Sim800Error::from)?;
                other.flush().map_err(Sim800Error::from)?;
            }

            Ok(())
        })?;
        let mut other = port.try_clone().map_err(Sim800Error::from)?;
        let response_read = Builder::new().spawn(move || {
            let mut buffer: [u8; 1] = [0; 1];
            let mut line = String::new();
            let mut text = String::new();

            loop {
                match other.read(&mut buffer)? {
                    0 => break,
                    _ if buffer[0] == b'\n' => {
                        if !line.ends_with("\r\r") {
                            text.push_str(&line);

                            match parse(&text) {
                                ParseResult::Success { response, tail } => {
                                    response_sender.send(response)?;

                                    text.clear();
                                    text.push_str(&tail);
                                }
                                ParseResult::Incomplete => {}
                                ParseResult::Error(error) => {
                                    warn!("<< Error: {}, Text: {}", error, text);

                                    text.clear();
                                }
                            }
                        }

                        line.clear();
                    }
                    _ => line.push(buffer[0] as char),
                }
            }

            Ok(())
        })?;

        Ok(Self {
            port,
            command_sender,
            response_receiver,
            command_write,
            response_read,
        })
    }

    pub fn send(&mut self, command: &str) -> Result<(), SendError<String>> {
        self.command_sender.send(command.into())?;

        for response in &self.response_receiver {
            info!("<< {:?}", response);

            match response {
                Response::Ok | Response::Error(_) => break,
                _ => {}
            }
        }

        Ok(())
    }

    pub fn send_one(&mut self, command: &str) -> Result<Option<Response>, SendError<String>> {
        let mut result = None;

        self.command_sender.send(command.into())?;

        for response in &self.response_receiver {
            info!("<< {:?}", response);

            match response {
                Response::Ok | Response::Error(_) => break,
                _ => result = Some(response),
            }
        }

        Ok(result)
    }

    pub fn send_list(&mut self, command: &str) -> Result<Vec<Response>, SendError<String>> {
        let mut result = Vec::new();

        self.command_sender.send(command.into())?;

        for response in &self.response_receiver {
            info!("<< {:?}", response);

            match response {
                Response::Ok | Response::Error(_) => break,
                _ => result.push(response),
            }
        }

        Ok(result)
    }

    pub fn join(self) -> Result<(), Sim800Error> {
        drop(self.port);
        drop(self.command_sender);
        drop(self.response_receiver);

        self.command_write
            .join()
            .expect("Command thread panicked")?;
        self.response_read
            .join()
            .expect("Command thread panicked")?;

        Ok(())
    }
}
