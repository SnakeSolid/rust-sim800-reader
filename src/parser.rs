use std::error::Error;

use nom::branch::alt;
use nom::bytes::streaming::is_not;
use nom::bytes::streaming::tag;
use nom::character::streaming::char;
use nom::character::streaming::hex_digit0;
use nom::character::streaming::i16;
use nom::character::streaming::i8;
use nom::character::streaming::u16;
use nom::character::streaming::u8;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::error::Error as NomError;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::Err;
use nom::Parser;
use time::Date;
use time::Month;
use time::OffsetDateTime;
use time::Time;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum EquipmentErrorSource {
    Mobile,
    Service,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MobileEquipmentError {
    Disabled,
    Code {
        source: EquipmentErrorSource,
        code: u16,
    },
    Message {
        source: EquipmentErrorSource,
        message: String,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum OperatorSelectionMode {
    Automatic,
    Manual,
    Unknown,
}

impl From<u8> for OperatorSelectionMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Automatic,
            1 => Self::Manual,
            2 | 3 | 4 => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum OperatorSelectionFormat {
    LongAlphanumeric,
    ShortAlphanumeric,
    Numeric,
}

impl From<u8> for OperatorSelectionFormat {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::LongAlphanumeric,
            1 => Self::ShortAlphanumeric,
            2 => Self::Numeric,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum NetworkRegistrationMode {
    DisableRegistration,
    EnableRegistration,
    EnableLocationRegistration,
}

impl From<u8> for NetworkRegistrationMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::DisableRegistration,
            1 => Self::EnableRegistration,
            2 => Self::EnableLocationRegistration,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum NetworkRegistrationStatus {
    Registered,
    SearchingOperator,
    RegistratonDenied,
    Unknown,
    RegisteredRoaming,
}

impl From<u8> for NetworkRegistrationStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Registered,
            2 => Self::SearchingOperator,
            3 => Self::RegistratonDenied,
            4 => Self::Unknown,
            5 => Self::RegisteredRoaming,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PhoneActivityStatus {
    Ready,
    Unknown,
    Ringing,
    CallInProgress,
}

impl From<u8> for PhoneActivityStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Ready,
            2 => Self::Unknown,
            3 => Self::Ringing,
            4 => Self::CallInProgress,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SignalQualityRssi {
    Value(i8),
    Unknown,
}

impl From<u8> for SignalQualityRssi {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Value(-115),
            1 => Self::Value(-111),
            31 => Self::Value(-52),
            99 => Self::Unknown,
            2..=30 => Self::Value(-110 + value as i8 * 2 - 4),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SignalQualityErrorRate {
    Value(u8),
    Unknown,
}

impl From<u8> for SignalQualityErrorRate {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Value(0),
            1 => Self::Value(2),
            2 => Self::Value(4),
            3 => Self::Value(8),
            4 => Self::Value(16),
            5 => Self::Value(32),
            6 => Self::Value(64),
            7 => Self::Value(128),
            99 => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BatteryChargeStatus {
    NotCharging,
    Charging,
    Finished,
}

impl From<u8> for BatteryChargeStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotCharging,
            1 => Self::Charging,
            2 => Self::Finished,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SmsMessageStatus {
    ReceivedUnread,
    ReceivedRead,
    StoredUnsent,
    StoredSent,
}

impl From<&str> for SmsMessageStatus {
    fn from(value: &str) -> Self {
        match value {
            "REC UNREAD" => Self::ReceivedUnread,
            "REC READ" => Self::ReceivedRead,
            "STO UNSEND" => Self::StoredUnsent,
            "STO SEND" => Self::StoredSent,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SmsMessageStorage {
    Sim,
    Phone,
}

impl From<&str> for SmsMessageStorage {
    fn from(value: &str) -> Self {
        match value {
            "SM" => Self::Sim,
            "ME" => Self::Phone,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CallDirection {
    MobileOriginated,
    MobileTerminated,
}

impl From<u8> for CallDirection {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::MobileOriginated,
            1 => Self::MobileTerminated,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CallState {
    Active,
    Held,
    Dialing,
    Alerting,
    Incoming,
    Waiting,
    Disconnect,
}

impl From<u8> for CallState {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Active,
            1 => Self::Held,
            2 => Self::Dialing,
            3 => Self::Alerting,
            4 => Self::Incoming,
            5 => Self::Waiting,
            6 => Self::Disconnect,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CallMode {
    Voice,
    Data,
    Fax,
}

impl From<u8> for CallMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Voice,
            1 => Self::Data,
            2 => Self::Fax,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CallMultiparty {
    NoMultiparty,
    Multiparty,
}

impl From<u8> for CallMultiparty {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoMultiparty,
            1 => Self::Multiparty,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Response {
    Ok,
    Error(MobileEquipmentError),
    OperatorSelection {
        mode: OperatorSelectionMode,
        format: Option<OperatorSelectionFormat>,
        operator: Option<String>,
    },
    NetworkRegistration {
        mode: NetworkRegistrationMode,
        status: NetworkRegistrationStatus,
        location: Option<String>,
        cell_id: Option<String>,
    },
    PhoneActivityStatus {
        status: PhoneActivityStatus,
    },
    SignalQuality {
        rssi: SignalQualityRssi,
        error_rate: SignalQualityErrorRate,
    },
    BatteryCharge {
        status: BatteryChargeStatus,
        level: u8,
        voltage: u16,
    },
    ListSmsMessage {
        index: u16,
        status: SmsMessageStatus,
        address: String,
        address_text: Option<String>,
        timestamp: OffsetDateTime,
        text: String,
    },
    ReadSmsMessage {
        status: SmsMessageStatus,
        address: String,
        address_text: Option<String>,
        timestamp: OffsetDateTime,
        text: String,
    },
    NewSmsMessage {
        storage: SmsMessageStorage,
        index: u16,
    },
    ListCurrentCalls {
        index: u16,
        direction: CallDirection,
        state: CallState,
        mode: CallMode,
        multiparty: CallMultiparty,
        number: Option<String>,
        number_type: Option<u16>,
        name: Option<String>,
    },
    CallReady,
    SmsReady,
    Ring,
    NoCarrier,
    Empty,
}

fn ucs2_to_uft8(text: &str) -> String {
    let mut chars = text.chars();
    let mut result = String::default();

    while let (Some(a), Some(b), Some(c), Some(d)) = (
        chars.next().and_then(|ch| ch.to_digit(16)),
        chars.next().and_then(|ch| ch.to_digit(16)),
        chars.next().and_then(|ch| ch.to_digit(16)),
        chars.next().and_then(|ch| ch.to_digit(16)),
    ) {
        if let Some(ch) = char::from_u32(a << 12 | b << 8 | c << 4 | d << 0) {
            result.push(ch as char);
        }
    }

    result
}

fn parse_quoted_text<'a>() -> impl Parser<&'a str, &'a str, NomError<&'a str>> {
    alt((tag("\"\""), delimited(char('"'), is_not("\""), char('"'))))
}

fn parse_quoted_hex<'a>() -> impl Parser<&'a str, String, NomError<&'a str>> {
    map(delimited(char('"'), hex_digit0, char('"')), ucs2_to_uft8)
}

fn parse_string_hex<'a>() -> impl Parser<&'a str, String, NomError<&'a str>> {
    map(hex_digit0, ucs2_to_uft8)
}

fn parse_timestamp<'a>() -> impl Parser<&'a str, OffsetDateTime, NomError<&'a str>> {
    map_res(
        tuple((char('"'),
        	i16,
        	char('/'),
        	u8,
        	char('/'),
        	u8,
        	char(','),
        	u8,
        	char(':'),
        	u8,
        	char(':'),
        	u8,
        	i8,
        	char('"'))),
        |(_, year,_, month,_, day,_, hour,_, minute,_, second, _, _)| -> Result<OffsetDateTime, Box<dyn Error>> {
        	let date = Date::from_calendar_date(2000 + year as i32, Month::try_from(month)?, day)?;
        	let time = Time::from_hms(hour, minute, second)?;

            Ok(OffsetDateTime::new_utc(date, time))
        },
    )
}

fn parse_operator_selection_mode<'a>(
) -> impl Parser<&'a str, OperatorSelectionMode, NomError<&'a str>> {
    map(u8, OperatorSelectionMode::from)
}

fn parse_network_registration_mode<'a>(
) -> impl Parser<&'a str, NetworkRegistrationMode, NomError<&'a str>> {
    map(u8, NetworkRegistrationMode::from)
}

fn parse_network_registration_status<'a>(
) -> impl Parser<&'a str, NetworkRegistrationStatus, NomError<&'a str>> {
    map(u8, NetworkRegistrationStatus::from)
}

fn parse_phone_activity_status_status<'a>(
) -> impl Parser<&'a str, PhoneActivityStatus, NomError<&'a str>> {
    map(u8, PhoneActivityStatus::from)
}

fn parse_signal_quality_rssi<'a>() -> impl Parser<&'a str, SignalQualityRssi, NomError<&'a str>> {
    map(u8, SignalQualityRssi::from)
}

fn parse_signal_quality_error_rate<'a>(
) -> impl Parser<&'a str, SignalQualityErrorRate, NomError<&'a str>> {
    map(u8, SignalQualityErrorRate::from)
}

fn parse_battery_charge_status<'a>() -> impl Parser<&'a str, BatteryChargeStatus, NomError<&'a str>>
{
    map(u8, BatteryChargeStatus::from)
}

fn parse_sms_messages_status<'a>() -> impl Parser<&'a str, SmsMessageStatus, NomError<&'a str>> {
    map(parse_quoted_text(), SmsMessageStatus::from)
}

fn parse_sms_message_storage<'a>() -> impl Parser<&'a str, SmsMessageStorage, NomError<&'a str>> {
    map(parse_quoted_text(), SmsMessageStorage::from)
}

fn parse_call_direction<'a>() -> impl Parser<&'a str, CallDirection, NomError<&'a str>> {
    map(u8, CallDirection::from)
}

fn parse_call_state<'a>() -> impl Parser<&'a str, CallState, NomError<&'a str>> {
    map(u8, CallState::from)
}

fn parse_call_mode<'a>() -> impl Parser<&'a str, CallMode, NomError<&'a str>> {
    map(u8, CallMode::from)
}

fn parse_call_multiparty<'a>() -> impl Parser<&'a str, CallMultiparty, NomError<&'a str>> {
    map(u8, CallMultiparty::from)
}

fn parse_operator_selection<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+COPS: "),
            parse_operator_selection_mode(),
            opt(map(
                tuple((char(','), u8, char(','), parse_quoted_text())),
                |(_, format, _, operator)| (format, operator),
            )),
            tag("\r\r"),
        )),
        |(_, mode, args, _)| Response::OperatorSelection {
            mode,
            format: args.map(|(format, _)| format.into()),
            operator: args.map(|(_, operator)| operator.into()),
        },
    )
}

fn parse_network_registration<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CREG: "),
            parse_network_registration_mode(),
            char(','),
            parse_network_registration_status(),
            opt(map(
                tuple((
                    char(','),
                    parse_quoted_text(),
                    char(','),
                    parse_quoted_text(),
                )),
                |(_, location, _, cell_id)| (location, cell_id),
            )),
            tag("\r\r"),
        )),
        |(_, mode, _, status, args, _)| Response::NetworkRegistration {
            mode,
            status,
            location: args.map(|(location, _)| location.into()),
            cell_id: args.map(|(_, cell_id)| cell_id.into()),
        },
    )
}

fn parse_phone_activity_status<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CPAS: "),
            parse_phone_activity_status_status(),
            tag("\r\r"),
        )),
        |(_, status, _)| Response::PhoneActivityStatus { status },
    )
}

fn parse_signal_quality<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CSQ: "),
            parse_signal_quality_rssi(),
            char(','),
            parse_signal_quality_error_rate(),
            tag("\r\r"),
        )),
        |(_, rssi, _, error_rate, _)| Response::SignalQuality { rssi, error_rate },
    )
}

fn parse_battery_charge<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CBC: "),
            parse_battery_charge_status(),
            char(','),
            u8,
            char(','),
            u16,
            tag("\r\r"),
        )),
        |(_, status, _, level, _, voltage, _)| Response::BatteryCharge {
            status,
            level,
            voltage,
        },
    )
}

fn parse_list_sms_messages<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CMGL: "),
            u16,
            char(','),
            parse_sms_messages_status(),
            char(','),
            parse_quoted_hex(),
            char(','),
            opt(parse_quoted_hex()),
            char(','),
            parse_timestamp(),
            char('\r'),
            parse_string_hex(),
            tag("\r\r"),
        )),
        |(_, index, _, status, _, address, _, address_text, _, timestamp, _, text, _)| {
            Response::ListSmsMessage {
                index,
                status,
                address: address.into(),
                address_text,
                timestamp,
                text,
            }
        },
    )
}

fn parse_read_sms_messages<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CMGR: "),
            parse_sms_messages_status(),
            char(','),
            parse_quoted_hex(),
            char(','),
            opt(parse_quoted_hex()),
            char(','),
            parse_timestamp(),
            char('\r'),
            parse_string_hex(),
            tag("\r\r"),
        )),
        |(_, status, _, address, _, address_text, _, timestamp, _, text, _)| {
            Response::ReadSmsMessage {
                status,
                address: address.into(),
                address_text,
                timestamp,
                text,
            }
        },
    )
}

fn parse_new_sms_messages<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CMTI: "),
            parse_sms_message_storage(),
            char(','),
            u16,
            char('\r'),
        )),
        |(_, storage, _, index, _)| Response::NewSmsMessage { storage, index },
    )
}

fn parse_list_current_calls<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(
        tuple((
            tag("+CLCC: "),
            u16,
            char(','),
            parse_call_direction(),
            char(','),
            parse_call_state(),
            char(','),
            parse_call_mode(),
            char(','),
            parse_call_multiparty(),
            opt(tuple((
                char(','),
                parse_quoted_text(),
                char(','),
                u16,
                char(','),
                parse_quoted_text(),
            ))),
            tag("\r\r"),
        )),
        |(_, index, _, direction, _, state, _, mode, _, multiparty, data, _)| {
            Response::ListCurrentCalls {
                index,
                direction,
                state,
                mode,
                multiparty,
                number: data.map(|(_, number, _, _, _, _)| number.into()),
                number_type: data.map(|(_, _, _, number_type, _, _)| number_type),
                name: data.map(|(_, _, _, _, _, name)| name.into()),
            }
        },
    )
}

fn parse_ok<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(tag("OK\r"), |_| Response::Ok)
}

fn parse_error<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(tag("ERROR\r"), |_| {
        Response::Error(MobileEquipmentError::Disabled)
    })
}

fn parse_error_code<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    alt((
        map(
            tuple((tag("+CME ERROR: "), u16, char('\r'))),
            |(_, code, _)| {
                Response::Error(MobileEquipmentError::Code {
                    source: EquipmentErrorSource::Mobile,
                    code,
                })
            },
        ),
        map(
            tuple((tag("+CMS ERROR: "), u16, char('\r'))),
            |(_, code, _)| {
                Response::Error(MobileEquipmentError::Code {
                    source: EquipmentErrorSource::Service,
                    code,
                })
            },
        ),
    ))
}

fn parse_error_message<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    alt((
        map(
            tuple((tag("+CME ERROR: "), is_not("\r"), char('\r'))),
            |(_, message, _): (_, &str, _)| {
                Response::Error(MobileEquipmentError::Message {
                    source: EquipmentErrorSource::Mobile,
                    message: message.into(),
                })
            },
        ),
        map(
            tuple((tag("+CMS ERROR: "), is_not("\r"), char('\r'))),
            |(_, message, _): (_, &str, _)| {
                Response::Error(MobileEquipmentError::Message {
                    source: EquipmentErrorSource::Service,
                    message: message.into(),
                })
            },
        ),
    ))
}

fn parse_call_ready<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(tag("Call Ready\r"), |_| Response::CallReady)
}

fn parse_sms_ready<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(tag("SMS Ready\r"), |_| Response::SmsReady)
}

fn parse_ring<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(tag("RING\r"), |_| Response::Ring)
}

fn parse_no_carrier<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(tag("NO CARRIER\r"), |_| Response::NoCarrier)
}

fn parse_empty<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    map(char('\r'), |_| Response::Empty)
}

fn parser<'a>() -> impl Parser<&'a str, Response, NomError<&'a str>> {
    alt((
        parse_ok(),
        parse_error(),
        parse_error_code(),
        parse_error_message(),
        parse_call_ready(),
        parse_sms_ready(),
        parse_ring(),
        parse_no_carrier(),
        parse_operator_selection(),
        parse_network_registration(),
        parse_phone_activity_status(),
        parse_signal_quality(),
        parse_battery_charge(),
        parse_list_sms_messages(),
        parse_read_sms_messages(),
        parse_new_sms_messages(),
        parse_list_current_calls(),
        parse_empty(),
    ))
}

#[derive(Debug)]
pub enum ParseResult {
    Success { response: Response, tail: String },
    Incomplete,
    Error(String),
}

pub fn parse(data: &str) -> ParseResult {
    match parser().parse(data) {
        Ok((tail, response)) => ParseResult::Success {
            response,
            tail: tail.into(),
        },
        Err(Err::Incomplete(_)) => ParseResult::Incomplete,
        Err(error) => ParseResult::Error(error.to_string()),
    }
}
