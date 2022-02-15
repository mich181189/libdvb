use {
    std::{
        fmt,
        mem,
    },
};


pub use {
    fe_caps::*,
    fe_type::*,
    fe_sec_voltage::*,
    fe_sec_tone_mode::*,
    fe_sec_mini_cmd::*,
    fe_status::*,
    fe_spectral_inversion::*,
    fe_code_rate::*,
    fe_modulation::*,
    fe_transmit_mode::*,
    fe_guard_interval::*,
    fe_hierarchy::*,
    fe_interleaving::*,
    fe_pilot::*,
    fe_rolloff::*,
    fe_delivery_system::*,
    fecap_scale_params::*,
    dtv_property_cmd::*,
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
    fn default() -> Self { unsafe { mem::zeroed::<Self>() } }
}


impl FeInfo {
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut FeInfo { self as *mut _ }
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
    fn default() -> Self { unsafe { mem::zeroed::<Self>() } }
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
    fn default() -> Self { unsafe { mem::zeroed::<Self>() } }
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

/// scale types for the quality parameters
mod fecap_scale_params {
    /// That QoS measure is not available. That could indicate
    /// a temporary or a permanent condition.
    pub const FE_SCALE_NOT_AVAILABLE: u8       = 0;
    /// The scale is measured in 0.001 dB steps, typically used on signal measures.
    pub const FE_SCALE_DECIBEL: u8             = 1;
    /// The scale is a relative percentual measure,
    /// ranging from 0 (0%) to 0xffff (100%).
    pub const FE_SCALE_RELATIVE: u8            = 2;
    /// The scale counts the occurrence of an event, like
    /// bit error, block error, lapsed time.
    pub const FE_SCALE_COUNTER: u8             = 3;
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
                s.field(FIELD_VALUE, &{(self.value as f64) / 1000.0});
            }
            FE_SCALE_RELATIVE => {
                s.field(FIELD_SCALE, &"FE_SCALE_RELATIVE");
                s.field(FIELD_VALUE, &{self.value as u64});
            }
            FE_SCALE_COUNTER => {
                s.field(FIELD_SCALE, &"FE_SCALE_COUNTER");
                s.field(FIELD_VALUE, &{self.value as u64});
            }
            _ => {
                s.field(FIELD_SCALE, &{self.scale});
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
        f.debug_list().entries(self.stat[0 .. len].iter()).finish()
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


#[repr(C)]
pub enum DtvPropertyData {
    data(u32),
    st(DtvFrontendStats),
    buffer(DtvPropertyBuffer),
}


/// DVBv5 property Commands
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, FromRepr)]
pub enum dtv_property_cmd {
    DTV_UNDEFINED = 0,
    DTV_TUNE = 1,
    DTV_CLEAR = 2,
    DTV_FREQUENCY = 3,
    DTV_MODULATION = 4,
    DTV_BANDWIDTH_HZ = 5,
    DTV_INVERSION = 6,
    DTV_DISEQC_MASTER = 7,
    DTV_SYMBOL_RATE = 8,
    DTV_INNER_FEC = 9,
    DTV_VOLTAGE = 10,
    DTV_TONE = 11,
    DTV_PILOT = 12,
    DTV_ROLLOFF = 13,
    DTV_DISEQC_SLAVE_REPLY = 14,

    /* Basic enumeration set for querying unlimited capabilities */

    DTV_FE_CAPABILITY_COUNT = 15,
    DTV_FE_CAPABILITY = 16,
    DTV_DELIVERY_SYSTEM = 17,

    /* ISDB-T and ISDB-Tsb */

    DTV_ISDBT_PARTIAL_RECEPTION = 18,
    DTV_ISDBT_SOUND_BROADCASTING = 19,

    DTV_ISDBT_SB_SUBCHANNEL_ID = 20,
    DTV_ISDBT_SB_SEGMENT_IDX = 21,
    DTV_ISDBT_SB_SEGMENT_COUNT = 22,

    DTV_ISDBT_LAYERA_FEC = 23,
    DTV_ISDBT_LAYERA_MODULATION = 24,
    DTV_ISDBT_LAYERA_SEGMENT_COUNT = 25,
    DTV_ISDBT_LAYERA_TIME_INTERLEAVING = 26,

    DTV_ISDBT_LAYERB_FEC = 27,
    DTV_ISDBT_LAYERB_MODULATION = 28,
    DTV_ISDBT_LAYERB_SEGMENT_COUNT = 29,
    DTV_ISDBT_LAYERB_TIME_INTERLEAVING = 30,

    DTV_ISDBT_LAYERC_FEC = 31,
    DTV_ISDBT_LAYERC_MODULATION = 32,
    DTV_ISDBT_LAYERC_SEGMENT_COUNT = 33,
    DTV_ISDBT_LAYERC_TIME_INTERLEAVING = 34,

    DTV_API_VERSION = 35,

    /* DVB-T/T2 */

    DTV_CODE_RATE_HP = 36,
    DTV_CODE_RATE_LP = 37,
    DTV_GUARD_INTERVAL = 38,
    DTV_TRANSMISSION_MODE = 39,
    DTV_HIERARCHY = 40,

    DTV_ISDBT_LAYER_ENABLED = 41,

    DTV_STREAM_ID = 42,
    DTV_DVBT2_PLP_ID_LEGACY = 43,

    DTV_ENUM_DELSYS = 44,

    /* ATSC-MH */

    DTV_ATSCMH_FIC_VER = 45,
    DTV_ATSCMH_PARADE_ID = 46,
    DTV_ATSCMH_NOG = 47,
    DTV_ATSCMH_TNOG = 48,
    DTV_ATSCMH_SGN = 49,
    DTV_ATSCMH_PRC = 50,
    DTV_ATSCMH_RS_FRAME_MODE = 51,
    DTV_ATSCMH_RS_FRAME_ENSEMBLE = 52,
    DTV_ATSCMH_RS_CODE_MODE_PRI = 53,
    DTV_ATSCMH_RS_CODE_MODE_SEC = 54,
    DTV_ATSCMH_SCCC_BLOCK_MODE = 55,
    DTV_ATSCMH_SCCC_CODE_MODE_A = 56,
    DTV_ATSCMH_SCCC_CODE_MODE_B = 57,
    DTV_ATSCMH_SCCC_CODE_MODE_C = 58,
    DTV_ATSCMH_SCCC_CODE_MODE_D = 59,

    DTV_INTERLEAVING = 60,
    DTV_LNA = 61,

    /* Quality parameters */

    DTV_STAT_SIGNAL_STRENGTH = 62,
    DTV_STAT_CNR = 63,
    DTV_STAT_PRE_ERROR_BIT_COUNT = 64,
    DTV_STAT_PRE_TOTAL_BIT_COUNT = 65,
    DTV_STAT_POST_ERROR_BIT_COUNT = 66,
    DTV_STAT_POST_TOTAL_BIT_COUNT = 67,
    DTV_STAT_ERROR_BLOCK_COUNT = 68,
    DTV_STAT_TOTAL_BLOCK_COUNT = 69,

    /* Physical layer scrambling */

    DTV_SCRAMBLING_SEQUENCE_INDEX = 70,
    DTV_INPUT = 71,
}


/// Store one of frontend command and its value
#[repr(C, packed)]
pub struct DtvProperty {
    pub cmd: dtv_property_cmd,
    __reserved_1: [u32; 3],
    pub u: DtvPropertyData,
    pub result: i32,
}


impl fmt::Debug for DtvProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("DtvProperty");

        const FIELD_CMD: &str = "cmd";
        const FIELD_DATA: &str = "data";
        const FIELD_STATS: &str = "stats";

        match self.cmd {
            DTV_FREQUENCY => {
                s.field(FIELD_CMD, &"DTV_FREQUENCY");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_MODULATION => {
                s.field(FIELD_CMD, &"DTV_MODULATION");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_BANDWIDTH_HZ => {
                s.field(FIELD_CMD, &"DTV_BANDWIDTH_HZ");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_INVERSION => {
                s.field(FIELD_CMD, &"DTV_INVERSION");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_SYMBOL_RATE  => {
                s.field(FIELD_CMD, &"DTV_SYMBOL_RATE");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_INNER_FEC => {
                s.field(FIELD_CMD, &"DTV_INNER_FEC");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_PILOT => {
                s.field(FIELD_CMD, &"DTV_PILOT");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_ROLLOFF => {
                s.field(FIELD_CMD, &"DTV_ROLLOFF");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_DELIVERY_SYSTEM => {
                s.field(FIELD_CMD, &"DTV_DELIVERY_SYSTEM");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }
            DTV_API_VERSION => {
                s.field(FIELD_CMD, &"DTV_API_VERSION");
                s.field(FIELD_DATA, unsafe { &self.u.data });
            }

            /* Quality parameters */

            DTV_STAT_SIGNAL_STRENGTH => {
                s.field(FIELD_CMD, &"DTV_STAT_SIGNAL_STRENGTH");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }
            DTV_STAT_CNR => {
                s.field(FIELD_CMD, &"DTV_STAT_CNR");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }

            DTV_STAT_PRE_ERROR_BIT_COUNT => {
                s.field(FIELD_CMD, &"DTV_STAT_PRE_ERROR_BIT_COUNT");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }
            DTV_STAT_PRE_TOTAL_BIT_COUNT => {
                s.field(FIELD_CMD, &"DTV_STAT_PRE_TOTAL_BIT_COUNT");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }
            DTV_STAT_POST_ERROR_BIT_COUNT => {
                s.field(FIELD_CMD, &"DTV_STAT_POST_ERROR_BIT_COUNT");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }
            DTV_STAT_POST_TOTAL_BIT_COUNT => {
                s.field(FIELD_CMD, &"DTV_STAT_POST_TOTAL_BIT_COUNT");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }
            DTV_STAT_ERROR_BLOCK_COUNT => {
                s.field(FIELD_CMD, &"DTV_STAT_ERROR_BLOCK_COUNT");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }
            DTV_STAT_TOTAL_BLOCK_COUNT => {
                s.field(FIELD_CMD, &"DTV_STAT_TOTAL_BLOCK_COUNT");
                s.field(FIELD_STATS, unsafe { &self.u.st });
            }

            // TODO: more values
            _ => {}
        }

        s.field("result", &{ self.result });
        s.finish()
    }
}


impl DtvProperty {
    #[inline]
    pub fn new(cmd: dtv_property_cmd, data: u32) -> Self {
        Self {
            cmd,
            __reserved_1: [0, 0, 0],
            u: DtvPropertyData { data },
            result: 0,
        }
    }
}

/// num of properties cannot exceed DTV_IOCTL_MAX_MSGS per ioctl
pub const DTV_IOCTL_MAX_MSGS: usize                     = 64;


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
    fn default() -> Self { unsafe { mem::zeroed::<Self>() } }
}


impl FeEvent {
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut FeEvent { self as *mut _ }
}
