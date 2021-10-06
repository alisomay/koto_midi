use crate::impl_midi_message;
use crate::Category;
use crate::MidiMessage;

#[derive(Debug)]
pub struct AllSoundOff {
    bytes: [u8; 3],
    pub category: Category,
}

impl AllSoundOff {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 120, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for AllSoundOff {
    fn from(raw_bytes: &[u8]) -> Self {
        AllSoundOff {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for AllSoundOff {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 120, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct ResetAllControllers {
    bytes: [u8; 3],
    pub category: Category,
}

impl ResetAllControllers {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 121, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for ResetAllControllers {
    fn from(raw_bytes: &[u8]) -> Self {
        ResetAllControllers {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for ResetAllControllers {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 121, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct LocalControl {
    bytes: [u8; 3],
    pub category: Category,
}

impl LocalControl {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 122, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for LocalControl {
    fn from(raw_bytes: &[u8]) -> Self {
        LocalControl {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for LocalControl {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 122, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct AllNotesOff {
    bytes: [u8; 3],
    pub category: Category,
}

impl AllNotesOff {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 123, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for AllNotesOff {
    fn from(raw_bytes: &[u8]) -> Self {
        AllNotesOff {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for AllNotesOff {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 123, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct OmniModeOff {
    bytes: [u8; 3],
    pub category: Category,
}

impl OmniModeOff {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 124, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for OmniModeOff {
    fn from(raw_bytes: &[u8]) -> Self {
        OmniModeOff {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for OmniModeOff {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 124, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct OmniModeOn {
    bytes: [u8; 3],
    pub category: Category,
}

impl OmniModeOn {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 125, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for OmniModeOn {
    fn from(raw_bytes: &[u8]) -> Self {
        OmniModeOn {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for OmniModeOn {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 125, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct MonoModeOn {
    bytes: [u8; 3],
    pub category: Category,
}

impl MonoModeOn {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 126, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for MonoModeOn {
    fn from(raw_bytes: &[u8]) -> Self {
        MonoModeOn {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for MonoModeOn {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 126, 127],
            category: Category::ChannelMode,
        }
    }
}

#[derive(Debug)]
pub struct PolyModeOn {
    bytes: [u8; 3],
    pub category: Category,
}

impl PolyModeOn {
    pub fn new(value: u64, channel: u64) -> Self {
        Self {
            bytes: [0xB0 | channel.min(15) as u8, 127, value.min(127) as u8],
            category: Category::ChannelMode,
        }
    }
    pub fn value(&self) -> u8 {
        self.bytes[2]
    }
    pub fn channel(&self) -> u8 {
        self.bytes[0] & 0x0F
    }
}

impl From<&[u8]> for PolyModeOn {
    fn from(raw_bytes: &[u8]) -> Self {
        PolyModeOn {
            bytes: [raw_bytes[0], raw_bytes[1], raw_bytes[2]],
            category: Category::ChannelMode,
        }
    }
}

impl Default for PolyModeOn {
    fn default() -> Self {
        Self {
            bytes: [0xB0, 127, 127],
            category: Category::ChannelMode,
        }
    }
}

impl_midi_message!(AllSoundOff);
impl_midi_message!(ResetAllControllers);
impl_midi_message!(LocalControl);
impl_midi_message!(AllNotesOff);
impl_midi_message!(OmniModeOff);
impl_midi_message!(OmniModeOn);
impl_midi_message!(MonoModeOn);
impl_midi_message!(PolyModeOn);
