extern crate clap;
use self::clap::ArgMatches;
extern crate chrono;
use self::chrono::{Duration, Local, NaiveDate, NaiveDateTime, Offset, ParseResult, Utc,};
extern crate clipboard;
use self::clipboard::{ClipboardContext, ClipboardProvider,};
use std::cmp::min;
use std::fmt;
use std::ascii::AsciiExt;
use std::u32::MAX;

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
    pub date:    NaiveDateTime,
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
            date:    Utc::now().naive_utc(),
            pmc:     PerpMinuteClock(None),
            goff:    Toffset(Some(0)),
            loff:    Toffset(Some(0)),
            aoff:    Toffset(None),
            pad:     Padding::None,
        }
    }
    pub fn new_from_args(cmdl: &ArgMatches) -> TodInfo {
        let mut todwork = TodInfo::new();
        if cmdl.is_present("reverse") {
            todwork.runtype = TodCalc::FromDateTime;
        }
        if cmdl.is_present("pmc") {
            todwork.runtype = TodCalc::FromPMC;
        }
        if cmdl.is_present("pl") {
            todwork.pad = Padding::Left;
        }
        if cmdl.is_present("pr") {
            todwork.pad = Padding::Right;
        }

        todwork.loff = match cmdl.value_of("zl") {
            None => Toffset(Some( Local::now().offset().fix().local_minus_utc()) ),
            Some(soff) => match soff.parse::<f32>() {
                Ok(noff) => Toffset(Some( (60.0 * noff).round() as i32 * 60) ),
                _ => { eprintln!("Invalid offset: --zl {}",soff);
                       Toffset(None) }
            },
        };

        todwork.aoff = match cmdl.value_of("za") {
            None => Toffset(None),
            Some(soff) => match soff.parse::<f32>() {
                Ok(noff) => Toffset(Some( (60.0 * noff).round() as i32 * 60) ),
                _ => { eprintln!("Invalid offset: --zl {}",soff);
                       Toffset(None) }
            }
        };

        if cmdl.is_present("ng") {
            if todwork.loff == Toffset(None) && todwork.aoff == Toffset(None) {
                eprintln!("No other offsets available; --ng ignored.");
            } else {
                todwork.goff = Toffset(None);
            };
        };

        if todwork.aoff == todwork.goff || todwork.aoff == todwork.loff {
            todwork.aoff = Toffset(None);
        }

        if todwork.loff == todwork.goff {
            todwork.loff = Toffset(None);
        }
        todwork
    }
    pub fn text(&self, offset: Toffset) -> String {
        let odate = self.date.format("%F %H:%M:%S%.6f");
        let ojd = self.date.format("%Y.%j");
        let oday = self.date.format("%a");
        format!("{} : {} {} {} {} {}",self.tod,odate,offset,ojd,oday,self.pmc)
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Toffset(pub Option<i32>);
impl fmt::Display for Toffset{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f,"No offset"),
            Some(x) => {
                let xmm = x / 60;
                let xhh = xmm / 60;
                write!(f,"UTC{:+03}:{:02}",xhh,xmm.abs()%60)
            }
        }
    }
}

#[derive(Clone,Copy,Debug,)]
pub struct PerpMinuteClock(pub Option<u32>);
impl PerpMinuteClock {
    pub fn new() -> PerpMinuteClock {
        PerpMinuteClock(None)
    }
    pub fn new_from_int(tval: u32) -> PerpMinuteClock {
        PerpMinuteClock(Some(tval))
    }
    pub fn new_from_hex(hex: &str) -> PerpMinuteClock {
        if AsciiExt::is_ascii_hexdigit(hex) {
            let pval = u32::from_str_radix(hex,16);
            match pval {
                Ok(n) => PerpMinuteClock(Some(n)),
                _ => PerpMinuteClock(None),
            }
        } else {
            PerpMinuteClock(None)
        }
    }
}

impl fmt::Display for PerpMinuteClock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(x) => write!(f,"{:08x}",x),
            None => write!(f,"--------"),
        }
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
            };
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
        write!(f,"{} {} {}---",&x[0..3],&x[3..11],&x[11..16])
    }
}

pub fn finddate(ds: String) -> ParseResult<NaiveDateTime> {
    if ds.to_uppercase() == "NOW" {
        NaiveDateTime::parse_from_str(&defaultdate(),"%F@%H:%M:%S%.f")
    } else {
        let xlen = ds.len();
        if xlen > 4 && ds.as_bytes()[4] == b'.' {
            let padding = "1900.001@00:00:00.000000";
            let xlen = min(xlen, padding.len());
            let x = &padding[xlen..];
            NaiveDateTime::parse_from_str(&(ds + &x),"%Y.%j@%H:%M:%S%.f")
        } else {
            let padding = "1900-01-01@00:00:00.000000";
            let xlen = min(xlen, padding.len());
            let x = &padding[xlen..];
            NaiveDateTime::parse_from_str(&(ds + &x),"%F@%H:%M:%S%.f")
        }
    }
}
pub fn defaultdate() -> String {
    Utc::now().format("%F@%H:%M:%S%.6f").to_string()
}

pub fn from_tod(a: String, todwork: &mut TodInfo) -> Vec<String> {
    let todbase = NaiveDate::from_ymd(1900,01,01).and_hms(0,0,0);
    let mut result: Vec<String> = Vec::new();
    let xtod = Tod::new_from_hex(&a.clone(),&todwork.pad);
    todwork.tod = match xtod {
        None => {
            result.push(format!("TOD value is invalid: {:?}",a));
            return result;
        },
        Some(x) => x,
    };
    let zdate = match todbase.checked_add_signed(Duration::microseconds(todwork.tod.0 as i64)) {
        None => {
            result.push(format!("Can't handle this TOD value: {:?}",a));
            return result;
        },
        Some(x) => x,
    };

    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x) => {
                todwork.date = zdate.checked_add_signed(Duration::seconds(x as i64))
                    .expect("Couldn't convert date");
                todwork.pmc = findpmc(*todwork);
                result.push(todwork.text(off));
            },
        };
    };
    result
}

pub fn from_datetime(a: String, todwork: &mut TodInfo) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let xdt = finddate(a.clone());
    todwork.date = match xdt {
        Err(_) => {
            result.push(format!("Date {:?} is invalid",a));
            return result;
        },
        Ok(x) => x,
    };
    let (zsec, zmic) = get_sec_mic(*todwork);
    todwork.pmc = findpmc(*todwork);
    
    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x) => {
                todwork.tod = Tod((zsec as i64 + x as i64) as u64 * 1_000_000 + zmic);
                result.push(todwork.text(off));
            },
        };
    };
   result
}
pub fn from_perpetual(a: String, todwork: &mut TodInfo) -> Vec<String> {
    let parsbase = NaiveDate::from_ymd(1966,01,03).and_hms(0,0,0);
    let mut result: Vec<String> = Vec::new();
    todwork.pmc = PerpMinuteClock::new_from_hex(&a);
    let pmc = match todwork.pmc.0 {
        None => {
            result.push(format!("Minute value is invalid: {:?}",a));
            return result;
        },
        Some(x) => x,
    };
    todwork.date = match parsbase.checked_add_signed(Duration::minutes(pmc as i64)) {
        None => {
            result.push(format!("Can't handle this pmc value: {:?}",a));
            return result;
        },
        Some(x) => x,
    };
    let (zsec, zmic) = get_sec_mic(*todwork);

    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x) => {
                todwork.tod = Tod((zsec as i64 + x as i64) as u64 * 1_000_000 + zmic);
                result.push(todwork.text(off));
            },
        };
    };
    result
}

pub fn args_or_clipboard(cmdl: &ArgMatches) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if cmdl.is_present("clipboard") {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        for item in ctx.get_contents().unwrap().split_whitespace() {
            result.push(item.trim_matches(char::from(0)).to_string());
        } 
    } else {
        for item in cmdl.values_of("values").unwrap() { 
            result.push(item.to_string());
        }
    };
    result
}
                
pub fn findpmc(todwork: TodInfo) -> PerpMinuteClock {
    let parsbase = NaiveDate::from_ymd(1966,01,03).and_hms(0,0,0);
    let pdiff = todwork.date.signed_duration_since(parsbase);
    let pmin = pdiff.num_seconds() / 60;
    if pmin >= 0 && pmin <= MAX as i64 {
        PerpMinuteClock(Some(pmin as u32))
    } else {
        PerpMinuteClock(None)
    }
}    
                
pub fn get_sec_mic(todwork: TodInfo) -> (u64, u64) {
    let todbase = NaiveDate::from_ymd(1900,01,01).and_hms(0,0,0);
    let tdiff = todwork.date.signed_duration_since(todbase);
    let zsec = tdiff.num_seconds();
    let zmic = (tdiff - Duration::seconds(zsec))
        .num_microseconds()
        .unwrap() as u64;
    let zsec = zsec as u64;
    (zsec, zmic)
}    
