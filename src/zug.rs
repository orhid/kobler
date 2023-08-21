use crate::parser::{Arg, InvalidArgument};
use itertools::Itertools;
use rand::{
    distributions::{Bernoulli, Distribution},
    seq::SliceRandom,
    thread_rng,
};
use serenity::framework::standard::CommandResult;
use std::fmt;
use strsim::damerau_levenshtein as dist;

const ZNAKI: &str = "𝚯𝚲";
const SUN: &str = "𝚯";
const MUN: &str = "𝚲";

/* # wzorzec */

#[derive(Clone, Copy)]
pub enum Wzorzec {
    Chojrak,
    Szelma,
}

impl Wzorzec {
    pub fn try_parse(arg: &Arg) -> Option<Self> {
        match arg {
            Arg::Short('c', _) => Some(Self::Chojrak),
            Arg::Long(word, _) if dist(word, "chojrak") < 4 || dist(word, "chojraczka") < 4 => {
                Some(Self::Chojrak)
            }
            Arg::Short('w', _) => Some(Self::Szelma),
            Arg::Long(word, _) if dist(word, "szelma") < 4 => Some(Self::Chojrak),
            _ => None,
        }
    }

    pub const fn die(self) -> [&'static str; 6] {
        match self {
            Self::Chojrak => [SUN, SUN, SUN, MUN, MUN, ""],
            Self::Szelma => [SUN, SUN, MUN, MUN, MUN, ""],
        }
    }
}

impl fmt::Display for Wzorzec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Chojrak => "Chojrak",
                Self::Szelma => "Szelma",
            }
        )
    }
}

/* # szkolenie */

#[derive(Clone, Copy)]
pub enum Fach {
    Zielony,
    Szkolony,
    Biegły,
}

impl Fach {
    const fn dice(self) -> usize {
        match self {
            Self::Zielony => 4,
            Self::Szkolony => 5,
            Self::Biegły => 6,
        }
    }

    pub fn try_parse(arg: &Arg) -> Option<Self> {
        match arg {
            Arg::Short('s', _) => Some(Self::Szkolony),
            Arg::Short('b', _) => Some(Self::Biegły),
            Arg::Long(word, _) if dist(word, "znakomita") < 4 => Some(Self::Szkolony),
            Arg::Long(word, _) if dist(word, "przyzwoita") < 4 => Some(Self::Biegły),
            _ => None,
        }
    }
}

impl Default for Fach {
    fn default() -> Self {
        Self::Zielony
    }
}

/* # narzędzia */

#[derive(Clone, Copy)]
pub enum Narzędzie {
    Kiepskie,
    Przyzwoite,
    Znakomite,
}

impl Narzędzie {
    pub fn try_parse(arg: &Arg) -> Option<Self> {
        match arg {
            Arg::Short('z', _) => Some(Self::Znakomite),
            Arg::Short('p', _) => Some(Self::Przyzwoite),
            Arg::Short('k', _) => Some(Self::Kiepskie),
            Arg::Long(word, _) if dist(word, "znakomita") < 4 => Some(Self::Znakomite),
            Arg::Long(word, _) if dist(word, "przyzwoita") < 4 => Some(Self::Przyzwoite),
            Arg::Long(word, _) if dist(word, "kiepska") < 4 => Some(Self::Kiepskie),
            _ => None,
        }
    }

    const fn die(self) -> [&'static str; 4] {
        match self {
            Self::Kiepskie => ["XX", "X", "", ""],
            Self::Przyzwoite => ["X", "X", "", ""],
            Self::Znakomite => ["X", "", "", ""],
        }
    }

    fn decay(self) -> CommandResult<f64> {
        Ok(f64::from(u8::try_from(
            self.die()
                .into_iter()
                .filter(|sigils| sigils.is_empty())
                .count(),
        )?) / 4.0)
    }
}

impl Default for Narzędzie {
    fn default() -> Self {
        Self::Przyzwoite
    }
}

/* # broń */

#[derive(Clone, Copy)]
pub enum Zasięg {
    Biała,
    Miotająca,
}

impl Zasięg {
    fn try_parse(arg: &Arg) -> Option<Self> {
        match arg {
            Arg::Short('z', params) => params.last().and_then(|s| match s {
                x if x == "b" => Some(Self::Biała),
                x if x == "m" || x == "z" => Some(Self::Miotająca),
                _ => None,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Waga {
    Lekka,
    Ciężka,
}

impl Waga {
    fn parse_helper(params: &[String]) -> Option<Self> {
        params.last().and_then(|s| match s {
            x if x == "l" || dist(x, "lekka") < 3 => Some(Self::Lekka),
            x if x == "c" || dist(x, "ciężka") < 3 => Some(Self::Ciężka),
            _ => None,
        })
    }

    fn try_parse(arg: &Arg) -> Option<Self> {
        match arg {
            Arg::Short('z', params) => Self::parse_helper(params),
            Arg::Long(p, params) if dist(p, "zasięg") < 3 => Self::parse_helper(params),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Broń {
    waga: Waga,
    zasięg: Zasięg,
}

impl Broń {
    pub fn try_parse(args: &[Arg]) -> Result<Self, InvalidArgument> {
        args.iter().filter_map(Waga::try_parse).last().map_or_else(
            || {
                Err(InvalidArgument::new(
                    "nie podano argumentu wagi.".to_owned(),
                ))
            },
            |waga| {
                args.iter()
                    .filter_map(Zasięg::try_parse)
                    .last()
                    .map_or_else(
                        || {
                            Err(InvalidArgument::new(
                                "nie podano argumentu zasięgu.".to_owned(),
                            ))
                        },
                        |zasięg| Ok(Self { waga, zasięg }),
                    )
            },
        )
    }

    pub fn die(self) -> [&'static str; 4] {
        Narzędzie::from(self).die()
    }

    pub const fn waga_str(self) -> &'static str {
        match self.waga {
            Waga::Lekka => "lekka",
            Waga::Ciężka => "ciężka",
        }
    }

    pub const fn zasięg_str(self) -> &'static str {
        match self.zasięg {
            Zasięg::Biała => "biała",
            Zasięg::Miotająca => "miotająca",
        }
    }
}

impl From<Broń> for Narzędzie {
    fn from(broń: Broń) -> Self {
        match (broń.waga, broń.zasięg) {
            (Waga::Lekka, Zasięg::Miotająca) => Self::Kiepskie,
            (Waga::Ciężka, Zasięg::Miotająca) | (Waga::Lekka, Zasięg::Biała) => {
                Self::Przyzwoite
            }
            (Waga::Ciężka, Zasięg::Biała) => Self::Znakomite,
        }
    }
}

/* to be sorted */

#[allow(clippy::match_bool)] // i think this is more readable
fn encapsulate(s: &str, i: bool) -> String {
    match i {
        true => "(".to_owned() + s + ")",
        false => "[".to_owned() + s + "]",
    }
}

pub fn próba(wzór: Wzorzec, fach: Fach, maybe_narzędzie: Option<Narzędzie>) -> String {
    let mut rng = thread_rng();
    let mut results = (0..fach.dice())
        .filter_map(|_| wzór.die().choose(&mut rng).copied())
        .map(|s| encapsulate(s, false))
        .collect::<Vec<String>>();
    if let Some(narzędzie) =
        maybe_narzędzie.and_then(|narzędzie| narzędzie.die().choose(&mut rng).copied())
    {
        results.push(encapsulate(narzędzie, true));
    };
    let mut concise = results.join("").chars().sorted().collect::<String>();
    concise.retain(|c| ZNAKI.contains(c));
    let results = results.join(" ");
    format!("{results}   =>>   {concise}")
}

pub fn bitwa<I>(wzór: Wzorzec, bronie: I, modyfikator: isize) -> String
where
    I: Iterator<Item = Broń>,
{
    let mut rng = thread_rng();
    let mut results = (0..(4 + modyfikator))
        .filter_map(|_| wzór.die().choose(&mut rng).copied())
        .map(|s| encapsulate(s, false))
        .collect::<Vec<String>>();
    for broń in bronie.filter_map(|broń| broń.die().choose(&mut rng).copied()) {
        results.push(encapsulate(broń, true));
    }
    let mut concise = results.join("").chars().sorted().collect::<String>();
    concise.retain(|c| ZNAKI.contains(c));
    let results = results.join(" ");
    format!("{results}   =>>   {concise}")
}

#[allow(clippy::match_bool)] // i think this is more readable
pub fn zanik(durability: usize, quality: Narzędzie) -> CommandResult<String> {
    match Bernoulli::new(quality.decay()?.powi(durability.try_into()?))?
        .sample(&mut rand::thread_rng())
    {
        true => Ok("porażka! trwałość twojego sprzętu maleje. ".to_owned()),
        false => Ok("sukces! twój sprzęt utrzymuje trwałość. ".to_owned()),
    }
}
