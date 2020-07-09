extern crate clap;
use self::clap::{App, Arg, ArgMatches};

/// Define and extract the command line arguments
pub fn utargs() -> ArgMatches<'static,> {
    App::new("untod",)
        // .version(crate_version!(),)
        .author("Brent Longborough",)
        .about("Converts among Date/Time, TOD, and PARS Perpetual Minute Clock for UTC, TAI or LORAN/IBM",)
        .before_help("untod: the Swiss Army Chainsaw for the TOD, and other, clocks",)
        .after_help("Default conversion is from hex TOD to UTC with leap-seconds",)
        .arg(
            Arg::with_name("lor",)
                .display_order(30,)
                .help("Ignore leap-seconds -- LORAN/IBM",)
                .long("loran",)
                .short("l",)
                .takes_value(false,)
                .conflicts_with("tai",),
        )
        .arg(
            Arg::with_name("tai",)
                .display_order(30,)
                .help("Ignore leap-seconds -- TAI (International Atomic Clock)",)
                .long("tai",)
                .short("t",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("ng",)
                .display_order(24,)
                .help("No Zulu timezone: suppress 0-offset result if others given",)
                .long("zulu",)
                .short("z",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("clipboard",)
                .display_order(20,)
                .help("Get values for conversion from clipboard",)
                .short("c",)
                .long("clipboard",)
                .conflicts_with_all(&["values","infile"])
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("infile",)
                .display_order(20,)
                .help("Get values for conversion from a file ( - for STDIN )",)
                .short("i",)
                .long("input",)
                .allow_hyphen_values(true)
                .value_name("FILE",),
        )
        .arg(
            Arg::with_name("headers",)
                .display_order(22,)
                .help("Display column headers",)
                .long("headers",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("csv",)
                .display_order(20,)
                .help("Output in CSV format",)
                .long("csv",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("pl",)
                .display_order(40,)
                .help("Pad Left: pad TOD with zeros on left",)
                .long("lpad",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("pr",)
                .display_order(42,)
                .conflicts_with("pl",)
                .help("Pad Right: pad TOD with zeros on right (default is intelligent padding)",)
                .long("rpad",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("za",)
                .display_order(50,)
                .help("Alternate timezone: specify additional timezone offset ([-+]n.n)",)
                .long("azone",)
                .env("UNTOD_AZONE",)
                .value_name("OFFSET",),
        )
        .arg(
            Arg::with_name("zl",)
                .display_order(52,)
                .help("Local timezone: override local time offset ([-+]n.n)",)
                .long("lzone",)
                .env("UNTOD_LZONE",)
                .value_name("OFFSET",),
        )
        .arg(
            Arg::with_name("forward",)
                .display_order(10,)
                .conflicts_with("pmc",)
                .help("Convert from TOD values (Default)",)
                .long("tod",)
                .short("o",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("reverse",)
                .display_order(12,)
                .conflicts_with("pmc",)
                .conflicts_with("forward",)
                .help("Convert from Date/Time values",)
                .long("date",)
                .short("d",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("pmc",)
                .display_order(14,)
                .help("Convert from Perpetual Minute Clock (hex) values",)
                .short("m",)
                .long("pmc",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("unix",)
                .conflicts_with("pmc",)
                .conflicts_with("reverse",)
                .display_order(16,)
                .help("Convert from Unix Seconds Clock values",)
                .short("u",)
                .long("unix",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("csec",)
                .conflicts_with("pmc")
                .conflicts_with("unix")
                .conflicts_with("forward",)
                .conflicts_with("reverse",)
                .display_order(18,)
                .help("Convert from 20th Century Seconds Clock values",)
                .short("s",)
                .long("seconds",)
                .takes_value(false,),
        )
        .arg(
            Arg::with_name("values",)
                .help("Values for conversion (if not from --input or --clipboard)",)
                .value_name("VALUE",)
                .required_unless_one(&["clipboard", "infile"])
                .default_value_if("reverse", None, "NOW",)
                .multiple(true,),
        )
        .get_matches()
}
