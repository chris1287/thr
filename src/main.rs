extern crate alsa;
extern crate getopts;

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

fn main() {
    let args : Vec<String> = std::env::args().collect();

    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print help");
    opts.optflag("c", "cards", "print available cards");
    opts.optflag("r", "rawmidis", "print available raw midi controllers");

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
}
