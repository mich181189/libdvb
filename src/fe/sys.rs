use anyhow::Context;
use std::fmt::Debug;
use std::str::FromStr;
use std::{fmt, mem};

pub use {
    fe_code_rate::*, fe_delivery_system::*, fe_guard_interval::*, fe_hierarchy::*,
    fe_interleaving::*, fe_modulation::*, fe_pilot::*, fe_rolloff::*, fe_sec_mini_cmd::*,
    fe_sec_tone_mode::*, fe_sec_voltage::*, fe_spectral_inversion::*, fe_transmit_mode::*,
    fe_type::*, DtvProperty::*, DtvStat::*,
};

use bitflags::bitflags;
use strum::{Display, EnumString, FromRepr};

bitflags! {
    /// Frontend capabilities
    #[repr(C)]
    pub struct fe_caps : u32 {
        /// There's something wrong at the frontend, and it can't report its capabilities
        const FE_IS_STUPID = 0x0;
        /// Can auto-detect frequency spectral band inversion
        const FE_CAN_INVERSION_AUTO = 0x1;
        /// Supports FEC 1/2
        const FE_CAN_FEC_1_2 = 0x2;
        /// Supports FEC 2/3
        const FE_CAN_FEC_2_3 = 0x4;
        /// Supports FEC 3/4
        const FE_CAN_FEC_3_4 = 0x8;
        /// Supports FEC 4/5
        const FE_CAN_FEC_4_5 = 0x10;
        /// Supports FEC 5/6
        const FE_CAN_FEC_5_6 = 0x20;
        /// Supports FEC 6/7
        const FE_CAN_FEC_6_7 = 0x40;
        /// Supports FEC 7/8
        const FE_CAN_FEC_7_8 = 0x80;
        /// Supports FEC 8/9
        const FE_CAN_FEC_8_9 = 0x100;
        /// Can auto-detect FEC
        const FE_CAN_FEC_AUTO = 0x200;
        /// Supports QPSK modulation
        const FE_CAN_QPSK = 0x400;
        /// Supports 16-QAM modulation
        const FE_CAN_QAM_16 = 0x800;
        /// Supports 32-QAM modulation
        const FE_CAN_QAM_32 = 0x1000;
        /// Supports 64-QAM modulation
        const FE_CAN_QAM_64 = 0x2000;
        /// Supports 128-QAM modulation
        const FE_CAN_QAM_128 = 0x4000;
        /// Supports 256-QAM modulation
        const FE_CAN_QAM_256 = 0x8000;
        /// Can auto-detect QAM modulation
        const FE_CAN_QAM_AUTO = 0x10000;
        /// Can auto-detect transmission mode
        const FE_CAN_TRANSMISSION_MODE_AUTO = 0x20000;
        /// Can auto-detect bandwidth
        const FE_CAN_BANDWIDTH_AUTO = 0x40000;
        /// Can auto-detect guard interval
        const FE_CAN_GUARD_INTERVAL_AUTO = 0x80000;
        /// Can auto-detect hierarchy
        const FE_CAN_HIERARCHY_AUTO = 0x100000;
        /// Supports 8-VSB modulation
        const FE_CAN_8VSB = 0x200000;
        /// Supports 16-VSB modulation
        const FE_CAN_16VSB = 0x400000;
        /// Unused
        const FE_HAS_EXTENDED_CAPS = 0x800000;
        /// Supports multistream filtering
        const FE_CAN_MULTISTREAM = 0x4000000;
        /// Supports "turbo FEC" modulation
        const FE_CAN_TURBO_FEC = 0x8000000;
        /// Supports "2nd generation" modulation, e. g. DVB-S2, DVB-T2, DVB-C2
        const FE_CAN_2G_MODULATION = 0x10000000;
        /// Unused
        const FE_NEEDS_BENDING = 0x20000000;
        /// Can recover from a cable unplug automatically
        const FE_CAN_RECOVER = 0x40000000;
        /// Can stop spurious TS data output
        const FE_CAN_MUTE_TS = 0x80000000;
    }
}

/// DEPRECATED: Should be kept just due to backward compatibility
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_type {
    FE_QPSK = 0,
    FE_QAM = 1,
    FE_OFDM = 2,
    FE_ATSC = 3,
}

/// Frontend properties and capabilities
/// The frequencies are specified in Hz for Terrestrial and Cable systems.
/// The frequencies are specified in kHz for Satellite systems.
#[repr(C)]
#[derive(Debug)]
pub struct FeInfo {
    /// Name of the frontend
    pub name: [std::os::raw::c_char; 128],
    /// DEPRECATED: frontend delivery system
    pub fe_type: fe_type,
    /// Minimal frequency supported by the frontend
    pub frequency_min: u32,
    /// Maximal frequency supported by the frontend
    pub frequency_max: u32,
    /// All frequencies are multiple of this value
    pub frequency_stepsize: u32,
    /// Frequency tolerance
    pub frequency_tolerance: u32,
    /// Minimal symbol rate, in bauds (for Cable/Satellite systems)
    pub symbol_rate_min: u32,
    /// Maximal symbol rate, in bauds (for Cable/Satellite systems)
    pub symbol_rate_max: u32,
    /// Maximal symbol rate tolerance, in ppm (for Cable/Satellite systems)
    pub symbol_rate_tolerance: u32,
    /// DEPRECATED
    pub notifier_delay: u32,
    /// Capabilities supported by the frontend
    pub caps: fe_caps,
}

impl Default for FeInfo {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

impl FeInfo {
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut FeInfo {
        self as *mut _
    }
}

/// DiSEqC master command
/// Check out the DiSEqC bus spec available on http://www.eutelsat.org/ for
/// the possible messages that can be used.
#[repr(C)]
#[derive(Debug)]
pub struct DiseqcMasterCmd {
    /// DiSEqC message to be sent. It contains a 3 bytes header with:
    /// framing + address + command, and an optional argument
    /// of up to 3 bytes of data.
    pub msg: [u8; 6],
    /// Length of the DiSEqC message. Valid values are 3 to 6.
    pub len: u8,
}

impl Default for DiseqcMasterCmd {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

/// DiSEqC received data
#[repr(C)]
#[derive(Debug)]
pub struct DiseqcSlaveReply {
    /// DiSEqC message buffer to store a message received via DiSEqC.
    /// It contains one byte header with: framing and
    /// an optional argument of up to 3 bytes of data.
    pub msg: [u8; 4],
    /// Length of the DiSEqC message. Valid values are 0 to 4,
    /// where 0 means no message.
    pub len: u8,
    /// Return from ioctl after timeout ms with errorcode when
    /// no message was received.
    pub timeout: u32,
}

impl Default for DiseqcSlaveReply {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

/// DC Voltage used to feed the LNBf
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
pub enum fe_sec_voltage {
    /// Output 13V to the LNB. Vertical linear. Right circular.
    SEC_VOLTAGE_13 = 0,
    /// Output 18V to the LNB. Horizontal linear. Left circular.
    SEC_VOLTAGE_18 = 1,
    /// Don't feed the LNB with a DC voltage
    SEC_VOLTAGE_OFF = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
pub enum fe_sec_tone_mode {
    /// Sends a 22kHz tone burst to the antenna
    SEC_TONE_ON = 0,
    /// Don't send a 22kHz tone to the antenna (except if the FE_DISEQC_* ioctl are called)
    SEC_TONE_OFF = 1,
}

/// Type of mini burst to be sent
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
pub enum fe_sec_mini_cmd {
    /// Sends a mini-DiSEqC 22kHz '0' Tone Burst to select satellite-A
    SEC_MINI_A = 0,
    /// Sends a mini-DiSEqC 22kHz '1' Data Burst to select satellite-B
    SEC_MINI_B = 1,
}

bitflags! {
    /// Enumerates the possible frontend status
    #[repr(C)]
    pub struct fe_status : u32 {
        /// The frontend doesn't have any kind of lock. That's the initial frontend status
        const FE_NONE = 0x00;
        /// Has found something above the noise level
        const FE_HAS_SIGNAL = 0x01;
        /// Has found a signal
        const FE_HAS_CARRIER = 0x02;
        /// FEC inner coding (Viterbi, LDPC or other inner code) is stable.
        const FE_HAS_VITERBI = 0x04;
        /// Synchronization bytes was found
        const FE_HAS_SYNC = 0x08;
        /// Digital TV were locked and everything is working
        const FE_HAS_LOCK = 0x10;
        /// Fo lock within the last about 2 seconds
        const FE_TIMEDOUT = 0x20;
        /// Frontend was reinitialized, application is recommended
        /// to reset DiSEqC, tone and parameters
        const FE_REINIT = 0x40;
    }
}

/// Spectral band inversion
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Clone, Copy)]
pub enum fe_spectral_inversion {
    #[strum(serialize = "OFF")]
    INVERSION_OFF = 0,
    #[strum(serialize = "ON")]
    INVERSION_ON = 1,
    #[strum(serialize = "AUTO")]
    INVERSION_AUTO = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum fe_code_rate {
    #[strum(serialize = "NONE")]
    FEC_NONE = 0,
    #[strum(serialize = "1/2")]
    FEC_1_2 = 1,
    #[strum(serialize = "2/3")]
    FEC_2_3 = 2,
    #[strum(serialize = "3/4")]
    FEC_3_4 = 3,
    #[strum(serialize = "4/5")]
    FEC_4_5 = 4,
    #[strum(serialize = "5/6")]
    FEC_5_6 = 5,
    #[strum(serialize = "6/7")]
    FEC_6_7 = 6,
    #[strum(serialize = "7/8")]
    FEC_7_8 = 7,
    #[strum(serialize = "8/9")]
    FEC_8_9 = 8,
    #[strum(serialize = "AUTO")]
    FEC_AUTO = 9,
    #[strum(serialize = "3/5")]
    FEC_3_5 = 10,
    #[strum(serialize = "9/10")]
    FEC_9_10 = 11,
    #[strum(serialize = "2/5")]
    FEC_2_5 = 12,
    #[strum(serialize = "1/4")]
    FEC_1_4 = 13,
    #[strum(serialize = "1/3")]
    FEC_1_3 = 14,
}

/// Type of modulation/constellation
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_modulation {
    QPSK = 0,
    #[strum(serialize = "QAM/16")]
    QAM_16 = 1,
    #[strum(serialize = "QAM/32")]
    QAM_32 = 2,
    #[strum(serialize = "QAM/64")]
    QAM_64 = 3,
    #[strum(serialize = "QAM/128")]
    QAM_128 = 4,
    #[strum(serialize = "QAM/256")]
    QAM_256 = 5,
    #[strum(serialize = "QAM/AUTO")]
    QAM_AUTO = 6,
    #[strum(serialize = "VSB/8")]
    VSB_8 = 7,
    #[strum(serialize = "VSB/16")]
    VSB_16 = 8,
    #[strum(serialize = "PSK/8")]
    PSK_8 = 9,
    #[strum(serialize = "APSK/16")]
    APSK_16 = 10,
    #[strum(serialize = "APSK/32")]
    APSK_32 = 11,
    #[strum(serialize = "DQPSK")]
    DQPSK = 12,
    #[strum(serialize = "QAM/4/NR")]
    QAM_4_NR = 13,
    #[strum(serialize = "APSK/64")]
    APSK_64 = 14,
    #[strum(serialize = "APSK/128")]
    APSK_128 = 15,
    #[strum(serialize = "APSK/256")]
    APSK_256 = 16,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_transmit_mode {
    #[strum(serialize = "2K")]
    TRANSMISSION_MODE_2K = 0,
    #[strum(serialize = "8K")]
    TRANSMISSION_MODE_8K = 1,
    #[strum(serialize = "AUTO")]
    TRANSMISSION_MODE_AUTO = 2,
    #[strum(serialize = "4K")]
    TRANSMISSION_MODE_4K = 3,
    #[strum(serialize = "1K")]
    TRANSMISSION_MODE_1K = 4,
    #[strum(serialize = "16K")]
    TRANSMISSION_MODE_16K = 5,
    #[strum(serialize = "32K")]
    TRANSMISSION_MODE_32K = 6,
    #[strum(serialize = "C1")]
    TRANSMISSION_MODE_C1 = 7,
    #[strum(serialize = "C3780")]
    TRANSMISSION_MODE_C3780 = 8,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_guard_interval {
    #[strum(serialize = "1/32")]
    GUARD_INTERVAL_1_32 = 0,
    #[strum(serialize = "1/16")]
    GUARD_INTERVAL_1_16 = 1,
    #[strum(serialize = "1/8")]
    GUARD_INTERVAL_1_8 = 2,
    #[strum(serialize = "1/4")]
    GUARD_INTERVAL_1_4 = 3,
    #[strum(serialize = "AUTO")]
    GUARD_INTERVAL_AUTO = 4,
    #[strum(serialize = "1/128")]
    GUARD_INTERVAL_1_128 = 5,
    #[strum(serialize = "19/128")]
    GUARD_INTERVAL_19_128 = 6,
    #[strum(serialize = "19/256")]
    GUARD_INTERVAL_19_256 = 7,
    #[strum(serialize = "PN420")]
    GUARD_INTERVAL_PN420 = 8,
    #[strum(serialize = "PN595")]
    GUARD_INTERVAL_PN595 = 9,
    #[strum(serialize = "PN945")]
    GUARD_INTERVAL_PN945 = 10,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_hierarchy {
    #[strum(serialize = "NONE")]
    HIERARCHY_NONE = 0,
    #[strum(serialize = "1")]
    HIERARCHY_1 = 1,
    #[strum(serialize = "2")]
    HIERARCHY_2 = 2,
    #[strum(serialize = "4")]
    HIERARCHY_4 = 3,
    #[strum(serialize = "AUTO")]
    HIERARCHY_AUTO = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_interleaving {
    #[strum(serialize = "NONE")]
    INTERLEAVING_NONE = 0,
    #[strum(serialize = "AUTO")]
    INTERLEAVING_AUTO = 1,
    #[strum(serialize = "240")]
    INTERLEAVING_240 = 2,
    #[strum(serialize = "720")]
    INTERLEAVING_720 = 3,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_pilot {
    PILOT_ON = 0,
    PILOT_OFF = 1,
    PILOT_AUTO = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_rolloff {
    ROLLOFF_35 = 0,
    ROLLOFF_20 = 1,
    ROLLOFF_25 = 2,
    ROLLOFF_AUTO = 3,
    ROLLOFF_15 = 4,
    ROLLOFF_10 = 5,
    ROLLOFF_5 = 6,
}

#[derive(EnumString, Display, FromRepr, Debug, Copy, Clone)]
#[repr(u32)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
pub enum fe_delivery_system {
    #[strum(to_string = "none")]
    SYS_UNDEFINED = 0,
    #[strum(to_string = "dbvc/annex_a")]
    SYS_DVBC_ANNEX_A = 1,
    #[strum(to_string = "dvbc/annex_b")]
    SYS_DVBC_ANNEX_B = 2,
    #[strum(to_string = "dvbt")]
    SYS_DVBT = 3,
    #[strum(to_string = "dss")]
    SYS_DSS = 4,
    #[strum(to_string = "dvbs")]
    SYS_DVBS = 5,
    #[strum(to_string = "dvbs2")]
    SYS_DVBS2 = 6,
    #[strum(to_string = "dvbh")]
    SYS_DVBH = 7,
    #[strum(to_string = "isdbt")]
    SYS_ISDBT = 8,
    #[strum(to_string = "isdbs")]
    SYS_ISDBS = 9,
    #[strum(to_string = "isdbc")]
    SYS_ISDBC = 10,
    #[strum(to_string = "atsc")]
    SYS_ATSC = 11,
    #[strum(to_string = "atsc-m/h")]
    SYS_ATSCMH = 12,
    #[strum(to_string = "dtmb")]
    SYS_DTMB = 13,
    #[strum(to_string = "cmmb")]
    SYS_CMMB = 14,
    #[strum(to_string = "dab")]
    SYS_DAB = 15,
    #[strum(to_string = "dvbt2", serialize = "dvbt22")]
    SYS_DVBT2 = 16,
    #[strum(to_string = "dvbs/turbo")]
    SYS_TURBO = 17,
    #[strum(to_string = "dvbc/annex_c")]
    SYS_DVBC_ANNEX_C = 18,
    #[strum(to_string = "dvbc2")]
    SYS_DVBC2 = 19,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(EnumString, Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_lna {
    LNA_OFF = 0,
    LNA_ON = 1,
    LNA_AUTO = 0xFFFFFFFF,
}

// From here on, structures passed to Linux
pub trait WrappedSlice<T> {
    fn slice(&self) -> &[T];
}

pub trait WrappedResult<T> {
    fn get(&self) -> anyhow::Result<T>;
}

pub trait DtvStatType {
    fn get_decibel(&self) -> Option<i64>;
    fn get_relative(&self) -> Option<u16>;
    fn get_counter(&self) -> Option<u64>;
    fn get_decibel_float(&self) -> Option<f64> {
        Some((self.get_decibel()? as f64) / 1000.0)
    }
    fn get_relative_percentage(&self) -> Option<u8> {
        Some((((self.get_relative()? as u32) * 100) / 65535) as u8)
    }
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct NoScale {
    __reserved: [u8; 8],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ScaleDecibel {
    pub scale: i64,
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ScaleRelative {
    pub scale: u16,
    __reserved: [u8; 6],
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct ScaleCounter {
    pub scale: u64,
}

/// Used for reading a DTV status property
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum DtvStat {
    /// That QoS measure is not available. That could indicate
    /// a temporary or a permanent condition.
    FE_SCALE_NOT_AVAILABLE(NoScale),
    /// The scale is measured in 0.001 dB steps, typically used on signal measures.
    FE_SCALE_DECIBEL(ScaleDecibel),
    /// The scale is a relative percentual measure,
    /// ranging from 0 (0%) to 0xffff (100%).
    FE_SCALE_RELATIVE(ScaleRelative),
    /// The scale counts the occurrence of an event, like
    /// bit error, block error, lapsed time.
    FE_SCALE_COUNTER(ScaleCounter),
}

impl DtvStatType for DtvStat {
    fn get_decibel(&self) -> Option<i64> {
        match self {
            FE_SCALE_DECIBEL(s) => Some(s.scale),
            _ => None,
        }
    }
    fn get_relative(&self) -> Option<u16> {
        match self {
            FE_SCALE_RELATIVE(s) => Some(s.scale),
            _ => None,
        }
    }
    fn get_counter(&self) -> Option<u64> {
        match self {
            FE_SCALE_COUNTER(s) => Some(s.scale),
            _ => None,
        }
    }
}

pub const MAX_DTV_STATS: usize = 4;

/// Store Digital TV frontend statistics
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvFrontendStats {
    len: u8,
    stat: [DtvStat; MAX_DTV_STATS],
}

impl WrappedSlice<DtvStat> for DtvFrontendStats {
    fn slice(&self) -> &[DtvStat] {
        let len = ::std::cmp::min(self.len as usize, self.stat.len());
        &self.stat[0..len]
    }
}

impl fmt::Debug for DtvFrontendStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.slice().into_iter()).finish()
    }
}

impl DtvStatType for DtvFrontendStats {
    fn get_decibel(&self) -> Option<i64> {
        for stat in self.slice() {
            if let Some(v) = stat.get_decibel() {
                return Some(v);
            }
        }
        None
    }
    fn get_relative(&self) -> Option<u16> {
        for stat in self.slice() {
            if let Some(v) = stat.get_relative() {
                return Some(v);
            }
        }
        None
    }
    fn get_counter(&self) -> Option<u64> {
        for stat in self.slice() {
            if let Some(v) = stat.get_counter() {
                return Some(v);
            }
        }
        None
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvPropertyBuffer {
    data: [u8; 32],
    len: u32,
}

impl WrappedSlice<u8> for DtvPropertyBuffer {
    fn slice(&self) -> &[u8] {
        let len = ::std::cmp::min(self.len as usize, self.data.len());
        &self.data[0..len]
    }
}

impl fmt::Debug for DtvPropertyBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.slice().into_iter()).finish()
    }
}

const DATA_SIZE: usize = 56;

#[repr(C, packed)]
pub struct DtvPropertyRequest<T, const N: usize> {
    __reserved: [u32; 3],
    data: T,
    padding: [u8; N],
    result: i32, // Unused
}

impl<T, const N: usize> DtvPropertyRequest<T, N> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            __reserved: [0; 3],
            data,
            padding: [0; N],
            result: 0,
        }
    }
}

impl<T, const N: usize> Default for DtvPropertyRequest<T, N> {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

pub type DtvPropertyRequestVoid = DtvPropertyRequest<(), DATA_SIZE>;

impl WrappedResult<()> for DtvPropertyRequestVoid {
    #[inline]
    fn get(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Debug for DtvPropertyRequestVoid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("()")
    }
}

pub type DtvPropertyRequestInt<T> = DtvPropertyRequest<T, { DATA_SIZE - 4 }>;

impl<T: Copy + Debug> WrappedResult<T> for DtvPropertyRequestInt<T> {
    #[inline]
    fn get(&self) -> anyhow::Result<T> {
        Ok(self.data)
    }
}

impl<T: Copy + Debug> Debug for DtvPropertyRequestInt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get().fmt(f)
    }
}

pub type DtvPropertyRequestFrontendStats = DtvPropertyRequest<DtvFrontendStats, { DATA_SIZE - 37 }>;

impl WrappedResult<DtvFrontendStats> for DtvPropertyRequestFrontendStats {
    #[inline]
    fn get(&self) -> anyhow::Result<DtvFrontendStats> {
        Ok(self.data)
    }
}

impl Debug for DtvPropertyRequestFrontendStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

pub type DtvPropertyRequestDeliverySystems =
    DtvPropertyRequest<DtvPropertyBuffer, { DATA_SIZE - 4 - 32 }>;

impl WrappedResult<Vec<fe_delivery_system>> for DtvPropertyRequestDeliverySystems {
    #[inline]
    fn get(&self) -> Result<Vec<fe_delivery_system>, anyhow::Error> {
        self.data
            .slice()
            .into_iter()
            .map(|&x| fe_delivery_system::from_repr(x as u32).context("Invalid delivery system"))
            .try_collect()
    }
}

impl Debug for DtvPropertyRequestDeliverySystems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.get().unwrap().iter()).finish()
    }
}

#[repr(C, packed)]
pub struct DtvPropertyNotImplementedLinux {
    __reserved: [u8; DATA_SIZE],
}

impl Debug for DtvPropertyNotImplementedLinux {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Not implemented")
    }
}

#[deprecated(
    note = "Not implemented, please fork libdvb and provide a correct implementation for this property."
)]
type DtvPropertyNotImplemented = DtvPropertyNotImplementedLinux;
#[deprecated]
type DtvPropertyDeprecated = DtvPropertyNotImplementedLinux;

/// DVBv5 property Commands
#[repr(u32, C)]
#[allow(non_camel_case_types)]
#[allow(deprecated)]
#[derive(Debug)]
pub enum DtvProperty {
    DTV_UNDEFINED(DtvPropertyNotImplementedLinux),
    DTV_TUNE(DtvPropertyRequestVoid),
    DTV_CLEAR(DtvPropertyRequestVoid),
    DTV_FREQUENCY(DtvPropertyRequestInt<u32>),
    DTV_MODULATION(DtvPropertyRequestInt<fe_modulation>),
    DTV_BANDWIDTH_HZ(DtvPropertyRequestInt<u32>),
    DTV_INVERSION(DtvPropertyRequestInt<fe_spectral_inversion>),
    DTV_DISEQC_MASTER(DtvPropertyNotImplementedLinux),
    DTV_SYMBOL_RATE(DtvPropertyRequestInt<u32>),
    DTV_INNER_FEC(DtvPropertyRequestInt<fe_code_rate>),
    DTV_VOLTAGE(DtvPropertyRequestInt<fe_sec_voltage>),
    DTV_TONE(DtvPropertyRequestInt<fe_sec_tone_mode>),
    DTV_PILOT(DtvPropertyRequestInt<fe_pilot>),
    DTV_ROLLOFF(DtvPropertyRequestInt<fe_rolloff>),
    DTV_DISEQC_SLAVE_REPLY(DtvPropertyNotImplementedLinux),

    /* Basic enumeration set for querying unlimited capabilities */
    DTV_FE_CAPABILITY_COUNT(DtvPropertyNotImplementedLinux),
    DTV_FE_CAPABILITY(DtvPropertyNotImplementedLinux),
    DTV_DELIVERY_SYSTEM(DtvPropertyRequestInt<fe_delivery_system>),

    /* ISDB-T and ISDB-Tsb */
    // Please fork
    DTV_ISDBT_PARTIAL_RECEPTION(DtvPropertyRequestInt<i32>),
    DTV_ISDBT_SOUND_BROADCASTING(DtvPropertyRequestInt<i32>),

    DTV_ISDBT_SB_SUBCHANNEL_ID(DtvPropertyRequestInt<i32>),
    DTV_ISDBT_SB_SEGMENT_IDX(DtvPropertyRequestInt<i32>),
    DTV_ISDBT_SB_SEGMENT_COUNT(DtvPropertyRequestInt<u32>),

    DTV_ISDBT_LAYERA_FEC(DtvPropertyRequestInt<fe_code_rate>),
    DTV_ISDBT_LAYERA_MODULATION(DtvPropertyRequestInt<fe_modulation>),
    DTV_ISDBT_LAYERA_SEGMENT_COUNT(DtvPropertyRequestInt<i32>),
    DTV_ISDBT_LAYERA_TIME_INTERLEAVING(DtvPropertyRequestInt<i32>),

    DTV_ISDBT_LAYERB_FEC(DtvPropertyRequestInt<fe_code_rate>),
    DTV_ISDBT_LAYERB_MODULATION(DtvPropertyRequestInt<fe_modulation>),
    DTV_ISDBT_LAYERB_SEGMENT_COUNT(DtvPropertyRequestInt<i32>),
    DTV_ISDBT_LAYERB_TIME_INTERLEAVING(DtvPropertyRequestInt<i32>),

    DTV_ISDBT_LAYERC_FEC(DtvPropertyRequestInt<fe_code_rate>),
    DTV_ISDBT_LAYERC_MODULATION(DtvPropertyRequestInt<fe_modulation>),
    DTV_ISDBT_LAYERC_SEGMENT_COUNT(DtvPropertyRequestInt<i32>),
    DTV_ISDBT_LAYERC_TIME_INTERLEAVING(DtvPropertyRequestInt<i32>),

    DTV_API_VERSION(DtvPropertyRequestInt<u32>),

    /* DVB-T/T2 */
    DTV_CODE_RATE_HP(DtvPropertyRequestInt<fe_code_rate>),
    DTV_CODE_RATE_LP(DtvPropertyRequestInt<fe_code_rate>),
    DTV_GUARD_INTERVAL(DtvPropertyRequestInt<fe_guard_interval>),
    DTV_TRANSMISSION_MODE(DtvPropertyRequestInt<fe_transmit_mode>),
    DTV_HIERARCHY(DtvPropertyRequestInt<fe_hierarchy>),

    DTV_ISDBT_LAYER_ENABLED(DtvPropertyRequestInt<u32>),

    DTV_STREAM_ID(DtvPropertyRequestInt<u32>),
    #[deprecated(note = "Obsolete, replaced with DTV_STREAM_ID.")]
    DTV_DVBT2_PLP_ID_LEGACY(DtvPropertyDeprecated),

    DTV_ENUM_DELSYS(DtvPropertyRequestDeliverySystems),

    /* ATSC-MH */
    DTV_ATSCMH_FIC_VER(DtvPropertyRequestInt<u32>),
    DTV_ATSCMH_PARADE_ID(DtvPropertyRequestInt<u32>),
    DTV_ATSCMH_NOG(DtvPropertyRequestInt<u32>),
    DTV_ATSCMH_TNOG(DtvPropertyRequestInt<u32>),
    DTV_ATSCMH_SGN(DtvPropertyRequestInt<u32>),
    DTV_ATSCMH_PRC(DtvPropertyRequestInt<u32>),
    DTV_ATSCMH_RS_FRAME_MODE(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_FRAME_ENSEMBLE(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_CODE_MODE_PRI(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_CODE_MODE_SEC(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_BLOCK_MODE(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_A(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_B(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_C(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_D(DtvPropertyNotImplemented),

    DTV_INTERLEAVING(DtvPropertyRequestInt<fe_interleaving>),
    DTV_LNA(DtvPropertyRequestInt<fe_lna>),

    /* Quality parameters */
    DTV_STAT_SIGNAL_STRENGTH(DtvPropertyRequestFrontendStats),
    DTV_STAT_CNR(DtvPropertyRequestFrontendStats),
    DTV_STAT_PRE_ERROR_BIT_COUNT(DtvPropertyRequestFrontendStats),
    DTV_STAT_PRE_TOTAL_BIT_COUNT(DtvPropertyRequestFrontendStats),
    DTV_STAT_POST_ERROR_BIT_COUNT(DtvPropertyRequestFrontendStats),
    DTV_STAT_POST_TOTAL_BIT_COUNT(DtvPropertyRequestFrontendStats),
    DTV_STAT_ERROR_BLOCK_COUNT(DtvPropertyRequestFrontendStats),
    DTV_STAT_TOTAL_BLOCK_COUNT(DtvPropertyRequestFrontendStats),

    /* Physical layer scrambling */
    DTV_SCRAMBLING_SEQUENCE_INDEX(DtvPropertyRequestInt<u32>),
}

#[macro_export]
macro_rules! dtv_property {
    ( $property:ident($data:expr) ) => {
        $property(DtvPropertyRequest::new($data))
    };
}

#[macro_export]
macro_rules! dtv_property_parse {
    ( $property:ident($data:expr)) => {
        $property(DtvPropertyRequest::new($data.parse().with_context(||format!("Invalid {}: {}", stringify!($property), $data))?))
    };
}

impl FromStr for DtvProperty {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, v) = s.split_once('=').context("Invalid line")?;
        let v = v.trim();
        Ok(match k.trim() {
            "FREQUENCY" => dtv_property_parse!(DTV_FREQUENCY(v)),
            "MODULATION" => dtv_property_parse!(DTV_MODULATION(v)),
            "BANDWIDTH_HZ" => dtv_property_parse!(DTV_BANDWIDTH_HZ(v)),
            "INVERSION" => dtv_property_parse!(DTV_INVERSION(v)),
            "SYMBOL_RATE" => dtv_property_parse!(DTV_SYMBOL_RATE(v)),
            "INNER_FEC" => dtv_property_parse!(DTV_INNER_FEC(v)),
            "VOLTAGE" => dtv_property_parse!(DTV_VOLTAGE(v)),
            "TONE" => dtv_property_parse!(DTV_TONE(v)),
            "PILOT" => dtv_property_parse!(DTV_PILOT(v)),
            "ROLLOFF" => dtv_property_parse!(DTV_ROLLOFF(v)),

            /* Basic enumeration set for querying unlimited capabilities */
            "DELIVERY_SYSTEM" => dtv_property_parse!(DTV_DELIVERY_SYSTEM(v)),

            /* ISDB-T and ISDB-Tsb */
            "ISDBT_PARTIAL_RECEPTION" => dtv_property_parse!(DTV_ISDBT_PARTIAL_RECEPTION(v)),
            "ISDBT_SOUND_BROADCASTING" => dtv_property_parse!(DTV_ISDBT_SOUND_BROADCASTING(v)),

            "ISDBT_SB_SUBCHANNEL_ID" => dtv_property_parse!(DTV_ISDBT_SB_SUBCHANNEL_ID(v)),
            "ISDBT_SB_SEGMENT_IDX" => dtv_property_parse!(DTV_ISDBT_SB_SEGMENT_IDX(v)),
            "ISDBT_SB_SEGMENT_COUNT" => dtv_property_parse!(DTV_ISDBT_SB_SEGMENT_COUNT(v)),

            "ISDBT_LAYERA_FEC" => dtv_property_parse!(DTV_ISDBT_LAYERA_FEC(v)),
            "ISDBT_LAYERA_MODULATION" => dtv_property_parse!(DTV_ISDBT_LAYERA_MODULATION(v)),
            "ISDBT_LAYERA_SEGMENT_COUNT" => dtv_property_parse!(DTV_ISDBT_LAYERA_SEGMENT_COUNT(v)),
            "ISDBT_LAYERA_TIME_INTERLEAVING" => {
                dtv_property_parse!(DTV_ISDBT_LAYERA_TIME_INTERLEAVING(v))
            }

            "ISDBT_LAYERB_FEC" => dtv_property_parse!(DTV_ISDBT_LAYERB_FEC(v)),
            "ISDBT_LAYERB_MODULATION" => dtv_property_parse!(DTV_ISDBT_LAYERB_MODULATION(v)),
            "ISDBT_LAYERB_SEGMENT_COUNT" => dtv_property_parse!(DTV_ISDBT_LAYERB_SEGMENT_COUNT(v)),
            "ISDBT_LAYERB_TIME_INTERLEAVING" => {
                dtv_property_parse!(DTV_ISDBT_LAYERB_TIME_INTERLEAVING(v))
            }

            "ISDBT_LAYERC_FEC" => dtv_property_parse!(DTV_ISDBT_LAYERC_FEC(v)),
            "ISDBT_LAYERC_MODULATION" => dtv_property_parse!(DTV_ISDBT_LAYERC_MODULATION(v)),
            "ISDBT_LAYERC_SEGMENT_COUNT" => dtv_property_parse!(DTV_ISDBT_LAYERC_SEGMENT_COUNT(v)),
            "ISDBT_LAYERC_TIME_INTERLEAVING" => {
                dtv_property_parse!(DTV_ISDBT_LAYERC_TIME_INTERLEAVING(v))
            }

            /* DVB-T/T2 */
            "CODE_RATE_HP" => dtv_property_parse!(DTV_CODE_RATE_HP(v)),
            "CODE_RATE_LP" => dtv_property_parse!(DTV_CODE_RATE_LP(v)),
            "GUARD_INTERVAL" => dtv_property_parse!(DTV_GUARD_INTERVAL(v)),
            "TRANSMISSION_MODE" => dtv_property_parse!(DTV_TRANSMISSION_MODE(v)),
            "HIERARCHY" => dtv_property_parse!(DTV_HIERARCHY(v)),

            "ISDBT_LAYER_ENABLED" => dtv_property_parse!(DTV_ISDBT_LAYER_ENABLED(v)),

            "STREAM_ID" => dtv_property_parse!(DTV_STREAM_ID(v)),

            /* ATSC-MH */
            "ATSCMH_FIC_VER" => dtv_property_parse!(DTV_ATSCMH_FIC_VER(v)),
            "ATSCMH_PARADE_ID" => dtv_property_parse!(DTV_ATSCMH_PARADE_ID(v)),
            "ATSCMH_NOG" => dtv_property_parse!(DTV_ATSCMH_NOG(v)),
            "ATSCMH_TNOG" => dtv_property_parse!(DTV_ATSCMH_TNOG(v)),
            "ATSCMH_SGN" => dtv_property_parse!(DTV_ATSCMH_SGN(v)),
            "ATSCMH_PRC" => dtv_property_parse!(DTV_ATSCMH_PRC(v)),

            "INTERLEAVING" => dtv_property_parse!(DTV_INTERLEAVING(v)),
            "LNA" => dtv_property_parse!(DTV_LNA(v)),
            &_ => bail!("Invalid key {}", k),
        })
    }
}

/// num of properties cannot exceed DTV_IOCTL_MAX_MSGS per ioctl
pub const DTV_IOCTL_MAX_MSGS: usize = 64;

#[repr(C)]
#[derive(Debug)]
pub struct FeParameters {
    /// (absolute) frequency in Hz for DVB-C/DVB-T/ATSC
    /// intermediate frequency in kHz for DVB-S
    pub frequency: u32,
    pub inversion: u32,
    /// unimplemented frontend parameters data
    __reserved_1: [u8; 28],
}

pub const FE_MAX_EVENT: usize = 8;

#[repr(C)]
#[derive(Debug)]
pub struct FeEvent {
    pub status: u32,
    pub parameters: FeParameters,
}

impl Default for FeEvent {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

impl FeEvent {
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut FeEvent {
        self as *mut _
    }
}
