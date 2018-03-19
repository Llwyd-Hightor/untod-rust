extern crate chrono;
use self::chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum TodCalc {
    FromTod,
    FromDateTime,
    FromPMC,
}
#[derive(Debug)]
pub enum Padding {
    Left,
    Right,
    None,
}
#[derive(Debug)]
pub struct TodInfo {
    pub runtype: TodCalc,
    pub tod:     u64,
    pub date:    DateTime<Utc>,
    pub pmc:     u32,
    pub gmt:     bool,
    pub loff:    Option<i32>,
    pub aoff:    Option<i32>,
    pub pad:     Padding,
}
