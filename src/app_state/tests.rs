use super::AppState;
use crate::{
    arg_state::{ArgKind, ArgState},
    settings::Localization,
};
use clap::builder::NonEmptyStringValueParser;
use clap::{CommandFactory, FromArgMatches, Parser, ValueHint};
use std::{fmt::Debug, path::PathBuf};
use uuid::Uuid;

#[derive(Debug, Parser, PartialEq, Eq)]
struct Simple {
    #[arg(long)]
    single: String,
    #[arg(long)]
    optional_no_enter: Option<String>,
    #[arg(long)]
    optional_enter: Option<String>,
    #[arg(long)]
    flag_true: bool,
    #[arg(long)]
    flag_false: bool,
    #[arg(long, action = clap::ArgAction::Count)]
    occurrences: u8,
}

#[test]
fn simple() {
    test_app(
        |args| {
            args[0].enter("a");
            args[2].enter("b");
            args[3].set(true);
            args[5].occurrences(3);
        },
        Simple {
            single: "a".into(),
            optional_no_enter: None,
            optional_enter: Some("b".into()),
            flag_true: true,
            flag_false: false,
            occurrences: 3,
        },
    )
}

#[derive(Debug, Parser, PartialEq, Eq)]
struct ForbidEmpty {
    #[arg(long, value_parser = NonEmptyStringValueParser::new())]
    optional_no_empty1: Option<String>,
    #[arg(long, value_parser = NonEmptyStringValueParser::new())]
    optional_no_empty2: Option<String>,
    #[arg(long, value_parser = NonEmptyStringValueParser::new())]
    optional_no_empty3: Option<String>,
}

#[test]
fn forbid_empty() {
    test_app(
        |args| {
            args[0].enter("a");
            args[2].enter("");
        },
        ForbidEmpty {
            optional_no_empty1: Some("a".into()),
            optional_no_empty2: None,
            optional_no_empty3: None,
        },
    );
}

#[derive(Debug, Parser, PartialEq, Eq)]
struct OptionalAndDefault {
    required: String,
    optional: Option<String>,
    #[arg(default_value = "d")]
    default: String,
}

#[test]
fn optional_and_default() {
    test_app(
        |args| args[0].enter("a"),
        OptionalAndDefault {
            required: "a".into(),
            optional: None,
            default: "d".into(),
        },
    );
}

#[derive(Debug, Parser, PartialEq, Eq)]
struct UseEquals {
    #[arg(long, require_equals = true)]
    long: String,
    #[arg(short, require_equals = true)]
    short: String,
    #[arg(long, require_equals = true, value_hint = ValueHint::AnyPath)]
    path: PathBuf,
    #[arg(long, require_equals = true, value_parser = ["P", "O"])]
    choose: String,
    #[arg(long, require_equals = true)]
    multiple_enter_one: Vec<String>,
    #[arg(long, require_equals = true)]
    multiple_occurrences: Vec<String>,
    #[arg(long, action = clap::ArgAction::Count)]
    occurrences: u8,
    #[arg(long)]
    flag: bool,
}

#[test]
fn use_equals() {
    test_app(
        |args| {
            enter_consecutive(args, ["a", "b", "c", "P"]);
            args[4].enter_multiple(["d"]);
            args[5].enter_multiple(["e", "f"]);
            args[6].occurrences(3);
            args[7].set(true);
        },
        UseEquals {
            long: "a".into(),
            short: "b".into(),
            path: "c".into(),
            choose: "P".into(),
            multiple_enter_one: vec!["d".into()],
            multiple_occurrences: vec!["e".into(), "f".into()],
            occurrences: 3,
            flag: true,
        },
    );
}

#[derive(Debug, Parser, PartialEq, Eq)]
struct DifferentMultipleValues {
    #[arg(long, require_equals = true)]
    multiple_equals_enter_one: Vec<String>,
    #[arg(long, require_equals = true)]
    multiple_occurrences_equals: Vec<String>,
    #[arg(long)]
    multiple_occurrences: Vec<String>,
    #[arg(long)]
    multiple: Vec<String>,
    #[arg(long, require_equals = true, value_delimiter = ',')]
    multiple_equals_use_delim: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    multiple_use_delim: Vec<String>,
    #[arg(long)]
    multiple_none_entered: Vec<String>,
    #[arg(long, require_equals = true)]
    multiple_equals_none_entered: Vec<String>,
    #[arg(long, value_delimiter = ',')]
    multiple_req_delim_none_entered: Vec<String>,
}

#[test]
fn different_multiple_values() {
    test_app(
        |args| {
            args[0].enter_multiple(["a"]);
            args[1].enter_multiple(["b", "c"]);
            args[2].enter_multiple(["d", "e"]);
            args[3].enter_multiple(["f", "g"]);
            args[4].enter_multiple(["h", "i"]);
            args[5].enter_multiple(["l", "m"]);
        },
        DifferentMultipleValues {
            multiple_equals_enter_one: vec!["a".into()],
            multiple_occurrences_equals: vec!["b".into(), "c".into()],
            multiple_occurrences: vec!["d".into(), "e".into()],
            multiple: vec!["f".into(), "g".into()],
            multiple_equals_use_delim: vec!["h".into(), "i".into()],
            multiple_use_delim: vec!["l".into(), "m".into()],
            multiple_none_entered: vec![],
            multiple_equals_none_entered: vec![],
            multiple_req_delim_none_entered: vec![],
        },
    )
}

#[derive(Debug, Parser, PartialEq, Eq)]
struct PositionalBool {
    verbose: bool,
}

#[derive(Debug, Parser, PartialEq, Eq)]
struct MultipleOccurrences {
    #[arg(short, long, num_args(1))]
    a: Vec<PathBuf>,
}

#[test]
fn multiple_occurrences() {
    test_app(
        |args| args[0].enter_multiple(["a", "b"]),
        MultipleOccurrences {
            a: vec!["a".into(), "b".into()],
        },
    )
}

fn test_app<C, F>(setup: F, expected: C)
where
    C: CommandFactory + FromArgMatches + Debug + Eq,
    F: FnOnce(&mut Vec<ArgState>),
{
    let app = C::command();
    let localization = Localization::default();
    let mut app_state = AppState::new(&app, &localization);
    setup(&mut app_state.args);
    let args = app_state.get_cmd_args(vec!["_name".into()]).unwrap();
    eprintln!("Args: {:?}", &args[1..]);
    let matches = app.try_get_matches_from(args.iter()).unwrap();
    let c = C::from_arg_matches(&matches).unwrap();
    assert_eq!(c, expected);
}

fn enter_consecutive<const N: usize>(args: &mut [ArgState], vals: [&str; N]) {
    for i in 0..N {
        args[i].enter(vals[i]);
    }
}

impl crate::arg_state::ArgState<'_> {
    fn enter(&mut self, val: &str) {
        if let ArgKind::String { value, .. } = &mut self.kind {
            value.0 = val.to_string();
        } else {
            panic!("Called enter on {:?}", self)
        }
    }

    fn enter_multiple<const N: usize>(&mut self, vals: [&str; N]) {
        if let ArgKind::MultipleStrings { values, .. } = &mut self.kind {
            *values = vals
                .iter()
                .map(|s| (s.to_string(), Uuid::new_v4()))
                .collect()
        } else {
            panic!("Called enter_multiple on {:?}", self)
        }
    }

    fn occurrences(&mut self, val: u8) {
        if let ArgKind::Occurrences(i) = &mut self.kind {
            *i = val;
        } else {
            panic!("Called occurrences on {:?}", self)
        }
    }

    fn set(&mut self, val: bool) {
        if let ArgKind::Bool(b) = &mut self.kind {
            *b = val;
        } else {
            panic!("Called set on {:?}", self)
        }
    }
}
