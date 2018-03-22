extern crate chrono;
use self::chrono::{DateTime, Utc};
use std::fmt;
use std::ascii::AsciiExt;

#[derive(Clone,Copy,Debug,)]
pub enum TodCalc {
    FromTod,
    FromDateTime,
    FromPMC,
}
#[derive(Clone,Copy,Debug,)]
pub enum Padding {
    Left,
    Right,
    None,
}
#[derive(Clone,Copy,Debug,)]
pub struct TodInfo {
    pub runtype: TodCalc,
    pub tod:     Tod,
    pub date:    DateTime<Utc>,
    pub pmc:     ParsDayNo,
    pub goff:    Option<Toffset>,
    pub loff:    Option<Toffset>,
    pub aoff:    Option<Toffset>,
    pub pad:     Padding,
}

#[derive(Clone,Copy,Debug,)]
pub struct Toffset(pub i32);
impl fmt::Display for Toffset{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let xmm = self.0 / 60 ;
        let xhh = xmm / 60 ;
        return write!(f,"UTC{:+03}:{:02}",xhh,xmm-xhh*60) ;
    }
}

#[derive(Clone,Copy,Debug,)]
pub struct ParsDayNo(pub u32);
impl fmt::Display for ParsDayNo{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f,"{:08x}",self.0);
    }
}

#[derive(Clone,Copy,Debug,)]
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
