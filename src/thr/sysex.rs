extern crate alsa;

use std::io::Write;

pub fn print_sysex(buf: &[u8]) {
    
    for i in buf {
        print!{" {:02X}", i}
    }
    println!("");
}

pub fn print_rawmidis() {
    for card in alsa::card::Iter::new(){
        match card {
            Ok(card) => {
                match alsa::Ctl::from_card(&card, false) {
                    Ok(ctl) => {
                        for rawmidi in alsa::rawmidi::Iter::new(&ctl) {
                            match rawmidi {
                                Ok(rawmidi) => {
                                    println!("rawmidi {:?} hw:{},{},{} - {} ({})",
                                        rawmidi.get_stream(),
                                        card.get_index(),
                                        rawmidi.get_device(),
                                        rawmidi.get_subdevice(),
                                        card.get_name().unwrap_or("".to_string()),
                                        card.get_longname().unwrap_or("".to_string()));
                                },
                                Err(e) => {
                                    println!("{}", e.to_string());
                                }
                            };
                        }
                    }, Err(e) => {
                        println!("{}", e.to_string());
                    }
                };
            },
            Err(e) => {
                println!("{}", e.to_string());
            }
        };
    }
}

pub fn send_sysex(name: &str, buf: &[u8]) -> Result<(), String> {
    match alsa::rawmidi::Rawmidi::new(name, alsa::Direction::Playback, false) {
        Ok(rawmidi) => {
            let mut writer = rawmidi.io();
            match writer.write(buf) {
                Ok(n) => { 
                    println!("{}: written {} bytes of {}", name, n, buf.len());
                    Ok(())
                },
                Err(e) => {
                    Err(e.to_string())
                }
            }
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub fn send_command(name: &str, knob: &u8, value: &u16, dry: bool) -> Result<(), String> {
    let sysex_set: [u8; 11] = [
        0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01,
        *knob, (value >> 8) as u8, (value & 0xFF) as u8,
        0xF7];

    if dry {
        print_sysex(&sysex_set);
    } else {
        send_sysex(name, &sysex_set)?
    }

    Ok(())
}
