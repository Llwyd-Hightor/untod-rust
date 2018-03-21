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
        tod:     0,
        date:    Utc::now(),
        pmc:     0,
        gmt:     true,
        loff:    0,
        aoff:    0,
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
    todwork.gmt = !cmdline.is_present("ng");

    todwork.loff = match cmdline.value_of("zl") {
        None => {
            match env::var("TODL") {
                Ok(soff) => match soff.parse::<f32>() {
                    Ok(noff) => (60.0 * noff).round() as i32 * 60 ,
                    _ => 0 ,
                },
                _ => Local::now().offset().fix().local_minus_utc() ,
            }
        },
        Some(soff) => match soff.parse::<f32>() {
            Ok(noff) => (60.0 * noff).round() as i32 * 60 ,
            _ => panic!(format!("Invalid offset: --zl {}",soff)) ,
        }
    };

    todwork.aoff = match cmdline.value_of("za") {
        None => {
            match env::var("TODA") {
                Ok(soff) => match soff.parse::<f32>() {
                    Ok(noff) => (60.0 * noff).round() as i32 * 60 ,
                    _ => 0 ,
                },
                _ => 0,
            }
        },
        Some(soff) => match soff.parse::<f32>() {
            Ok(noff) => (60.0 * noff).round() as i32 * 60 ,
            _ => panic!(format!("Invalid offset: --zl {}",soff)) ,
        }
    };
    
    println!("{:?}",todwork.loff);   
    println!("{:?}",todwork.aoff);
    // for a in cmdline.values_of("values").unwrap() {
    //     match Tod::new_from_hex(a,&todwork.pad) {
    //         Some(tod) => println!("{}",tod),
    //         None => println!("Nothing!")
    //     }
    // }
}
