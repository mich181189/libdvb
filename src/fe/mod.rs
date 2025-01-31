#![allow(dead_code)]
mod status;
pub mod sys;

use {
    anyhow::{Context, Result},
    nix::{ioctl_read, ioctl_write_int_bad, ioctl_write_ptr, request_code_none},
    std::{
        ffi::CStr,
        fmt,
        fs::{File, OpenOptions},
        ops::Range,
        os::unix::{
            fs::{FileTypeExt, OpenOptionsExt},
            io::{AsRawFd, RawFd},
        },
    },
    sys::*,
};

pub use status::FeStatus;

/// A reference to the frontend device and device information
#[derive(Debug)]
pub struct FeDevice {
    adapter: u32,
    device: u32,

    file: File,

    api_version: u16,

    name: String,
    delivery_system_list: Vec<fe_delivery_system>,
    frequency_range: Range<u32>,
    symbolrate_range: Range<u32>,
    caps: fe_caps,
}

impl fmt::Display for FeDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "DVB API: {}.{}",
            self.api_version >> 8,
            self.api_version & 0xFF
        )?;
        writeln!(f, "Frontend: {}", self.name)?;

        write!(f, "Delivery system:")?;
        for v in &self.delivery_system_list {
            write!(f, " {}", v)?;
        }
        writeln!(f, "")?;

        writeln!(
            f,
            "Frequency range: {} .. {}",
            self.frequency_range.start / 1000,
            self.frequency_range.end / 1000
        )?;

        writeln!(
            f,
            "Symbolrate range: {} .. {}",
            self.symbolrate_range.start / 1000,
            self.symbolrate_range.end / 1000
        )?;

        write!(f, "Frontend capabilities: 0x{:08x}", self.caps)?;

        Ok(())
    }
}

impl AsRawFd for FeDevice {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

#[macro_export]
macro_rules! get_dtv_properties {
    ( $device:expr, $( $property:ident ),+ ) => { (|| -> ::anyhow::Result<_> {
        let mut input = [ $( $property($crate::fe::sys::DtvPropertyRequest::default()), )* ];
        ::anyhow::Context::context($device.get_properties(&mut input), "Error fetching properties")?;
        let mut iterator = input.iter();
        Ok((
            $(
                ::anyhow::Context::with_context(match iterator.next() {
                    Some($property(d)) => d.get(),
                    _ => ::anyhow::Result::Err(::anyhow::anyhow!("Missing value")),
                }, || format!("Error unpacking {}", stringify!($property)))?,
            )*
        ))
    })()}
}

#[macro_export]
macro_rules! set_dtv_properties {
    ( $device:expr, $( $property:ident($data:expr) ),+ ) => {
        $device.set_properties(&[
            $( $crate::dtv_property!($property($data)), )*
        ])
    };
}

impl FeDevice {
    /// Clears frontend settings and event queue
    pub fn clear(&self) -> Result<()> {
        set_dtv_properties!(
            self,
            DTV_VOLTAGE(SEC_VOLTAGE_OFF),
            DTV_TONE(SEC_TONE_OFF),
            DTV_CLEAR(())
        )
        .context("FE: clear")?;

        let mut event = FeEvent::default();

        for _ in 0..FE_MAX_EVENT {
            if self.get_event(&mut event).is_err() {
                break;
            }
        }

        Ok(())
    }

    fn get_info(&mut self) -> Result<()> {
        let mut feinfo = FeInfo::default();

        // FE_GET_INFO
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            61,
            FeInfo
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut feinfo as *mut _) }.context("FE: get info")?;

        if let Some(len) = feinfo.name.iter().position(|&b| b == 0) {
            let name = unsafe { CStr::from_ptr(feinfo.name[..len + 1].as_ptr()) };
            if let Ok(name) = name.to_str() {
                self.name = name.to_owned();
            }
        }

        self.frequency_range = feinfo.frequency_min..feinfo.frequency_max;
        self.symbolrate_range = feinfo.symbol_rate_min..feinfo.symbol_rate_max;

        self.caps = feinfo.caps;

        // DVB v5 properties
        let (api_version, enum_delsys) =
            get_dtv_properties!(self, DTV_API_VERSION, DTV_ENUM_DELSYS)
                .context("FE: get api version (deprecated driver)")?;

        // DVB API Version
        self.api_version = api_version as u16;

        // Suppoerted delivery systems
        self.delivery_system_list = enum_delsys;

        // dev-file metadata

        let metadata = self.file.metadata().context("FE: get device metadata")?;

        ensure!(
            metadata.file_type().is_char_device(),
            "FE: path is not to char device"
        );

        Ok(())
    }

    fn open(adapter: u32, device: u32, is_write: bool) -> Result<FeDevice> {
        let path = format!("/dev/dvb/adapter{}/frontend{}", adapter, device);
        let file = OpenOptions::new()
            .read(true)
            .write(is_write)
            .custom_flags(::nix::libc::O_NONBLOCK)
            .open(&path)
            .with_context(|| format!("FE: failed to open device {}", &path))?;

        let mut fe = FeDevice {
            adapter,
            device,

            file,

            api_version: 0,

            name: String::default(),
            delivery_system_list: Vec::default(),
            frequency_range: 0..0,
            symbolrate_range: 0..0,
            caps: fe_caps::FE_IS_STUPID,
        };

        fe.get_info()?;

        Ok(fe)
    }

    /// Attempts to open frontend device in read-only mode
    #[inline]
    pub fn open_ro(adapter: u32, device: u32) -> Result<FeDevice> {
        Self::open(adapter, device, false)
    }

    /// Attempts to open frontend device in read-write mode
    #[inline]
    pub fn open_rw(adapter: u32, device: u32) -> Result<FeDevice> {
        Self::open(adapter, device, true)
    }

    fn check_properties(&self, cmdseq: &[DtvProperty]) -> Result<()> {
        for p in cmdseq {
            match p {
                DTV_FREQUENCY(d) => {
                    ensure!(
                        self.frequency_range.contains(&d.get()?),
                        "FE: frequency out of range"
                    );
                }
                DTV_SYMBOL_RATE(d) => {
                    ensure!(
                        self.symbolrate_range.contains(&d.get()?),
                        "FE: symbolrate out of range"
                    );
                }
                DTV_INVERSION(d) => {
                    if d.get()? == INVERSION_AUTO {
                        ensure!(
                            self.caps.contains(fe_caps::FE_CAN_INVERSION_AUTO),
                            "FE: auto inversion is not available"
                        );
                    }
                }
                DTV_TRANSMISSION_MODE(d) => {
                    if d.get()? == TRANSMISSION_MODE_AUTO {
                        ensure!(
                            self.caps.contains(fe_caps::FE_CAN_TRANSMISSION_MODE_AUTO),
                            "FE: no auto transmission mode"
                        );
                    }
                }
                DTV_GUARD_INTERVAL(d) => {
                    if d.get()? == GUARD_INTERVAL_AUTO {
                        ensure!(
                            self.caps.contains(fe_caps::FE_CAN_GUARD_INTERVAL_AUTO),
                            "FE: no auto guard interval"
                        );
                    }
                }
                DTV_HIERARCHY(d) => {
                    if d.get()? == HIERARCHY_AUTO {
                        ensure!(
                            self.caps.contains(fe_caps::FE_CAN_HIERARCHY_AUTO),
                            "FE: no auto hierarchy"
                        );
                    }
                }
                DTV_STREAM_ID(..) => {
                    ensure!(
                        self.caps.contains(fe_caps::FE_CAN_MULTISTREAM),
                        "FE: no multistream"
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Sets properties on frontend device
    pub fn set_properties(&self, cmdseq: &[DtvProperty]) -> Result<()> {
        self.check_properties(cmdseq)?;

        #[repr(C)]
        pub struct DtvProperties {
            num: u32,
            props: *const DtvProperty,
        }

        let cmd = DtvProperties {
            num: cmdseq.len() as u32,
            props: cmdseq.as_ptr(),
        };

        // FE_SET_PROPERTY
        ioctl_write_ptr!(
            #[inline]
            ioctl_call,
            b'o',
            82,
            DtvProperties
        );
        unsafe { ioctl_call(self.as_raw_fd(), &cmd as *const _) }.context("FE: set properties")?;

        Ok(())
    }

    /// Gets properties from frontend device
    pub fn get_properties(&self, cmdseq: &mut [DtvProperty]) -> Result<()> {
        #[repr(C)]
        pub struct DtvProperties {
            num: u32,
            props: *mut DtvProperty,
        }

        let mut cmd = DtvProperties {
            num: cmdseq.len() as u32,
            props: cmdseq.as_mut_ptr(),
        };

        // FE_GET_PROPERTY
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            83,
            DtvProperties
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut cmd as *mut _) }
            .context("FE: get properties")?;

        Ok(())
    }

    /// Returns a frontend events if available
    pub fn get_event(&self, event: &mut FeEvent) -> Result<()> {
        // FE_GET_EVENT
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            78,
            FeEvent
        );
        unsafe { ioctl_call(self.as_raw_fd(), event as *mut _) }.context("FE: get event")?;

        Ok(())
    }

    /// Returns frontend status
    /// - [`FE_NONE`]
    /// - [`FE_HAS_SIGNAL`]
    /// - [`FE_HAS_CARRIER`]
    /// - [`FE_HAS_VITERBI`]
    /// - [`FE_HAS_SYNC`]
    /// - [`FE_HAS_LOCK`]
    /// - [`FE_TIMEDOUT`]
    /// - [`FE_REINIT`]
    pub fn read_status(&self) -> Result<fe_status> {
        let mut result: u32 = 0;

        // FE_READ_STATUS
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            69,
            u32
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut result as *mut _) }
            .context("FE: read status")?;

        Ok(fe_status::from_bits(result).context("Invalid status")?)
    }

    /// Reads and returns a signal strength relative value (DVBv3 API)
    pub fn read_signal_strength(&self) -> Result<u16> {
        let mut result: u16 = 0;

        // FE_READ_SIGNAL_STRENGTH
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            71,
            u16
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut result as *mut _) }
            .context("FE: read signal strength")?;

        Ok(result)
    }

    /// Reads and returns a signal-to-noise ratio, relative value (DVBv3 API)
    pub fn read_snr(&self) -> Result<u16> {
        let mut result: u16 = 0;

        // FE_READ_SNR
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            72,
            u16
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut result as *mut _) }.context("FE: read snr")?;

        Ok(result)
    }

    /// Reads and returns a bit error counter (DVBv3 API)
    pub fn read_ber(&self) -> Result<u64> {
        let mut result: u32 = 0;

        // FE_READ_BER
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            70,
            u32
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut result as *mut _) }.context("FE: read ber")?;

        Ok(result as u64)
    }

    /// Reads and returns an uncorrected blocks counter (DVBv3 API)
    pub fn read_unc(&self) -> Result<u64> {
        let mut result: u32 = 0;

        // FE_READ_UNCORRECTED_BLOCKS
        ioctl_read!(
            #[inline]
            ioctl_call,
            b'o',
            73,
            u32
        );
        unsafe { ioctl_call(self.as_raw_fd(), &mut result as *mut _) }
            .context("FE: read uncorrected blocks")?;

        Ok(result as u64)
    }

    /// Turns on/off generation of the continuous 22kHz tone
    ///
    /// allowed `value`'s:
    ///
    /// - SEC_TONE_ON - turn 22kHz on
    /// - SEC_TONE_OFF - turn 22kHz off
    pub fn set_tone(&self, value: u32) -> Result<()> {
        // FE_SET_TONE
        ioctl_write_int_bad!(
            #[inline]
            ioctl_call,
            request_code_none!(b'o', 66)
        );

        unsafe { ioctl_call(self.as_raw_fd(), value as _) }.context("FE: set tone")?;

        Ok(())
    }

    /// Sets the DC voltage level for LNB
    ///
    /// allowed `value`'s:
    ///
    /// - SEC_VOLTAGE_13 for 13V
    /// - SEC_VOLTAGE_18 for 18V
    /// - SEC_VOLTAGE_OFF turns LNB power supply off
    ///
    /// Different power levels used to select internal antennas for different polarizations:
    ///
    /// - 13V:
    ///     - Vertical in linear LNB
    ///     - Right in circular LNB
    /// - 18V:
    ///     - Horizontal in linear LNB
    ///     - Left in circular LNB
    /// - OFF is needed with external power supply, for example
    ///   to use same LNB with several receivers.
    pub fn set_voltage(&self, value: u32) -> Result<()> {
        // FE_SET_VOLTAGE
        ioctl_write_int_bad!(
            #[inline]
            ioctl_call,
            request_code_none!(b'o', 67)
        );

        unsafe { ioctl_call(self.as_raw_fd(), value as _) }.context("FE: set voltage")?;

        Ok(())
    }

    /// Sets DiSEqC master command
    ///
    /// `msg` is a message no more 6 bytes length
    ///
    /// Example DiSEqC commited command:
    ///
    /// ```text
    /// [0xE0, 0x10, 0x38, 0xF0 | value]
    /// ```
    ///
    /// - byte 1 is a framing (master command without response)
    /// - byte 2 is an address (any LNB)
    /// - byte 3 is a command (commited)
    /// - last 4 bits of byte 4 is:
    ///     - xx00 - switch input
    ///     - 00x0 - bit is set on SEC_VOLTAGE_18
    ///     - 000x - bit is set on SEC_TONE_ON
    ///
    pub fn diseqc_master_cmd(&self, msg: &[u8]) -> Result<()> {
        let mut cmd = DiseqcMasterCmd::default();
        debug_assert!(msg.len() <= cmd.msg.len());

        cmd.msg[0..msg.len()].copy_from_slice(msg);
        cmd.len = msg.len() as u8;

        // FE_DISEQC_SEND_MASTER_CMD
        ioctl_write_ptr!(ioctl_call, b'o', 63, DiseqcMasterCmd);
        unsafe { ioctl_call(self.as_raw_fd(), &cmd as *const _) }
            .context("FE: diseqc master cmd")?;

        Ok(())
    }

    /// Returns the current API version
    /// major - first byte
    /// minor - second byte
    #[inline]
    pub fn get_api_version(&self) -> u16 {
        self.api_version
    }

    /// Returns the name of the device
    #[inline]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    pub fn get_delivery_system_list(&self) -> &Vec<fe_delivery_system> {
        &self.delivery_system_list
    }

    #[inline]
    pub fn get_frequency_range(&self) -> Range<u32> {
        self.frequency_range.clone()
    }

    #[inline]
    pub fn get_symbolrate_range(&self) -> Range<u32> {
        self.symbolrate_range.clone()
    }

    #[inline]
    pub fn get_caps(&self) -> fe_caps {
        self.caps
    }

}
