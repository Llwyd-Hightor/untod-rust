// extern crate clap ;
// use clap::{Arg, App} ;
extern crate untod ;
use untod::todinfo::* ;
use untod::args::utargs ;
extern crate chrono ;

fn main() {
    let cmdargs = utargs() ;
    let todwork = TodInfo::new_from_args(&cmdargs) ;
    match todwork.runtype {
        TodCalc::FromTod => {
        },
        TodCalc::FromDateTime => {
        },
        TodCalc::FromPMC => {
        },
    }
    println!("{}",&todwork.goff) ;
    println!("{}",&todwork.loff) ;
    println!("{}",&todwork.aoff) ;
    println!("{}",todwork.pmc) ;
    println!("{}",todwork.tod) ;
    println!("{:?}",cmdargs) ;
    println!("{:?}",todwork) ;
//    let vlist = match &cmdargs.values_of("values") {
//    }
    // let vlist = cmdargs.values_of("values") ;
    // let vlist = match vlist {
    //     Some(x) => &x.collect() ,
    //     None => {
    //         let x = Vec::new() ;
    //         x.push(defaultdate()) ; 
    //         &x.iter()
    //     } , 
    // } ;

    let vlist: Vec<&str> = cmdargs.values_of("values").unwrap().collect() ;
    println!("{:?}",vlist) ;

//    for a in vlist {
//        println!{"{} : {:?}",a.to_string(),finddate(a.to_string())}
    //     match Tod::new_from_hex(a,&todwork.pad) {
    //         Some(tod) => println!("{}",tod),
    //         None => println!("Nothing!")
    //     }
//    }
}
