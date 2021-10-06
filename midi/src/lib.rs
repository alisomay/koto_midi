#![feature(exclusive_range_pattern)]

mod message;
use message::*;

use koto::runtime::{
    runtime_error, RuntimeError, Value, ValueList, ValueMap, ValueNumber, ValueString,
};

macro_rules! add_types {
    ($map:ident,$($type_literal:literal),*) => {
        $($map.add_value($type_literal,Value::Str(ValueString::from($type_literal)));)*
    }
}
macro_rules! impl_pack {
    ($map:ident, $message:ident) => {
        $map.add_fn("pack", move |_, _| {
            Ok(Value::List(ValueList::from_slice(
                &$message
                    .pack()
                    .into_iter()
                    .map(|byte| Value::Number(ValueNumber::from(byte)))
                    .collect::<Vec<Value>>()[..],
            )))
        });
    };
}
macro_rules! koto_string {
    ($l:literal) => {
        Value::Str(ValueString::from($l))
    };
}
macro_rules! koto_number {
    ($e:expr) => {
        Value::Number(ValueNumber::from($e))
    };
}

// TODO: Solve unnecessary repetition of list collectors for different types ot cases if there is.
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
                ValueNumber::I64(midi_byte) if *midi_byte >= 0 => Ok(*midi_byte as u8),
                _ => runtime_error!(error),
            },
            _ => {
                runtime_error!(error)
            }
        })
        .collect::<std::result::Result<Vec<u8>, RuntimeError>>();
    arguments
}

pub fn collect_list_of_u8_for_sysex(
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

macro_rules! add_message_constructor_with_fields {
        ($map:ident, $name_literal:literal, $($field:ident),*, $constructor_content:block, $error_literal:literal) => {
            $map.add_fn($name_literal, |vm, args| {
                if vm.get_args(&args).len() == 1 {
                    match vm.get_args(&args) {
                        [Value::List(message)] => {
                            if let Ok(arguments) = collect_list_of_u64(message, $error_literal) {
                                if let [$($field),*] = &arguments[..]
                                    $constructor_content
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
        }
    }

macro_rules! add_plain_message_constructor {
    ($map:ident, $name_literal:literal, $category_literal:literal, $enum_key:ty, $error_literal:literal) => {
        $map.add_fn($name_literal, |vm, args| {
            if vm.get_args(&args).len() == 0 {
                let mut message_koto = ValueMap::new();
                let message = <$enum_key>::default();
                message_koto.add_value("type", koto_string!($name_literal));
                message_koto.add_value("category", koto_string!($category_literal));
                impl_pack!(message_koto, message);
                Ok(Value::Map(message_koto))
            } else {
                runtime_error!($error_literal)
            }
        })
    };
}

pub fn make_module() -> ValueMap {
    let mut module = ValueMap::new();

    let mut types = ValueMap::new();
    add_types!(
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
    add_types!(
        categories,
        "channel_voice",
        "channel_mode",
        "system_common",
        "system_realtime",
        "unknown"
    );

    let mut message_constructors = ValueMap::new();

    add_message_constructor_with_fields!(
        message_constructors,
        "note_off",
        note,
        velocity,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = NoteOff::new(*note, *velocity, *channel);
            message_koto.add_value("type", koto_string!("note_off"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("note", koto_number!(message.note()));
            message_koto.add_value("velocity", koto_number!(message.velocity()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "note_off requires a single list of exactly three integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "note_on",
        note,
        velocity,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = NoteOn::new(*note, *velocity, *channel);
            message_koto.add_value("type", koto_string!("note_on"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("note", koto_number!(message.note()));
            message_koto.add_value("velocity", koto_number!(message.velocity()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "note_on requires a single list of exactly three integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "poly_after_touch",
        note,
        pressure,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = PolyAfterTouch::new(*note, *pressure, *channel);
            message_koto.add_value("type", koto_string!("poly_after_touch"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("note", koto_number!(message.note()));
            message_koto.add_value("pressure", koto_number!(message.pressure()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "poly_after_touch requires a single list of exactly three integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "control_change",
        note,
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = ControlChange::new(*note, *value, *channel);
            message_koto.add_value("type", koto_string!("control_change"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("note", koto_number!(message.note()));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "control_change requires a single list of exactly three integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "program_change",
        program,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = ProgramChange::new(*program, *channel);
            message_koto.add_value("type", koto_string!("program_change"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("program", koto_number!(message.program()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "program_change requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "after_touch",
        pressure,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = AfterTouch::new(*pressure, *channel);
            message_koto.add_value("type", koto_string!("after_touch"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("pressure", koto_number!(message.pressure()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "after_touch requires a single list of exactly two positive integers as its argument"
    );

    add_message_constructor_with_fields!(
        message_constructors,
        "pitch_bend",
        bend_amount,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = PitchBend::new(*bend_amount, *channel);
            message_koto.add_value("type", koto_string!("pitch_bend"));
            message_koto.add_value("category", koto_string!("channel_voice"));
            message_koto.add_value("bend_amount", koto_number!(message.bend_amount() as u64));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "pitch_bend requires a single list of exactly two positive integers as its argument"
    );

    add_message_constructor_with_fields!(
        message_constructors,
        "all_sound_off",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = AllSoundOff::new(*value, *channel);
            message_koto.add_value("type", koto_string!("all_sound_off"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "all_sound_off requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "reset_all_controllers",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = ResetAllControllers::new(*value, *channel);
            message_koto.add_value("type", koto_string!("reset_all_controllers"));
                        message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "reset_all_controllers requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "local_control",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = LocalControl::new(*value, *channel);
            message_koto.add_value("type", koto_string!("local_control"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "local_control requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "all_notes_off",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = AllNotesOff::new(*value, *channel);
            message_koto.add_value("type", koto_string!("all_notes_off"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "all_notes_off requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "omni_mode_off",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = OmniModeOff::new(*value, *channel);
            message_koto.add_value("type", koto_string!("omni_mode_off"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "omni_mode_off requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "omni_mode_on",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = OmniModeOn::new(*value, *channel);
            message_koto.add_value("type", koto_string!("omni_mode_on"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "omni_mode_on requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "mono_mode_on",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = MonoModeOn::new(*value, *channel);
            message_koto.add_value("type", koto_string!("mono_mode_on"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "mono_mode_on requires a single list of exactly two positive integers as its argument"
    );
    add_message_constructor_with_fields!(
        message_constructors,
        "poly_mode_on",
        value,
        channel,
        {
            let mut message_koto = ValueMap::new();
            let message = PolyModeOn::new(*value, *channel);
            message_koto.add_value("type", koto_string!("poly_mode_on"));
            message_koto.add_value("category", koto_string!("channel_mode"));
            message_koto.add_value("value", koto_number!(message.value()));
            message_koto.add_value("channel", koto_number!(message.channel()));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
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
                                                    if let Ok(m_id) = collect_list_of_u8_for_sysex(manufacturer_id, error_literal) {
                                                        if let Ok(data) = collect_list_of_u8_for_sysex(message, error_literal) {
                                                                let mut message_koto = ValueMap::new();
                                                                let message = SystemExclusive::new(&m_id[..], &data[..]);
                                                                message_koto.add_value("type", koto_string!("system_exclusive"));
                                                                message_koto.add_value("category", koto_string!("system_common"));
                                                                let m_id = m_id.iter().map(|&x| koto_number!(x)).collect::<Vec<Value>>();
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

    add_message_constructor_with_fields!(
        message_constructors,
        "time_code_quarter_frame",
        message_type,
        values,
        {
            let mut message_koto = ValueMap::new();
            let message = TimeCodeQuarterFrame::new(*message_type, *values);
            message_koto.add_value("type", koto_string!("song_position"));
            message_koto.add_value("category", koto_string!("system_common"));
            message_koto.add_value(
                "message_type",
                koto_number!(message.message_type() as u64),
            );
            message_koto.add_value(
                "values",
                koto_number!(message.values() as u64),
            );
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "time_code_quarter_frame requires a single list of exactly two positive integers as its argument"
    );

    add_message_constructor_with_fields!(
        message_constructors,
        "song_position",
        midi_beats_elapsed,
        {
            let mut message_koto = ValueMap::new();
            let message = SongPosition::new(*midi_beats_elapsed);
            message_koto.add_value("type", koto_string!("song_position"));
            message_koto.add_value("category", koto_string!("system_common"));
            message_koto.add_value(
                "midi_beats_elapsed",
                koto_number!(message.midi_beats_elapsed() as u64),
            );
            message_koto.add_value(
                "midi_clocks_elapsed",
                koto_number!((message.midi_beats_elapsed() as u64) * 6),
            );
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "song_position requires a single list of exactly one positive integer as its argument"
    );

    add_message_constructor_with_fields!(
        message_constructors,
        "song_select",
        number,
        {
            let mut message_koto = ValueMap::new();
            let message = SongSelect::new(*number);
            message_koto.add_value("type", koto_string!("song_select"));
            message_koto.add_value("category", koto_string!("system_common"));
            message_koto.add_value("number", koto_number!(message.number() as u64));
            impl_pack!(message_koto, message);
            Ok(Value::Map(message_koto))
        },
        "song_select requires a single list of exactly one positive integer as its argument"
    );

    add_plain_message_constructor!(
        message_constructors,
        "tune_request",
        "system_common",
        TuneRequest,
        "tune_request does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "end_of_exclusive",
        "system_common",
        EndOfExclusive,
        "end_of_exclusive does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "timing_clock",
        "system_realtime",
        TimingClock,
        "timing_clock does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "start",
        "system_realtime",
        Start,
        "start does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "continue",
        "system_realtime",
        Continue,
        "continue does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "stop",
        "system_realtime",
        Stop,
        "stop does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "active_sensing",
        "system_realtime",
        ActiveSensing,
        "active_sensing does not take any arguments"
    );
    add_plain_message_constructor!(
        message_constructors,
        "reset",
        "system_realtime",
        Reset,
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
                                message_koto.add_value("category", koto_string!("channel_voice"))
                            }
                            Message::AllSoundOff(_)
                            | Message::ResetAllControllers(_)
                            | Message::LocalControl(_)
                            | Message::AllNotesOff(_)
                            | Message::OmniModeOff(_)
                            | Message::OmniModeOn(_)
                            | Message::MonoModeOn(_)
                            | Message::PolyModeOn(_) => {
                                message_koto.add_value("category", koto_string!("channel_mode"))
                            }
                            Message::SystemExclusive(_)
                            | Message::SongPosition(_)
                            | Message::SongSelect(_)
                            | Message::TuneRequest(_)
                            | Message::EndOfExclusive(_)
                            | Message::TimeCodeQuarterFrame(_) => {
                                message_koto.add_value("category", koto_string!("system_common"))
                            }
                            Message::TimingClock(_)
                            | Message::Start(_)
                            | Message::Continue(_)
                            | Message::Stop(_)
                            | Message::ActiveSensing(_)
                            | Message::Reset(_) => {
                                message_koto.add_value("category", koto_string!("system_realtime"))
                            }
                            Message::Undefined | Message::Malformed => {
                                message_koto.add_value("category", koto_string!("unknown"))
                            }
                        };

                        match message {
                            Message::NoteOff(message) => {
                                message_koto.add_value("type", koto_string!("note_off"));
                                message_koto.add_value("note", koto_number!(message.note()));
                                message_koto
                                    .add_value("velocity", koto_number!(message.velocity()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::NoteOn(message) => {
                                message_koto.add_value("type", koto_string!("note_on"));
                                message_koto.add_value("note", koto_number!(message.note()));
                                message_koto
                                    .add_value("velocity", koto_number!(message.velocity()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::ControlChange(message) => {
                                message_koto.add_value("type", koto_string!("control_change"));
                                message_koto.add_value("note", koto_number!(message.note()));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::ProgramChange(message) => {
                                message_koto.add_value("type", koto_string!("program_change"));
                                message_koto.add_value("program", koto_number!(message.program()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }

                            Message::AfterTouch(message) => {
                                message_koto.add_value("type", koto_string!("after_touch"));

                                message_koto
                                    .add_value("pressure", koto_number!(message.pressure()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::PolyAfterTouch(message) => {
                                message_koto.add_value("note", koto_number!(message.note()));
                                message_koto.add_value("type", koto_string!("poly_after_touch"));
                                message_koto
                                    .add_value("pressure", koto_number!(message.pressure()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::PitchBend(message) => {
                                message_koto.add_value("type", koto_string!("pitch_bend"));
                                message_koto.add_value(
                                    "bend_amount",
                                    koto_number!(message.bend_amount() as i64),
                                );
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::AllSoundOff(message) => {
                                message_koto.add_value("type", koto_string!("all_sound_off"));
                                message_koto.add_value("note", koto_number!(120));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::ResetAllControllers(message) => {
                                message_koto
                                    .add_value("type", koto_string!("reset_all_controllers"));
                                message_koto.add_value("note", koto_number!(121));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::LocalControl(message) => {
                                message_koto.add_value("type", koto_string!("local_control"));
                                message_koto.add_value("note", koto_number!(122));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::AllNotesOff(message) => {
                                message_koto.add_value("type", koto_string!("all_notes_off"));
                                message_koto.add_value("note", koto_number!(123));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::OmniModeOff(message) => {
                                message_koto.add_value("type", koto_string!("omni_mode_off"));
                                message_koto.add_value("note", koto_number!(124));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::OmniModeOn(message) => {
                                message_koto.add_value("type", koto_string!("omni_mode_on"));
                                message_koto.add_value("note", koto_number!(125));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::MonoModeOn(message) => {
                                message_koto.add_value("type", koto_string!("mono_mode_on"));
                                message_koto.add_value("note", koto_number!(126));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::PolyModeOn(message) => {
                                message_koto.add_value("type", koto_string!("poly_mode_on"));
                                message_koto.add_value("note", koto_number!(127));
                                message_koto.add_value("value", koto_number!(message.value()));
                                message_koto.add_value("channel", koto_number!(message.channel()));
                                impl_pack!(message_koto, message);
                            }
                            Message::SystemExclusive(message) => {
                                message_koto.add_value("type", koto_string!("system_exclusive"));
                                let m_id = message.manufacturer_id.iter().map(|&x| koto_number!(x)).collect::<Vec<Value>>();
                                message_koto.add_value("manufacturer_id", Value::List(ValueList::from_slice(&m_id[..])));
                                impl_pack!(message_koto, message);
                            }
                            Message::SongPosition(message) => {
                                message_koto.add_value("type", koto_string!("song_position"));
                                message_koto.add_value(
                                    "midi_beats_elapsed",
                                    koto_number!(message.midi_beats_elapsed() as i64),
                                );
                                message_koto.add_value(
                                    "midi_clocks_elapsed",
                                    koto_number!(message.midi_clocks_elapsed()),
                                );
                                impl_pack!(message_koto, message);
                            }
                            Message::SongSelect(message) => {
                                message_koto.add_value("type", koto_string!("song_select"));
                                message_koto.add_value("number", koto_number!(message.number()));
                                impl_pack!(message_koto, message);
                            }
                            Message::TuneRequest(message) => {
                                message_koto.add_value("type", koto_string!("tune_request"));
                                impl_pack!(message_koto, message);
                            }
                            Message::EndOfExclusive(message) => {
                                message_koto.add_value("type", koto_string!("end_of_exclusive"));
                                impl_pack!(message_koto, message);
                            }
                            Message::TimeCodeQuarterFrame(message) => {
                                message_koto
                                    .add_value("type", koto_string!("time_code_quarter_frame"));
                                message_koto.add_value(
                                    "message_type",
                                    koto_number!(message.message_type()),
                                );
                                message_koto.add_value("values", koto_number!(message.values()));
                                impl_pack!(message_koto, message);
                            }
                            Message::TimingClock(message) => {
                                message_koto.add_value("type", koto_string!("timing_clock"));
                                impl_pack!(message_koto, message);
                            }
                            Message::Start(message) => {
                                message_koto.add_value("type", koto_string!("start"));
                                impl_pack!(message_koto, message);
                            }
                            Message::Continue(message) => {
                                message_koto.add_value("type", koto_string!("continue"));
                                impl_pack!(message_koto, message);
                            }
                            Message::Stop(message) => {
                                message_koto.add_value("type", koto_string!("stop"));
                                impl_pack!(message_koto, message);
                            }
                            Message::ActiveSensing(message) => {
                                message_koto.add_value("type", koto_string!("active_sensing"));
                                impl_pack!(message_koto, message);
                            }
                            Message::Reset(message) => {
                                message_koto.add_value("type", koto_string!("reset"));
                                impl_pack!(message_koto, message);
                            }
                            Message::Undefined => {
                                message_koto.add_value("type", koto_string!("undefined"));
                                message_koto.add_value("category", koto_string!("unknown"));
                            }
                            Message::Malformed => {
                                message_koto.add_value("type", koto_string!("malformed"));
                                message_koto.add_value("category", koto_string!("unknown"));
                            }
                        }

                        Ok(Value::Map(message_koto))
                    } else {
                        message_koto.add_value("type", koto_string!("malformed"));
                        message_koto.add_value("category", koto_string!("unknown"));
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
