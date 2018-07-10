extern crate alsa;
extern crate getopts;

use std::io::Write;
use std::fs::File;
use std::io::prelude::Read;

fn print_cards() {
    let card_iterator = alsa::card::Iter::new();
    for card in card_iterator.map(|card| card.unwrap()){
        println!("Card#{}: {} ({})", card.get_index(), card.get_name().unwrap(), card.get_longname().unwrap());
    }
}

fn print_rawmidis() {
    let card_iterator = alsa::card::Iter::new();
    for card in card_iterator.map(|card| card.unwrap()){
        let ctl = alsa::Ctl::from_card(&card, false).unwrap();
        let rawmidi_iterator = alsa::rawmidi::Iter::new(&ctl);
        for rawmidi in rawmidi_iterator.map(|rawmidi| rawmidi.unwrap()) {
            println!("rawmidi {:?} hw:{},{},{} - {} ({})",
                rawmidi.get_stream(),
                card.get_index(),
                rawmidi.get_device(),
                rawmidi.get_subdevice(),
                card.get_name().unwrap(),
                card.get_longname().unwrap()
            );
        }
    }
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_sysex(name: &str, buf: &[u8]) {
    
    print!("{}:", name);
    for i in buf {
        print!{" {:02X}", i}
    }
    println!("");
}

fn send_sysex(name: &str, buf: &[u8]) {
    let rawmidi = 
        match alsa::rawmidi::Rawmidi::new(name, alsa::Direction::Playback, false) {
            Ok(rawmidi) => { rawmidi },
            Err(e) => { panic!(e) }
        };

    let mut writer = rawmidi.io();

    match writer.write(buf) {
        Ok(n) => { println!("{}: written {} bytes of {}", name, n, buf.len()) },
        Err(e) => { panic!(e.to_string()) }
    };
}

fn send_command(name: &str, knob: &u8, value: &u16, dry: bool) {
    let sysex_set: [u8; 11] = [
        0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01,
        *knob, (value >> 8) as u8, (value & 0xFF) as u8,
        0xF7];

    if dry {
        print_sysex(name, &sysex_set);
    } else {
        send_sysex(name, &sysex_set);
    }
}

fn get_amplifier(name: &str) -> u16 {
    match name {
        "clean" => 0x00,
        "crunch" => 0x01,
        "lead" => 0x02,
        "brit" => 0x03,
        "modern" => 0x04,
        "bass" => 0x05,
        "aco" => 0x06,
        "flat" => 0x07,
        _ => panic!("unrecognized amplifier: {}", name)
    }
}

fn get_knob(name: &str) -> u8 {
    match name {
        "amplifier"        => 0x00,
        "gain"             => 0x01,
        "master"           => 0x02,
        "bass"             => 0x03,
        "middle"           => 0x04,
        "treble"           => 0x05,
        "cabinet"          => 0x06,
        "gate"             => 0x5F,
        "gate-thr"         => 0x51,
        "gate-rel"         => 0x52,
        "compressor"       => 0x1F,
        "comp-type"        => 0x10,
        "stomp-sus"        => 0x11,
        "stomp-out"        => 0x12,
        "rack-thr"         => 0x11,
        "rack-att"         => 0x13,
        "rack-rel"         => 0x14,
        "rack-ratio"       => 0x15,
        "rack-knee"        => 0x16,
        "rack-out"         => 0x17,
        "modulation"       => 0x2F,
        "mod-select"       => 0x20,
        "chorus-speed"     => 0x21,
        "chorus-depth"     => 0x22,
        "chorus-mix"       => 0x23,
        "flanger-speed"    => 0x21,
        "flanger-manual"   => 0x22,
        "flanger-depth"    => 0x23,
        "flanger-feedback" => 0x24,
        "flanger-spread"   => 0x25,
        "tremolo-freq"     => 0x21,
        "tremolo-depth"    => 0x22,
        "phaser-speed"     => 0x21,
        "phaser-manual"    => 0x22,
        "phaser-depth"     => 0x23,
        "phaser-feedback"  => 0x24,
        "delay"            => 0x3F,
        "delay-time"       => 0x31,
        "delay-feedback"   => 0x33,
        "delay-hcut"       => 0x34,
        "delay-lcut"       => 0x36,
        "delay-level"      => 0x38,
        _ => panic!("unrecognized knob: {}", name)
    }
}

fn get_cabinet(name: &str) -> u16 {
    match name {
        "usa4x12" => 0x00,
        "usa2x12" => 0x01,
        "brit4x12" => 0x02,
        "brit2x12" => 0x03,
        "cab1x12" => 0x04,
        "cab4x10" => 0x05,
        _ => panic!("unrecognized cabinet: {}", name)
    }
}

fn get_compressor(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => panic!("unrecognized compressor: {}", name)
    }
}

fn get_compressor_type(name: &str) -> u16 {
    match name {
        "stomp" => 0x00,
        "rack" => 0x01,
        _ => panic!("unrecognized compressor type: {}", name)
    }
}

fn get_gate(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => panic!("unrecognized gate: {}", name)
    }
}

fn get_knee(name: &str) -> u16 {
    match name {
        "soft" => 0x00,
        "medium" => 0x01,
        "hard" => 0x02,
        _ => panic!("unrecognized knee: {}", name)
    }
}

fn get_ratio(name: &str) -> u16 {
    match name {
        "1:1" => 0x00,
        "1:4" => 0x01,
        "1:8" => 0x02,
        "1:12" => 0x03,
        "1:20" => 0x04,
        "1:inf" => 0x05,
        _ => panic!("unrecognized ratio: {}", name)
    }
}

fn get_modulation(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => panic!("unrecognized modulation: {}", name)
    }
}

fn get_modulation_selector(name: &str) -> u16 {
    match name {
        "chorus" => 0x00,
        "flanger" => 0x01,
        "tremolo" => 0x02,
        "phaser" => 0x03,
        _ => panic!("unrecognized modulation selector: {}", name)
    }
}

fn get_delay(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => panic!("unrecognized delay: {}", name)
    }
}

fn load_file(device_name: &str, file_name: &str, dry: bool) {
    let header: [u8; 18] = [
        0xF0, 0x43, 0x7D, 0x00,
        0x02,
        0x0C,
        b'D', b'T', b'A', b'1', b'A', b'l', b'l', b'P', 0x00, 0x00, 0x7F, 0x7F
    ];

    let mut file_content = [0; 265];

    let mut file_handler = match File::open(file_name) {
        Ok(file_handler) => file_handler,
        Err(e) => panic!(e.to_string())
    };

    match file_handler.read(&mut file_content) {
        Ok(x) => println!("read {} bytes from {}", x, file_name),
        Err(e) => panic!(e)
    };

    let file_content = &file_content[9..];

    let hcrc: u32 = header[6..].iter().map(|&x| x as u32).sum();
    let fcrc: u32 = file_content.iter().map(|&x| x as u32).sum();
    let mut crc: u32 = hcrc + fcrc;
    crc = (!crc + 1) & 0x7F;

    let mut sysex:Vec<u8> = Vec::new();
    sysex.extend_from_slice(&header);
    sysex.extend_from_slice(&file_content);
    sysex.push(crc as u8);
    sysex.push(0xF7);

    if dry {
        println!("{} {}", device_name, file_name);

        for i in sysex.iter() {
            print!{" {:02X}", i}
        }
        println!("");
    } else {
        send_sysex(&device_name, &sysex);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.long_only(true);
    opts.optflag("" , "help"             , "print help");
    opts.optflag("" , "cards"            , "print available cards");
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
    opts.optopt(""  , "delay-lcut"       , "set delay low cut"                      , "[31-15936]");
    opts.optopt(""  , "delay-level"      , "set delay level"                        , "[0-100]");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("cards") {
        print_cards();
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
        Some(x) => load_file(device_name.as_ref(), &x, dry),
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

}
