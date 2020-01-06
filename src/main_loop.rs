use std::io::Read;

const KEEP_ALIVE : &'static [u8] = &[0xF0, 0x43, 0x7D, 0x60, 0x44, 0x54, 0x41, 0x31, 0xF7];
const KNOB_TURN : &'static [u8] = &[0xF0, 0x43, 0x7D, 0x10, 0x41, 0x30, 0x01];

fn dump_cmd(cmd : &[u8]) {
    for i in cmd {
        print!(" {:02X}", i);
    }
    println!("");
}

fn is_keep_alive(cmd: &[u8]) -> bool {
    return cmd == KEEP_ALIVE;
}

fn is_knob_turn(cmd: &[u8]) -> bool {
    if KNOB_TURN.len() > cmd.len() {
        return false;
    }

    return &cmd[0..KNOB_TURN.len()] == KNOB_TURN;
}

fn dump_knob_turn(cmd : &[u8]) {
    if cmd.len() < 11 {
        return;
    }
    let knob = ::getters::rev_knob(cmd[7]);
    let value : u32 = (cmd[8] as u32) * 10 + (cmd[9] as u32);
    println!("{} = {}", knob, value);
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
                            if is_keep_alive(&cmd)  {
                                // ignore
                            } else if is_knob_turn(&cmd) {
                                dump_knob_turn(&cmd);
                            } else {
                                dump_cmd(&cmd);
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