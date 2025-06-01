use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Port name or path.
    #[arg(short, long)]
    serial_port: String,

    /// Port baud rate.
    #[arg(short, long, default_value_t = 115_200)]
    baud_rate: u32,

    /// Port read timeout.
    #[arg(short, long, default_value_t = 30)]
    timeout: u64,

    /// List all SMS messages.
    #[arg(short, long, default_value_t = false)]
    list_messages: bool,

    /// Remove all SMS messages.
    #[arg(short, long, default_value_t = false)]
    delete_messages: bool,
}

impl Arguments {
    pub fn serial_port(&self) -> &str {
        &self.serial_port
    }

    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    pub fn timeout(&self) -> u64 {
        self.timeout
    }

    pub fn list_messages(&self) -> bool {
        self.list_messages
    }

    pub fn delete_messages(&self) -> bool {
        self.delete_messages
    }
}
