use {
    anyhow::{Context, Result},
    nix::{ioctl_write_int_bad, ioctl_none_bad, ioctl_write_ptr, request_code_none},
    std::{
        fs::{File, OpenOptions},
        os::unix::{
            fs::{OpenOptionsExt},
            io::{AsRawFd, RawFd},
        },
    },
    sys::*,
};


pub mod sys;

/// A reference to the demux device and device information
#[derive(Debug)]
pub struct DmxDevice {
    file: File,
    buffer_size: u32
}

impl AsRawFd for DmxDevice {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl DmxDevice {
    fn open(adapter: u32, device: u32, is_write: bool) -> Result<Self> {
        let path = format!("/dev/dvb/adapter{}/demux{}", adapter, device);
        let file = OpenOptions::new()
            .read(true)
            .write(is_write)
            .custom_flags(::nix::libc::O_NONBLOCK)
            .open(&path)
            .with_context(|| format!("DMX: failed to open device {}", &path))?;

        Ok(DmxDevice {
            file,
            buffer_size: 2 * 4096
        })
    }

    /// Attempts to open frontend device in read-only mode
    #[inline]
    pub fn open_ro(adapter: u32, device: u32) -> Result<Self> {
        Self::open(adapter, device, false)
    }

    /// Attempts to open frontend device in read-write mode
    #[inline]
    pub fn open_rw(adapter: u32, device: u32) -> Result<Self> {
        Self::open(adapter, device, true)
    }

    /// Attempts to set demux PES filter parameters.
    /// By a PES filter is meant a filter that is based just on the packet identifier (PID),
    /// i.e. no PES header or payload filtering capability is supported.
    /// 
    /// There is a flag field where it is possible to state whether a section should be CRC-checked,
    /// whether the filter should be a “one-shot” filter, i.e. if the filtering operation should be stopped
    /// after the first section is received, and whether the filtering operation should be started immediately
    /// (without waiting for a DMX_START ioctl call).
    pub fn set_pes_filter(&self, filter: &DmxPesFilterParams) -> Result<()> {
        // DMX_SET_PES_FILTER
        ioctl_write_ptr!(
            #[inline]
            ioctl_call,
            b'o',
            44,
            DmxPesFilterParams
        );

        unsafe { ioctl_call(self.as_raw_fd(), filter as *const _) }.context("DMX: set PES filter")?;

        Ok(())
    }


    /// Tries to add multiple PIDs to a transport stream filter previously set up with 
    /// set_pes_filter and output equal to DMX_OUT_TSDEMUX_TAP.
    pub fn add_pid(&self, pid: u16) -> Result<()> {
        // DMX_ADD_PID
        ioctl_write_ptr!(
            #[inline]
            ioctl_call,
            b'o',
            51,
            u16
        );

        unsafe { ioctl_call(self.as_raw_fd(), &pid as *const _) }.context("DMX: add PID")?;

        Ok(())
    }

    /// This ioctl call allows to remove a PID when multiple PIDs are set on a transport stream filter, 
    /// e. g. a filter previously set up with output equal to DMX_OUT_TSDEMUX_TAP, 
    /// created via either set_pes_filter or add_pid.
    pub fn remove_pid(&self, pid: u16) -> Result<()> {
        // DMX_REMOVE_PID
        ioctl_write_ptr!(
            #[inline]
            ioctl_call,
            b'o',
            52,
            u16
        );

        unsafe { ioctl_call(self.as_raw_fd(), &pid as *const _) }.context("DMX: remove PID")?;

        Ok(())
    }

    /// Attempts to set demux SCT filter parameters.
    /// A timeout may be defined stating number of seconds to wait for a section to be loaded.
    /// A value of 0 means that no timeout should be applied.
    /// Finally there is a flag field where it is possible to state whether a section should be CRC-checked,
    /// whether the filter should be a “one-shot” filter, i.e. if the filtering operation should be stopped
    /// after the first section is received, and whether the filtering operation should be started immediately
    /// (without waiting for a DMX_START ioctl call).
    /// 
    /// If a filter was previously set-up, this filter will be canceled, and the receive buffer will be flushed.
    pub fn set_filter(&self, filter: &DmxSctFilterParams) -> Result<()> {
        // DMX_SET_FILTER
        ioctl_write_ptr!(
            #[inline]
            ioctl_call,
            b'o',
            43,
            DmxSctFilterParams
        );

        unsafe { ioctl_call(self.as_raw_fd(), filter as *const _) }.context("DMX: set SCT filter")?;

        Ok(())
    }

    /// Attempts to set the size of the circular buffer used for filtered data.
    /// The default size is two maximum sized sections, 
    /// i.e. if this function is not called a buffer size of 2 * 4096 bytes will be used.
    pub fn set_buffer_size(&mut self, size: u32) -> Result<()> {
        // DMX_SET_BUFFER_SIZE
        ioctl_write_int_bad!(
            #[inline]
            ioctl_call,
            request_code_none!(b'o', 45)
        );

        unsafe { ioctl_call(self.as_raw_fd(), size as _) }.context("DMX: set buffer size")?;

        self.buffer_size = size;

        Ok(())
    }

    /// Attempts to start the actual filtering operation defined via the ioctl calls set_filter or set_pes_filter.
    pub fn start(&self) -> Result<()> {
        // DMX_START
        ioctl_none_bad!(
            #[inline]
            ioctl_call,
            request_code_none!(b'o', 41)
        );

        unsafe { ioctl_call(self.as_raw_fd()) }.context("DMX: start")?;

        Ok(())
    }

    /// Attempts to stop the actual filtering operation defined via the ioctl calls set_filter or set_pes_filter and started via start.
    pub fn stop(&self) -> Result<()> {
        // DMX_STOP
        ioctl_none_bad!(
            #[inline]
            ioctl_call,
            request_code_none!(b'o', 42)
        );

        unsafe { ioctl_call(self.as_raw_fd()) }.context("DMX: stop")?;

        Ok(())
    }
}
