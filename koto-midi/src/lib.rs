mod message;
use message::*;

use koto::prelude::*;
use koto::runtime::KList;
use koto::runtime::KNumber;
use koto::runtime::KMap;
use koto::runtime::KValue;
use koto::Error as RuntimeError;

// TODO: Solve unnecessary repetition of list collectors for different types ot cases if there is.
pub fn collect_list_of_midi_bytes_as_u8(
    message: &KList,
    error: &str,
) -> std::result::Result<Vec<u8>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            KValue::Number(num) => match num {
                // Truncate.
                KNumber::I64(midi_byte) if *midi_byte >= 0 && *midi_byte < 128 => Ok(*midi_byte as u8),
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
    message: &KList,
    error: &str,
) -> std::result::Result<Vec<u8>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            KValue::Number(num) => match num {
                // Truncate.
                KNumber::I64(byte) if *byte >= 0 && *byte <= 255 => Ok(*byte as u8),
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
    message: &KList,
    error: &str,
) -> std::result::Result<Vec<u64>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            KValue::Number(num) => match num {
                // Truncate.
                KNumber::I64(midi_byte) if *midi_byte >= 0 => Ok(*midi_byte as u64),
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
    message: &KList,
    error: &str,
) -> std::result::Result<Vec<KList>, RuntimeError> {
    let arguments = message
        .data()
        .iter()
        .map(|v| match v {
            KValue::List(list) => Ok(list.clone()),
            _ => {
                runtime_error!(error)
            }
        })
        .collect::<std::result::Result<Vec<KList>, RuntimeError>>();
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

macro_rules! types {
    ($map:ident,$($type_literal:literal),*) => {
        $($map.insert($type_literal, $type_literal);)*
    }
}

macro_rules! impl_pack {
    ($map:ident, $message:ident) => {
        $map.add_fn("pack", move |_| {
            Ok(KValue::List(KList::from_slice(
                &$message
                    .pack()
                    .into_iter()
                    .map(|byte| byte.into())
                    .collect::<Vec<KValue>>()[..],
            )))
        });
    };
}

macro_rules! make_koto_message_constructor {
    ($map:ident, $enum_key:ident, $category_literal:literal, $($field:ident),*, $error_literal:literal) => {
        let name_literal = pascal_case_to_underscore_separated_literal(stringify!($enum_key));
        $map.add_fn(&name_literal.clone(), move |ctx| {
            let args = ctx.args();
            if args.len() == 1 {
                match args {
                    [KValue::List(message)] => {
                        if let Ok(arguments) = collect_list_of_u64(message, $error_literal) {
                            if let [$($field),*] = &arguments[..] {
                                let message_koto = KMap::new();
                                // dbg!($(*$field),*);
                                let message = <$enum_key>::new($(*$field),*);
                                // dbg!(&message);
                                message_koto.insert("type", name_literal.clone());
                                message_koto.insert("category", $category_literal);
                                $(
                                //   dbg!($field);
                                  message_koto.insert(stringify!($field), message.$field());
                                )*
                                impl_pack!(message_koto, message);
                                Ok(KValue::Map(message_koto))
                            }
                            else {
                            runtime_error!($error_literal)
                            }
                        } else {
                            Ok(KValue::Null)
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
        $map.add_fn(&name_literal.clone(), move |ctx| {
            let args = ctx.args();
            if args.is_empty() {
                let message_koto = KMap::new();
                let message = <$enum_key>::default();
                message_koto.insert("type", name_literal.clone());
                message_koto.insert("category", $category_literal);
                impl_pack!(message_koto, message);
                Ok(KValue::Map(message_koto))
            } else {
                runtime_error!($error_literal)
            }
        })
    }
}


macro_rules! make_koto_message {
    ($map:ident, $message:ident, $name_literal:literal,$($field:ident),*) => { 
        $map.insert("type", $name_literal);
        $(
            $map.insert(stringify!($field),$message.$field());
        )*
        impl_pack!($map, $message);
    }
}


pub fn make_module() -> KMap {
    let module = KMap::new();
    let types = KMap::new();
    
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

    let categories = KMap::new();
    
    types!(
        categories,
        "channel_voice",
        "channel_mode",
        "system_common",
        "system_realtime",
        "unknown"
    );

    let message_constructors = KMap::new();

    make_koto_message_constructor!(
        message_constructors,
        NoteOff,
        "channel_voice",
        note,
        velocity,
        channel,
        "note_off requires a single list of exactly three integers as its argument"
    );
   
    make_koto_message_constructor!(
        message_constructors,
        NoteOn,
        "channel_voice",
        note,
        velocity,
        channel,
        "note_on requires a single list of exactly three integers as its argument"
    );
   
    make_koto_message_constructor!(
        message_constructors,
        PolyAfterTouch,
        "channel_voice",
        note,
        pressure,
        channel,
        "poly_after_touch requires a single list of exactly three integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        ControlChange,
        "channel_voice",
        note,
        value,
        channel,
        "control_change requires a single list of exactly three integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        ProgramChange,
        "channel_voice",
        program,
        channel,
        "program_change requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        AfterTouch,
        "channel_voice",
        pressure,
        channel,
        "after_touch requires a single list of exactly two positive integers as its argument"
    );

    make_koto_message_constructor!(
        message_constructors,
        PitchBend,
        "channel_voice",
        bend_amount,
        channel,
        "pitch_bend requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        AllSoundOff,
        "channel_mode",
        value,
        channel,
        "all_sound_off requires a single list of exactly two positive integers as its argument"
    );
   
    make_koto_message_constructor!(
        message_constructors,
        ResetAllControllers,
        "channel_mode",
        value,
        channel,
        "reset_all_controllers requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        LocalControl,
        "channel_mode",
        value,
        channel,
        "local_control requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        AllNotesOff,
        "channel_mode",
        value,
        channel,
        "all_notes_off requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        OmniModeOff,
        "channel_mode",
        value,
        channel,
        "omni_mode_off requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        OmniModeOn,
        "channel_mode",
        value,
        channel,
        "omni_mode_on requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        MonoModeOn,
        "channel_mode",
        value,
        channel,
        "mono_mode_on requires a single list of exactly two positive integers as its argument"
    );
    
    make_koto_message_constructor!(
        message_constructors,
        PolyModeOn,
        "channel_mode",
        value,
        channel,
        "poly_mode_on requires a single list of exactly two positive integers as its argument"
    );

    // TODO: This is a very basic sysex implementation. It might be extended later. Find out if it is necessary.

    message_constructors.add_fn("system_exclusive", |ctx| {
                let error_literal = "system_exclusive requires a list with single or 3 bytes for its first argument and a list with one or more bytes for its second argument";
                let args = ctx.args();
                if args.len() == 2 {
                    match args {
                        [KValue::List(message)] => {
                            if let Ok(arguments) = collect_list_of_value_list(message, error_literal) {
                                if let [manufacturer_id,message] = &arguments[..] {
                                    match manufacturer_id.len() {
                                        1 | 3 => {
                                            match message.len() {
                                                0 => runtime_error!(error_literal),
                                                _ => {                                                        
                                                    if let Ok(m_id) = collect_list_of_u8(manufacturer_id, error_literal) {
                                                        if let Ok(data) = collect_list_of_u8(message, error_literal) {
                                                                let message_koto = KMap::new();
                                                                let message = SystemExclusive::new(&m_id[..], &data[..]);
                                                                message_koto.insert("type", "system_exclusive");
                                                                message_koto.insert("category", "system_common");
                                                                let m_id = m_id.iter().map(|&x| x.into()).collect::<Vec<KValue>>();
                                                                message_koto.insert("manufacturer_id", KValue::List(KList::from_slice(&m_id[..])));
                                                                impl_pack!(message_koto, message);
                                                                Ok(KValue::Map(message_koto))
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
                                Ok(KValue::Null)
                            }
                        }
                        _ => runtime_error!(error_literal),
                    }
                } else {
                    runtime_error!(error_literal)
                }
            });

    // TODO: Find out what are possible message types and values for time_code_quarter_frame

    make_koto_message_constructor!(
        message_constructors,
        TimeCodeQuarterFrame,
        "system_common",
        message_type,
        values,
        "time_code_quarter_frame requires a single list of exactly two positive integers as its argument"
    );

    make_koto_message_constructor!(
        message_constructors,
        SongPosition,
        "system_common",
        midi_beats_elapsed,
        "song_position requires a single list of exactly one positive integer as its argument"
    );

    make_koto_message_constructor!(
        message_constructors,
        SongSelect,
        "system_common",
        number,
        "song_select requires a single list of exactly one positive integer as its argument"
    );

    make_koto_message_constructor!(
        message_constructors,
        TuneRequest,
        "system_common",
        "tune_request does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,       
        EndOfExclusive,
        "system_common",
        "end_of_exclusive does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,
        TimingClock,
        "system_realtime",      
        "timing_clock does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,
        Start,
        "system_realtime", 
        "start does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,
        Continue,
        "system_realtime",
        "continue does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,
        Stop,
        "system_realtime",
        "stop does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,
        ActiveSensing,
        "system_realtime",
        "active_sensing does not take any arguments"
    );

    make_koto_message_constructor!(
        message_constructors,
        Reset,
        "system_realtime",
        "reset does not take any arguments"
    );

    module.add_fn("parse", |ctx| {
        let args = ctx.args();
        if args.len() == 1 {
            match args {
                [KValue::List(message)] => {
                    let message_koto = KMap::new();
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
                                message_koto.insert("category", "channel_voice")
                            }
                            Message::AllSoundOff(_)
                            | Message::ResetAllControllers(_)
                            | Message::LocalControl(_)
                            | Message::AllNotesOff(_)
                            | Message::OmniModeOff(_)
                            | Message::OmniModeOn(_)
                            | Message::MonoModeOn(_)
                            | Message::PolyModeOn(_) => {
                                message_koto.insert("category", "channel_mode")
                            }
                            Message::SystemExclusive(_)
                            | Message::SongPosition(_)
                            | Message::SongSelect(_)
                            | Message::TuneRequest(_)
                            | Message::EndOfExclusive(_)
                            | Message::TimeCodeQuarterFrame(_) => {
                                message_koto.insert("category", "system_common")
                            }
                            Message::TimingClock(_)
                            | Message::Start(_)
                            | Message::Continue(_)
                            | Message::Stop(_)
                            | Message::ActiveSensing(_)
                            | Message::Reset(_) => {
                                message_koto.insert("category", "system_realtime")
                            }
                            Message::Undefined | Message::Malformed => {
                                message_koto.insert("category", "unknown")
                            }
                        };

                        match message {
                            Message::NoteOff(message) => {
                                make_koto_message!(message_koto, message, "note_off", note, velocity, channel);
                            }
                            Message::NoteOn(message) => {
                                make_koto_message!(message_koto, message, "note_on", note, velocity, channel);
                            }
                            Message::ControlChange(message) => {
                                make_koto_message!(message_koto, message, "control_change", note, value, channel);
                            }
                            Message::ProgramChange(message) => {
                                make_koto_message!(message_koto, message, "program_change", program, channel);
                            }

                            Message::AfterTouch(message) => {
                                make_koto_message!(message_koto, message, "after_touch", pressure, channel);
                            }
                            Message::PolyAfterTouch(message) => {
                                make_koto_message!(message_koto, message, "poly_after_touch", note, pressure, channel);
                            }
                            Message::PitchBend(message) => {
                                make_koto_message!(message_koto, message, "pitch_bend", bend_amount, channel);
                            }
                            Message::AllSoundOff(message) => {
                                make_koto_message!(message_koto, message, "all_sound_off", value, channel);
                                message_koto.insert("note", 120);
                            }
                            Message::ResetAllControllers(message) => {
                                make_koto_message!(message_koto, message, "reset_all_controllers", value, channel);
                                message_koto.insert("note", 121);
                            }
                            Message::LocalControl(message) => {
                                make_koto_message!(message_koto, message, "local_control", value, channel);
                                message_koto.insert("note", 122);
                            }
                            Message::AllNotesOff(message) => {
                                make_koto_message!(message_koto, message, "all_notes_off", value, channel);
                                message_koto.insert("note", 123);
                            }
                            Message::OmniModeOff(message) => {
                                make_koto_message!(message_koto, message, "omni_mode_off", value, channel);
                                message_koto.insert("note", 124);
                            }
                            Message::OmniModeOn(message) => {
                                make_koto_message!(message_koto, message, "omni_mode_on", value, channel);
                                message_koto.insert("note", 125);
                            }
                            Message::MonoModeOn(message) => {
                                make_koto_message!(message_koto, message, "mono_mode_on", value, channel);
                                message_koto.insert("note", 126);
                            }
                            Message::PolyModeOn(message) => {
                                make_koto_message!(message_koto, message, "poly_mode_on", value, channel);
                                message_koto.insert("note", 127);
                            }
                            Message::SystemExclusive(message) => {
                                message_koto.insert("type", "system_exclusive");
                                let m_id = message.manufacturer_id.iter().map(|&x| x.into()).collect::<Vec<KValue>>();
                                message_koto.insert("manufacturer_id", KValue::List(KList::from_slice(&m_id[..])));
                                impl_pack!(message_koto, message);
                            }
                            Message::SongPosition(message) => {
                                make_koto_message!(message_koto, message, "song_position", midi_beats_elapsed);
                            }
                            Message::SongSelect(message) => {
                                make_koto_message!(message_koto, message, "song_select", number);
                            }
                            Message::TuneRequest(message) => {
                                message_koto.insert("type", "tune_request");
                                impl_pack!(message_koto, message);
                            }
                            Message::EndOfExclusive(message) => {
                                message_koto.insert("type", "end_of_exclusive");
                                impl_pack!(message_koto, message);
                            }
                            Message::TimeCodeQuarterFrame(message) => {
                                make_koto_message!(message_koto, message, "time_code_quarter_frame", message_type, values);
                            }
                            Message::TimingClock(message) => {
                                message_koto.insert("type", "timing_clock");
                                impl_pack!(message_koto, message);
                            }
                            Message::Start(message) => {
                                message_koto.insert("type", "start");
                                impl_pack!(message_koto, message);
                            }
                            Message::Continue(message) => {
                                message_koto.insert("type", "continue");
                                impl_pack!(message_koto, message);
                            }
                            Message::Stop(message) => {
                                message_koto.insert("type", "stop");
                                impl_pack!(message_koto, message);
                            }
                            Message::ActiveSensing(message) => {
                                message_koto.insert("type", "active_sensing");
                                impl_pack!(message_koto, message);
                            }
                            Message::Reset(message) => {
                                message_koto.insert("type", "reset");
                                impl_pack!(message_koto, message);
                            }
                            Message::Undefined => {
                                message_koto.insert("type", "undefined");
                            }
                            Message::Malformed => {
                                message_koto.insert("type", "malformed");
                            }
                        }

                        Ok(KValue::Map(message_koto))
                    } else {
                        message_koto.insert("type", "malformed");
                        message_koto.insert("category", "unknown");
                        // Returns an empty value if the message is malformed.
                        Ok(KValue::Map(message_koto))
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

    module.insert("types", types);
    module.insert("categories", categories);
    module.insert("message", message_constructors);
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
