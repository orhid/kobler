use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use std::fmt;
use strsim::damerau_levenshtein as dist;

#[derive(Clone, Copy)]
pub enum Archetype {
    Fighter,
    Thief,
    Priest,
}

impl Archetype {
    pub fn parse(arg: &str) -> Option<Self> {
        match arg.to_lowercase() {
            word if word == "w" || dist(&word, "wojownik") < 4 => Some(Self::Fighter),
            word if word == "z" || dist(&word, "złodziej") < 4 => Some(Self::Thief),
            word if word == "k" || dist(&word, "kapłan") < 4 => Some(Self::Priest),
            _ => None,
        }
    }

    pub fn die(&self) -> [&str; 6] {
        match self {
            Self::Fighter => ["BB", "B", "B", "A", "A", "C"],
            Self::Thief => ["AA", "A", "A", "C", "C", "B"],
            Self::Priest => ["CC", "C", "C", "B", "B", "A"],
        }
    }
}

impl fmt::Display for Archetype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Archetype::Fighter => "Wojownik",
                Archetype::Thief => "Złodziej",
                Archetype::Priest => "Kapłan",
            }
        )
    }
}

pub enum Field {
    Untrained,
    Trained,
    Proficient,
}

impl Field {
    fn dice(&self) -> usize {
        match self {
            Self::Untrained => 3,
            Self::Trained => 4,
            Self::Proficient => 5,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Group {
    Agile,
    Brute,
    Clever,
}

impl Group {
    fn valid(c: &str) -> bool {
        c.len() > 0 && "abc".contains(c)
    }

    fn parse(c: &str) -> Self {
        match c {
            "a" => Self::Agile,
            "b" => Self::Brute,
            "c" => Self::Clever,
            _ => panic!("niepoprawny argument grupy."),
        }
    }

    fn sigil(&self) -> &str {
        match self {
            Self::Agile => "A",
            Self::Brute => "B",
            Self::Clever => "C",
        }
    }
}

#[derive(Copy, Clone)]
pub enum Quality {
    Fine,
    Decent,
    Crude,
}

impl Quality {
    fn valid(c: &str) -> bool {
        c.len() > 0 && "fdc".contains(c)
    }

    fn parse(c: &str) -> Self {
        match c {
            "f" => Self::Fine,
            "d" => Self::Decent,
            "c" => Self::Crude,
            _ => panic!("niepoprawny argument jakości."),
        }
    }

    fn die(&self) -> [&str; 6] {
        match self {
            Self::Fine => ["XX", "X", "X", "X", " ", " "],
            Self::Decent => ["XX", "X", "X", " ", " ", " "],
            Self::Crude => ["XX", "X", " ", " ", " ", " "],
        }
    }
}

pub enum Range {
    Melee,
    Ranged,
}

impl Range {
    fn valid(c: &str) -> bool {
        c.len() > 0 && "bd".contains(c)
    }

    fn parse(c: &str) -> Self {
        match c {
            "b" => Self::Melee,
            "d" => Self::Ranged,
            _ => panic!("niepoprawny argument zasięgu."),
        }
    }
}

pub enum Weight {
    Light,
    Heavy,
}

impl Weight {
    fn valid(c: &str) -> bool {
        c.len() > 0 && "lc".contains(c)
    }

    fn parse(c: &str) -> Self {
        match c {
            "l" => Self::Light,
            "c" => Self::Heavy,
            _ => panic!("niepoprawny argument wagi."),
        }
    }
}

pub enum Tool {
    Pure(Quality, Group),
    Weapon(Weight, Range, Group),
    Bare,
}

impl Tool {
    fn simplify(&self) -> Self {
        match self {
            Self::Pure(q, g) => Self::Pure(*q, *g),
            Self::Weapon(w, r, g) => match (w, r) {
                (Weight::Light, Range::Ranged) => Self::Pure(Quality::Crude, *g),
                (Weight::Heavy, Range::Ranged) => Self::Pure(Quality::Decent, *g),
                (Weight::Light, Range::Melee) => Self::Pure(Quality::Decent, *g),
                (Weight::Heavy, Range::Melee) => Self::Pure(Quality::Fine, *g),
            },
            Self::Bare => Self::Bare,
        }
    }

    pub fn parse_tool(arg: Option<&str>) -> Result<Self, String> {
        match arg {
            Some(a) => match a.to_lowercase().split_at(1) {
                (q, g) if Quality::valid(q) && Group::valid(g) => {
                    Ok(Self::Pure(Quality::parse(q), Group::parse(g)))
                }
                _ => Err(format!("niepoprawny argument narzędzia: {}. ", a)),
            },
            None => Err("nie podano argumentu narzędzia. ".to_string()),
        }
    }

    pub fn parse_weapon(arg: Option<&str>) -> Result<Self, String> {
        match arg {
            Some(a) => match a.to_lowercase().split_at(1) {
                (w, rest) if Weight::valid(w) => match rest.split_at(1) {
                    (r, g) if Range::valid(r) && Group::valid(g) => {
                        Ok(
                            Self::Weapon(Weight::parse(w), Range::parse(r), Group::parse(g))
                                .simplify(),
                        )
                    }
                    _ => Err(format!("niepoprawny argument broni: {}. ", a)),
                },
                _ => Err(format!("niepoprawny argument broni: {}. ", a)),
            },
            None => Err("nie podano argumentu broni. ".to_string()),
        }
    }

    fn die_safe(&self) -> [String; 6] {
        match self {
            Self::Pure(q, g) => q.die().map(|s| s.replace("X", g.sigil())),
            _ => panic!("niepoprawny argument bezpiecznej kości."),
        }
    }

    pub fn die(&self) -> Option<[String; 6]> {
        match self {
            Self::Pure(_q, _g) => Some(self.die_safe()),
            Self::Weapon(_w, _r, _g) => Some(self.simplify().die_safe()),
            Self::Bare => None,
        }
    }
}

fn encapsulate(s: &str, i: bool) -> String {
    match i {
        true => ("(".to_owned() + s + ")").to_string(),
        false => ("[".to_owned() + s + "]").to_string(),
    }
}

pub fn dice(wzór: &Archetype, field: &Field, tool: &Tool) -> String {
    let mut rng = thread_rng();
    let mut results = (0..field.dice())
        .map(|_| {
            encapsulate(
                wzór
                    .die()
                    .choose(&mut rng)
                    .expect("kostka nie będzie pusta"),
                false,
            )
        })
        .collect::<Vec<String>>();
    match tool.die() {
        Some(die) => results.push(encapsulate(
            die.choose(&mut rng).expect("kostka nie będzie pusta"),
            true,
        )),
        None => (),
    };
    let mut concise = results.join("").chars().sorted().collect::<String>();
    concise.retain(|c| "ABCR".contains(c));
    let results = results.join(" ");
    format!("{}   =>>   {}", results, concise)
}
