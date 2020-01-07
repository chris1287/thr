extern crate alsa;

use std::fs::File;
use std::io::Read;
use std::io::Write;

const HDR_SIZE: usize = 18;
const NAME_SIZE: usize = 128;
const DATA_START: usize = HDR_SIZE + NAME_SIZE;
const KEEP_ALIVE: &[u8] = &[0xF0, 0x43, 0x7D, 0x60, 0x44, 0x54, 0x41, 0x31, 0xF7];
const KNOB_TURN: &[u8] = &[0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01];
const PRESET_CHANGE: &[u8] = &[0xF0, 0x43, 0x7D, 0x00, 0x02, 0x0C, 0x44, 0x54, 0x41, 0x31, 0x41, 0x6C, 0x6C, 0x50, 0x00, 0x00, 0x7F, 0x7F];

#[no_mangle]
pub extern fn load_file(file_name: &str) -> Option<Vec<u8>> {
    let header: [u8; 18] = [
        0xF0, 0x43, 0x7D, 0x00,
        0x02,
        0x0C,
        b'D', b'T', b'A', b'1', b'A', b'l', b'l', b'P', 0x00, 0x00, 0x7F, 0x7F
    ];

    let mut sysex:Vec<u8> = Vec::new();

    let mut file_content = [0; 265];

    let mut file_handler = match File::open(file_name) {
        Ok(file_handler) => file_handler,
        Err(e) => {
            println!("{}", e.to_string());
            return None;
        }
    };

    match file_handler.read(&mut file_content) {
        Ok(x) => println!("read {} bytes from {}", x, file_name),
        Err(e) => {
            println!("{}", e.to_string());
            return None;
        }
    };

    let file_content = &file_content[9..];

    let hcrc: u32 = header[6..].iter().map(|&x| x as u32).sum();
    let fcrc: u32 = file_content.iter().map(|&x| x as u32).sum();
    let mut crc: u32 = hcrc + fcrc;
    crc = (!crc + 1) & 0x7F;

    sysex.extend_from_slice(&header);
    sysex.extend_from_slice(&file_content);
    sysex.push(crc as u8);
    sysex.push(0xF7);

    Some(sysex)
}

#[no_mangle]
pub extern fn print_rawmidis() {
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
                                        card.get_name().unwrap_or_else(|_| "".to_string()),
                                        card.get_longname().unwrap_or_else(|_| "".to_string()));
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

#[no_mangle]
pub extern fn send_sysex(name: &str, buf: &[u8]) -> Result<(), String> {
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

#[no_mangle]
pub extern fn send_command(name: &str, knob: &u8, value: &u16, dry: bool) -> Result<(), String> {
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

#[no_mangle]
pub extern fn start(name: &str) -> Result<(), String> {
    match alsa::rawmidi::Rawmidi::new(name, alsa::Direction::Capture, false) {
        Ok(rawmidi) => {
            let mut handler = rawmidi.io();
            let mut cmd = Vec::new();
            loop {
                let mut buffer : [u8; 1] = [0x00];
                match handler.read_exact(&mut buffer) {
                    Ok(_) => {
                        cmd.push(buffer[0]);
                        if buffer[0] == 0xF7 {
                            if is_cmd(&cmd, &KEEP_ALIVE)  {
                                // ignore
                            } else if is_cmd(&cmd, &KNOB_TURN) {
                                dump_knob_turn(&cmd);
                            } else if is_cmd(&cmd, PRESET_CHANGE) {
                                dump_preset_name(&cmd);
                                dump_amplifier(&cmd);
                                dump_gain(&cmd);
                                dump_master(&cmd);
                                dump_bass(&cmd);
                                dump_middle(&cmd);
                                dump_treble(&cmd);
                                dump_cabinet(&cmd);
                            }
                            else {
                                print_sysex(&cmd);
                            }
                            cmd.clear();
                        }
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    } 
                };
            }
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

pub fn print_sysex(buf: &[u8]) {
    for i in buf {
        print!{" {:02X}", i}
    }
    println!();
}

pub fn get_u16(s: &str) -> u16 {
    s.parse::<u16>().unwrap_or(0)
}

pub fn get_amplifier(name: &str) -> u16 {
    match name {
        "clean" => 0x00,
        "crunch" => 0x01,
        "lead" => 0x02,
        "brit" => 0x03,
        "modern" => 0x04,
        "bass" => 0x05,
        "aco" => 0x06,
        "flat" => 0x07,
        _ => 0x00
    }
}

pub fn rev_amplifier(value: u8) -> &'static str {
    match value {
        0x00 => "clean",
        0x01 => "crunch",
        0x02 => "lead",
        0x03 => "brit",
        0x04 => "modern",
        0x05 => "bass",
        0x06 => "aco",
        0x07 => "flat",
        _ => ""
    }
}

pub fn get_knob(name: &str) -> u8 {
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
        "reverb"           => 0x4F,
        "reverb-type"      => 0x40,
        "reverb-time"      => 0x41,
        "reverb-pre"       => 0x43,
        "reverb-lcut"      => 0x45,
        "reverb-hcut"      => 0x47,
        "reverb-hratio"    => 0x49,
        "reverb-lratio"    => 0x4A,
        "reverb-level"     => 0x4B,
        "spring-reverb"    => 0x41,
        "spring-filter"    => 0x42,
        _ => 0x00
    }
}

pub fn rev_knob(value: u8) -> &'static str {
    match value {
        0x00 => "amplifier",
        0x01 => "gain",
        0x02 => "master",
        0x03 => "bass",
        0x04 => "middle",
        0x05 => "treble",
        0x06 => "cabinet",
        0x5F => "gate",
        0x51 => "gate-thr",
        0x52 => "gate-rel",
        0x1F => "compressor",
        0x10 => "comp-type",
        0x11 => "stomp-sus",
        0x12 => "stomp-out",
        0x13 => "rack-att",
        0x14 => "rack-rel",
        0x15 => "rack-ratio",
        0x16 => "rack-knee",
        0x17 => "rack-out",
        0x2F => "modulation",
        0x20 => "mod-select",
        0x21 => "chorus-speed",
        0x22 => "chorus-depth",
        0x23 => "chorus-mix",
        0x24 => "flanger-feedback",
        0x25 => "flanger-spread",
        0x3F => "delay",
        0x31 => "delay-time",
        0x33 => "delay-feedback",
        0x34 => "delay-hcut",
        0x36 => "delay-lcut",
        0x38 => "delay-level",
        0x4F => "reverb",
        0x40 => "reverb-type",
        0x41 => "reverb-time",
        0x43 => "reverb-pre",
        0x45 => "reverb-lcut",
        0x47 => "reverb-hcut",
        0x49 => "reverb-hratio",
        0x4A => "reverb-lratio",
        0x4B => "reverb-level",
        0x42 => "spring-filter",
        _ => ""
    }
}

pub fn get_cabinet(name: &str) -> u16 {
    match name {
        "usa4x12" => 0x00,
        "usa2x12" => 0x01,
        "brit4x12" => 0x02,
        "brit2x12" => 0x03,
        "cab1x12" => 0x04,
        "cab4x10" => 0x05,
        _ => 0x00
    }
}

pub fn rev_cabinet(value: u8) -> &'static str {
    match value {
        0x00 => "usa4x12",
        0x01 => "usa2x12",
        0x02 => "brit4x12",
        0x03 => "brit2x12",
        0x04 => "cab1x12",
        0x05 => "cab4x10",
        _ => ""
    }
}

pub fn get_compressor(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => 0x00
    }
}

pub fn get_compressor_type(name: &str) -> u16 {
    match name {
        "stomp" => 0x00,
        "rack" => 0x01,
        _ => 0x00
    }
}

pub fn get_gate(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => 0x00
    }
}

pub fn get_knee(name: &str) -> u16 {
    match name {
        "soft" => 0x00,
        "medium" => 0x01,
        "hard" => 0x02,
        _ => 0x00
    }
}

pub fn get_ratio(name: &str) -> u16 {
    match name {
        "1:1" => 0x00,
        "1:4" => 0x01,
        "1:8" => 0x02,
        "1:12" => 0x03,
        "1:20" => 0x04,
        "1:inf" => 0x05,
        _ => 0x00
    }
}

pub fn get_modulation(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => 0x00
    }
}

pub fn get_modulation_selector(name: &str) -> u16 {
    match name {
        "chorus" => 0x00,
        "flanger" => 0x01,
        "tremolo" => 0x02,
        "phaser" => 0x03,
        _ => 0x00
    }
}

pub fn get_delay(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => 0x00
    }
}

pub fn get_reverb(name: &str) -> u16 {
    match name {
        "on" => 0x00,
        "off" => 0x7F,
        _ => 0x00
    }
}

pub fn get_reverb_type(name: &str) -> u16 {
    match name {
        "room" => 0x01,
        "plate" => 0x02,
        "hall" => 0x00,
        "spring" => 0x03,
        _ => 0x00
    }
}

fn is_cmd(cmd: &[u8], cmd_to_check: &[u8]) -> bool {
    if cmd_to_check.len() > cmd.len() {
        return false;
    }

    &cmd[0..cmd_to_check.len()] == cmd_to_check
}

fn dump_knob_turn(cmd: &[u8]) {
    if cmd.len() < 11 {
        return;
    }
    let knob = rev_knob(cmd[7]);
    let value : u32 = (cmd[8] as u32) * 10 + (cmd[9] as u32);
    println!("{} = {}", knob, value);
}

fn dump_preset_name(cmd: &[u8]) {
    let mut name = String::new();
    for c in &cmd[HDR_SIZE..NAME_SIZE + 1] {
        if *c != 0u8 {
            name.push(*c as char);
        } else {
            break;
        }
    }

    println!("preset name: {}", name);
}

fn dump_amplifier(cmd: &[u8]) {
    println!("amplifier: {}", rev_amplifier(cmd[DATA_START + 0]));
}

fn dump_gain(cmd: &[u8]) {
    println!("gain: {}", cmd[DATA_START + 1]);
}

fn dump_master(cmd: &[u8]) {
    println!("master: {}", cmd[DATA_START + 2]);
}

fn dump_bass(cmd: &[u8]) {
    println!("bass: {}", cmd[DATA_START + 3]);
}

fn dump_middle(cmd: &[u8]) {
    println!("middle: {}", cmd[DATA_START + 4]);
}

fn dump_treble(cmd: &[u8]) {
    println!("treble: {}", cmd[DATA_START + 5]);
}

fn dump_cabinet(cmd: &[u8]) {
    println!("cabinet: {}", rev_cabinet(cmd[DATA_START + 6]));
}

