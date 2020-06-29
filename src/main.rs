extern crate untod;
use untod::args::utargs;
use untod::todinfo::*;

fn main() {
    let cmdl = utargs();
    let mut todwork = TodInfo::new_from_args(&cmdl,);
    let vlist = args_or_elsewhere(&cmdl,);
    if cmdl.is_present("headers",) {
        if cmdl.is_present("csv",) {
            println!("ExtTOD,Date,Time,Zone,Julian,D,Perp,Unix,Leap");
        } else {
            println!("Ext       TOD              Date          Time        Zone     Julian   D    Perp        Unix      Leap");
            println!("--- ----------------- : ---------- --------------- --------- -------- --- -------- -------------- ----");
        }
    }
    for a in vlist {
        let result: Vec<String,> = match todwork.runtype {
            TodCalc::FromTod => from_tod(&a, &mut todwork,),
            TodCalc::FromDateTime => from_datetime(&a, &mut todwork,),
            TodCalc::FromPMC => from_perpetual(&a, &mut todwork,),
            TodCalc::FromUnix => from_unix(&a, &mut todwork,),
        };
        for line in result {
            println!("{}", line);
        }
    }
}
