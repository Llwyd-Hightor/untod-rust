extern crate clap;
use self::clap::{App, Arg, ArgMatches};

/// Define and extract the command line arguments
pub fn utargs() -> ArgMatches<'static,> {
    App::new("untod",)
        .version(crate_version!(),)
        .author("Brent Longborough",)
        .about("Converts among Date/Time, TOD, and PARS Perpetual Minute Clock for UTC, TAI or ETR",)
        .before_help("untod: the Swiss Army Chainsaw for the TOD, and other, clocks",)
        .after_help("Default conversion is from hex TOD to UTC with leap-seconds",)
        .arg(
            Arg::with_name("etr",)
                .display_order(3,)
                .help("Ignore leap-seconds -- ETR (IBM/LORAN)",)
                .long("etr",)
                .short("e",)
                .takes_value(false,)
                .conflicts_with("tai",),
        )
        .arg(
            Arg::with_name("tai",)
                .display_order(3,)
                .help("Ignore leap-seconds -- TAI (International Atomic Clock)",)
                .long("tai",)
                .short("t",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("ng",)
                .display_order(5,)
                .help("No Zulu timezone: suppress 0-offset result if others given",)
                .long("zulu",)
                .short("z",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("clipboard",)
                .display_order(2,)
                .help("Get values for conversion from clipboard (default is from command line)",)
                .short("c",)
                .long("clipboard",)
                .conflicts_with("values",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("pl",)
                .display_order(4,)
                .help("Pad Left: pad TOD or Unix time with zeros on left",)
                .long("lpad",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("pr",)
                .display_order(4,)
                .conflicts_with("pl",)
                .help("Pad Right: pad TOD with zeros on right (default is intelligent padding)",)
                .long("rpad",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("zl",)
                .help("Local timezone: override local time offset ([-+]n.n)",)
                .long("lzone",)
                .short("l",)
                .env("UNTOD_LZONE",)
                .takes_value(true,)
                .value_name("offset",),
        )
        .arg(
            Arg::with_name("za",)
                .help("Alternate timezone: specify additional timezone offset ([-+]n.n)",)
                .long("azone",)
                .short("a",)
                .env("UNTOD_AZONE",)
                .takes_value(true,)
                .value_name("offset",),
        )
        .arg(
            Arg::with_name("reverse",)
                .display_order(1,)
                .conflicts_with("pmc",)
                .help("Convert from Date/Time values",)
                .long("date",)
                .short("d",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("pmc",)
                .display_order(1,)
                .help("Convert from Perpetual Minute Clock (hex) values",)
                .short("m",)
                .long("pmc",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("unix",)
                .conflicts_with("pmc",)
                .conflicts_with("reverse",)
                .display_order(1,)
                .help("Convert from Unix Seconds Clock (hex) values",)
                .short("u",)
                .long("unix",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("values",)
                .help("Values for conversion",)
                .takes_value(true,)
                .required_unless("clipboard",)
                .default_value_if("reverse", None, "NOW",)
                .multiple(true,),
        )
        .get_matches()
}
