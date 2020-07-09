extern crate clap;
use self::clap::ArgMatches;

extern crate chrono;
use self::chrono::{Duration, Local, NaiveDate, NaiveDateTime, Offset, ParseResult, Utc};

extern crate clipboard;
use self::clipboard::{ClipboardContext, ClipboardProvider};

use super::leapsectab::*;

use std::cmp::min;
use std::num::ParseIntError;
use std::fmt;
use std::u32::MAX;
use std::fs::File;
use std::io::{self, Read};

/// Defines calculation type (input value interpretation):
/// *    FromTod: Inputs are (hex) TOD Clock values
/// *   FromDateTime: Inputs are Date/Time values
/// *   FromPMC: Inputs are (hex) Permetual Minute Clock
/// values

#[derive(Clone, Copy, Debug)]
pub enum TodCalc {
    FromTod,
    FromDateTime,
    FromPMC,
    FromUnix,
    FromCsec,
}

/// Defines type of padding for input TOD Clock values
/// * Left: Pad with zeros on the left
/// * Right: Pad with zeros on the right
/// * None: "Intelligent" padding
/// <br/> If first digit is 0-b,
/// pad with two zero digits on the left,
/// then fill with zeros on the right.
/// <br/>If c-f, pad with three zeros.

#[derive(Clone, Copy, Debug)]
pub enum Padding {
    Left,
    Right,
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum Source {
    Clip,
    File,
    None,
}

/// Work area for assembling the calculation

#[derive(Debug)]
pub struct TodInfo {
    pub runtype: TodCalc,
    pub tod:     Tod,
    pub date:    NaiveDateTime,
    pub pmc:     PerpMinuteClock,
    pub usc:     UnixSecondsClock,
    pub goff:    Toffset,
    pub loff:    Toffset,
    pub aoff:    Toffset,
    pub pad:     Padding,
    pub src:     Source,
    pub cname:   String,
    pub csv:     bool,
    pub utc:     bool,
    pub tai:     i64,
    pub lsec:    i64,
    pub lstab:   LeapSecTable,
}

impl TodInfo {
    /// Makes a new default (empty) work area
    fn new() -> TodInfo {
        TodInfo {
            runtype: TodCalc::FromTod,
            tod:     Tod(0,),
            date:    Utc::now().naive_utc(),
            pmc:     PerpMinuteClock(None,),
            usc:     UnixSecondsClock(None,),
            goff:    Toffset(Some(0,),),
            loff:    Toffset(Some(0,),),
            aoff:    Toffset(None,),
            pad:     Padding::None,
            src:     Source::None,
            cname:   "UTC".to_string(),
            csv:     false,
            utc:     true,
            tai:     0,
            lsec:    0,
            lstab:   LeapSecTable::new(),
        }
    }

    /// Builds a new work area from command line arguments
    pub fn new_from_args(cmdl: &ArgMatches) -> TodInfo {
        let mut todwork = TodInfo::new();
        if cmdl.is_present("reverse",) {
            todwork.runtype = TodCalc::FromDateTime;
        }
        if cmdl.is_present("unix",) {
            todwork.runtype = TodCalc::FromUnix;
        }
        if cmdl.is_present("csec",) {
            todwork.runtype = TodCalc::FromCsec;
        }
        if cmdl.is_present("pmc",) {
            todwork.runtype = TodCalc::FromPMC;
        }
        if cmdl.is_present("clipboard",) {
            todwork.src = Source::Clip ;
        }
        if cmdl.is_present("infile",) {
            todwork.src = Source::File ;
        }
        if cmdl.is_present("pl",) {
            todwork.pad = Padding::Left;
        }
        if cmdl.is_present("pr",) {
            todwork.pad = Padding::Right;
        }
        todwork.loff = match cmdl.value_of("zl",) {
            None => Toffset(Some(i64::from(
                Local::now().offset().fix().local_minus_utc(),
            ),),),
            Some(soff,) => match soff.parse::<f32>() {
                Ok(noff,) => Toffset(Some((60.0 * noff).round() as i64 * 60,),),
                _ => {
                    eprintln!("Invalid offset: --zl {}", soff);
                    Toffset(None,)
                },
            },
        };
        todwork.aoff = match cmdl.value_of("za",) {
            None => Toffset(None,),
            Some(soff,) => match soff.parse::<f32>() {
                Ok(noff,) => Toffset(Some((60.0 * noff).round() as i64 * 60,),),
                _ => {
                    eprintln!("Invalid offset: --zl {}", soff);
                    Toffset(None,)
                },
            },
        };
        if cmdl.is_present("ng",) {
            if todwork.loff == Toffset(None,) && todwork.aoff == Toffset(None,) {
                eprintln!("No other offsets available; --ng ignored.");
            } else {
                todwork.goff = Toffset(None,);
            };
        };
        if cmdl.is_present("lor",) {
            todwork.utc = false;
            todwork.cname = "LOR".to_string();
        }
        if cmdl.is_present("tai",) {
            todwork.utc = false;
            todwork.cname = "TAI".to_string();
            todwork.tai = -10;
        }
        if todwork.aoff == todwork.goff || todwork.aoff == todwork.loff {
            todwork.aoff = Toffset(None,);
        }
        if todwork.loff == todwork.goff {
            todwork.loff = Toffset(None,);
        }
        todwork.csv = cmdl.is_present("csv",) ;
        todwork
    }

    /// Formats the work area values as a line of text
    pub fn text(&self, offset: Toffset,) -> String {
        let ojd = self.date.format("%Y.%j",);
        let oday = self.date.format("%a",);
        if self.csv {
            let odate = self.date.format("%F,%H:%M:%S%.6f",);
            if self.utc {
                format!("{},{},{}{},{},{},{},{:0},*{:+}",
                self.tod, odate, self.cname, offset, ojd, oday, self.pmc, self.usc_csv(), 
                self.lsec 
                )
            } else {
                format!("{},{},{}{},{},{},{},{:0},NA",
                self.tod, odate, self.cname, offset, ojd, oday, self.pmc, self.usc_csv()
                )
            }    
        } else {
            let odate = self.date.format("%F %H:%M:%S%.6f",);
            if self.utc {
                format!("{} : {} {}{} {} {} {} {} *{:+}",
                self.tod, odate, self.cname, offset, ojd, oday, self.pmc, self.usc, self.lsec
                )
            } else {
                format!("{} : {} {}{} {} {} {} {}",
                self.tod, odate, self.cname, offset, ojd, oday, self.pmc, self.usc
                )
            }    
        }
    }
    
    pub fn usc_csv(&self) -> String {
        match self.usc.0 {
            Some(x,) => format!("{}", x),
            None     => "--".to_string(),
        }
    }
}

/// Time zone offset for a calculation
///
/// Optional, signed, number of seconds
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Toffset(pub Option<i64,>,);
impl fmt::Display for Toffset {
    /// Displays as *+hh:mm*
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match self.0 {
            None => write!(f, "No offset"),
            Some(x,) => {
                let xmm = x / 60;
                let xhh = xmm / 60;
                write!(f, "{:+03}:{:02}", xhh, xmm.abs() % 60)
            },
        }
    }
}

/// Perpetual Minute Clock (*PMC*) value
///
/// Optional, unsigned, number of minutes
/// since 1963-01-03T00:00:00
#[derive(Clone, Copy, Debug, Default)]
pub struct PerpMinuteClock(pub Option<u32,>,);
impl PerpMinuteClock {
    /// Makes a zero PMC
    pub fn new() -> PerpMinuteClock { PerpMinuteClock(None,) }

    /// Makes a PMC from an integer
    pub fn new_from_int(tval: u32) -> PerpMinuteClock { PerpMinuteClock(Some(tval,),) }

    /// Makes a PMC from a hex string, zero-padded on the
    /// right
    pub fn new_from_hex(hex: &str) -> PerpMinuteClock {
        if hex.bytes().all(|b| b.is_ascii_hexdigit(),) {
            let pval = u32::from_str_radix(&[hex, "0000000"].join("",)[..8], 16,);
            match pval {
                Ok(n,) => PerpMinuteClock(Some(n,),),
                _ => PerpMinuteClock(None,),
            }
        } else {
            PerpMinuteClock(None,)
        }
    }
}

impl fmt::Display for PerpMinuteClock {
    /// Displays as eight hex digits, without the *0x*
    /// prefix, or as out-of-range
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match self.0 {
            Some(x,) => write!(f, "{:08x}", x),
            None => write!(f, "--------"),
        }
    }
}

/// Unix Seconds Clock (*USC*) value
///
/// Optional, unsigned, number of seconds
/// since 1970-01-01T00:00:00
#[derive(Clone, Copy, Debug, Default)]
pub struct UnixSecondsClock(pub Option<i64,>,);
impl UnixSecondsClock {
    /// Makes a zero USC
    pub fn new() -> UnixSecondsClock { UnixSecondsClock(None,) }

    /// Makes a USC from an integer
    pub fn new_from_int(tval: i64) -> UnixSecondsClock {
        UnixSecondsClock(Some(tval,),)
        }

    /// Makes a USC from a decimal string
    pub fn new_from_decimal(dec: &str, offset: &i64) -> UnixSecondsClock {
        let uval: Result<i64, ParseIntError> = dec.parse();
        match uval {
            Ok(n,) => UnixSecondsClock(Some(n-offset,),),
            _ => UnixSecondsClock(None,),
            }
        }
}

impl fmt::Display for UnixSecondsClock {
    /// Displays as decimal digits
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        match self.0 {
            Some(x,) => write!(f, "{:14}", x),
            None => write!(f, "--"),
        }
    }
}

/// TOD Clock value
///
/// Extended TOD clock bits 0-59 padded on the left with 4
/// bits <br/>
/// Equivalent to the *original* TOD clock bits 0-51
/// padded on the left with 12 bits
///
/// An unsigned number of microseconds
#[derive(Clone, Copy, Debug)]
pub struct Tod(pub u64,);
impl Tod {
    /// Makes a new clock from a number of microseconds
    pub fn new(tval: u64) -> Tod { Tod(tval,) }

    /// Makes a new clock from a hex string
    pub fn new_from_hex(hex: &str, pad: &Padding,) -> Option<Tod,> {
        if hex.bytes().all(|b| b.is_ascii_hexdigit(),) {
            let chex = match *pad {
                Padding::Left => ["000000000000000", hex].join("",)[hex.len()..].to_string(),
                Padding::Right => [hex, "000000000000000"].join("",)[..16].to_string(),
                _ => if &hex.to_uppercase().as_bytes()[..1] > b"B" {
                    ["000", hex, "000000000000"].join("",)[..16].to_string()
                } else {
                    ["00", hex, "0000000000000"].join("",)[..16].to_string()
                },
            };
            let tval = u64::from_str_radix(&chex, 16,);
            match tval {
                Ok(n,) => Some(Tod(n,),),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Tod {
    /// Displays as `xxx xxxxxxxx xxxxx---`
    ///
    /// First string is 8-bit epoch index extended by 4
    /// buts on the left; the other two ewpewawnt the
    /// *traditional* 64-bit TOD
    fn fmt(&self, f: &mut fmt::Formatter,) -> fmt::Result {
        let x = format!("{:016x}", self.0);
        write!(f, "{} {} {}---", &x[0..3], &x[3..11], &x[11..16])
    }
}

/// Attempts to parse a date and time string after padding
/// on the left.
///
/// The string "NOW" is interpreted (independent of case)
///  as the current UTC time
/// Anything longer than 3 characters is accepted.
/// If a time is specified,
/// it should be separated from the date by an "@"
/// character
pub fn finddate(ds: String) -> ParseResult<NaiveDateTime,> {
    if ds.to_uppercase() == "NOW" {
        NaiveDateTime::parse_from_str(&defaultdate(), "%F@%H:%M:%S%.f",)
    } else {
        let xlen = ds.len();
        if xlen > 4 && ds.as_bytes()[4] == b'.' {
            let padding = "1900.001@00:00:00.000000";
            let xlen = min(xlen, padding.len(),);
            let x = &padding[xlen..];
            NaiveDateTime::parse_from_str(&(ds + x), "%Y.%j@%H:%M:%S%.f",)
        } else {
            let padding = "1900-01-01@00:00:00.000000";
            let xlen = min(xlen, padding.len(),);
            let x = &padding[xlen..];
            NaiveDateTime::parse_from_str(&(ds + x), "%F@%H:%M:%S%.f",)
        }
    }
}

/// Provides the current UTC date and time as a parseable
/// string
pub fn defaultdate() -> String { Utc::now().format("%F@%H:%M:%S%.6f",).to_string() }

/// Uses a TOD Clock value to calculate the others,
///  with up to three different yime zone offsets
pub fn from_tod(a: &str, todwork: &mut TodInfo,) -> Vec<String,> {
    let todbase = NaiveDate::from_ymd(1900, 1, 1,).and_hms(0, 0, 0,);
    let mut result: Vec<String,> = Vec::new();
    let xtod = Tod::new_from_hex(&a.to_string(), &todwork.pad,);
    todwork.tod = match xtod {
        None => {
            result.push(format!("TOD value is invalid: {:?}", a),);
            return result;
        },
        Some(x,) => x,
    };
    todwork.lsec = todwork.lstab.ls_search_tod(todwork,);
    let x = todbase.checked_add_signed(Duration::microseconds(todwork.tod.0 as i64,),);
    let zdate = match x {
        None => {
            result.push(format!("Can't handle this TOD value: {:?}", a),);
            return result;
        },
        Some(x,) => x,
    };
    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x,) => {
                todwork.date = zdate
                    .checked_add_signed(Duration::seconds(x - todwork.lsec - todwork.tai,),)
                    .expect("Couldn't convert date",);
                todwork.pmc = findpmc(todwork,);
                todwork.usc = findusc(todwork,);
                result.push(todwork.text(off,),);
            },
        };
    }
    result
}

/// Uses a date/time value to calculate the others,
///  with up to three different yime zone offsets
pub fn from_datetime(a: &str, todwork: &mut TodInfo,) -> Vec<String,> {
    let mut result: Vec<String,> = Vec::new();
    let xdt = finddate(a.to_string(),);
    todwork.date = match xdt {
        Err(_,) => {
            result.push(format!("Date {:?} is invalid", a),);
            return result;
        },
        Ok(x,) => x,
    };
    todwork.lsec = todwork.lstab.ls_search_day(todwork,);
    let (zsec, zmic,) = get_sec_mic(todwork,);
    todwork.pmc = findpmc(todwork,);
    todwork.usc = findusc(todwork,);
    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x,) => {
                let x = zsec as i64 + x + todwork.lsec + todwork.tai;
                if x >= 0 {
                    todwork.tod = Tod(x as u64 * 1_000_000 + zmic,);
                    result.push(todwork.text(off,),);
                } else {
                    result.push(format!("Date is out of range: {} {}", a, off),);
                    return result;
                };
            },
        };
    }
    result
}

/// Uses a Perpetial Minute Clock value to calculate the
/// others,  with up to three different time zone offsets
pub fn from_perpetual(a: &str, todwork: &mut TodInfo,) -> Vec<String,> {
    let parsbase = NaiveDate::from_ymd(1966, 1, 3,).and_hms(0, 0, 0,);
    let mut result: Vec<String,> = Vec::new();
    todwork.pmc = PerpMinuteClock::new_from_hex(a,);
    let pmc = match todwork.pmc.0 {
        None => {
            result.push(format!("Minute value is invalid: {:?}", a),);
            return result;
        },
        Some(x,) => x,
    };
    todwork.date = match parsbase.checked_add_signed(Duration::minutes(i64::from(pmc,),),) {
        None => {
            result.push(format!("Can't handle this pmc value: {:?}", a),);
            return result;
        },
        Some(x,) => x,
    };
    todwork.usc = findusc(todwork,);
    todwork.lsec = todwork.lstab.ls_search_day(todwork,);
    let (zsec, zmic,) = get_sec_mic(todwork,);
    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x,) => {
                todwork.tod =
                    Tod((zsec as i64 + x + todwork.lsec + todwork.tai) as u64 * 1_000_000 + zmic,);
                result.push(todwork.text(off,),);
            },
        };
    }
    result
}

/// Uses a Unix Seconds Clock value to calculate the
/// others,  with up to three different time zone offsets
pub fn from_unix(a: &str, todwork: &mut TodInfo, csec: &bool) -> Vec<String,> {
    let unixbase = NaiveDate::from_ymd(1970, 1, 1,).and_hms(0, 0, 0,);
    let csecbase = NaiveDate::from_ymd(1900, 1, 1,).and_hms(0, 0, 0,);
    let mut csecoff = 0 ;
    if *csec {
        csecoff = unixbase.signed_duration_since(csecbase).num_seconds() ;
    };
    let mut result: Vec<String,> = Vec::new();
    todwork.usc = UnixSecondsClock::new_from_decimal(a,&csecoff);
    let tusc = match todwork.usc.0 {
        None => {
            result.push(format!("Seconds value is invalid: {:?}", a),);
            return result;
        },
        Some(x,) => x,
    };
    todwork.date = match unixbase.checked_add_signed(Duration::seconds(i64::from(tusc,),),) {
        None => {
            result.push(format!("Can't handle this pmc value: {:?}", a),);
            return result;
        },
        Some(x,) => x,
    };
    todwork.pmc = findpmc(todwork,);
    todwork.lsec = todwork.lstab.ls_search_day(todwork,);
    let (zsec, zmic,) = get_sec_mic(todwork,);
    let olist = vec![todwork.goff, todwork.loff, todwork.aoff];
    for off in olist {
        match off.0 {
            None => {},
            Some(x,) => {
                todwork.tod =
                    Tod((zsec as i64 + x + todwork.lsec + todwork.tai) as u64 * 1_000_000 + zmic,);
                result.push(todwork.text(off,),);
            },
        };
    }
    result
}

/// Builds a list of values for conversion either from the
/// command line  or optionally from the clipboard
pub fn args_or_elsewhere(cmdl: &ArgMatches) -> Vec<String,> {
    let mut result: Vec<String,> = Vec::new();
    if cmdl.is_present("clipboard",) {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        for item in ctx.get_contents().unwrap().split_whitespace() {
            result.push(item.to_string(),);
        }
        return result
    }

    if cmdl.is_present("infile",) {
        let filename = match cmdl.value_of("infile",) {
            Some(fname,) => fname,
            None => "-",
            };
        let mut rdr: Box<dyn io::Read> = if filename == "-" {
            Box::new(io::stdin())
            } else {
            Box::new(File::open(filename).unwrap())
            };
        let mut buffer = String::new();
        rdr.read_to_string(&mut buffer).unwrap();
        for item in buffer.split_whitespace() {
            result.push(item.to_string(),);
            }
        return result;
    }

    for item in cmdl.values_of("values",).unwrap() {
        result.push(item.to_string(),);
        }
    return result;
    }

/// Calculates a Perpetual Minute Clock from a date and
/// time, or *None* if out-of-range
pub fn findpmc(todwork: &TodInfo) -> PerpMinuteClock {
    let parsbase = NaiveDate::from_ymd(1966, 1, 3,).and_hms(0, 0, 0,);
    let pdiff = todwork.date.signed_duration_since(parsbase,);
    let pmin = pdiff.num_seconds() / 60;
    if pmin >= 0 && pmin <= i64::from(MAX,) {
        PerpMinuteClock(Some(pmin as u32,),)
    } else {
        PerpMinuteClock(None,)
    }
}

/// Calculates a Unix Seconds Clock from a date and
/// time, or *None* if out-of-range
pub fn findusc(todwork: &TodInfo) -> UnixSecondsClock {
    let unixbase = NaiveDate::from_ymd(1970, 1, 1,).and_hms(0, 0, 0,);
    let usec = todwork.date.signed_duration_since(unixbase,).num_seconds();
    UnixSecondsClock(Some(usec,),)
}

/// Converts date and time into seconds and microseconds of
/// the TOD epoch
pub fn get_sec_mic(todwork: &TodInfo) -> (u64, u64,) {
    let todbase = NaiveDate::from_ymd(1900, 1, 1,).and_hms(0, 0, 0,);
    let tdiff = todwork.date.signed_duration_since(todbase,);
    let zsec = tdiff.num_seconds();
    let zmic = (tdiff - Duration::seconds(zsec,))
        .num_microseconds()
        .unwrap() as u64;
    let zsec = zsec as u64;
    (zsec, zmic,)
}
