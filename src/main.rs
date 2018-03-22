// extern crate clap ;
// use clap::{Arg, App} ;
extern crate untod ;
use untod::todinfo::* ;
use untod::args::utargs ;
extern crate chrono ;

fn main() {
    let mut todwork = TodInfo::new_from_args(utargs()) ;
    println!("{}",&todwork.goff) ;
    println!("{}",&todwork.loff) ;
    println!("{}",&todwork.aoff) ;
    println!("{}",todwork.pmc) ;
    println!("{}",todwork.tod) ;
    println!("{:?}",todwork) ;

    // for a in cmdline.values_of("values").unwrap() {
    //     match Tod::new_from_hex(a,&todwork.pad) {
    //         Some(tod) => println!("{}",tod),
    //         None => println!("Nothing!")
    //     }
    // }
}
