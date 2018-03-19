// extern crate clap;
// use clap::{Arg, App};
extern crate untod;
use untod::args::*;
use untod::todinfo::*;
extern crate chrono;
use self::chrono::{Utc,Local,Offset};
fn main() {
    let mut todwork = TodInfo{
        runtype: TodCalc::FromTod,
        tod:     0,
        date:    Utc::now(),
        pmc:     0,
        gmt:     true,
        loff:    None,
        aoff:    None,
        pad:     Padding::None,
    };
    let matches = utargs();
    if matches.is_present("reverse") {
        todwork.runtype = TodCalc::FromDateTime;
    }
    if matches.is_present("pmc") {
        todwork.runtype = TodCalc::FromPMC;
    }
    if matches.is_present("pl") {
        todwork.pad = Padding::Left;
    }
    if matches.is_present("pr") {
        todwork.pad = Padding::Right;
    }
    todwork.gmt = !matches.is_present("ng");
    let lzone = matches.value_of("zl"); 
    let azone = matches.value_of("za"); 
    println!("{:?}",matches);   
    println!("{:?}",todwork);   
    println!("{:?}",lzone);   
    let x: i32 = 2;
    println!("{:?}",Local::now().offset().fix().local_minus_utc()+x)
}
