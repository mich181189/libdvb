mod status;
pub mod sys;


use {
    std::{
        ffi::CStr,
        fmt,
        fs::{
            File,
            OpenOptions,
        },
        ops::Range,
        os::unix::{
            fs::{
                OpenOptionsExt,
                FileTypeExt,
            },
            io::{
                AsRawFd,
                RawFd,
            },
        },
        path::Path,
    },

    anyhow::{
        Context,
        Result,
    },
    thiserror::Error,

    nix::{
        ioctl_read,
        ioctl_write_ptr,
        ioctl_write_int_bad,
        request_code_none,
    },

    sys::*,
};


pub use {
    status::{
        FeStatus,
        FeStatusDisplay,
    },
};


/// The error type for frontend operations
#[derive(Debug, Error)]
pub enum FeError {
    #[error("frontend is not char device")]
    InvalidDeviceFormat,
    #[error("frequency out of range")]
    InvalidFrequency,
    #[error("symbolrate out of range")]
    InvalidSymbolrate,
    #[error("unknown subsystem")]
    InvalidSubsystem,
    #[error("no auto inversion")]
    NoAutoInversion,
    #[error("no auto transmission mode")]
    NoAutoTransmitMode,
    #[error("no auto guard interval")]
    NoAutoGuardInterval,
    #[error("no auto hierarchy")]
    NoAutoHierarchy,
    #[error("multistream not supported")]
    NoMultistream,
}


/// A reference to the frontend device and device information
#[derive(Debug)]
pub struct FeDevice {
    file: File,

    api_version: u16,

    name: String,
    delivery_system_list: Vec<u32>,
    frequency_range: Range<u32>,
    symbolrate_range: Range<u32>,
    caps: u32,
}


impl fmt::Display for FeDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "DVB API: {}.{}", self.api_version >> 8, self.api_version & 0xFF)?;
        writeln!(f, "Frontend: {}", self.name)?;

        write!(f, "Delivery system:")?;
        for v in &self.delivery_system_list {
            write!(f, " {}", &DeliverySystemDisplay(*v))?;
        }
        writeln!(f, "")?;

        writeln!(f, "Frequency range: {} .. {}",
            self.frequency_range.start / 1000,
            self.frequency_range.end / 1000)?;

        writeln!(f, "Symbolrate range: {} .. {}",
            self.symbolrate_range.start / 1000,
            self.symbolrate_range.end / 1000)?;

        write!(f, "Frontend capabilities: 0x{:08x}", self.caps)?;

        Ok(())
    }
}


impl AsRawFd for FeDevice {
    #[inline]
    fn as_raw_fd(&self) -> RawFd { self.file.as_raw_fd() }
}


impl FeDevice {
    /// Clears frontend settings and event queue
    pub fn clear(&self) -> Result<()> {
        let cmdseq = [
            DtvProperty::new(DTV_VOLTAGE, SEC_VOLTAGE_OFF),
            DtvProperty::new(DTV_TONE, SEC_TONE_OFF),
            DtvProperty::new(DTV_CLEAR, 0),
        ];
        self.set_properties(&cmdseq).context("frontend clear")?;

        let mut event = FeEvent::default();

        for _ in 0 .. FE_MAX_EVENT {
            if self.get_event(&mut event).is_err() {
                break;
            }
        }

        Ok(())
    }

    fn get_info(&mut self) -> Result<()> {
        let mut feinfo = FeInfo::default();

        // FE_GET_INFO
        ioctl_read!(#[inline] ioctl_call, b'o', 61, FeInfo);
        unsafe {
            ioctl_call(self.as_raw_fd(), &mut feinfo as *mut _)
        }.context("frontend get info")?;

        if let Some(len) = feinfo.name.iter().position(|&b| b == 0) {
            let name = unsafe { CStr::from_ptr(feinfo.name[.. len + 1].as_ptr()) };
            if let Ok(name) = name.to_str() {
                self.name = name.to_owned();
            }
        }

        self.frequency_range = feinfo.frequency_min .. feinfo.frequency_max;
        self.symbolrate_range = feinfo.symbol_rate_min .. feinfo.symbol_rate_max;

        self.caps = feinfo.caps;

        // DVB v5 properties

        let mut cmdseq = [
            DtvProperty::new(DTV_API_VERSION, 0),
            DtvProperty::new(DTV_ENUM_DELSYS, 0),
        ];
        self.get_properties(&mut cmdseq).context("frontend get api version (deprecated driver)")?;

        // DVB API Version

        self.api_version = unsafe { cmdseq[0].u.data as u16 };

        // Suppoerted delivery systems

        let u_buffer = unsafe { &cmdseq[1].u.buffer };
        let u_buffer_len = ::std::cmp::min(u_buffer.len as usize, u_buffer.data.len());
        u_buffer.data[.. u_buffer_len]
            .iter()
            .for_each(|v| self.delivery_system_list.push(*v as u32));

        // dev-file metadata

        let metadata = self.file.metadata().context("frontend get device metadata")?;

        ensure!(
            metadata.file_type().is_char_device(),
            FeError::InvalidDeviceFormat
        );

        Ok(())
    }

    fn open<P: AsRef<Path>>(path: P, w: bool) -> Result<FeDevice> {
        let file = OpenOptions::new()
            .read(true)
            .write(w)
            .custom_flags(::nix::libc::O_NONBLOCK)
            .open(path)
            .context("fe open")?;

        let mut fe = FeDevice {
            file,

            api_version: 0,

            name: String::default(),
            delivery_system_list: Vec::default(),
            frequency_range: 0 .. 0,
            symbolrate_range: 0 .. 0,
            caps: 0,
        };

        fe.get_info()?;

        Ok(fe)
    }

    /// Attempts to open a frontend device in read-only mode
    #[inline]
    pub fn open_rd<P: AsRef<Path>>(path: P) -> Result<FeDevice> { Self::open(path, false) }

    /// Attempts to open a frontend device in read-write mode
    #[inline]
    pub fn open_rw<P: AsRef<Path>>(path: P) -> Result<FeDevice> { Self::open(path, true) }

    fn check_properties(&self, cmdseq: &[DtvProperty]) -> Result<()> {
        for p in cmdseq {
            match p.cmd {
                DTV_FREQUENCY => {
                    let v = p.get_data();
                    ensure!(
                        self.frequency_range.contains(&v),
                        FeError::InvalidFrequency
                    );
                }
                DTV_SYMBOL_RATE => {
                    let v = p.get_data();
                    ensure!(
                        self.symbolrate_range.contains(&v),
                        FeError::InvalidSymbolrate
                    );
                }
                DTV_INVERSION => {
                    if p.get_data() == INVERSION_AUTO {
                        ensure!(
                            self.caps & FE_CAN_INVERSION_AUTO != 0,
                            FeError::NoAutoInversion
                        );
                    }
                }
                DTV_TRANSMISSION_MODE => {
                    if p.get_data() == TRANSMISSION_MODE_AUTO {
                        ensure!(
                            self.caps & FE_CAN_TRANSMISSION_MODE_AUTO != 0,
                            FeError::NoAutoTransmitMode
                        );
                    }
                }
                DTV_GUARD_INTERVAL => {
                    if p.get_data() == GUARD_INTERVAL_AUTO {
                        ensure!(
                            self.caps & FE_CAN_GUARD_INTERVAL_AUTO != 0,
                            FeError::NoAutoGuardInterval
                        );
                    }
                }
                DTV_HIERARCHY => {
                    if p.get_data() == HIERARCHY_AUTO {
                        ensure!(
                            self.caps & FE_CAN_HIERARCHY_AUTO != 0,
                            FeError::NoAutoHierarchy
                        );
                    }
                }
                DTV_STREAM_ID => {
                    ensure!(
                        self.caps & FE_CAN_MULTISTREAM != 0,
                        FeError::NoMultistream
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Sets properties on frontend device
    pub fn set_properties(&self, cmdseq: &[DtvProperty]) -> Result<()> {
        self.check_properties(cmdseq).context("fe property check")?;

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
        ioctl_write_ptr!(#[inline] ioctl_call, b'o', 82, DtvProperties);
        unsafe {
            ioctl_call(self.as_raw_fd(), &cmd as *const _)
        }.context("frontend set properties")?;

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
        ioctl_read!(#[inline] ioctl_call, b'o', 83, DtvProperties);
        unsafe {
            ioctl_call(self.as_raw_fd(), &mut cmd as *mut _)
        }.context("frontend get properties")?;

        Ok(())
    }

    /// Returns a frontend events if available
    pub fn get_event(&self, event: &mut FeEvent) -> Result<()> {
        // FE_GET_EVENT
        ioctl_read!(#[inline] ioctl_call, b'o', 78, FeEvent);

        unsafe {
            ioctl_call(self.as_raw_fd(), event as *mut _)
        }.context("frontend get event")?;

        Ok(())
    }

    /// Turns on/off generation of the continuous 22kHz tone
    ///
    /// allowed `value`'s:
    ///
    /// - SEC_TONE_ON - turn 22kHz on
    /// - SEC_TONE_OFF - turn 22kHz off
    pub fn set_tone(&self, value: u32) -> Result<()> {
        // FE_SET_TONE
        ioctl_write_int_bad!(#[inline] ioctl_call, request_code_none!(b'o', 66));

        unsafe {
            ioctl_call(self.as_raw_fd(), value as _)
        }.context("frontend set tone")?;

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
        ioctl_write_int_bad!(#[inline] ioctl_call, request_code_none!(b'o', 67));

        unsafe {
            ioctl_call(self.as_raw_fd(), value as _)
        }.context("frontend set voltage")?;

        Ok(())
    }

    /// Sets DiSEqC master command
    ///
    /// `msg` is a message no more 6 bytes length
    ///
    /// Example DiSEqC commited command:
    ///
    /// ```
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

        cmd.msg[0 .. msg.len()].copy_from_slice(msg);
        cmd.len = msg.len() as u8;

        // FE_DISEQC_SEND_MASTER_CMD
        ioctl_write_ptr!(ioctl_call, b'o', 63, DiseqcMasterCmd);
        unsafe {
            ioctl_call(self.as_raw_fd(), &cmd as *const _)
        }.context("frontend diseqc master cmd")?;

        Ok(())
    }

    /// Returns the current API version
    /// major - first byte
    /// minor - second byte
    #[inline]
    pub fn get_api_version(&self) -> u16 {
        self.api_version
    }
}
