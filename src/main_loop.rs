use std::io::Read;

const HDR_SIZE: usize = 18;
const NAME_SIZE: usize = 128;
const DATA_START: usize = HDR_SIZE + NAME_SIZE;
const KEEP_ALIVE: &[u8] = &[0xF0, 0x43, 0x7D, 0x60, 0x44, 0x54, 0x41, 0x31, 0xF7];
const KNOB_TURN: &[u8] = &[0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01];
const PRESET_CHANGE: &[u8] = &[0xF0, 0x43, 0x7D, 0x00, 0x02, 0x0C, 0x44, 0x54, 0x41, 0x31, 0x41, 0x6C, 0x6C, 0x50, 0x00, 0x00, 0x7F, 0x7F];

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
    let knob = ::getters::rev_knob(cmd[7]);
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
    println!("amplifier: {}", ::getters::rev_amplifier(cmd[DATA_START + 0]));
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
    println!("cabinet: {}", ::getters::rev_cabinet(cmd[DATA_START + 6]));
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
                                ::sysex::print_sysex(&cmd);
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