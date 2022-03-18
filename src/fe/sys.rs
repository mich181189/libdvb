use anyhow::{Context};
use std::{fmt, mem, iter::{FromIterator}};

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
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_spectral_inversion {
    INVERSION_OFF = 0,
    INVERSION_ON = 1,
    INVERSION_AUTO = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_code_rate {
    FEC_NONE = 0,
    FEC_1_2 = 1,
    FEC_2_3 = 2,
    FEC_3_4 = 3,
    FEC_4_5 = 4,
    FEC_5_6 = 5,
    FEC_6_7 = 6,
    FEC_7_8 = 7,
    FEC_8_9 = 8,
    FEC_AUTO = 9,
    FEC_3_5 = 10,
    FEC_9_10 = 11,
    FEC_2_5 = 12,
    FEC_1_4 = 13,
    FEC_1_3 = 14,
}

/// Type of modulation/constellation
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_modulation {
    QPSK = 0,
    QAM_16 = 1,
    QAM_32 = 2,
    QAM_64 = 3,
    QAM_128 = 4,
    QAM_256 = 5,
    QAM_AUTO = 6,
    VSB_8 = 7,
    VSB_16 = 8,
    PSK_8 = 9,
    APSK_16 = 10,
    APSK_32 = 11,
    DQPSK = 12,
    QAM_4_NR = 13,
    APSK_64 = 14,
    APSK_128 = 15,
    APSK_256 = 16,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_transmit_mode {
    TRANSMISSION_MODE_2K = 0,
    TRANSMISSION_MODE_8K = 1,
    TRANSMISSION_MODE_AUTO = 2,
    TRANSMISSION_MODE_4K = 3,
    TRANSMISSION_MODE_1K = 4,
    TRANSMISSION_MODE_16K = 5,
    TRANSMISSION_MODE_32K = 6,
    TRANSMISSION_MODE_C1 = 7,
    TRANSMISSION_MODE_C3780 = 8,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_guard_interval {
    GUARD_INTERVAL_1_32 = 0,
    GUARD_INTERVAL_1_16 = 1,
    GUARD_INTERVAL_1_8 = 2,
    GUARD_INTERVAL_1_4 = 3,
    GUARD_INTERVAL_AUTO = 4,
    GUARD_INTERVAL_1_128 = 5,
    GUARD_INTERVAL_19_128 = 6,
    GUARD_INTERVAL_19_256 = 7,
    GUARD_INTERVAL_PN420 = 8,
    GUARD_INTERVAL_PN595 = 9,
    GUARD_INTERVAL_PN945 = 10,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_hierarchy {
    HIERARCHY_NONE = 0,
    HIERARCHY_1 = 1,
    HIERARCHY_2 = 2,
    HIERARCHY_4 = 3,
    HIERARCHY_AUTO = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_interleaving {
    INTERLEAVING_NONE = 0,
    INTERLEAVING_AUTO = 1,
    INTERLEAVING_240 = 2,
    INTERLEAVING_720 = 3,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
pub enum fe_pilot {
    PILOT_ON = 0,
    PILOT_OFF = 1,
    PILOT_AUTO = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
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
pub enum fe_delivery_system {
    #[strum(to_string = "none")]
    SYS_UNDEFINED = 0,
    #[strum(to_string = "dvb-c")]
    SYS_DVBC_ANNEX_A = 1,
    #[strum(to_string = "dvb-c/b")]
    SYS_DVBC_ANNEX_B = 2,
    #[strum(to_string = "dvb-t")]
    SYS_DVBT = 3,
    #[strum(to_string = "dss")]
    SYS_DSS = 4,
    #[strum(to_string = "dvb-s")]
    SYS_DVBS = 5,
    #[strum(to_string = "dvb-s2")]
    SYS_DVBS2 = 6,
    #[strum(to_string = "dvb-h")]
    SYS_DVBH = 7,
    #[strum(to_string = "isdb-t")]
    SYS_ISDBT = 8,
    #[strum(to_string = "isdb-s")]
    SYS_ISDBS = 9,
    #[strum(to_string = "isdb-c")]
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
    #[strum(to_string = "dvb-t2")]
    SYS_DVBT2 = 16,
    #[strum(to_string = "dvb-s/turbo")]
    SYS_TURBO = 17,
    #[strum(to_string = "dvb-c/c")]
    SYS_DVBC_ANNEX_C = 18,
    #[strum(to_string = "dvb-c2")]
    SYS_DVBC2 = 19,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr, Copy, Clone)]
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
    __reserved_1: [u32; 3],
    __reserved_2: *mut std::ffi::c_void,
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

#[repr(C, packed)]
#[derive(Debug)]
pub struct DtvPropertyRequest<T> {
    __reserved: [u32; 3],
    data: T,
    result: i32, // Unused
}

impl<T> DtvPropertyRequest<T> {
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            __reserved: [0, 0, 0],
            data,
            result: 0,
        }
    }
}

impl<T> Default for DtvPropertyRequest<T> {
    #[inline]
    fn default() -> Self {
        unsafe { mem::zeroed::<Self>() }
    }
}

impl<T: Copy> WrappedResult<T> for DtvPropertyRequest<T> {
    #[inline]
    fn get(&self) -> anyhow::Result<T> {
        Ok(self.data)
    }
}

pub type DtvPropertyRequestVoid = DtvPropertyRequest<u32>;

pub type DtvPropertyRequestDeliverySystems = DtvPropertyRequest<DtvPropertyBuffer>;

impl<T: FromIterator<fe_delivery_system>> WrappedResult<T> for DtvPropertyRequestDeliverySystems {
    #[inline]
    fn get(&self) -> Result<T, anyhow::Error> {
        self.data
            .slice()
            .into_iter()
            .map(|&x| fe_delivery_system::from_repr(x as u32).context("Invalid delivery system"))
            .try_collect()
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct DtvPropertyNotImplementedLinux {
    __reserved: [u32; 6],
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
#[derive(Debug)]
#[allow(deprecated)]
pub enum DtvProperty {
    DTV_UNDEFINED(DtvPropertyNotImplementedLinux),
    DTV_TUNE(DtvPropertyRequestVoid),
    DTV_CLEAR(DtvPropertyRequestVoid),
    DTV_FREQUENCY(DtvPropertyRequest<u32>),
    DTV_MODULATION(DtvPropertyRequest<fe_modulation>),
    DTV_BANDWIDTH_HZ(DtvPropertyRequest<u32>),
    DTV_INVERSION(DtvPropertyRequest<fe_spectral_inversion>),
    DTV_DISEQC_MASTER(DtvPropertyNotImplementedLinux),
    DTV_SYMBOL_RATE(DtvPropertyRequest<u32>),
    DTV_INNER_FEC(DtvPropertyRequest<fe_code_rate>),
    DTV_VOLTAGE(DtvPropertyRequest<fe_sec_voltage>),
    DTV_TONE(DtvPropertyRequest<fe_sec_tone_mode>),
    DTV_PILOT(DtvPropertyRequest<fe_pilot>),
    DTV_ROLLOFF(DtvPropertyRequest<fe_rolloff>),
    DTV_DISEQC_SLAVE_REPLY(DtvPropertyNotImplementedLinux),

    /* Basic enumeration set for querying unlimited capabilities */
    DTV_FE_CAPABILITY_COUNT(DtvPropertyNotImplementedLinux),
    DTV_FE_CAPABILITY(DtvPropertyNotImplementedLinux),
    DTV_DELIVERY_SYSTEM(DtvPropertyRequest<fe_delivery_system>),

    /* ISDB-T and ISDB-Tsb */
    // Please fork
    DTV_ISDBT_PARTIAL_RECEPTION(DtvPropertyRequest<i32>),
    DTV_ISDBT_SOUND_BROADCASTING(DtvPropertyRequest<i32>),

    DTV_ISDBT_SB_SUBCHANNEL_ID(DtvPropertyRequest<i32>),
    DTV_ISDBT_SB_SEGMENT_IDX(DtvPropertyRequest<i32>),
    DTV_ISDBT_SB_SEGMENT_COUNT(DtvPropertyRequest<u32>),

    DTV_ISDBT_LAYERA_FEC(DtvPropertyRequest<fe_code_rate>),
    DTV_ISDBT_LAYERA_MODULATION(DtvPropertyRequest<fe_modulation>),
    DTV_ISDBT_LAYERA_SEGMENT_COUNT(DtvPropertyRequest<i32>),
    DTV_ISDBT_LAYERA_TIME_INTERLEAVING(DtvPropertyRequest<i32>),

    DTV_ISDBT_LAYERB_FEC(DtvPropertyRequest<fe_code_rate>),
    DTV_ISDBT_LAYERB_MODULATION(DtvPropertyRequest<fe_modulation>),
    DTV_ISDBT_LAYERB_SEGMENT_COUNT(DtvPropertyRequest<i32>),
    DTV_ISDBT_LAYERB_TIME_INTERLEAVING(DtvPropertyRequest<i32>),

    DTV_ISDBT_LAYERC_FEC(DtvPropertyRequest<fe_code_rate>),
    DTV_ISDBT_LAYERC_MODULATION(DtvPropertyRequest<fe_modulation>),
    DTV_ISDBT_LAYERC_SEGMENT_COUNT(DtvPropertyRequest<i32>),
    DTV_ISDBT_LAYERC_TIME_INTERLEAVING(DtvPropertyRequest<i32>),

    DTV_API_VERSION(DtvPropertyRequest<u32>),

    /* DVB-T/T2 */
    DTV_CODE_RATE_HP(DtvPropertyRequest<fe_transmit_mode>),
    DTV_CODE_RATE_LP(DtvPropertyRequest<fe_transmit_mode>),
    DTV_GUARD_INTERVAL(DtvPropertyRequest<fe_guard_interval>),
    DTV_TRANSMISSION_MODE(DtvPropertyRequest<fe_transmit_mode>),
    DTV_HIERARCHY(DtvPropertyRequest<fe_hierarchy>),

    DTV_ISDBT_LAYER_ENABLED(DtvPropertyRequest<u32>),

    DTV_STREAM_ID(DtvPropertyRequest<u32>),
    #[deprecated(note = "Obsolete, replaced with DTV_STREAM_ID.")]
    DTV_DVBT2_PLP_ID_LEGACY(DtvPropertyDeprecated),

    DTV_ENUM_DELSYS(DtvPropertyRequestDeliverySystems),

    /* ATSC-MH */
    DTV_ATSCMH_FIC_VER(DtvPropertyRequest<u32>),
    DTV_ATSCMH_PARADE_ID(DtvPropertyRequest<u32>),
    DTV_ATSCMH_NOG(DtvPropertyRequest<u32>),
    DTV_ATSCMH_TNOG(DtvPropertyRequest<u32>),
    DTV_ATSCMH_SGN(DtvPropertyRequest<u32>),
    DTV_ATSCMH_PRC(DtvPropertyRequest<u32>),
    DTV_ATSCMH_RS_FRAME_MODE(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_FRAME_ENSEMBLE(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_CODE_MODE_PRI(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_CODE_MODE_SEC(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_BLOCK_MODE(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_A(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_B(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_C(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_D(DtvPropertyNotImplemented),

    DTV_INTERLEAVING(DtvPropertyRequest<fe_interleaving>),
    DTV_LNA(DtvPropertyRequest<fe_lna>),

    /* Quality parameters */
    DTV_STAT_SIGNAL_STRENGTH(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_CNR(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_PRE_ERROR_BIT_COUNT(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_PRE_TOTAL_BIT_COUNT(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_POST_ERROR_BIT_COUNT(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_POST_TOTAL_BIT_COUNT(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_ERROR_BLOCK_COUNT(DtvPropertyRequest<DtvFrontendStats>),
    DTV_STAT_TOTAL_BLOCK_COUNT(DtvPropertyRequest<DtvFrontendStats>),

    /* Physical layer scrambling */
    DTV_SCRAMBLING_SEQUENCE_INDEX(DtvPropertyRequest<u32>),
}

#[macro_export]
macro_rules! dtv_property {
    ( $property:ident, $data:expr ) => {
        $property(DtvPropertyRequest::new($data))
    };
    ( $property:ident($data:expr) ) => {
        $property(DtvPropertyRequest::new($data))
    };
    ( $property:expr ) => {
        $property
    };
}

#[macro_export]
macro_rules! req_dtv_properties {
    ( $device:expr, $( $property:ident ),+ ) => { (|| -> anyhow::Result<_> {
        let mut input = [ $( $property(DtvPropertyRequest::default()), )* ];
        $device.get_properties(&mut input)?;
        let mut iterator = input.iter();
        Ok((
            $(
                match iterator.next() {
                    Some($property(d)) => d.get(),
                    _ => ::anyhow::Result::Err(anyhow!("Error unpacking")),
                }?,
            )*
        ))
    })()}
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
