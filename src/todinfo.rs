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
#[derive(Debug,)]
pub struct TodInfo {
    pub runtype: TodCalc,
    pub tod:     Tod,
    pub date:    NaiveDateTime,
    pub pmc:     PerpMinuteClock,
    pub goff:    Toffset,
    pub loff:    Toffset,
    pub aoff:    Toffset,
    pub pad:     Padding,
    pub leap:    bool,
    pub lsec:    i64,
    pub lstab:   LeapSecTable,
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
            leap:    false,
            lsec:    0,
            lstab:   LeapSecTable::new(),
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

        todwork.leap = cmdl.is_present("leapsec");

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
        if self.leap {
            format!("{} : {} {} {} {} {} *{}",self.tod,odate,offset,ojd,oday,self.pmc,self.lsec)
        } else {
            format!("{} : {} {} {} {} {}",self.tod,odate,offset,ojd,oday,self.pmc)
        }
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

#[derive(Clone,Debug,)]
pub struct LeapSec{
    day: NaiveDate,
    tod: u64,
    count: i64,
}

#[derive(Debug,)]
pub struct LeapSecTable(Vec<LeapSec>);
impl LeapSecTable{
    pub fn new() -> LeapSecTable {
        LeapSecTable(vec![
            LeapSec{day: NaiveDate::from_ymd(2017,01,01) , tod: 0x000D1E0D68173CC0, count: 27},
            LeapSec{day: NaiveDate::from_ymd(2015,07,01) , tod: 0x000CF2D54B4FBA80, count: 26},
            LeapSec{day: NaiveDate::from_ymd(2012,07,01) , tod: 0x000C9CC9A704D840, count: 25},
            LeapSec{day: NaiveDate::from_ymd(2009,01,01) , tod: 0x000C3870CB9BB600, count: 24},
            LeapSec{day: NaiveDate::from_ymd(2006,01,01) , tod: 0x000BE251097973C0, count: 23},
            LeapSec{day: NaiveDate::from_ymd(1999,01,01) , tod: 0x000B1962F9305180, count: 22},
            LeapSec{day: NaiveDate::from_ymd(1997,07,01) , tod: 0x000AEE3EFA402F40, count: 21},
            LeapSec{day: NaiveDate::from_ymd(1996,01,01) , tod: 0x000AC34336FECD00, count: 20},
            LeapSec{day: NaiveDate::from_ymd(1994,07,01) , tod: 0x000A981F380EAAC0, count: 19},
            LeapSec{day: NaiveDate::from_ymd(1993,07,01) , tod: 0x000A7B70ABEB8880, count: 18},
            LeapSec{day: NaiveDate::from_ymd(1992,07,01) , tod: 0x000A5EC21FC86640, count: 17},
            LeapSec{day: NaiveDate::from_ymd(1991,01,01) , tod: 0x000A33C65C870400, count: 16},
            LeapSec{day: NaiveDate::from_ymd(1990,01,01) , tod: 0x000A1717D063E1C0, count: 15},
            LeapSec{day: NaiveDate::from_ymd(1988,01,01) , tod: 0x0009DDA69A557F80, count: 14},
            LeapSec{day: NaiveDate::from_ymd(1985,07,01) , tod: 0x000995D40F517D40, count: 13},
            LeapSec{day: NaiveDate::from_ymd(1983,07,01) , tod: 0x00095C62D9431B00, count: 12},
            LeapSec{day: NaiveDate::from_ymd(1982,07,01) , tod: 0x00093FB44D1FF8C0, count: 11},
            LeapSec{day: NaiveDate::from_ymd(1981,07,01) , tod: 0x00092305C0FCD680, count: 10},
            LeapSec{day: NaiveDate::from_ymd(1980,01,01) , tod: 0x0008F809FDBB7440, count:  9},
            LeapSec{day: NaiveDate::from_ymd(1979,01,01) , tod: 0x0008DB5B71985200, count:  8},
            LeapSec{day: NaiveDate::from_ymd(1978,01,01) , tod: 0x0008BEACE5752FC0, count:  7},
            LeapSec{day: NaiveDate::from_ymd(1977,01,01) , tod: 0x0008A1FE59520D80, count:  6},
            LeapSec{day: NaiveDate::from_ymd(1976,01,01) , tod: 0x0008853BAF578B40, count:  5},
            LeapSec{day: NaiveDate::from_ymd(1975,01,01) , tod: 0x0008688D23346900, count:  4},
            LeapSec{day: NaiveDate::from_ymd(1974,01,01) , tod: 0x00084BDE971146C0, count:  3},
            LeapSec{day: NaiveDate::from_ymd(1973,01,01) , tod: 0x00082F300AEE2480, count:  2},
            LeapSec{day: NaiveDate::from_ymd(1972,07,01) , tod: 0x000820BA9811E240, count:  1},
            LeapSec{day: NaiveDate::from_ymd(0000,01,01) , tod: 0x0000000000000000, count:  0},
        ])
    }

    pub fn ls_search_day(&self, todwork: &TodInfo) -> i64 {
        let thedate = todwork.date.date();
        match todwork.leap {
            true => match self.0.iter().find( |ref x| x.day <= thedate ) {
                Some(ref x) => x.count,
                None => self.0[self.0.len()-1].count,
            },
            false => 0,
        } 
    }
    
    pub fn ls_search_tod(&self, todwork: &TodInfo) -> i64 {
        match todwork.leap {
            true => match self.0.iter().find(|ref x| x.tod <= todwork.tod.0) {
                Some(ref x) => x.count,
                None => self.0[0].count,
            },
            false => 0,
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
    todwork.lsec = todwork.lstab.ls_search_tod(todwork);
    let x = todbase.checked_add_signed(Duration::microseconds(todwork.tod.0 as i64));
    let zdate = match x {
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
                todwork.date = zdate.checked_add_signed(Duration::seconds(x as i64 - todwork.lsec))
                    .expect("Couldn't convert date");
                todwork.pmc = findpmc(&todwork);
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
    todwork.lsec = todwork.lstab.ls_search_day(todwork);
    let (zsec, zmic) = get_sec_mic(&todwork);
    todwork.pmc = findpmc(&todwork);
    
    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x) => {
                let x = zsec as i64 + x as i64 + todwork.lsec;
                if x >= 0 {
                    todwork.tod = Tod(x as u64 * 1_000_000 + zmic);
                    result.push(todwork.text(off));
                } else {
                    result.push(format!("Date is out of range: {} {}",a,off));
                    return result;
                } ; 
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
    todwork.lsec = todwork.lstab.ls_search_tod(todwork);
    let (zsec, zmic) = get_sec_mic(&todwork);

    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x) => {
                todwork.tod = Tod((zsec as i64 + x as i64 - todwork.lsec) as u64 * 1_000_000 + zmic);
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
                
pub fn findpmc(todwork: &TodInfo) -> PerpMinuteClock {
    let parsbase = NaiveDate::from_ymd(1966,01,03).and_hms(0,0,0);
    let pdiff = todwork.date.signed_duration_since(parsbase);
    let pmin = pdiff.num_seconds() / 60;
    if pmin >= 0 && pmin <= MAX as i64 {
        PerpMinuteClock(Some(pmin as u32))
    } else {
        PerpMinuteClock(None)
    }
}    
                
pub fn get_sec_mic(todwork: &TodInfo) -> (u64, u64) {
    let todbase = NaiveDate::from_ymd(1900,01,01).and_hms(0,0,0);
    let tdiff = todwork.date.signed_duration_since(todbase);
    let zsec = tdiff.num_seconds();
    let zmic = (tdiff - Duration::seconds(zsec))
        .num_microseconds()
        .unwrap() as u64;
    let zsec = zsec as u64;
    (zsec, zmic)
}    
