#![allow(unused)]

mod channel_mode;
mod channel_voice;
mod system_common;
mod system_realtime;
pub use channel_mode::*;
pub use channel_voice::*;
pub use system_common::*;
pub use system_realtime::*;

#[derive(Debug)]
pub enum Category {
    ChannelVoice,
    ChannelMode,
    SystemCommon,
    SystemRealtime,
    Unknown,
}

#[derive(Debug)]
pub enum Message {
    NoteOn(NoteOn),
    NoteOff(NoteOff),
    ControlChange(ControlChange),
    ProgramChange(ProgramChange),
    PitchBend(PitchBend),
    AfterTouch(AfterTouch),
    PolyAfterTouch(PolyAfterTouch),
    SystemExclusive(SystemExclusive),
    SongPosition(SongPosition),
    SongSelect(SongSelect),
    TuneRequest(TuneRequest),
    TimeCodeQuarterFrame(TimeCodeQuarterFrame),
    TimingClock(TimingClock),
    Start(Start),
    Continue(Continue),
    Stop(Stop),
    ActiveSensing(ActiveSensing),
    Reset(Reset),
    Undefined,
    Malformed,
    EndOfExclusive(EndOfExclusive),
    AllSoundOff(AllSoundOff),
    ResetAllControllers(ResetAllControllers),
    LocalControl(LocalControl),
    AllNotesOff(AllNotesOff),
    OmniModeOff(OmniModeOff),
    OmniModeOn(OmniModeOn),
    MonoModeOn(MonoModeOn),
    PolyModeOn(PolyModeOn),
}

#[derive(Debug)]
pub struct ParsedMessage {
    pub message: Message,
}

impl From<&[u8]> for ParsedMessage {
    fn from(raw_message: &[u8]) -> Self {
        let status_byte = raw_message[0];
        let data_bytes = raw_message[1..].to_vec();
        let data_bytes_length = data_bytes.len();
        let message = match status_byte & 0xF0 {
            0x80 => match data_bytes_length {
                2 if data_bytes[0] <= 127 && data_bytes[1] <= 127 => {
                    Message::NoteOff(NoteOff::from(raw_message))
                }
                _ => Message::Malformed,
            },

            0x90 => match data_bytes_length {
                2 if data_bytes[0] <= 127 && data_bytes[1] <= 127 => {
                    Message::NoteOn(raw_message.into())
                }
                _ => Message::Malformed,
            },
            0xA0 => match data_bytes_length {
                2 if data_bytes[0] <= 127 && data_bytes[1] <= 127 => {
                    Message::PolyAfterTouch(PolyAfterTouch::from(raw_message))
                }
                _ => Message::Malformed,
            },
            0xB0 => match data_bytes_length {
                2 if data_bytes[0] <= 127 && data_bytes[1] <= 127 => {
                    match data_bytes[0] & 0b0111_1111 {
                        0..120 => Message::ControlChange(ControlChange::from(raw_message)),
                        120 => Message::AllSoundOff(AllSoundOff::from(raw_message)),
                        121 => Message::ResetAllControllers(ResetAllControllers::from(raw_message)),
                        122 => Message::LocalControl(LocalControl::from(raw_message)),
                        123 => Message::AllNotesOff(AllNotesOff::from(raw_message)),
                        124 => Message::OmniModeOff(OmniModeOff::from(raw_message)),
                        125 => Message::OmniModeOn(OmniModeOn::from(raw_message)),
                        126 => Message::MonoModeOn(MonoModeOn::from(raw_message)),
                        127 => Message::PolyModeOn(PolyModeOn::from(raw_message)),
                        _ => Message::Malformed,
                    }
                }
                _ => Message::Malformed,
            },

            0xC0 => match data_bytes_length {
                1 if data_bytes[0] <= 127 => {
                    Message::ProgramChange(ProgramChange::from(raw_message))
                }
                _ => Message::Malformed,
            },
            0xD0 => match data_bytes_length {
                1 if data_bytes[0] <= 127 => Message::AfterTouch(AfterTouch::from(raw_message)),
                _ => Message::Malformed,
            },
            0xE0 => match data_bytes_length {
                2 if data_bytes[0] <= 127 && data_bytes[1] <= 127 => {
                    Message::PitchBend(PitchBend::from(raw_message))
                }
                _ => Message::Malformed,
            },
            _ => match status_byte {
                0xF0 => match data_bytes.last() {
                    Some(0xF7) => Message::SystemExclusive(SystemExclusive::from(raw_message)),
                    _ => Message::Malformed,
                },
                0xF1 => match data_bytes_length {
                    1 if data_bytes[0] <= 127 => {
                        Message::TimeCodeQuarterFrame(TimeCodeQuarterFrame::from(raw_message))
                    }
                    _ => Message::Malformed,
                },
                0xF2 => match data_bytes_length {
                    2 if data_bytes[0] <= 127 && data_bytes[1] <= 127 => {
                        Message::SongPosition(SongPosition::from(raw_message))
                    }
                    _ => Message::Malformed,
                },
                0xF3 => match data_bytes_length {
                    1 if data_bytes[0] <= 127 => Message::SongSelect(SongSelect::from(raw_message)),
                    _ => Message::Malformed,
                },
                0xF4 | 0xF5 => Message::Undefined,
                0xF6 => match data_bytes_length {
                    0 => Message::TuneRequest(TuneRequest::default()),
                    _ => Message::Malformed,
                },
                0xF7 => match data_bytes_length {
                    0 => Message::EndOfExclusive(EndOfExclusive::default()),
                    _ => Message::Malformed,
                },
                0xF8 => match data_bytes_length {
                    0 => Message::TimingClock(TimingClock::default()),
                    _ => Message::Malformed,
                },
                0xF9 => Message::Undefined,
                0xFA => match data_bytes_length {
                    0 => Message::Start(Start::default()),
                    _ => Message::Malformed,
                },
                0xFB => match data_bytes_length {
                    0 => Message::Continue(Continue::default()),
                    _ => Message::Malformed,
                },
                0xFC => match data_bytes_length {
                    0 => Message::Stop(Stop::default()),
                    _ => Message::Malformed,
                },
                0xFD => Message::Undefined,
                0xFE => match data_bytes_length {
                    0 => Message::ActiveSensing(ActiveSensing::default()),
                    _ => Message::Malformed,
                },
                0xFF => match data_bytes_length {
                    0 => Message::Reset(Reset::default()),
                    _ => Message::Malformed,
                },
                _ => Message::Malformed,
            },
        };

        ParsedMessage { message }
    }
}
