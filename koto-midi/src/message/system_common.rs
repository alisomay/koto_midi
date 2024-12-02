use crate::impl_midi_message;
use crate::Category;
use crate::MidiMessage;

#[derive(Debug)]
pub struct SystemExclusive {
    bytes: Vec<u8>,
    pub manufacturer_id: Vec<u8>,
    pub category: Category,
}
impl SystemExclusive {
    pub fn new(manufacturer_id: &[u8], message_content: &[u8]) -> Self {
        let mut message = message_content.to_vec();
        message.insert(0, 0xF0);
        if manufacturer_id.len() == 1 {
            message.insert(1, manufacturer_id[0]);
        } else {
            message.insert(1, manufacturer_id[0]);
            message.insert(2, manufacturer_id[1]);
            message.insert(3, manufacturer_id[2]);
        }

        message.push(0xF7);
        Self {
            bytes: message,
            manufacturer_id: manufacturer_id.to_vec(),
            category: Category::SystemCommon,
        }
    }
}

impl From<&[u8]> for SystemExclusive {
    fn from(raw_bytes: &[u8]) -> Self {
        let mut manufacturer_id: Vec<u8> = vec![];
        if raw_bytes[1] != 0 {
            manufacturer_id.push(raw_bytes[1]);
        } else {
            manufacturer_id.push(raw_bytes[1]);
            manufacturer_id.push(raw_bytes[2]);
            manufacturer_id.push(raw_bytes[3]);
        }

        SystemExclusive {
            bytes: raw_bytes.to_vec(),
            manufacturer_id,
            category: Category::SystemCommon,
        }
    }
}

impl Default for SystemExclusive {
    fn default() -> Self {
        Self {
            bytes: vec![0xF0, 0x01, 0x0, 0x0, 0xF7],
            manufacturer_id: vec![0x01],
            category: Category::SystemCommon,
        }
    }
}

#[derive(Debug)]
pub struct TimeCodeQuarterFrame {
    bytes: [u8; 2],
    message_type: u8,
    values: u8,
    pub category: Category,
}

impl TimeCodeQuarterFrame {
    pub fn new(message_type: u64, values: u64) -> Self {
        Self {
            bytes: [
                0xF1,
                (message_type.min(7) << 4) as u8 | values.min(15) as u8,
            ],
            message_type: message_type.min(7) as u8,
            values: values.min(15) as u8,
            category: Category::SystemCommon,
        }
    }
    pub fn message_type(&self) -> u8 {
        self.message_type
    }
    pub fn values(&self) -> u8 {
        self.values
    }

    pub fn change_message_type(&mut self, message_type: u8) {
        self.message_type = message_type;
        self.bytes[1] = (self.message_type.min(7) << 4) | self.values.min(15);
    }
    pub fn change_values(&mut self, values: u8) {
        self.values = values;
        self.bytes[1] = (self.message_type.min(7) << 4) | self.values.min(15);
    }
}

impl From<&[u8]> for TimeCodeQuarterFrame {
    fn from(raw_bytes: &[u8]) -> Self {
        TimeCodeQuarterFrame {
            bytes: [raw_bytes[0], raw_bytes[1]],
            message_type: (raw_bytes[1] & 0b0111_0000) >> 4,
            values: raw_bytes[1] & 0b0000_1111,
            category: Category::SystemCommon,
        }
    }
}

impl Default for TimeCodeQuarterFrame {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct SongPosition {
    bytes: [u8; 3],
    midi_beats_elapsed: u16,
    pub category: Category,
}

impl SongPosition {
    pub fn new(midi_beats_elapsed: u64) -> Self {
        let midi_beats_elapsed = midi_beats_elapsed.min(16383) as u16;
        let msb = ((midi_beats_elapsed >> 7) as u8) & 0b0111_1111;
        let lsb = (midi_beats_elapsed as u8) & 0b0111_1111;
        Self {
            bytes: [0xF2, lsb.min(127), msb.min(127)],
            midi_beats_elapsed,
            category: Category::SystemCommon,
        }
    }
    pub fn midi_beats_elapsed(&self) -> u16 {
        self.midi_beats_elapsed
    }
    pub fn change_midi_beats_elapsed(&mut self, midi_beats_elapsed: u16) {
        self.midi_beats_elapsed = midi_beats_elapsed.min(16383);
        let msb = ((midi_beats_elapsed >> 7) as u8) & 0b0111_1111;
        let lsb = (midi_beats_elapsed as u8) & 0b0111_1111;
        self.bytes[1] = lsb;
        self.bytes[2] = msb;
    }
}

impl From<&[u8]> for SongPosition {
    fn from(raw_bytes: &[u8]) -> Self {
        let midi_beats_elapsed = ((raw_bytes[2] as u16) << 7) | raw_bytes[1] as u16;
        SongPosition {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            midi_beats_elapsed,
            category: Category::SystemCommon,
        }
    }
}

impl Default for SongPosition {
    fn default() -> Self {
        Self {
            bytes: [0xF2, 0, 0],
            midi_beats_elapsed: 0,
            category: Category::SystemCommon,
        }
    }
}

#[derive(Debug)]
pub struct SongSelect {
    bytes: [u8; 2],
    number: u8,
    pub category: Category,
}

impl SongSelect {
    pub fn new(number: u64) -> Self {
        Self {
            bytes: [0xF3, number.min(127) as u8],
            number: number.min(127) as u8,
            category: Category::SystemCommon,
        }
    }
    pub fn number(&self) -> u8 {
        self.number
    }
}

impl From<&[u8]> for SongSelect {
    fn from(raw_bytes: &[u8]) -> Self {
        SongSelect {
            bytes: [raw_bytes[0], raw_bytes[1]],
            number: raw_bytes[1],
            category: Category::SystemCommon,
        }
    }
}

impl Default for SongSelect {
    fn default() -> Self {
        Self {
            bytes: [0xF3, 0],
            number: 0,
            category: Category::SystemCommon,
        }
    }
}

#[derive(Debug)]
pub struct TuneRequest {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for TuneRequest {
    fn default() -> Self {
        TuneRequest {
            bytes: [0xF6],
            category: Category::SystemCommon,
        }
    }
}

impl TuneRequest {
    pub fn new() -> Self {
        TuneRequest::default()
    }
}

impl From<&[u8]> for TuneRequest {
    fn from(raw_bytes: &[u8]) -> Self {
        TuneRequest {
            bytes: [raw_bytes[0]],
            category: Category::SystemCommon,
        }
    }
}

#[derive(Debug)]
pub struct EndOfExclusive {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for EndOfExclusive {
    fn default() -> Self {
        EndOfExclusive {
            bytes: [0xF7],
            category: Category::SystemCommon,
        }
    }
}

impl EndOfExclusive {
    pub fn new() -> Self {
        EndOfExclusive::default()
    }
}

impl From<&[u8]> for EndOfExclusive {
    fn from(raw_bytes: &[u8]) -> Self {
        EndOfExclusive {
            bytes: [raw_bytes[0]],
            category: Category::SystemCommon,
        }
    }
}

impl_midi_message!(SystemExclusive);
impl_midi_message!(TimeCodeQuarterFrame);
impl_midi_message!(SongPosition);
impl_midi_message!(SongSelect);
impl_midi_message!(TuneRequest);
impl_midi_message!(EndOfExclusive);
