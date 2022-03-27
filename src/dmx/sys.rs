use bitflags::bitflags;
use strum::FromRepr;

pub use {
    DmxOutput::*,
    DmxInput::*,
    DmxTsPes::*,
};


/// Output for the demux
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
pub enum DmxOutput {
    /// Streaming directly to decoder
    DMX_OUT_DECODER = 0,
    /// Output going to a memory buffer (to be retrieved via the read command).
    /// Delivers the stream output to the demux device on which the ioctl
    /// is called.
    DMX_OUT_TAP = 1,
    /// Output multiplexed into a new TS (to be retrieved by reading from the
    /// logical DVR device). Routes output to the logical DVR device
    /// `/dev/dvb/adapter?/dvr?`, which delivers a TS multiplexed from all
    /// filters for which DMX_OUT_TS_TAP was specified.
    DMX_OUT_TS_TAP = 2,
    /// Like DMX_OUT_TS_TAP but retrieved from the DMX device.
    DMX_OUT_TSDEMUX_TAP = 3
}


/// Input from the demux
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
pub enum DmxInput {
    /// Input from a front-end device
    DMX_IN_FRONTEND = 0,
    /// Input from the logical DVR device
    DMX_IN_DVR = 1
}


/// type of the PES filter
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromRepr)]
pub enum DmxTsPes {
    /// first audio PID
    DMX_PES_AUDIO0               = 0,
    /// first video PID
    DMX_PES_VIDEO0               = 1,
    /// first teletext PID
    DMX_PES_TELETEXT0            = 2,
    /// first subtitle PID
    DMX_PES_SUBTITLE0            = 3,
    /// first Program Clock Reference PID
    DMX_PES_PCR0                 = 4,

    /// second audio PID.
    DMX_PES_AUDIO1               = 5,
    /// second video PID.
    DMX_PES_VIDEO1               = 6,
    /// second teletext PID.
    DMX_PES_TELETEXT1            = 7,
    /// second subtitle PID.
    DMX_PES_SUBTITLE1            = 8,
    /// second Program Clock Reference PID.
    DMX_PES_PCR1                 = 9,

    /// third audio PID.
    DMX_PES_AUDIO2               = 10,
    /// third video PID.
    DMX_PES_VIDEO2               = 11,
    /// third teletext PID.
    DMX_PES_TELETEXT2            = 12,
    /// third subtitle PID.
    DMX_PES_SUBTITLE2            = 13,
    /// third Program Clock Reference PID.
    DMX_PES_PCR2                 = 14,

    /// fourth audio PID.
    DMX_PES_AUDIO3               = 15,
    /// fourth video PID.
    DMX_PES_VIDEO3               = 16,
    /// fourth teletext PID.
    DMX_PES_TELETEXT3            = 17,
    /// fourth subtitle PID.
    DMX_PES_SUBTITLE3            = 18,
    /// fourth Program Clock Reference PID.
    DMX_PES_PCR3                 = 19,

    /// any other PID.
    DMX_PES_OTHER                = 20,
}


bitflags! {
    /// Flags for the demux filter
    #[repr(C)]
    pub struct DmxFilterFlags : u32 {
        /// Only deliver sections where the CRC check succeeded
        const DMX_CHECK_CRC                = 1;
        /// Disable the section filter after one section has been delivered
        const DMX_ONESHOT                  = 2;
        /// Start filter immediately without requiring a `DMX_START`
        const DMX_IMMEDIATE_START          = 4;
    }
}


/// Specifies Packetized Elementary Stream (PES) filter parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxPesFilterParams {
    /// PID to be filtered. 8192 to pass all PID's
    pub pid: u16,
    /// Demux input, as specified by `DMX_IN_*`
    pub input: DmxInput,
    /// Demux output, as specified by `DMX_OUT_*`
    pub output: DmxOutput,
    /// Type of the pes filter, as specified by `DMX_PES_*`
    pub pes_type: DmxTsPes,
    /// Demux PES flags
    pub flags: DmxFilterFlags,
}

pub const DMX_FILTER_SIZE: usize = 16;

/// Specifies demux section header filter parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxFilter {
    /// Bit array with bits to be matched at the section header
    pub filter: [u8; DMX_FILTER_SIZE],
    /// Bits that are valid at the filter bit array
    pub mask: [u8; DMX_FILTER_SIZE],
    /// Mode of match: if bit is zero, it will match if equal (positive match); if bit is one, it will match if the bit is negated.
    pub mode: [u8; DMX_FILTER_SIZE],
}

/// Specifies Section header (SCT) filter parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxSctFilterParams {
    /// PID to be filtered. 8192 to pass all PID's
    pub pid: u16,
    /// Section header filter, as defined by DmxFilter
    pub filter: DmxFilter,
    /// Maximum time to filter, in milliseconds
    pub timeout: u32,
    /// Extra flags for the section filter, as specified by DmxFilterFlags
    pub flags: DmxFilterFlags
}