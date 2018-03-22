extern crate clap ;
use self::clap::ArgMatches ;
extern crate chrono ;
use self::chrono::{DateTime,Local,Offset,Utc,} ;
use std::fmt ;
use std::ascii::AsciiExt ;
use std::env ;

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
    pub pmc:     PerpMinuteClock,
    pub goff:    Toffset,
    pub loff:    Toffset,
    pub aoff:    Toffset,
    pub pad:     Padding,
}
impl TodInfo {
    fn new() -> TodInfo{
        TodInfo{
            runtype: TodCalc::FromTod,
            tod:     Tod(0),
            date:    Utc::now(),
            pmc:     PerpMinuteClock(0),
            goff:    Toffset(Some(0)),
            loff:    Toffset(Some(0)),
            aoff:    Toffset(None),
            pad:     Padding::None,
        }
    } 
    pub fn new_from_args(cmdl: ArgMatches) -> TodInfo {
        let mut todwork = TodInfo::new() ;
        if cmdl.is_present("reverse") {
            todwork.runtype = TodCalc::FromDateTime ;
        }
        if cmdl.is_present("pmc") {
            todwork.runtype = TodCalc::FromPMC ;
        }
        if cmdl.is_present("pl") {
            todwork.pad = Padding::Left ;
        }
        if cmdl.is_present("pr") {
            todwork.pad = Padding::Right ;
        }
        
        todwork.loff = match cmdl.value_of("zl") {
            None => {
                match env::var("TODL") {
                    Ok(soff) => match soff.parse::<f32>() {
                        Ok(noff) => Toffset(Some( (60.0 * noff).round() as i32 * 60) ),
                        _ => Toffset(None) ,
                    },
                    _ => Toffset(Some( Local::now().offset().fix().local_minus_utc()) ) ,
                }
            },
            Some(soff) => match soff.parse::<f32>() {
                Ok(noff) => Toffset(Some( (60.0 * noff).round() as i32 * 60) ),
                _ => { eprintln!("Invalid offset: --zl {}",soff) ;
                       Toffset(None) }
            },
        } ;
        
        todwork.aoff = match cmdl.value_of("za") {
            None => {
                match env::var("TODA") {
                    Ok(soff) => match soff.parse::<f32>() {
                        Ok(noff) => Toffset(Some( (60.0 * noff).round() as i32 * 60) ),
                        _ => Toffset(None) ,
                    },
                    _ => Toffset(None),
                }
            },
            Some(soff) => match soff.parse::<f32>() {
                Ok(noff) => Toffset(Some( (60.0 * noff).round() as i32 * 60) ),
                _ => { eprintln!("Invalid offset: --zl {}",soff) ;
                       Toffset(None) }
            }
        } ;
        
        if cmdl.is_present("ng") {
            if todwork.loff == Toffset(None) && todwork.aoff == Toffset(None) {
                eprintln!("No other offsets available; --ng ignored.") ;
            } else {
                todwork.goff = Toffset(None) ;
            } ;
        } ;
        todwork
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Toffset(pub Option<i32>) ;
impl fmt::Display for Toffset{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f,"No offset"),
            Some(x) => {
                let xmm = x / 60 ;
                let xhh = xmm / 60 ;
                write!(f,"UTC{:+03}:{:02}",xhh,xmm-xhh*60)
            }
        }
    }
}

#[derive(Clone,Copy,Debug,)]
pub struct PerpMinuteClock(pub u32) ;
impl fmt::Display for PerpMinuteClock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:08x}",self.0)
    }
}

#[derive(Clone,Copy,Debug,)]
pub struct Tod(pub u64) ;
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
            let tval = u64::from_str_radix(&chex,16) ;
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
        let x = format!("{:016x}",self.0) ;
        write!(f,"{} {} {}---",&x[0..3],&x[3..11],&x[11..16])
    }
}
