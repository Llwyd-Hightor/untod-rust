// extern crate clap ;
// use clap::{Arg, App} ;
extern crate untod ;
use untod::todinfo::* ;
use untod::args::utargs ;
extern crate clipboard ;
use self::clipboard::{ClipboardContext, ClipboardProvider,} ;
//extern crate chrono ;

fn main() {
    let cmdl = utargs() ;
    let mut todwork = TodInfo::new_from_args(&cmdl) ;
    println!("{}",&todwork.goff) ;
    println!("{}",&todwork.loff) ;
    println!("{}",&todwork.aoff) ;
    println!("{}",todwork.pmc) ;
    println!("{}",todwork.tod) ;
    println!("{:?}",cmdl) ;
    println!("todwork: {:?}",todwork) ;

    let mut vlist: Vec<String> = Vec::new() ;
    if cmdl.is_present("clipboard") {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap() ;
        for item in ctx.get_contents().unwrap().split_whitespace() {
            vlist.push(item.to_string()) ;
        } 
    } else {
        for item in cmdl.values_of("values").unwrap() { 
            vlist.push(item.to_string()) ;
        }
    } ;
    
    println!("{:?}",vlist) ;
    
    for a in vlist {
        let result: Vec<String> = match todwork.runtype {
            TodCalc::FromTod => from_tod(a, &mut todwork) ,
            TodCalc::FromDateTime => from_datetime (a, &mut todwork) ,
            TodCalc::FromPMC => from_perpetual (a, &mut todwork) ,
        } ;
        for line in result {
            println!("{:?}",line) ; 
        }
    }
}
