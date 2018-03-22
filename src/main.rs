// extern crate clap;
// use clap::{Arg, App};
extern crate untod;
use untod::args::*;
use untod::todinfo::*;
extern crate chrono;
use self::chrono::{Utc,Local,Offset};
use std::env;

fn main() {
    let mut todwork = TodInfo{
        runtype: TodCalc::FromTod,
        tod:     Tod(0),
        date:    Utc::now(),
        pmc:     ParsDayNo(0),
        goff:    Some(Toffset(0)),
        loff:    Some(Toffset(0)),
        aoff:    Some(Toffset(0)),
        pad:     Padding::None,
    };
    let cmdline = utargs();
    if cmdline.is_present("reverse") {
        todwork.runtype = TodCalc::FromDateTime;
    }
    if cmdline.is_present("pmc") {
        todwork.runtype = TodCalc::FromPMC;
    }
    if cmdline.is_present("pl") {
        todwork.pad = Padding::Left;
    }
    if cmdline.is_present("pr") {
        todwork.pad = Padding::Right;
    }
    if cmdline.is_present("ng") {
        todwork.goff = None;
    };

    todwork.loff = match cmdline.value_of("zl") {
        None => {
            match env::var("TODL") {
                Ok(soff) => match soff.parse::<f32>() {
                    Ok(noff) => Some( Toffset((60.0 * noff).round() as i32 * 60) ),
                    _ => None ,
                },
                _ => Some( Toffset(Local::now().offset().fix().local_minus_utc()) ) ,
            }
        },
        Some(soff) => match soff.parse::<f32>() {
            Ok(noff) => Some( Toffset((60.0 * noff).round() as i32 * 60) ),
            _ => { eprintln!("Invalid offset: --zl {}",soff);
                   None }
        },
    };
    
    todwork.aoff = match cmdline.value_of("za") {
        None => {
            match env::var("TODA") {
                Ok(soff) => match soff.parse::<f32>() {
                    Ok(noff) => Some( Toffset((60.0 * noff).round() as i32 * 60) ),
                    _ => None ,
                },
                _ => None,
            }
        },
        Some(soff) => match soff.parse::<f32>() {
            Ok(noff) => Some( Toffset((60.0 * noff).round() as i32 * 60) ), 
            _ => { eprintln!("Invalid offset: --zl {}",soff);
                   None }
        }
    };
    
    println!("{}",&todwork.goff.unwrap_or(Toffset(0)));   
    println!("{}",&todwork.loff.unwrap_or(Toffset(0)));   
    println!("{}",&todwork.aoff.unwrap_or(Toffset(0)));
    println!("{}",todwork.pmc);   
    println!("{}",todwork.tod);   
    println!("{:?}",todwork);
    
    // for a in cmdline.values_of("values").unwrap() {
    //     match Tod::new_from_hex(a,&todwork.pad) {
    //         Some(tod) => println!("{}",tod),
    //         None => println!("Nothing!")
    //     }
    // }
}
