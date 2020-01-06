extern crate getopts;
extern crate thr;

use thr::getters::*;
use thr::sysex::*;
use thr::ydp_interaction::*;

struct Option {
    pub short: &'static str,
    pub long: &'static str,
    pub function: fn(&str) -> u16,
    pub description: &'static str,
    pub domain: &'static str
}

static OPTIONS : &[Option] = &[
    Option {short: "" , long: "amplifier"        , function: get_amplifier           , description: "set amplifier"                 , domain: "[clean; crunch; lead; brit; modern; bass; aco; flat]"},
    Option {short: "" , long: "gain"             , function: get_u16                 , description: "set gain"                      , domain: "[0-100]"},
    Option {short: "" , long: "master"           , function: get_u16                 , description: "set master"                    , domain: "[0-100]"},
    Option {short: "" , long: "bass"             , function: get_u16                 , description: "set bass"                      , domain: "[0-100]"},
    Option {short: "" , long: "middle"           , function: get_u16                 , description: "set middle"                    , domain: "[0-100]"},
    Option {short: "" , long: "treble"           , function: get_u16                 , description: "set treble"                    , domain: "[0-100]"},
    Option {short: "" , long: "cabinet"          , function: get_cabinet             , description: "set cabinet"                   , domain: "[usa4x12; usa2x12; brit4x12; brit2x12; cab1x12; cab4x10]"},
    Option {short: "" , long: "gate"             , function: get_gate                , description: "set gate"                      , domain: "[on; off]"},
    Option {short: "" , long: "gate-thr"         , function: get_u16                 , description: "set gate threshold"            , domain: "[0-100]"},
    Option {short: "" , long: "gate-rel"         , function: get_u16                 , description: "set gate release"              , domain: "[0-100]"},
    Option {short: "" , long: "compressor"       , function: get_compressor          , description: "set compressor"                , domain: "[on; off]"},
    Option {short: "" , long: "comp-type"        , function: get_compressor_type     , description: "set compressor type"           , domain: "[stomp; rack]"},
    Option {short: "" , long: "stomp-sus"        , function: get_u16                 , description: "set compressor stomp sustain"  , domain: "[0-100]"},
    Option {short: "" , long: "stomp-out"        , function: get_u16                 , description: "set compressor stomp output"   , domain: "[0-100]"},
    Option {short: "" , long: "rack-thr"         , function: get_u16                 , description: "set compressor rack threshold" , domain: "[0-1112]"},
    Option {short: "" , long: "rack-att"         , function: get_u16                 , description: "set compressor rack attack"    , domain: "[0-100]"},
    Option {short: "" , long: "rack-rel"         , function: get_u16                 , description: "set compressor rack release"   , domain: "[0-100]"},
    Option {short: "" , long: "rack-ratio"       , function: get_ratio               , description: "set compressor rack ratio"     , domain: "[1:1; 1:4; 1:8; 1:12; 1:20; 1:inf]"},
    Option {short: "" , long: "rack-knee"        , function: get_knee                , description: "set compressor rack knee"      , domain: "[soft; medium; hard]"},
    Option {short: "" , long: "rack-out"         , function: get_u16                 , description: "set compressor rack output"    , domain: "[0-1112]"},
    Option {short: "" , long: "modulation"       , function: get_modulation          , description: "set modulation"                , domain: "[on; off]"},
    Option {short: "" , long: "mod-select"       , function: get_modulation_selector , description: "set modulation selector"       , domain: "[chorus; flanger; tremolo; phaser]"},
    Option {short: "" , long: "chorus-speed"     , function: get_u16                 , description: "set chorus speed"              , domain: "[0-100]"},
    Option {short: "" , long: "chorus-depth"     , function: get_u16                 , description: "set chorus depth"              , domain: "[0-100]"},
    Option {short: "" , long: "chorus-mix"       , function: get_u16                 , description: "set chorus mix"                , domain: "[0-100]"},
    Option {short: "" , long: "flanger-speed"    , function: get_u16                 , description: "set flanger speed"             , domain: "[0-100]"},
    Option {short: "" , long: "flanger-manual"   , function: get_u16                 , description: "set flanger manual"            , domain: "[0-100]"},
    Option {short: "" , long: "flanger-depth"    , function: get_u16                 , description: "set flanger depth"             , domain: "[0-100]"},
    Option {short: "" , long: "flanger-feedback" , function: get_u16                 , description: "set flanger feedback"          , domain: "[0-100]"},
    Option {short: "" , long: "flanger-spread"   , function: get_u16                 , description: "set flanger spread"            , domain: "[0-100]"},
    Option {short: "" , long: "tremolo-freq"     , function: get_u16                 , description: "set tremolo frequency"         , domain: "[0-100]"},
    Option {short: "" , long: "tremolo-depth"    , function: get_u16                 , description: "set tremolo depth"             , domain: "[0-100]"},
    Option {short: "" , long: "phaser-speed"     , function: get_u16                 , description: "set phaser speed"              , domain: "[0-100]"},
    Option {short: "" , long: "phaser-manual"    , function: get_u16                 , description: "set phaser manual"             , domain: "[0-100]"},
    Option {short: "" , long: "phaser-depth"     , function: get_u16                 , description: "set phaser depth"              , domain: "[0-100]"},
    Option {short: "" , long: "phaser-feedback"  , function: get_u16                 , description: "set phaser feedback"           , domain: "[0-100]"},
    Option {short: "" , long: "delay"            , function: get_delay               , description: "set delay"                     , domain: "[on; off]"},
    Option {short: "" , long: "delay-time"       , function: get_u16                 , description: "set delay time"                , domain: "[1-19983]"},
    Option {short: "" , long: "delay-feedback"   , function: get_u16                 , description: "set delay feedback"            , domain: "[0-100]"},
    Option {short: "" , long: "delay-hcut"       , function: get_u16                 , description: "set delay high cut"            , domain: "[1896-32001]"},
    Option {short: "" , long: "delay-lcut"       , function: get_u16                 , description: "set delay low cut"             , domain: "[21-15936]"},
    Option {short: "" , long: "delay-level"      , function: get_u16                 , description: "set delay level"               , domain: "[0-100]"},
    Option {short: "" , long: "reverb"           , function: get_reverb              , description: "set reverb"                    , domain: "[on; off]"},
    Option {short: "" , long: "reverb-type"      , function: get_reverb_type         , description: "set reverb type"               , domain: "[room; plate; hall; spring]"},
    Option {short: "" , long: "reverb-time"      , function: get_u16                 , description: "set reverb time"               , domain: "[3-328]"},
    Option {short: "" , long: "reverb-pre"       , function: get_u16                 , description: "set reverb pre"                , domain: "[1-3920]"},
    Option {short: "" , long: "reverb-lcut"      , function: get_u16                 , description: "set reverb low cut"            , domain: "[21-15936]"},
    Option {short: "" , long: "reverb-hcut"      , function: get_u16                 , description: "set reverb high cut"           , domain: "[1896-32001]"},
    Option {short: "" , long: "reverb-hratio"    , function: get_u16                 , description: "set reverb high ratio"         , domain: "[1-10]"},
    Option {short: "" , long: "reverb-lratio"    , function: get_u16                 , description: "set reverb low ratio"          , domain: "[1-14]"},
    Option {short: "" , long: "reverb-level"     , function: get_u16                 , description: "set reverb level"              , domain: "[0-100]"},
    Option {short: "" , long: "spring-reverb"    , function: get_u16                 , description: "set spring reverb"             , domain: "[0-100]"},
    Option {short: "" , long: "spring-filter"    , function: get_u16                 , description: "set spring filter"             , domain: "[0-100]"}
];

fn get_opts() -> getopts::Options {
    let mut opts = getopts::Options::new();
    opts.long_only(true);
    opts.optflag("" , "help"             , "print help");
    opts.optflag("" , "rawmidis"         , "print available raw midi controllers");
    opts.optflag("" , "dryrun"           , "do not send sysex to device");
    opts.optopt(""  , "select"           , "select raw midi controller"             , "[hw:?,?,?]");
    opts.optopt(""  , "file"             , "load file"                              , "[file name]");
    opts.optflag("" , "monitor"          , "monitor THR activity");

    for o in OPTIONS {
        opts.optopt(o.short, o.long, o.description, o.domain);
    }

    opts
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let opts = get_opts();

    match opts.parse(&args) {
        Ok(matches) => { 
            if matches.opt_present("help") {
                print_usage("thr", opts);
                return;
            }

            if matches.opt_present("rawmidis") {
                print_rawmidis();
            }

            let device_name = matches.opt_str("select");
            let device_name = match device_name {
                Some(x) => x,
                None => String::from("")
            };

            let dry = matches.opt_present("dryrun");

            if matches.opt_present("monitor") {
                match thr::main_loop::start(device_name.as_ref()) {
                    Ok(_) => {},
                    Err(_) => {}
                };
            }

            let cmd = matches.opt_str("file"); 
            if let Some(x) = cmd {
                match load_file(&x) {
                    Some(sysex) => {
                        if dry {
                            print_sysex(&sysex);
                        } else {
                            match send_sysex(device_name.as_ref(), &sysex) {
                                Ok(()) => {},
                                Err(e) => { println!("{}", e); }
                            }
                        }
                    },
                    None => {
                        println!("invalid file");
                    }
                };
            };

            for o in OPTIONS {
                let cmd = matches.opt_str(o.long); 
                if let Some(x) = cmd {
                    match send_command(device_name.as_ref(), &get_knob(o.long), &(o.function)(&x), dry) {
                        Ok(()) => {},
                        Err(e) => { println!("{}", e); }
                    };
                }
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    };
}
