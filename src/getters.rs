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
