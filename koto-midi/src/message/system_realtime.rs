use crate::impl_midi_message;
use crate::Category;
use crate::MidiMessage;

#[derive(Debug)]
pub struct TimingClock {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for TimingClock {
    fn default() -> Self {
        TimingClock {
            bytes: [0xF8],
            category: Category::SystemRealtime,
        }
    }
}

impl TimingClock {
    pub fn new() -> Self {
        TimingClock::default()
    }
}

impl From<&[u8]> for TimingClock {
    fn from(raw_bytes: &[u8]) -> Self {
        TimingClock {
            bytes: [raw_bytes[0]],
            category: Category::SystemRealtime,
        }
    }
}

#[derive(Debug)]
pub struct Start {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for Start {
    fn default() -> Self {
        Start {
            bytes: [0xFA],
            category: Category::SystemRealtime,
        }
    }
}

impl Start {
    pub fn new() -> Self {
        Start::default()
    }
}

impl From<&[u8]> for Start {
    fn from(raw_bytes: &[u8]) -> Self {
        Start {
            bytes: [raw_bytes[0]],
            category: Category::SystemRealtime,
        }
    }
}

#[derive(Debug)]
pub struct Continue {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for Continue {
    fn default() -> Self {
        Continue {
            bytes: [0xFB],
            category: Category::SystemRealtime,
        }
    }
}

impl Continue {
    pub fn new() -> Self {
        Continue::default()
    }
}

impl From<&[u8]> for Continue {
    fn from(raw_bytes: &[u8]) -> Self {
        Continue {
            bytes: [raw_bytes[0]],
            category: Category::SystemRealtime,
        }
    }
}

#[derive(Debug)]
pub struct Stop {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for Stop {
    fn default() -> Self {
        Stop {
            bytes: [0xFC],
            category: Category::SystemRealtime,
        }
    }
}

impl Stop {
    pub fn new() -> Self {
        Stop::default()
    }
}

impl From<&[u8]> for Stop {
    fn from(raw_bytes: &[u8]) -> Self {
        Stop {
            bytes: [raw_bytes[0]],
            category: Category::SystemRealtime,
        }
    }
}

#[derive(Debug)]
pub struct ActiveSensing {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for ActiveSensing {
    fn default() -> Self {
        ActiveSensing {
            bytes: [0xFE],
            category: Category::SystemRealtime,
        }
    }
}

impl ActiveSensing {
    pub fn new() -> Self {
        ActiveSensing::default()
    }
}

impl From<&[u8]> for ActiveSensing {
    fn from(raw_bytes: &[u8]) -> Self {
        ActiveSensing {
            bytes: [raw_bytes[0]],
            category: Category::SystemRealtime,
        }
    }
}

#[derive(Debug)]
pub struct Reset {
    bytes: [u8; 1],
    pub category: Category,
}

impl Default for Reset {
    fn default() -> Self {
        Reset {
            bytes: [0xFF],
            category: Category::SystemRealtime,
        }
    }
}

impl Reset {
    pub fn new() -> Self {
        Reset::default()
    }
}

impl From<&[u8]> for Reset {
    fn from(raw_bytes: &[u8]) -> Self {
        Reset {
            bytes: [raw_bytes[0]],
            category: Category::SystemRealtime,
        }
    }
}

impl_midi_message!(TimingClock);
impl_midi_message!(Start);
impl_midi_message!(Continue);
impl_midi_message!(Stop);
impl_midi_message!(ActiveSensing);
impl_midi_message!(Reset);
