mod asn1;
mod tpdu;
mod spdu;
mod apdu;
pub mod sys;


use {
    std::{
        path::{
            Path,
        },
        fs::{
            File,
            OpenOptions,
        },
        os::unix::{
            fs::OpenOptionsExt,
            io::{
                AsRawFd,
                RawFd,
            },
        },
        time::Duration,
        thread,
    },

    anyhow::{
        Result,
        Context,
    },

    nix::{
        ioctl_none,
        ioctl_read,
    },

    sys::*,
};


const CA_DELAY: Duration = Duration::from_millis(100);


#[derive(Debug)]
pub struct CaDevice {
    file: File,
    slot: CaSlotInfo,
}


impl AsRawFd for CaDevice {
    #[inline]
    fn as_raw_fd(&self) -> RawFd { self.file.as_raw_fd() }
}


impl CaDevice {
    /// Sends reset command to CA device
    #[inline]
    pub fn reset(&mut self) -> Result<()> {
        // CA_RESET
        ioctl_none!(#[inline] ca_reset, b'o', 128);
        unsafe {
            ca_reset(self.as_raw_fd())
        }.context("CA: failed to reset")?;

        Ok(())
    }

    /// Gets CA capabilities
    #[inline]
    pub fn get_caps(&self, caps: &mut CaCaps) -> Result<()> {
        // CA_GET_CAP
        ioctl_read!(#[inline] ca_get_cap, b'o', 129, CaCaps);
        unsafe {
            ca_get_cap(self.as_raw_fd(), caps as *mut _)
        }.context("CA: failed to get caps")?;

        Ok(())
    }

    /// Gets CA slot information
    #[inline]
    pub fn get_slot_info(&mut self) -> Result<()> {
        // CA_GET_SLOT_INFO
        ioctl_read!(#[inline] ca_get_slot_info, b'o', 130, CaSlotInfo);
        unsafe {
            ca_get_slot_info(self.as_raw_fd(), &mut self.slot as *mut _)
        }.context("CA: failed to get slot info")?;

        Ok(())
    }

    /// Attempts to open a CA device
    pub fn open(path: &Path, slot: u32) -> Result<CaDevice> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(::nix::libc::O_NONBLOCK)
            .open(path)
            .with_context(|| format!("CA: failed to open device {}", path.display()))?;

        let mut ca = CaDevice {
            file,
            slot: CaSlotInfo::default(),
        };

        ca.reset()?;

        thread::sleep(CA_DELAY);

        let mut caps = CaCaps::default();

        for _ in 0 .. 5 {
            ca.get_caps(&mut caps)?;

            if caps.slot_num != 0 {
                break;
            }

            thread::sleep(CA_DELAY);
        }

        if slot >= caps.slot_num {
            return Err(anyhow!("CA: slot {} not found", slot));
        }

        ca.slot.slot_num = slot;
        ca.get_slot_info()?;

        if ca.slot.slot_type != CA_CI_LINK {
            return Err(anyhow!("CA: incompatible interface"));
        }

        // reset flags
        ca.slot.flags = CA_CI_MODULE_NOT_FOUND;

        Ok(ca)
    }

    pub fn poll(&mut self) -> Result<()> {
        thread::sleep(CA_DELAY);

        let flags = self.slot.flags;

        self.get_slot_info()?;

        match self.slot.flags {
            CA_CI_MODULE_PRESENT => {
                if flags == CA_CI_MODULE_READY {
                    // TODO: de-init
                }
                return Ok(())
            }
            CA_CI_MODULE_READY => {
                if flags != CA_CI_MODULE_READY {
                    tpdu::init(self, self.slot.slot_num as u8)?;
                }
            }
            CA_CI_MODULE_NOT_FOUND => {
                return Err(anyhow!("CA: module not found"));
            }
            _ => {
                return Err(anyhow!("CA: invalid slot flags"));
            }
        };

        // TODO: poll self.as_raw_fd()

        unimplemented!()
    }
}
