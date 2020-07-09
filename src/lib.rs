//! TOD Clock Conversion Library
//!
//! Part of `untod`
//!
//! Converts between TOD Clock (partially extended),
//! PARS Perpetual Minute Clock, and Date/Time
//!
//! * Converts values from command line or from clipboard
//! * Converts for up to three time zones (Zulu/Greenwich, Local, Alternate)
//! * Supports three clock discplines:
//!   - UTC, with allowance for leap seconds
//!   - IBM ETR, or LORAN without leap seconds
//!   - TAI, without leap seconds
//!
//! Input for a given run can be hex TOD clock values,
//! hex Perpetual Minute Clock values, or Date and Time values.
//! Dates can be specified as *yyyy.ddd* or as *yyyy-mm-dd*.
//! Partial date and time combinations are padded on the right.

extern crate clap;

pub mod args;
pub mod leapsectab;
pub mod todinfo;
