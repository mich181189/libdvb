# libdvb

libdvb is an interface library for DVB-API v5 devices in Linux.

Supports three types of delivery systems:

- Satellite: DVB-S, DVB-S2
- Terretrial: DVB-T, DVB-T2, ATSC, ISDB-T
- Cable: DVB-C

TODO:

- Cenelec EN 50221 - Common Interface Specification for Conditional Access and
  other Digital Video BroadcastingDecoder Applications
- DiSEqC 1.0
- DiSEqC 1.1
- EN 50494 - Unicable I
- EN 50607 - Unicable II

## FeDevice

Example DVB-S2 tune:

```rust
let cmdseq = vec![
    dtv_property!(DTV_DELIVERY_SYSTEM, SYS_DVBS2),
    dtv_property!(DTV_FREQUENCY, (11044 - 9750) * 1000),
    dtv_property!(DTV_MODULATION, PSK_8),
    dtv_property!(DTV_VOLTAGE, SEC_VOLTAGE_13),
    dtv_property!(DTV_TONE, SEC_TONE_OFF),
    dtv_property!(DTV_INVERSION, INVERSION_AUTO),
    dtv_property!(DTV_SYMBOL_RATE, 27500 * 1000),
    dtv_property!(DTV_INNER_FEC, FEC_AUTO),
    dtv_property!(DTV_PILOT, PILOT_AUTO),
    dtv_property!(DTV_ROLLOFF, ROLLOFF_35),
    dtv_property!(DTV_TUNE, 0),
];

let fe = FeDevice::open_rw(0, 0)?;
fe.set_properties(&cmdseq)?;
```

Frontend information:

```rust
let fe = FeDevice::open_ro(0, 0)?;
println!("{}", &fe);
```

Frontend status:

```rust
let fe = FeDevice::open_ro(0, 0)?;
let mut status = FeStatus::default();
status.read(&fe)?;
println!("{}", &status);
```
