use crate::get_dtv_properties;

use {
    super::{sys::*, FeDevice},
    anyhow::Result,
    std::fmt,
};

/// Frontend status
#[derive(Debug)]
pub struct FeStatus {
    /// `sys::frontend::FeStatus`
    status: fe_status,

    delivery_system: Option<fe_delivery_system>,
    modulation: Option<fe_modulation>,
    signal_strength_decibel: Option<f64>,
    signal_strength_percentage: Option<u8>,
    snr_decibel: Option<f64>,
    snr_percentage: Option<u8>,
    // ber - number of bit errors
    ber: Option<u64>,
    // unc - number of block errors
    unc: Option<u64>,
}

impl Default for FeStatus {
    fn default() -> FeStatus {
        FeStatus {
            status: fe_status::FE_NONE,
            delivery_system: None,
            modulation: None,
            signal_strength_decibel: None,
            signal_strength_percentage: None,
            snr_decibel: None,
            snr_percentage: None,
            ber: None,
            unc: None,
        }
    }
}

/// Returns an object that implements `Display` for different verbosity levels
///
/// Tuner is turned off:
///
/// ```text
/// OFF
/// ```
///
/// Tuner acquiring signal but has no lock:
///
/// ```text
/// NO-LOCK 0x01 | Signal -38.56dBm (59%)
/// NO-LOCK 0x03 | Signal -38.56dBm (59%) | Quality 5.32dB (25%)
/// ```
///
/// Hex number after `NO-LOCK` this is tuner status bit flags:
/// - 0x01 - has signal
/// - 0x02 - has carrier
/// - 0x04 - has viterbi
/// - 0x08 - has sync
/// - 0x10 - has lock
/// - 0x20 - timed-out
/// - 0x40 - re-init
///
/// Tuner has lock
///
/// ```text
/// LOCK dvb-s2 | Signal -38.56dBm (59%) | Quality 14.57dB (70%) | BER:0 | UNC:0
/// ```
impl fmt::Display for FeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.status == fe_status::FE_NONE {
            write!(f, "OFF")?;
            return Ok(());
        }

        if self.status.contains(fe_status::FE_HAS_LOCK) {
            write!(
                f,
                "LOCK {}",
                self.get_delivery_system().as_ref().unwrap_or(&fe_delivery_system::SYS_UNDEFINED)
            )?;
        } else {
            write!(f, "NO-LOCK 0x{:02X}", self.status)?;
        }

        if !self.status.contains(fe_status::FE_HAS_SIGNAL) {
            return Ok(());
        }

        write!(
            f,
            " | Signal {:.02}dBm ({}%)",
            self.get_signal_strength_decibel().unwrap_or(0.0),
            self.get_signal_strength().unwrap_or(0)
        )?;

        if !self.status.contains(fe_status::FE_HAS_CARRIER) {
            return Ok(());
        }

        write!(
            f,
            " | Quality {:.02}dB ({}%)",
            self.get_snr_decibel().unwrap_or(0.0),
            self.get_snr().unwrap_or(0)
        )?;

        if !self.status.contains(fe_status::FE_HAS_LOCK) {
            return Ok(());
        }

        write!(f, " | BER:")?;
        if let Some(ber) = self.get_ber() {
            write!(f, "{}", ber)?;
        } else {
            write!(f, "-")?;
        }

        write!(f, " | UNC:")?;
        if let Some(unc) = self.get_unc() {
            write!(f, "{}", unc)?;
        } else {
            write!(f, "-")?;
        }

        Ok(())
    }
}

impl FeStatus {
    /// Returns current delivery system
    #[inline]
    pub fn get_delivery_system(&self) -> &Option<fe_delivery_system> {
        &self.delivery_system
    }

    /// Returns current modulation
    #[inline]
    pub fn get_modulation(&self) -> &Option<fe_modulation> {
        &self.modulation
    }

    /// Returns Signal Strength in dBm
    pub fn get_signal_strength_decibel(&self) -> &Option<f64> {
        &self.signal_strength_decibel
    }

    /// Returns Signal Strength in percentage
    pub fn get_signal_strength(&self) -> &Option<u8> {
        &self.signal_strength_percentage
    }

    /// Returns Signal to noise ratio in dB
    pub fn get_snr_decibel(&self) -> &Option<f64> {
        &self.snr_decibel
    }

    /// Returns Signal Strength in percentage
    pub fn get_snr(&self) -> &Option<u8> {
        &self.snr_percentage
    }

    /// Returns BER value if available
    pub fn get_ber(&self) -> &Option<u64> {
        &self.ber
    }

    /// Returns UNC value if available
    pub fn get_unc(&self) -> &Option<u64> {
        &self.unc
    }

    fn normalize_signal_strength(&mut self, stats: DtvFrontendStats) {
        self.signal_strength_decibel = stats.get_decibel_float();
        self.signal_strength_percentage = match (stats.get_relative(), stats.get_decibel()) {
            (Some(v), _) => Some(((v as u32) * 100 / 65535) as u8),
            (None, Some(decibel)) if self.status.contains(fe_status::FE_HAS_SIGNAL) => {
                // TODO: check delivery_system
                // TODO: this logic looks very sus
                let lo: i64 = -85000;
                let hi: i64 = -6000;
                Some({
                    if decibel > hi {
                        100
                    } else if decibel < lo {
                        0
                    } else {
                        (((lo - decibel) * 100) / (lo - hi)) as u8
                    }
                })
            }
            _ => None,
        };
    }

    fn normalize_snr(&mut self, stats: DtvFrontendStats) {
        self.signal_strength_decibel = stats.get_decibel_float();
        self.signal_strength_percentage = match (stats.get_relative(), stats.get_decibel()) {
            (Some(v), _) => Some(((v as u32) * 100 / 65535) as u8),
            (None, Some(decibel)) if self.status.contains(fe_status::FE_HAS_CARRIER) => {
                match match self.delivery_system {
                    Some(SYS_DVBS | SYS_DVBS2) => Some(15000),

                    Some(SYS_DVBC_ANNEX_A | SYS_DVBC_ANNEX_B | SYS_DVBC_ANNEX_C | SYS_DVBC2) => {
                        Some(28000)
                    }

                    Some(SYS_DVBT | SYS_DVBT2) => Some(19000),

                    Some(SYS_ATSC) => Some(match self.modulation {
                        Some(VSB_8 | VSB_16) => 19000,
                        _ => 28000,
                    }),

                    _ => None,
                } {
                    Some(_) if decibel <= 0 => Some(0),
                    Some(vhi) if decibel >= vhi => Some(100),
                    Some(vhi) => Some(((decibel * 100) / vhi) as u8),
                    _ => None,
                }
            }
            _ => None,
        };
    }

    /// Reads frontend status with fallback to DVBv3 API
    pub fn read(&mut self, fe: &FeDevice) -> Result<()> {
        self.status = fe.read_status()?;

        if self.status == fe_status::FE_NONE {
            return Ok(());
        }

        let (delivery_system, modulation, signal_strength, snr, ber, unc) = get_dtv_properties!(
            fe,
            DTV_DELIVERY_SYSTEM,
            DTV_MODULATION,
            DTV_STAT_SIGNAL_STRENGTH,
            DTV_STAT_CNR,
            DTV_STAT_PRE_ERROR_BIT_COUNT,
            DTV_STAT_ERROR_BLOCK_COUNT
        )?;
        self.delivery_system = Some(delivery_system);
        self.modulation = Some(modulation);
        self.normalize_signal_strength(signal_strength);
        self.normalize_snr(snr);
        self.ber = match ber.get_counter() {
            Some(v) => Some(v),
            None if self.status.contains(fe_status::FE_HAS_LOCK) => Some(fe.read_ber()?),
            None => None,
        };
        self.unc = match unc.get_counter() {
            Some(v) => Some(v),
            None if self.status.contains(fe_status::FE_HAS_LOCK) => Some(fe.read_unc()?),
            None => None,
        };

        Ok(())
    }
}
