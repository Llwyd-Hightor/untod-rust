extern crate chrono;
use self::chrono::{DateTime, Utc};
use std::fmt;
use std::ascii::AsciiExt;

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

pub struct Tod(pub u64);
impl Tod{
    pub fn new(tval: u64) -> Tod {
        Tod(tval)
    }
    pub fn new_from_hex(hex: &str, pad: &Padding ) -> Option<Tod> {
        if AsciiExt::is_ascii_hexdigit(hex) {
            let chex = match pad {
                &Padding::Left  => ["000000000000000",hex].join("")[hex.len()..].to_string(),
                &Padding::Right => [hex,"000000000000000"].join("")[..16].to_string(),
                _ => {
                    if &hex.as_bytes()[..1] > b"b" {
                        ["000",hex,"000000000000"].join("")[..16].to_string()
                    } else {
                        ["00",hex,"0000000000000"].join("")[..16].to_string()
                    }
                }
            } ;
            let tval = u64::from_str_radix(&chex,16);
            match tval {
                Ok(n) => Some(Tod(n)),
                _ => None
            }
        } else {
            None
        }
    }
}
impl fmt::Display for Tod{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = format!("{:016x}",self.0);
        return write!(f,"{} {} {}---",&x[0..3],&x[3..11],&x[11..16]);
    }
}
