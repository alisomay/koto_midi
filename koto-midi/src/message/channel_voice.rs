use crate::impl_midi_message;
use crate::Category;
use crate::MidiMessage;

#[derive(Debug)]
pub struct NoteOff {
    bytes: [u8; 3],
    pub category: Category,
}
impl NoteOff {
    pub fn new(note: u64, velocity: u64, channel: u64) -> Self {
        Self {
            bytes: [
                0x80 | channel.min(15) as u8,
                note.min(127) as u8,
                velocity.min(127) as u8,
            ],
            category: Category::ChannelVoice,
        }
    }
    pub fn note(&self) -> u8 {
        self.bytes[1]
    }
    pub fn velocity(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for NoteOff {
    fn from(raw_bytes: &[u8]) -> Self {
        NoteOff {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelVoice,
        }
    }
}

impl Default for NoteOff {
    fn default() -> Self {
        Self {
            bytes: [0x80, 64, 0],
            category: Category::ChannelVoice,
        }
    }
}

#[derive(Debug)]
pub struct NoteOn {
    bytes: [u8; 3],
    pub category: Category,
}
impl NoteOn {
    pub fn new(note: u64, velocity: u64, channel: u64) -> Self {
        Self {
            bytes: [
                0x90 | channel.min(15) as u8,
                note.min(127) as u8,
                velocity.min(127) as u8,
            ],
            category: Category::ChannelVoice,
        }
    }
    pub fn note(&self) -> u8 {
        self.bytes[1]
    }
    pub fn velocity(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for NoteOn {
    fn from(raw_bytes: &[u8]) -> Self {
        NoteOn {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelVoice,
        }
    }
}

impl Default for NoteOn {
    fn default() -> Self {
        Self {
            bytes: [0x90, 64, 0],
            category: Category::ChannelVoice,
        }
    }
}

#[derive(Debug)]
pub struct PolyAfterTouch {
    bytes: [u8; 3],
    pub category: Category,
}

impl PolyAfterTouch {
    pub fn new(note: u64, pressure: u64, channel: u64) -> Self {
        Self {
            bytes: [
                0xA0 | channel.min(15) as u8,
                note.min(127) as u8,
                pressure.min(127) as u8,
            ],
            category: Category::ChannelVoice,
        }
    }
    pub fn note(&self) -> u8 {
        self.bytes[1]
    }
    pub fn pressure(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for PolyAfterTouch {
    fn from(raw_bytes: &[u8]) -> Self {
        PolyAfterTouch {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelVoice,
        }
    }
}

impl Default for PolyAfterTouch {
    fn default() -> Self {
        Self {
            bytes: [0xA0, 64, 127],
            category: Category::ChannelVoice,
        }
    }
}

#[derive(Debug)]
pub struct ControlChange {
    bytes: [u8; 3],
    pub category: Category,
}

impl ControlChange {
    pub fn new(note: u64, value: u64, channel: u64) -> Self {
        Self {
            bytes: [
                0xB0 | channel.min(15) as u8,
                note.min(127) as u8,
                value.min(127) as u8,
            ],
            category: Category::ChannelVoice,
        }
    }
    pub fn note(&self) -> u8 {
        self.bytes[1]
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for ControlChange {
    fn from(raw_bytes: &[u8]) -> Self {
        ControlChange {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelVoice,
        }
    }
}

impl Default for ControlChange {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 64, 127],
            category: Category::ChannelVoice,
        }
    }
}

#[derive(Debug)]
pub struct ProgramChange {
    bytes: [u8; 2],
    pub category: Category,
}

impl ProgramChange {
    pub fn new(program: u64, channel: u64) -> Self {
        Self {
            bytes: [0xC0 | channel.min(15) as u8, program.min(127) as u8],
            category: Category::ChannelVoice,
        }
    }
    pub fn program(&self) -> u8 {
        self.bytes[1]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for ProgramChange {
    fn from(raw_bytes: &[u8]) -> Self {
        ProgramChange {
            bytes: [raw_bytes[0], raw_bytes[1]],
            category: Category::ChannelVoice,
        }
    }
}

impl Default for ProgramChange {
    fn default() -> Self {
        Self {
            bytes: [0xC0, 0],
            category: Category::ChannelVoice,
        }
    }
}

#[derive(Debug)]
pub struct AfterTouch {
    bytes: [u8; 2],
    pub category: Category,
}

impl AfterTouch {
    pub fn new(pressure: u64, channel: u64) -> Self {
        Self {
            bytes: [0xC0 | channel.min(15) as u8, pressure.min(127) as u8],
            category: Category::ChannelVoice,
        }
    }
    pub fn pressure(&self) -> u8 {
        self.bytes[1]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for AfterTouch {
    fn from(raw_bytes: &[u8]) -> Self {
        AfterTouch {
            bytes: [raw_bytes[0], raw_bytes[1]],
            category: Category::ChannelVoice,
        }
    }
}

impl Default for AfterTouch {
    fn default() -> Self {
        Self {
            bytes: [0xD0, 0],
            category: Category::ChannelVoice,
        }
    }
}

#[derive(Debug)]
pub struct PitchBend {
    bytes: [u8; 3],
    bend_amount: u16,
    pub category: Category,
}

impl PitchBend {
    pub fn new(bend_amount: u64, channel: u64) -> Self {
        let bend_amount = bend_amount.min(16383) as u16;
        let msb = ((bend_amount >> 7) as u8) & 0b0111_1111;
        let lsb = (bend_amount as u8) & 0b0111_1111;
        Self {
            bytes: [0xE0 | channel.min(15) as u8, lsb.min(127), msb.min(127)],
            bend_amount,
            category: Category::ChannelVoice,
        }
    }
    pub fn bend_amount(&self) -> u16 {
        self.bend_amount
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
    pub fn change_bend_amount(&mut self, bend_amount: u16) {
        self.bend_amount = bend_amount.min(16383);
        let msb = ((bend_amount >> 7) as u8) & 0b0111_1111;
        let lsb = (bend_amount as u8) & 0b0111_1111;
        self.bytes[1] = lsb;
        self.bytes[2] = msb;
    }
}

impl From<&[u8]> for PitchBend {
    fn from(raw_bytes: &[u8]) -> Self {
        let bend_amount = ((raw_bytes[2] as u16) << 7) | raw_bytes[1] as u16;
        PitchBend {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            bend_amount,
            category: Category::ChannelVoice,
        }
    }
}

impl Default for PitchBend {
    fn default() -> Self {
        // Middle point
        let bend_amount = 8821_u16;
        let msb = ((bend_amount >> 7) as u8) & 0b0111_1111;
        let lsb = (bend_amount as u8) & 0b0111_1111;

        Self {
            bytes: [0xE0, lsb, msb],
            bend_amount,
            category: Category::ChannelVoice,
        }
    }
}

impl_midi_message!(NoteOff);
impl_midi_message!(NoteOn);
impl_midi_message!(PolyAfterTouch);
impl_midi_message!(ControlChange);
impl_midi_message!(ProgramChange);
impl_midi_message!(AfterTouch);
impl_midi_message!(PitchBend);
