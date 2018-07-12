extern crate getopts;
extern crate thr;

use thr::sysex::*;
use thr::ydp_interaction::*;
use thr::getters::*;

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.long_only(true);
    opts.optflag("" , "help"             , "print help");
    opts.optflag("" , "rawmidis"         , "print available raw midi controllers");
    opts.optflag("" , "dryrun"           , "do not send sysex to device");
    opts.optopt(""  , "select"           , "select raw midi controller"             , "[hw:?, ?, ?]");
    opts.optopt(""  , "amplifier"        , "set amplifier"                          , "[clean, crunch, lead, brit, modern, bass, aco, flat]");
    opts.optopt(""  , "gain"             , "set gain"                               , "[0-100]");
    opts.optopt(""  , "master"           , "set master"                             , "[0-100]");
    opts.optopt(""  , "bass"             , "set bass"                               , "[0-100]");
    opts.optopt(""  , "middle"           , "set middle"                             , "[0-100]");
    opts.optopt(""  , "treble"           , "set treble"                             , "[0-100]");
    opts.optopt(""  , "cabinet"          , "set cabinet"                            , "[usa4x12, usa2x12, brit4x12, brit2x12, cab1x12, cab4x10]");
    opts.optopt(""  , "file"             , "load file"                              , "[file name]");
    opts.optopt(""  , "gate"             , "set gate"                               , "[on, off]");
    opts.optopt(""  , "gate-thr"         , "set gate threshold"                     , "[0-100]");
    opts.optopt(""  , "gate-rel"         , "set gate release"                       , "[0-100]");
    opts.optopt(""  , "compressor"       , "set compressor"                         , "[on, off]");
    opts.optopt(""  , "comp-type"        , "set compressor type"                    , "[stomp, rack]");
    opts.optopt(""  , "stomp-sus"        , "set compressor stomp sustain"           , "[0-100]");
    opts.optopt(""  , "stomp-out"        , "set compressor stomp output"            , "[0-100]");
    opts.optopt(""  , "rack-thr"         , "set compressor rack threshold"          , "[0-1112]");
    opts.optopt(""  , "rack-att"         , "set compressor rack attack"             , "[0-100]");
    opts.optopt(""  , "rack-rel"         , "set compressor rack release"            , "[0-100]");
    opts.optopt(""  , "rack-ratio"       , "set compressor rack ratio"              , "[1:1, 1:4, 1:8, 1:12, 1:20, 1:inf]");
    opts.optopt(""  , "rack-knee"        , "set compressor rack knee"               , "[soft, medium, hard]");
    opts.optopt(""  , "rack-out"         , "set compressor rack output"             , "[0-1112]");
    opts.optopt(""  , "modulation"       , "set modulation"                         , "[on, off]");
    opts.optopt(""  , "mod-select"       , "set modulation selector"                , "[chorus, flanger, tremolo, phaser]");
    opts.optopt(""  , "chorus-speed"     , "set chorus speed"                       , "[0-100]");
    opts.optopt(""  , "chorus-depth"     , "set chorus depth"                       , "[0-100]");
    opts.optopt(""  , "chorus-mix"       , "set chorus mix"                         , "[0-100]");
    opts.optopt(""  , "flanger-speed"    , "set flanger speed"                      , "[0-100]");
    opts.optopt(""  , "flanger-manual"   , "set flanger manual"                     , "[0-100]");
    opts.optopt(""  , "flanger-depth"    , "set flanger depth"                      , "[0-100]");
    opts.optopt(""  , "flanger-feedback" , "set flanger feedback"                   , "[0-100]");
    opts.optopt(""  , "flanger-spread"   , "set flanger spread"                     , "[0-100]");
    opts.optopt(""  , "tremolo-freq"     , "set tremolo frequency"                  , "[0-100]");
    opts.optopt(""  , "tremolo-depth"    , "set tremolo depth"                      , "[0-100]");
    opts.optopt(""  , "phaser-speed"     , "set phaser speed"                       , "[0-100]");
    opts.optopt(""  , "phaser-manual"    , "set phaser manual"                      , "[0-100]");
    opts.optopt(""  , "phaser-depth"     , "set phaser depth"                       , "[0-100]");
    opts.optopt(""  , "phaser-feedback"  , "set phaser feedback"                    , "[0-100]");
    opts.optopt(""  , "delay"            , "set delay"                              , "[on, off]");
    opts.optopt(""  , "delay-time"       , "set delay time"                         , "[1-19983]");
    opts.optopt(""  , "delay-feedback"   , "set delay feedback"                     , "[0-100]");
    opts.optopt(""  , "delay-hcut"       , "set delay high cut"                     , "[1896-32001]");
    opts.optopt(""  , "delay-lcut"       , "set delay low cut"                      , "[21-15936]");
    opts.optopt(""  , "delay-level"      , "set delay level"                        , "[0-100]");
    opts.optopt(""  , "reverb"           , "set reverb"                             , "[on, off]");
    opts.optopt(""  , "reverb-type"      , "set reverb type"                        , "[room, plate, hall, spring]");
    opts.optopt(""  , "reverb-time"      , "set reverb time"                        , "[3-328]");
    opts.optopt(""  , "reverb-pre"       , "set reverb pre"                         , "[1-3920]");
    opts.optopt(""  , "reverb-lcut"      , "set reverb low cut"                     , "[21-15936]");
    opts.optopt(""  , "reverb-hcut"      , "set reverb high cut"                    , "[1896-32001]");
    opts.optopt(""  , "reverb-hratio"    , "set reverb high ratio"                  , "[1-10]");
    opts.optopt(""  , "reverb-lratio"    , "set reverb low ratio"                   , "[1-14]");
    opts.optopt(""  , "reverb-level"     , "set reverb level"                       , "[0-100]");
    opts.optopt(""  , "spring-reverb"    , "set spring reverb"                      , "[0-100]");
    opts.optopt(""  , "spring-filter"    , "set spring filter"                      , "[0-100]");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
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

    let x = matches.opt_str("amplifier"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("amplifier"), &get_amplifier(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("gain");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("gain"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("master");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("master"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("bass");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("bass"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("middle");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("middle"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("treble");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("treble"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let cabinet = matches.opt_str("cabinet"); 
    match cabinet {
        Some(x) => send_command(device_name.as_ref(), &get_knob("cabinet"), &get_cabinet(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("file"); 
    match x {
        Some(x) => {
            let sysex = load_file(&x);
            if dry {
                print_sysex(&sysex);
            } else {
                send_sysex(device_name.as_ref(), &sysex);
            }
        },
        None => {}
    };

    let x = matches.opt_str("gate"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("gate"), &get_gate(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("gate-thr");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("gate-thr"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let gate_rel = matches.opt_str("gate-rel");
    match gate_rel {
        Some(x) => send_command(device_name.as_ref(), &get_knob("gate-rel"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("compressor"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("compressor"), &get_compressor(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("comp-type"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("comp-type"), &get_compressor_type(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("stomp-sus");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("stomp-sus"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("stomp-out");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("stomp-out"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("rack-thr");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("rack-thr"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("rack-att");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("rack-att"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("rack-rel");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("rack-rel"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("rack-ratio"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("rack-ratio"), &get_ratio(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("rack-knee"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("rack-knee"), &get_knee(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("rack-out");
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("rack-out"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("modulation"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("modulation"), &get_modulation(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("mod-select"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("mod-select"), &get_modulation_selector(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("chorus-speed"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("chorus-speed"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("chorus-depth"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("chorus-depth"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("chorus-mix"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("chorus-mix"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("flanger-speed"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("flanger-speed"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("flanger-manual"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("flanger-manual"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("flanger-depth"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("flanger-depth"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("flanger-feedback"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("flanger-feedback"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("flanger-spread"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("flanger-spread"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("tremolo-freq"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("tremolo-freq"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("tremolo-depth"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("tremolo-depth"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("phaser-speed"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("phaser-speed"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("phaser-manual"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("phaser-manual"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("phaser-depth"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("phaser-depth"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("phaser-feedback"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("phaser-feedback"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("delay"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("delay"), &get_delay(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("delay-time"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("delay-time"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("delay-feedback"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("delay-feedback"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("delay-hcut"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("delay-hcut"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("delay-lcut"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("delay-lcut"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("delay-level"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("delay-level"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb"), &get_reverb(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-type"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-type"), &get_reverb_type(x.as_ref()), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-time"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-time"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-pre"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-pre"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-lcut"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-lcut"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-hcut"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-hcut"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-hratio"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-hratio"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-lratio"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-lratio"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("reverb-level"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("reverb-level"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("spring-reverb"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("spring-reverb"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

    let x = matches.opt_str("spring-filter"); 
    match x {
        Some(x) => send_command(device_name.as_ref(), &get_knob("spring-filter"), &x.parse::<u16>().unwrap(), dry),
        None => {}
    };

}
