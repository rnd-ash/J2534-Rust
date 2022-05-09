#[macro_use]
extern crate bitflags;
use std::{fmt::Display, convert::TryFrom};

// SAE J2534 API definition,
// Based on J2534.h

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum Protocol {
    J1850VPW = 0x01,
    J1850PWM = 0x02,
    ISO9141 = 0x03,
    ISO14230 = 0x04,
    CAN = 0x05,
    ISO15765 = 0x06,
    SCI_A_ENGINE = 0x07,
    SCI_A_TRANS = 0x08,
    SCI_B_ENGINE = 0x09,
    SCI_B_TRANS = 0x0A,
}

impl TryFrom<u32> for Protocol {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => Protocol::J1850VPW,
            0x02 => Protocol::J1850PWM,
            0x03 => Protocol::ISO9141,
            0x04 => Protocol::ISO14230,
            0x05 => Protocol::CAN,
            0x06 => Protocol::ISO15765,
            0x07 => Protocol::SCI_A_ENGINE,
            0x08 => Protocol::SCI_A_TRANS,
            0x09 => Protocol::SCI_B_ENGINE,
            0x0A => Protocol::SCI_B_TRANS,
            _ => return Err(())
        })
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Protocol::J1850VPW => "J1850 VPW",
            Protocol::J1850PWM => "J1850 PWM",
            Protocol::ISO9141 => "ISO 9141",
            Protocol::ISO14230 => "ISO 14230",
            Protocol::CAN => "CAN",
            Protocol::ISO15765 => "ISO 15765",
            Protocol::SCI_A_ENGINE => "SCI A ENGINE",
            Protocol::SCI_A_TRANS => "SCI A TRANS",
            Protocol::SCI_B_ENGINE => "SCI B ENGINE",
            Protocol::SCI_B_TRANS => "SCI B TRANS",
        })
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum IoctlID {
    GET_CONFIG = 0x01,
    SET_CONFIG = 0x02,
    READ_VBATT = 0x03,
    FIVE_BAUD_INIT = 0x04,
    FAST_INIT = 0x05,
    CLEAR_TX_BUFFER = 0x07,
    CLEAR_RX_BUFFER = 0x08,
    CLEAR_PERIODIC_MSGS = 0x09,
    CLEAR_MSG_FILTERS = 0x0A,
    CLEAR_FUNCT_MSG_LOOKUP_TABLE = 0x0B,
    ADD_TO_FUNCT_MSG_LOOKUP_TABLE = 0x0C,
    DELETE_FROM_FUNCT_MSG_LOOKUP_TABLE = 0x0D,
    READ_PROG_VOLTAGE = 0x0E,
}

impl TryFrom<u32> for IoctlID {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => IoctlID::GET_CONFIG,
            0x02 => IoctlID::SET_CONFIG,
            0x03 => IoctlID::READ_VBATT,
            0x04 => IoctlID::FIVE_BAUD_INIT,
            0x05 => IoctlID::FAST_INIT,
            0x06 => IoctlID::CLEAR_TX_BUFFER,
            0x07 => IoctlID::CLEAR_RX_BUFFER,
            0x08 => IoctlID::CLEAR_PERIODIC_MSGS,
            0x09 => IoctlID::CLEAR_MSG_FILTERS,
            0x0A => IoctlID::CLEAR_FUNCT_MSG_LOOKUP_TABLE,
            0x0B => IoctlID::ADD_TO_FUNCT_MSG_LOOKUP_TABLE,
            0x0D => IoctlID::DELETE_FROM_FUNCT_MSG_LOOKUP_TABLE,
            0x0E => IoctlID::READ_PROG_VOLTAGE,
            _ => return Err(())
        })
    }
}

impl std::fmt::Display for IoctlID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum IoctlParam {
    DATA_RATE = 0x01,
    LOOPBACK = 0x03,

    NODE_ADDRESS = 0x04,
    NETWORK_LINE = 0x05,
    P1_MIN = 0x06,
    P1_MAX = 0x07,
    P2_MIN = 0x08,
    P2_MAX = 0x09,
    P3_MIN = 0x0A,
    P3_MAX = 0x0B,
    P4_MIN = 0x0C,
    P4_MAX = 0x0D,
    W1 = 0x0E,
    W2 = 0x0F,
    W3 = 0x10,
    W4 = 0x11,
    W5 = 0x12,

    TIDLE = 0x13,
    TINL = 0x14,
    TWUP = 0x15,
    PARITY = 0x16,
    BIT_SAMPLE_POINT = 0x17,
    SYNCH_JUMP_WIDTH = 0x18,
    W0 = 0x19,
    T1_MAX = 0x1A,
    T2_MAX = 0x1B,
    T4_MAX = 0x1C,
    T5_MAX = 0x1D,
    ISO15765_BS = 0x1E,
    ISO15765_STMIN = 0x1F,

    DATA_BITS = 0x20,
    FIVE_BAUD_MOD = 0x21,
    BS_TX = 0x22,
    STMIN_TX = 0x23,
    T3_MAX = 0x24,
    ISO15765_WFT_MAX = 0x25,
    CAN_MIXED_FORMAT = 0x8000
}

impl TryFrom<u32> for IoctlParam {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => IoctlParam::DATA_RATE,
            0x03 => IoctlParam::LOOPBACK,
            0x04 => IoctlParam::NODE_ADDRESS,
            0x05 => IoctlParam::NETWORK_LINE,
            0x06 => IoctlParam::P1_MIN,
            0x07 => IoctlParam::P1_MAX,
            0x08 => IoctlParam::P2_MIN,
            0x09 => IoctlParam::P2_MAX,
            0x0A => IoctlParam::P3_MIN,
            0x0B => IoctlParam::P3_MAX,
            0x0C => IoctlParam::P4_MIN,
            0x0D => IoctlParam::P4_MAX,
            0x0E => IoctlParam::W1,
            0x0F => IoctlParam::W2,
            0x10 => IoctlParam::W3,
            0x11 => IoctlParam::W4,
            0x12 => IoctlParam::W5,
            0x13 => IoctlParam::TIDLE,
            0x14 => IoctlParam::TINL,
            0x15 => IoctlParam::TWUP,
            0x16 => IoctlParam::PARITY,
            0x17 => IoctlParam::BIT_SAMPLE_POINT,
            0x18 => IoctlParam::SYNCH_JUMP_WIDTH,
            0x19 => IoctlParam::W0,
            0x1A => IoctlParam::T1_MAX,
            0x1B => IoctlParam::T2_MAX,
            0x1C => IoctlParam::T4_MAX,
            0x1D => IoctlParam::T5_MAX,
            0x1E => IoctlParam::ISO15765_BS,
            0x1F => IoctlParam::ISO15765_STMIN,
            0x20 => IoctlParam::DATA_BITS,
            0x21 => IoctlParam::FIVE_BAUD_MOD,
            0x22 => IoctlParam::BS_TX,
            0x23 => IoctlParam::STMIN_TX,
            0x24 => IoctlParam::T3_MAX,
            0x25 => IoctlParam::ISO15765_WFT_MAX,
            0x8000 => IoctlParam::CAN_MIXED_FORMAT,
            _ => return Err(())
        })
    }
}

impl std::fmt::Display for IoctlParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum PassthruError {
    STATUS_NOERROR = 0x00,
    ERR_NOT_SUPPORTED = 0x01,
    ERR_INVALID_CHANNEL_ID = 0x02,
    ERR_INVALID_PROTOCOL_ID = 0x03,
    ERR_NULL_PARAMETER = 0x04,
    ERR_INVALID_IOCTL_VALUE = 0x05,
    ERR_INVALID_FLAGS = 0x06,
    ERR_FAILED = 0x07,
    ERR_DEVICE_NOT_CONNECTED = 0x08,
    ERR_TIMEOUT = 0x09,

    ERR_INVALID_MSG = 0x0A,
    ERR_INVALID_TIME_INTERVAL = 0x0B,
    ERR_EXCEEDED_LIMIT = 0x0C,
    ERR_INVALID_MSG_ID = 0x0D,
    ERR_DEVICE_IN_USE = 0x0E,
    ERR_INVALID_IOCTL_ID = 0x0F,
    ERR_BUFFER_EMPTY = 0x10,
    ERR_BUFFER_FULL = 0x11,
    ERR_BUFFER_OVERFLOW = 0x12,
    ERR_PIN_INVALID = 0x13,
    ERR_CHANNEL_IN_USE = 0x14,
    ERR_MSG_PROTOCOL_ID = 0x15,

    ERR_INVALID_FILTER_ID = 0x16,
    ERR_NO_FLOW_CONTROL = 0x17,
    ERR_NOT_UNIQUE = 0x18,
    ERR_INVALID_BAUDRATE = 0x19,
    ERR_INVALID_DEVICE_ID = 0x1A,
}

impl TryFrom<u32> for PassthruError {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => PassthruError::STATUS_NOERROR,
            0x01 => PassthruError::ERR_NOT_SUPPORTED,
            0x02 => PassthruError::ERR_INVALID_CHANNEL_ID,
            0x03 => PassthruError::ERR_INVALID_PROTOCOL_ID,
            0x04 => PassthruError::ERR_NULL_PARAMETER,
            0x05 => PassthruError::ERR_INVALID_IOCTL_VALUE,
            0x06 => PassthruError::ERR_INVALID_FLAGS,
            0x07 => PassthruError::ERR_FAILED,
            0x08 => PassthruError::ERR_DEVICE_NOT_CONNECTED,
            0x09 => PassthruError::ERR_TIMEOUT,
            0x0A => PassthruError::ERR_INVALID_MSG,
            0x0B => PassthruError::ERR_INVALID_TIME_INTERVAL,
            0x0C => PassthruError::ERR_EXCEEDED_LIMIT,
            0x0D => PassthruError::ERR_INVALID_MSG_ID,
            0x0E => PassthruError::ERR_DEVICE_IN_USE,
            0x0F => PassthruError::ERR_INVALID_IOCTL_ID,
            0x10 => PassthruError::ERR_BUFFER_EMPTY,
            0x11 => PassthruError::ERR_BUFFER_FULL,
            0x12 => PassthruError::ERR_BUFFER_OVERFLOW,
            0x13 => PassthruError::ERR_PIN_INVALID,
            0x14 => PassthruError::ERR_CHANNEL_IN_USE,
            0x15 => PassthruError::ERR_MSG_PROTOCOL_ID,
            0x16 => PassthruError::ERR_INVALID_FILTER_ID,
            0x17 => PassthruError::ERR_NO_FLOW_CONTROL,
            0x18 => PassthruError::ERR_NOT_UNIQUE,
            0x19 => PassthruError::ERR_INVALID_BAUDRATE,
            0x1A => PassthruError::ERR_INVALID_DEVICE_ID,
            _ => return Err(())
        })
    }
}

impl Display for PassthruError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            PassthruError::STATUS_NOERROR => "No Error",
            PassthruError::ERR_NOT_SUPPORTED => "Operation not supported",
            PassthruError::ERR_INVALID_CHANNEL_ID => "Invalid channel ID",
            PassthruError::ERR_INVALID_PROTOCOL_ID => "Invalid protocol ID",
            PassthruError::ERR_NULL_PARAMETER => "Null parameter received",
            PassthruError::ERR_INVALID_IOCTL_VALUE => "Invalid IOCTL Value",
            PassthruError::ERR_INVALID_FLAGS => "Invalid flags",
            PassthruError::ERR_FAILED => "Unspecified error",
            PassthruError::ERR_DEVICE_NOT_CONNECTED => "Device not connected",
            PassthruError::ERR_TIMEOUT => "Device timeout",
            PassthruError::ERR_INVALID_MSG => "Invalid or malformed message",
            PassthruError::ERR_INVALID_TIME_INTERVAL => "Time interval outside specified range",
            PassthruError::ERR_EXCEEDED_LIMIT => "Too many filters or periodic messages",
            PassthruError::ERR_INVALID_MSG_ID => "Message ID / Handle ID not recognized",
            PassthruError::ERR_DEVICE_IN_USE => "Device is already in use",
            PassthruError::ERR_INVALID_IOCTL_ID => "IOCTL ID not recognized",
            PassthruError::ERR_BUFFER_EMPTY => "Receive buffer is empty",
            PassthruError::ERR_BUFFER_FULL => "Transmit buffer is full",
            PassthruError::ERR_BUFFER_OVERFLOW => "Device buffer overflow",
            PassthruError::ERR_PIN_INVALID => "Unknown pin specified",
            PassthruError::ERR_CHANNEL_IN_USE => "Channel is already in use",
            PassthruError::ERR_MSG_PROTOCOL_ID => "Message protocol ID does not match that of the communication channel",
            PassthruError::ERR_INVALID_FILTER_ID => "Filter ID not recognized",
            PassthruError::ERR_NO_FLOW_CONTROL => "No flow control filter is set",
            PassthruError::ERR_NOT_UNIQUE => "An existing filter already matches",
            PassthruError::ERR_INVALID_BAUDRATE => "Unable to set requested baudrate",
            PassthruError::ERR_INVALID_DEVICE_ID => "Device ID not recognized",
        })
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum FilterType {
    PASS_FILTER = 0x01,
    BLOCK_FILTER = 0x02,
    FLOW_CONTROL_FILTER = 0x03,
}

impl TryFrom<u32> for FilterType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => FilterType::PASS_FILTER,
            0x02 => FilterType::BLOCK_FILTER,
            0x03 => FilterType::FLOW_CONTROL_FILTER,
            _ => return Err(())
        })
    }
}

impl std::fmt::Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            FilterType::PASS_FILTER => f.write_str("Pass filter"),
            FilterType::BLOCK_FILTER => f.write_str("Block filter"),
            FilterType::FLOW_CONTROL_FILTER => f.write_str("ISO-TP flow control filter"),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum LoopBackSetting {
    OFF = 0x00,
    ON = 0x01,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum DataBits {
    DATA_BITS_8 = 0x00,
    DATA_BITS_7 = 0x01,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum ParitySetting {
    NO_PARITY = 0x00,
    ODD_PARITY = 0x01,
    EVEN_PARITY = 0x02,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types, dead_code)]
pub enum J1850PWMNetworkLine {
    BUS_NORMAL = 0x00,
    BUS_PLUS = 0x01,
    BUS_MINUS = 0x02,
}

bitflags! {
    pub struct ConnectFlags: u32 {
        const CAN_29BIT_ID = 0x00000100;
        const ISO9141_NO_CHECKSUM = 0x00000200;
        const CAN_ID_BOTH = 0x00000800;
        const ISO15765_ADDR_TYPE = 0x00000080;
        const ISO9141_K_LINE_ONLY = 0x00001000;
    }
}

bitflags! {
    pub struct RxFlag: u32 {
        const CAN_29BIT_ID = 0x00000100;
        const ISO15765_ADDR_TYPE = 0x00000080;
        const ISO15765_PADDING_ERROR = 0x00000010;
        const TX_DONE = 0x00000008;
        const RX_BREAK = 0x00000004;
        const ISO15765_FIRST_FRAME = 0x00000002;
        const START_OF_MESSAGE = 0x00000002;
        const TX_MSG_TYPE = 0x00000001;
    }
}

bitflags! {
    pub struct TxFlag: u32 {
        const SCI_TX_VOLTAGE = 0x00800000;
        const SCI_MODE = 0x00400000;
        const BLOCKING = 0x00010000;
        const WAIT_P3_MIN_ONLY = 0x00000200;
        const CAN_29BIT_ID = 0x00000100;
        const CAN_EXTENDED_ID = 0x00000100;
        const ISO15765_ADDR_TYPE = 0x00000080;
        const ISO15765_EXT_ADDR = 0x00000080;
        const ISO15765_FRAME_PAD = 0x00000040;
        const TX_NORMAL_TRANSMIT = 0x00000000;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct PASSTHRU_MSG {
    pub protocol_id: u32,
    pub rx_status: u32,
    pub tx_flags: u32,
    pub timestamp: u32,
    pub data_size: u32,
    pub extra_data_size: u32,
    pub data: [u8; 4128],
}

impl Default for PASSTHRU_MSG {
    fn default() -> Self {
        PASSTHRU_MSG {
            protocol_id: 0,
            rx_status: 0,
            tx_flags: 0,
            timestamp: 0,
            data_size: 0,
            extra_data_size: 0,
            data: [0; 4128],
        }
    }
}

impl std::fmt::Display for PASSTHRU_MSG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!(
            "Protocol: {}, RxStatus: {:08X}, TxFlags: {:08X}, Data: {:02X?}", 
            Protocol::try_from(self.protocol_id).unwrap(),
            self.rx_status,
            self.tx_flags,
            &self.data[0..self.data_size as usize]
            
        ).as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct SBYTE_ARRAY {
    pub num_of_bytes: u32,
    pub byte_ptr: *mut u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct SConfig {
    pub parameter: u32,
    pub value: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct SConfigList {
    pub num_of_params: u32,
    pub config_ptr: *mut SConfig,
}