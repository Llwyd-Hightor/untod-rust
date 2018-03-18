extern crate chrono;
// extern crate time;
use chrono::prelude::*;
use chrono::Duration;
// use time::Duration;

fn main() {
    let xdt = Utc::now();
    let xdo = Utc.ymd(1900,1,1).and_hms(0,0,0);
    let xdu = xdt.signed_duration_since(xdo);
    println!("{:?}",xdu);
    let xsec = xdu.num_seconds();
    let xmic = (xdu - Duration::seconds(xsec))
        .num_microseconds()
        .unwrap() as u64;
    println!("{:?} {:?}",xsec,xmic);   
    let xsec = xsec as u64;
    let tod = format!("{:016x}",xsec * 1_000_000 + xmic);
    println!("{} {} {}---",&tod[0..3],&tod[3..11],&tod[11..16]);
}
