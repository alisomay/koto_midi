#![feature(exclusive_range_pattern)]

mod message;
use message::*;

use koto::runtime::{
    runtime_error, RuntimeError, Value, ValueList, ValueMap, ValueNumber,
};

macro_rules! types {
    ($map:ident,$($type_literal:literal),*) => {
        $($map.add_value($type_literal, $type_literal.into());)*
    }
}
macro_rules! impl_pack {
    ($map:ident, $message:ident) => {
        $map.add_fn("pack", move |_, _| {
            Ok(Value::List(ValueList::from_slice(
                &$message
                    .pack()
                    .into_iter()
                    .map(|byte| byte.into())
                    .collect::<Vec<Value>>()[..],
            )))
        });
    };
}

// TODO: Solve unnecessary repetition of list collectors for different types ot cases if there is.
pub fn collect_list_of_midi_bytes_as_u8(
    message: &ValueList,
    error: &str,
) -> std::result::Result<Vec<u8>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            Value::Number(num) => match num {
                // Truncate.
                ValueNumber::I64(midi_byte) if *midi_byte >= 0 && *midi_byte < 128 => Ok(*midi_byte as u8),
                _ => runtime_error!(error),
            },
            _ => {
                runtime_error!(error)
            }
        })
        .collect::<std::result::Result<Vec<u8>, RuntimeError>>();
    arguments
}

pub fn collect_list_of_u8(
    message: &ValueList,
    error: &str,
) -> std::result::Result<Vec<u8>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            Value::Number(num) => match num {
                // Truncate.
                ValueNumber::I64(byte) if *byte >= 0 && *byte <= 255 => Ok(*byte as u8),
                _ => runtime_error!(error),
            },
            _ => {
                runtime_error!(error)
            }
        })
        .collect::<std::result::Result<Vec<u8>, RuntimeError>>();
    arguments
}
pub fn collect_list_of_u64(
    message: &ValueList,
    error: &str,
) -> std::result::Result<Vec<u64>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            Value::Number(num) => match num {
                // Truncate.
                ValueNumber::I64(midi_byte) if *midi_byte >= 0 => Ok(*midi_byte as u64),
                _ => runtime_error!(error),
            },
            _ => {
                runtime_error!(error)
            }
        })
        .collect::<std::result::Result<Vec<u64>, RuntimeError>>();
    arguments
}

pub fn collect_list_of_value_list(
    message: &ValueList,
    error: &str,
) -> std::result::Result<Vec<ValueList>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            Value::List(list) => Ok(list.clone()),
            _ => {
                runtime_error!(error)
            }
        })
        .collect::<std::result::Result<Vec<ValueList>, RuntimeError>>();
    arguments
}


fn pascal_case_to_underscore_separated_literal(string_to_process: &str) -> std::string::String {
    let mut literal = String::new();
    for (i,ch) in string_to_process.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            literal.push('_');
            literal.push_str(&format!("{}",ch.to_lowercase())[..]);
            continue;
        }
        else if ch.is_uppercase() {
            literal.push_str(&format!("{}",ch.to_lowercase())[..]);
            continue;
        }
        literal.push(ch);
    }
    literal
}


macro_rules! message_constructor {
    ($map:ident, $enum_key:ident, $category_literal:literal, $($field:ident),*, $error_literal:literal) => {
        let name_literal = pascal_case_to_underscore_separated_literal(stringify!($enum_key));
        $map.add_fn(&name_literal.clone(), move |vm, args| {
            if vm.get_args(&args).len() == 1 {
                match vm.get_args(&args) {
                    [Value::List(message)] => {
                        if let Ok(arguments) = collect_list_of_u64(message, $error_literal) {
                            if let [$($field),*] = &arguments[..] {
                                let mut message_koto = ValueMap::new();
                                // dbg!($(*$field),*);
                                let message = <$enum_key>::new($(*$field),*);
                                dbg!(&message);
                                message_koto.add_value("type", name_literal.clone().into());
                                message_koto.add_value("category", $category_literal.into());
                                $(
                                //   dbg!($field);
                                  message_koto.add_value(stringify!($field), message.$field().into());
                                )*
                                impl_pack!(message_koto, message);
                                Ok(Value::Map(message_koto))
                            }
                            else {
                            runtime_error!($error_literal)
                            }
                        } else {
                            Ok(Value::Empty)
                        }
                    }
                    _ => runtime_error!($error_literal),
                }
            } else {
                runtime_error!($error_literal)
            }
        })
    };

    ($map:ident, $enum_key:ty, $category_literal:literal, $error_literal:literal) => {
        let name_literal = pascal_case_to_underscore_separated_literal(stringify!($enum_key));
        $map.add_fn(&name_literal.clone(), move |vm, args| {
            if vm.get_args(&args).len() == 0 {
                let mut message_koto = ValueMap::new();
                let message = <$enum_key>::default();
                message_koto.add_value("type", name_literal.clone().into());
                message_koto.add_value("category", $category_literal.into());
                impl_pack!(message_koto, message);
                Ok(Value::Map(message_koto))
            } else {
                runtime_error!($error_literal)
            }
        })
    }
}



pub fn make_module() -> ValueMap {
    let mut module = ValueMap::new();

    let mut types = ValueMap::new();
    types!(
        types,
        "note_off",
        "note_on",
        "poly_after_touch",
        "control_change",
        "program_change",
        "after_touch",
        "pitch_bend",
        "all_sound_off",
        "reset_all_controllers",
        "local_control",
        "all_notes_off",
        "omni_mode_off",
        "omni_mode_on",
        "mono_mode_on",
        "poly_mode_on",
        "system_exclusive",
        "time_code_quarter_frame",
        "song_position",
        "song_select",
        "tune_request",
        "end_of_exclusive",
        "timing_clock",
        "start",
        "continue",
        "stop",
        "active_sensing",
        "reset",
        "undefined",
        "malformed"
    );

    let mut categories = ValueMap::new();
    types!(
        categories,
        "channel_voice",
        "channel_mode",
        "system_common",
        "system_realtime",
        "unknown"
    );

    let mut message_constructors = ValueMap::new();

    message_constructor!(
        message_constructors,
        NoteOff,
        "channel_voice",
        note,
        velocity,
        channel,
        "note_off requires a single list of exactly three integers as its argument"
    );
    message_constructor!(
        message_constructors,
        NoteOn,
        "channel_voice",
        note,
        velocity,
        channel,
        "note_on requires a single list of exactly three integers as its argument"
    );
    message_constructor!(
        message_constructors,
        PolyAfterTouch,
        "channel_voice",
        note,
        pressure,
        channel,
        "poly_after_touch requires a single list of exactly three integers as its argument"
    );
    message_constructor!(
        message_constructors,
        ControlChange,
        "channel_voice",
        note,
        value,
        channel,
        "control_change requires a single list of exactly three integers as its argument"
    );
    message_constructor!(
        message_constructors,
        ProgramChange,
        "channel_voice",
        program,
        channel,
        "program_change requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        AfterTouch,
        "channel_voice",
        pressure,
        channel,
        "after_touch requires a single list of exactly two positive integers as its argument"
    );

    message_constructor!(
        message_constructors,
        PitchBend,
        "channel_voice",
        bend_amount,
        channel,
        "pitch_bend requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        AllSoundOff,
        "channel_mode",
        value,
        channel,
        "all_sound_off requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        ResetAllControllers,
        "channel_mode",
        value,
        channel,
        "reset_all_controllers requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        LocalControl,
        "channel_mode",
        value,
        channel,
        "local_control requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        AllNotesOff,
        "channel_mode",
        value,
        channel,
        "all_notes_off requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        OmniModeOff,
        "channel_mode",
        value,
        channel,
        "omni_mode_off requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        OmniModeOn,
        "channel_mode",
        value,
        channel,
        "omni_mode_on requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        MonoModeOn,
        "channel_mode",
        value,
        channel,
        "mono_mode_on requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        PolyModeOn,
        "channel_mode",
        value,
        channel,
        "poly_mode_on requires a single list of exactly two positive integers as its argument"
    );

    // TODO: This is a very basic sysex implementation. It might be extended later. Find out if it is necessary.

    message_constructors.add_fn("system_exclusive", |vm, args| {
                let error_literal = "system_exclusive requires a list with single or 3 bytes for its first argument and a list with one or more bytes for its second argument";
                if vm.get_args(&args).len() == 2 {
                    match vm.get_args(&args) {
                        [Value::List(message)] => {
                            if let Ok(arguments) = collect_list_of_value_list(message, error_literal) {
                                if let [manufacturer_id,message] = &arguments[..] {
                                    match manufacturer_id.len() {
                                        1 | 3 => {
                                            match message.len() {
                                                0 => runtime_error!(error_literal),
                                                _ => {                                                        
                                                    if let Ok(m_id) = collect_list_of_u8(manufacturer_id, error_literal) {
                                                        if let Ok(data) = collect_list_of_u8(message, error_literal) {
                                                                let mut message_koto = ValueMap::new();
                                                                let message = SystemExclusive::new(&m_id[..], &data[..]);
                                                                message_koto.add_value("type", "system_exclusive".into());
                                                                message_koto.add_value("category", "system_common".into());
                                                                let m_id = m_id.iter().map(|&x| x.into()).collect::<Vec<Value>>();
                                                                message_koto.add_value("manufacturer_id", Value::List(ValueList::from_slice(&m_id[..])));
                                                                impl_pack!(message_koto, message);
                                                                Ok(Value::Map(message_koto))
                                                        }
                                                        else{ 
                                                            runtime_error!(error_literal)
                                                        }
                                                    }
                                                    else {
                                                        runtime_error!(error_literal)
                                                    }
                                                }
                                            }
                                        }
                                        _ => runtime_error!(error_literal)
                                    }
                                }
                                else {
                                runtime_error!(error_literal)
                                }
                            } else {
                                Ok(Value::Empty)
                            }
                        }
                        _ => runtime_error!(error_literal),
                    }
                } else {
                    runtime_error!(error_literal)
                }
            });

    // TODO: Find out what are possible message types and values for time_code_quarter_frame

    message_constructor!(
        message_constructors,
        TimeCodeQuarterFrame,
        "system_common",
        message_type,
        values,
        "time_code_quarter_frame requires a single list of exactly two positive integers as its argument"
    );
    message_constructor!(
        message_constructors,
        SongPosition,
        "system_common",
        midi_beats_elapsed,
        "song_position requires a single list of exactly one positive integer as its argument"
    );
    message_constructor!(
        message_constructors,
        SongSelect,
        "system_common",
        number,
        "song_select requires a single list of exactly one positive integer as its argument"
    );

    message_constructor!(
        message_constructors,
        TuneRequest,
        "system_common",
        "tune_request does not take any arguments"
    );
    message_constructor!(
        message_constructors,       
        EndOfExclusive,
        "system_common",
        "end_of_exclusive does not take any arguments"
    );
    message_constructor!(
        message_constructors,
        TimingClock,
        "system_realtime",      
        "timing_clock does not take any arguments"
    );
    message_constructor!(
        message_constructors,
        Start,
        "system_realtime", 
        "start does not take any arguments"
    );
    message_constructor!(
        message_constructors,
        Continue,
        "system_realtime",
        "continue does not take any arguments"
    );
    message_constructor!(
        message_constructors,
        Stop,
        "system_realtime",
        "stop does not take any arguments"
    );
    message_constructor!(
        message_constructors,
        ActiveSensing,
        "system_realtime",
        "active_sensing does not take any arguments"
    );
    message_constructor!(
        message_constructors,
        Reset,
        "system_realtime",
        "reset does not take any arguments"
    );

    module.add_fn("parse", |vm, args| {
        if vm.get_args(&args).len() == 1 {
            match vm.get_args(&args) {
                [Value::List(message)] => {
                    let mut message_koto = ValueMap::new();
                    if let Ok(midi_message) = collect_list_of_u8(
                        message,
                        "parse requires a single list of one or more positive integers as its argument",
                    ) {
                        let parsed = ParsedMessage::from(&midi_message[..]);
                        let message = parsed.message;

                        match message {
                            Message::NoteOn(_)
                            | Message::NoteOff(_)
                            | Message::ControlChange(_)
                            | Message::ProgramChange(_)
                            | Message::PitchBend(_)
                            | Message::AfterTouch(_)
                            | Message::PolyAfterTouch(_) => {
                                message_koto.add_value("category", "channel_voice".into())
                            }
                            Message::AllSoundOff(_)
                            | Message::ResetAllControllers(_)
                            | Message::LocalControl(_)
                            | Message::AllNotesOff(_)
                            | Message::OmniModeOff(_)
                            | Message::OmniModeOn(_)
                            | Message::MonoModeOn(_)
                            | Message::PolyModeOn(_) => {
                                message_koto.add_value("category", "channel_mode".into())
                            }
                            Message::SystemExclusive(_)
                            | Message::SongPosition(_)
                            | Message::SongSelect(_)
                            | Message::TuneRequest(_)
                            | Message::EndOfExclusive(_)
                            | Message::TimeCodeQuarterFrame(_) => {
                                message_koto.add_value("category", "system_common".into())
                            }
                            Message::TimingClock(_)
                            | Message::Start(_)
                            | Message::Continue(_)
                            | Message::Stop(_)
                            | Message::ActiveSensing(_)
                            | Message::Reset(_) => {
                                message_koto.add_value("category", "system_realtime".into())
                            }
                            Message::Undefined | Message::Malformed => {
                                message_koto.add_value("category", "unknown".into())
                            }
                        };

                        match message {
                            Message::NoteOff(message) => {
                                message_koto.add_value("type", "note_off".into());
                                message_koto.add_value("note", message.note().into());
                                message_koto
                                    .add_value("velocity", message.velocity().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::NoteOn(message) => {
                                message_koto.add_value("type", "note_on".into());
                                message_koto.add_value("note", message.note().into());
                                message_koto
                                    .add_value("velocity", message.velocity().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::ControlChange(message) => {
                                message_koto.add_value("type", "control_change".into());
                                message_koto.add_value("note", message.note().into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::ProgramChange(message) => {
                                message_koto.add_value("type", "program_change".into());
                                message_koto.add_value("program", message.program().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }

                            Message::AfterTouch(message) => {
                                message_koto.add_value("type", "after_touch".into());

                                message_koto
                                    .add_value("pressure", message.pressure().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::PolyAfterTouch(message) => {
                                message_koto.add_value("note", message.note().into());
                                message_koto.add_value("type", "poly_after_touch".into());
                                message_koto
                                    .add_value("pressure", message.pressure().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::PitchBend(message) => {
                                message_koto.add_value("type", "pitch_bend".into());
                                message_koto.add_value(
                                    "bend_amount",
                                    (message.bend_amount() as i64).into(),
                                );
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::AllSoundOff(message) => {
                                message_koto.add_value("type", "all_sound_off".into());
                                message_koto.add_value("note", 120.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::ResetAllControllers(message) => {
                                message_koto
                                    .add_value("type", "reset_all_controllers".into());
                                message_koto.add_value("note", 121.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::LocalControl(message) => {
                                message_koto.add_value("type", "local_control".into());
                                message_koto.add_value("note", 122.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::AllNotesOff(message) => {
                                message_koto.add_value("type", "all_notes_off".into());
                                message_koto.add_value("note", 123.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::OmniModeOff(message) => {
                                message_koto.add_value("type", "omni_mode_off".into());
                                message_koto.add_value("note", 124.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::OmniModeOn(message) => {
                                message_koto.add_value("type", "omni_mode_on".into());
                                message_koto.add_value("note", 125.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::MonoModeOn(message) => {
                                message_koto.add_value("type", "mono_mode_on".into());
                                message_koto.add_value("note", 126.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::PolyModeOn(message) => {
                                message_koto.add_value("type", "poly_mode_on".into());
                                message_koto.add_value("note", 127.into());
                                message_koto.add_value("value", message.value().into());
                                message_koto.add_value("channel", message.channel().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::SystemExclusive(message) => {
                                message_koto.add_value("type", "system_exclusive".into());
                                let m_id = message.manufacturer_id.iter().map(|&x| x.into()).collect::<Vec<Value>>();
                                message_koto.add_value("manufacturer_id", Value::List(ValueList::from_slice(&m_id[..])));
                                impl_pack!(message_koto, message);
                            }
                            Message::SongPosition(message) => {
                                message_koto.add_value("type", "song_position".into());
                                message_koto.add_value(
                                    "midi_beats_elapsed",
                                    (message.midi_beats_elapsed() as u64).into(),
                                );
                                impl_pack!(message_koto, message);
                            }
                            Message::SongSelect(message) => {
                                message_koto.add_value("type", "song_select".into());
                                message_koto.add_value("number", message.number().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::TuneRequest(message) => {
                                message_koto.add_value("type", "tune_request".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::EndOfExclusive(message) => {
                                message_koto.add_value("type", "end_of_exclusive".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::TimeCodeQuarterFrame(message) => {
                                message_koto
                                    .add_value("type", "time_code_quarter_frame".into());
                                message_koto.add_value(
                                    "message_type",
                                    message.message_type().into(),
                                );
                                message_koto.add_value("values", message.values().into());
                                impl_pack!(message_koto, message);
                            }
                            Message::TimingClock(message) => {
                                message_koto.add_value("type", "timing_clock".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::Start(message) => {
                                message_koto.add_value("type", "start".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::Continue(message) => {
                                message_koto.add_value("type", "continue".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::Stop(message) => {
                                message_koto.add_value("type", "stop".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::ActiveSensing(message) => {
                                message_koto.add_value("type", "active_sensing".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::Reset(message) => {
                                message_koto.add_value("type", "reset".into());
                                impl_pack!(message_koto, message);
                            }
                            Message::Undefined => {
                                message_koto.add_value("type", "undefined".into());
                                message_koto.add_value("category", "unknown".into());
                            }
                            Message::Malformed => {
                                message_koto.add_value("type", "malformed".into());
                                message_koto.add_value("category", "unknown".into());
                            }
                        }

                        Ok(Value::Map(message_koto))
                    } else {
                        message_koto.add_value("type", "malformed".into());
                        message_koto.add_value("category", "unknown".into());
                        // Returns an empty value if the message is malformed.
                        Ok(Value::Map(message_koto))
                    }
                }
                _ => runtime_error!(
                    "parse requires a single list of one or more positive integers as its argument"
                ),
            }
        } else {
            runtime_error!("parse requires a single list of one or more positive integers as its argument")
        }
    });

    module.add_map("types", types);
    module.add_map("categories", categories);
    module.add_map("message", message_constructors);
    module
}

pub trait MidiMessage {
    fn pack(&self) -> &[u8];
}

macro_rules! impl_midi_message {
    ($type:ty) => {
        impl MidiMessage for $type {
            fn pack(&self) -> &[u8] {
                &self.bytes
            }
        }
    };
}

pub(crate) use impl_midi_message;
