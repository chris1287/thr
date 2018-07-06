extern crate alsa;
extern crate getopts;
use std::io::Write;

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
    let rawmidi = 
        match std::fs::File::create("/tmp/out.bin") {
            Ok(rawmidi) => { rawmidi },
            Err(e) => { panic!(e) }
        };

    let mut writer = std::io::BufWriter::new(rawmidi);

    match writer.write(buf) {
        Ok(n) => { println!("{}: written {} bytes of {}", name, n, buf.len()) },
        Err(e) => { panic!(e.to_string()) }
    };
}

fn send_sysex(name: &str, buf: &[u8]) {
    // TODO check input

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

fn send_command(name: &str, knob: &u8, value: &u8, dry: bool) {
    let sysex_set: [u8; 11] = [
        0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01,
        *knob, 0x00, *value,
        0xF7];

    if dry {
        print_sysex(name, &sysex_set);
    } else {
        send_sysex(name, &sysex_set);
    }
}

fn get_amplifier(name: &str) -> u8 {
    match name {
        "clean" => 0,
        "crunch" => 1,
        "lead" => 2,
        "brit" => 3,
        "modern" => 4,
        "bass" => 5,
        "aco" => 6,
        "flat" => 7,
        _ => panic!("unrecognized amplifier: {}", name)
    }
}

fn get_knob(name: &str) -> u8 {
    match name {
        "amplifier" => 0,
        "gain" => 1,
        "master" => 2,
        "bass" => 3,
        "middle" => 4,
        "treble" => 5,
        "cabinet" => 6,
        _ => panic!("unrecognized knob: {}", name)
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();

    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print help");
    opts.optflag("c", "cards", "print available cards");
    opts.optflag("r", "rawmidis", "print available raw midi controllers");
    opts.optflag("d", "dryrun", "do not send sysex to device");
    opts.optopt("s", "select", "select raw midi controller", "hw:?,?,?");
    opts.optopt("a", "amplifier", "set amplifier", "[clean, crunch, lead, brit, modern, bass, aco, flat]");
    opts.optopt("g", "gain", "set gain", "[0-99]");
    opts.optopt("m", "master", "set master", "[0-99]");
    opts.optopt("b", "bass", "set bass", "[0-99]");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("c") {
        print_cards();
    }

    if matches.opt_present("r") {
        print_rawmidis();
    }

    let device_name = matches.opt_str("s");
    let device_name = match device_name {
        Some(x) => x,
        None => String::from("")
    };

    let amplifier = matches.opt_str("a"); 
    match amplifier {
        Some(x) => send_command(device_name.as_ref(), &get_knob("amplifier"), &get_amplifier(x.as_ref()), matches.opt_present("d")),
        None => {}
    };

    let gain = matches.opt_str("g");
    match gain {
        Some(x) => send_command(device_name.as_ref(), &get_knob("gain"), &x.parse::<u8>().unwrap(), matches.opt_present("d")),
        None => {}
    };

    let master = matches.opt_str("m");
    match master {
        Some(x) => send_command(device_name.as_ref(), &get_knob("master"), &x.parse::<u8>().unwrap(), matches.opt_present("d")),
        None => {}
    };

    let bass = matches.opt_str("b");
    match bass {
        Some(x) => send_command(device_name.as_ref(), &get_knob("bass"), &x.parse::<u8>().unwrap(), matches.opt_present("d")),
        None => {}
    };
}
