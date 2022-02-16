use std::{fmt, mem};

pub use {
    fe_caps::*, fe_code_rate::*, fe_delivery_system::*, fe_guard_interval::*, fe_hierarchy::*,
    fe_interleaving::*, fe_modulation::*, fe_pilot::*, fe_rolloff::*, fe_sec_mini_cmd::*,
    fe_sec_tone_mode::*, fe_sec_voltage::*, fe_spectral_inversion::*, fe_status::*,
    fe_transmit_mode::*, fe_type::*, fecap_scale_params::*, DtvProperty::*,
};

use strum::{Display, FromRepr};

/// Frontend capabilities
#[repr(u32)]
#[allow(non_camel_case_types)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_caps {
    /// There's something wrong at the frontend, and it can't report its capabilities
    FE_IS_STUPID = 0,
    /// Can auto-detect frequency spectral band inversion
    FE_CAN_INVERSION_AUTO = 0x1,
    /// Supports FEC 1/2
    FE_CAN_FEC_1_2 = 0x2,
    /// Supports FEC 2/3
    FE_CAN_FEC_2_3 = 0x4,
    /// Supports FEC 3/4
    FE_CAN_FEC_3_4 = 0x8,
    /// Supports FEC 4/5
    FE_CAN_FEC_4_5 = 0x10,
    /// Supports FEC 5/6
    FE_CAN_FEC_5_6 = 0x20,
    /// Supports FEC 6/7
    FE_CAN_FEC_6_7 = 0x40,
    /// Supports FEC 7/8
    FE_CAN_FEC_7_8 = 0x80,
    /// Supports FEC 8/9
    FE_CAN_FEC_8_9 = 0x100,
    /// Can auto-detect FEC
    FE_CAN_FEC_AUTO = 0x200,
    /// Supports QPSK modulation
    FE_CAN_QPSK = 0x400,
    /// Supports 16-QAM modulation
    FE_CAN_QAM_16 = 0x800,
    /// Supports 32-QAM modulation
    FE_CAN_QAM_32 = 0x1000,
    /// Supports 64-QAM modulation
    FE_CAN_QAM_64 = 0x2000,
    /// Supports 128-QAM modulation
    FE_CAN_QAM_128 = 0x4000,
    /// Supports 256-QAM modulation
    FE_CAN_QAM_256 = 0x8000,
    /// Can auto-detect QAM modulation
    FE_CAN_QAM_AUTO = 0x10000,
    /// Can auto-detect transmission mode
    FE_CAN_TRANSMISSION_MODE_AUTO = 0x20000,
    /// Can auto-detect bandwidth
    FE_CAN_BANDWIDTH_AUTO = 0x40000,
    /// Can auto-detect guard interval
    FE_CAN_GUARD_INTERVAL_AUTO = 0x80000,
    /// Can auto-detect hierarchy
    FE_CAN_HIERARCHY_AUTO = 0x100000,
    /// Supports 8-VSB modulation
    FE_CAN_8VSB = 0x200000,
    /// Supports 16-VSB modulation
    FE_CAN_16VSB = 0x400000,
    /// Unused
    FE_HAS_EXTENDED_CAPS = 0x800000,
    /// Supports multistream filtering
    FE_CAN_MULTISTREAM = 0x4000000,
    /// Supports "turbo FEC" modulation
    FE_CAN_TURBO_FEC = 0x8000000,
    /// Supports "2nd generation" modulation, e. g. DVB-S2, DVB-T2, DVB-C2
    FE_CAN_2G_MODULATION = 0x10000000,
    /// Unused
    FE_NEEDS_BENDING = 0x20000000,
    /// Can recover from a cable unplug automatically
    FE_CAN_RECOVER = 0x40000000,
    /// Can stop spurious TS data output
    FE_CAN_MUTE_TS = 0x80000000,
}

/// DEPRECATED: Should be kept just due to backward compatibility
#[repr(u32)]
#[allow(non_camel_case_types)]
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
    pub fe_type: u32,
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
    pub caps: u32,
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_sec_tone_mode {
    /// Sends a 22kHz tone burst to the antenna
    SEC_TONE_ON = 0,
    /// Don't send a 22kHz tone to the antenna (except if the FE_DISEQC_* ioctl are called)
    SEC_TONE_OFF = 1,
}

/// Type of mini burst to be sent
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_sec_mini_cmd {
    /// Sends a mini-DiSEqC 22kHz '0' Tone Burst to select satellite-A
    SEC_MINI_A = 0,
    /// Sends a mini-DiSEqC 22kHz '1' Data Burst to select satellite-B
    SEC_MINI_B = 1,
}

/// Enumerates the possible frontend status
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_status {
    /// The frontend doesn't have any kind of lock. That's the initial frontend status
    FE_NONE = 0x00,
    /// Has found something above the noise level
    FE_HAS_SIGNAL = 0x01,
    /// Has found a signal
    FE_HAS_CARRIER = 0x02,
    /// FEC inner coding (Viterbi, LDPC or other inner code) is stable.
    FE_HAS_VITERBI = 0x04,
    /// Synchronization bytes was found
    FE_HAS_SYNC = 0x08,
    /// Digital TV were locked and everything is working
    FE_HAS_LOCK = 0x10,
    /// Fo lock within the last about 2 seconds
    FE_TIMEDOUT = 0x20,
    /// Frontend was reinitialized, application is recommended
    /// to reset DiSEqC, tone and parameters
    FE_REINIT = 0x40,
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_hierarchy {
    HIERARCHY_NONE = 0,
    HIERARCHY_1 = 1,
    HIERARCHY_2 = 2,
    HIERARCHY_4 = 3,
    HIERARCHY_AUTO = 4,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_interleaving {
    INTERLEAVING_NONE = 0,
    INTERLEAVING_AUTO = 1,
    INTERLEAVING_240 = 2,
    INTERLEAVING_720 = 3,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_pilot {
    PILOT_ON = 0,
    PILOT_OFF = 1,
    PILOT_AUTO = 2,
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_rolloff {
    ROLLOFF_35 = 0,
    ROLLOFF_20 = 1,
    ROLLOFF_25 = 2,
    ROLLOFF_AUTO = 3,
    ROLLOFF_15 = 4,
    ROLLOFF_10 = 5,
    ROLLOFF_5 = 6,
}

#[derive(Display, Debug)]
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
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum fe_lna {
    LNA_OFF = 0,
    LNA_ON = 1,
    LNA_AUTO = 0xFFFFFFFF,
}

/// scale types for the quality parameters
mod fecap_scale_params {
    /// That QoS measure is not available. That could indicate
    /// a temporary or a permanent condition.
    pub const FE_SCALE_NOT_AVAILABLE: u8 = 0;
    /// The scale is measured in 0.001 dB steps, typically used on signal measures.
    pub const FE_SCALE_DECIBEL: u8 = 1;
    /// The scale is a relative percentual measure,
    /// ranging from 0 (0%) to 0xffff (100%).
    pub const FE_SCALE_RELATIVE: u8 = 2;
    /// The scale counts the occurrence of an event, like
    /// bit error, block error, lapsed time.
    pub const FE_SCALE_COUNTER: u8 = 3;
}

/// Used for reading a DTV status property
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvStats {
    pub scale: u8, // fecap_scale_params
    pub value: i64,
}

impl fmt::Debug for DtvStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("DtvStats");

        const FIELD_SCALE: &str = "scale";
        const FIELD_VALUE: &str = "value";

        match self.scale {
            FE_SCALE_NOT_AVAILABLE => {
                s.field(FIELD_SCALE, &"FE_SCALE_NOT_AVAILABLE");
                s.field(FIELD_VALUE, &"not available");
            }
            FE_SCALE_DECIBEL => {
                s.field(FIELD_SCALE, &"FE_SCALE_DECIBEL");
                s.field(FIELD_VALUE, &{ (self.value as f64) / 1000.0 });
            }
            FE_SCALE_RELATIVE => {
                s.field(FIELD_SCALE, &"FE_SCALE_RELATIVE");
                s.field(FIELD_VALUE, &{ self.value as u64 });
            }
            FE_SCALE_COUNTER => {
                s.field(FIELD_SCALE, &"FE_SCALE_COUNTER");
                s.field(FIELD_VALUE, &{ self.value as u64 });
            }
            _ => {
                s.field(FIELD_SCALE, &{ self.scale });
                s.field(FIELD_VALUE, &"invalid scale format");
            }
        };
        s.finish()
    }
}

pub const MAX_DTV_STATS: usize = 4;

/// Store Digital TV frontend statistics
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct DtvFrontendStats {
    pub len: u8,
    pub stat: [DtvStats; MAX_DTV_STATS],
}

impl fmt::Debug for DtvFrontendStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = ::std::cmp::min(self.len as usize, self.stat.len());
        f.debug_list().entries(self.stat[0..len].iter()).finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DtvPropertyBuffer {
    pub data: [u8; 32],
    pub len: u32,
    __reserved_1: [u32; 3],
    __reserved_2: *mut std::ffi::c_void,
}

impl fmt::Debug for DtvPropertyBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = ::std::cmp::min(self.len as usize, self.data.len());
        f.debug_list().entries(self.data[0..len].iter()).finish()
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct DtvPropertyData<T> {
    __reserved: [u32; 3],
    pub data: T,
    pub result: i32,
}

impl<T> DtvPropertyData<T> {
    pub fn new(data: T) -> Self {
        Self {
            __reserved: [0, 0, 0],
            data,
            result: 0
        }
    }
}

pub type DtvPropertyRequestStats = DtvPropertyData<DtvFrontendStats>;
pub type DtvPropertyRequestData = DtvPropertyData<DtvPropertyBuffer>;
pub type DtvPropertyRequest = DtvPropertyData<u32>;

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
pub enum DtvProperty {
    DTV_UNDEFINED(DtvPropertyNotImplementedLinux),
    DTV_TUNE(DtvPropertyRequest),
    DTV_CLEAR(DtvPropertyRequest),
    DTV_FREQUENCY(DtvPropertyData<u32>),
    DTV_MODULATION(DtvPropertyData<fe_modulation>),
    DTV_BANDWIDTH_HZ(DtvPropertyData<u32>),
    DTV_INVERSION(DtvPropertyData<fe_spectral_inversion>),
    DTV_DISEQC_MASTER(DtvPropertyNotImplementedLinux),
    DTV_SYMBOL_RATE(DtvPropertyData<u32>),
    DTV_INNER_FEC(DtvPropertyData<fe_code_rate>),
    DTV_VOLTAGE(DtvPropertyData<fe_sec_voltage>),
    DTV_TONE(DtvPropertyNotImplementedLinux),
    DTV_PILOT(DtvPropertyData<fe_pilot>),
    DTV_ROLLOFF(DtvPropertyData<fe_rolloff>),
    DTV_DISEQC_SLAVE_REPLY(DtvPropertyNotImplementedLinux),

    /* Basic enumeration set for querying unlimited capabilities */
    DTV_FE_CAPABILITY_COUNT(DtvPropertyNotImplementedLinux),
    DTV_FE_CAPABILITY(DtvPropertyNotImplementedLinux),
    DTV_DELIVERY_SYSTEM(DtvPropertyData<fe_delivery_system>),

    /* ISDB-T and ISDB-Tsb */
    // Please fork
    DTV_ISDBT_PARTIAL_RECEPTION(DtvPropertyData<i32>),
    DTV_ISDBT_SOUND_BROADCASTING(DtvPropertyData<i32>),

    DTV_ISDBT_SB_SUBCHANNEL_ID(DtvPropertyData<i32>),
    DTV_ISDBT_SB_SEGMENT_IDX(DtvPropertyData<i32>),
    DTV_ISDBT_SB_SEGMENT_COUNT(DtvPropertyData<u32>),

    DTV_ISDBT_LAYERA_FEC(DtvPropertyData<fe_code_rate>),
    DTV_ISDBT_LAYERA_MODULATION(DtvPropertyData<fe_modulation>),
    DTV_ISDBT_LAYERA_SEGMENT_COUNT(DtvPropertyData<i32>),
    DTV_ISDBT_LAYERA_TIME_INTERLEAVING(DtvPropertyData<i32>),

    DTV_ISDBT_LAYERB_FEC(DtvPropertyData<fe_code_rate>),
    DTV_ISDBT_LAYERB_MODULATION(DtvPropertyData<fe_modulation>),
    DTV_ISDBT_LAYERB_SEGMENT_COUNT(DtvPropertyData<i32>),
    DTV_ISDBT_LAYERB_TIME_INTERLEAVING(DtvPropertyData<i32>),

    DTV_ISDBT_LAYERC_FEC(DtvPropertyData<fe_code_rate>),
    DTV_ISDBT_LAYERC_MODULATION(DtvPropertyData<fe_modulation>),
    DTV_ISDBT_LAYERC_SEGMENT_COUNT(DtvPropertyData<i32>),
    DTV_ISDBT_LAYERC_TIME_INTERLEAVING(DtvPropertyData<i32>),

    DTV_API_VERSION(DtvPropertyRequest),

    /* DVB-T/T2 */
    DTV_CODE_RATE_HP(DtvPropertyData<fe_transmit_mode>),
    DTV_CODE_RATE_LP(DtvPropertyData<fe_transmit_mode>),
    DTV_GUARD_INTERVAL(DtvPropertyData<fe_guard_interval>),
    DTV_TRANSMISSION_MODE(DtvPropertyData<fe_transmit_mode>),
    DTV_HIERARCHY(DtvPropertyData<fe_hierarchy>),

    DTV_ISDBT_LAYER_ENABLED(DtvPropertyData<u32>),

    DTV_STREAM_ID(DtvPropertyData<u32>),
    #[deprecated(note = "Obsolete, replaced with DTV_STREAM_ID.")]
    DTV_DVBT2_PLP_ID_LEGACY(DtvPropertyDeprecated),

    DTV_ENUM_DELSYS(DtvPropertyRequestData),

    /* ATSC-MH */
    DTV_ATSCMH_FIC_VER(DtvPropertyData<u32>),
    DTV_ATSCMH_PARADE_ID(DtvPropertyData<u32>),
    DTV_ATSCMH_NOG(DtvPropertyData<u32>),
    DTV_ATSCMH_TNOG(DtvPropertyData<u32>),
    DTV_ATSCMH_SGN(DtvPropertyData<u32>),
    DTV_ATSCMH_PRC(DtvPropertyData<u32>),
    DTV_ATSCMH_RS_FRAME_MODE(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_FRAME_ENSEMBLE(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_CODE_MODE_PRI(DtvPropertyNotImplemented),
    DTV_ATSCMH_RS_CODE_MODE_SEC(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_BLOCK_MODE(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_A(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_B(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_C(DtvPropertyNotImplemented),
    DTV_ATSCMH_SCCC_CODE_MODE_D(DtvPropertyNotImplemented),

    DTV_INTERLEAVING(DtvPropertyData<fe_interleaving>),
    DTV_LNA(DtvPropertyData<fe_lna>),

    /* Quality parameters */
    DTV_STAT_SIGNAL_STRENGTH(DtvPropertyRequestStats),
    DTV_STAT_CNR(DtvPropertyRequestStats),
    DTV_STAT_PRE_ERROR_BIT_COUNT(DtvPropertyRequestStats),
    DTV_STAT_PRE_TOTAL_BIT_COUNT(DtvPropertyRequestStats),
    DTV_STAT_POST_ERROR_BIT_COUNT(DtvPropertyRequestStats),
    DTV_STAT_POST_TOTAL_BIT_COUNT(DtvPropertyRequestStats),
    DTV_STAT_ERROR_BLOCK_COUNT(DtvPropertyRequestStats),
    DTV_STAT_TOTAL_BLOCK_COUNT(DtvPropertyRequestStats),

    /* Physical layer scrambling */
    DTV_SCRAMBLING_SEQUENCE_INDEX(DtvPropertyRequest),
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
