extern crate alsa;

use std::io::Write;

pub fn print_sysex(buf: &[u8]) {
    
    for i in buf {
        print!{" {:02X}", i}
    }
    println!("");
}

pub fn print_rawmidis() {
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

pub fn send_sysex(name: &str, buf: &[u8]) {
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

pub fn send_command(name: &str, knob: &u8, value: &u16, dry: bool) {
    let sysex_set: [u8; 11] = [
        0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01,
        *knob, (value >> 8) as u8, (value & 0xFF) as u8,
        0xF7];

    if dry {
        print_sysex(&sysex_set);
    } else {
        send_sysex(name, &sysex_set);
    }
}
