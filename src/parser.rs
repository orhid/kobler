use serenity::framework::standard::Args;
use std::error::Error;

#[derive(Debug)]
pub struct InvalidArgument(String);

impl InvalidArgument {
    pub const fn new(string: String) -> Self {
        Self(string)
    }
}

impl std::fmt::Display for InvalidArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "niepoprawny argument: {}", self.0)
    }
}

impl Error for InvalidArgument {}

enum ArgType {
    Plain,
    Short,
    Long,
}

impl ArgType {
    #[allow(clippy::option_if_let_else)] // s is moved
    fn from(s: &str) -> (Self, String) {
        if let Some(inside) = s.strip_prefix("--") {
            (Self::Long, inside.to_owned().to_lowercase())
        } else if let Some(inside) = s.strip_prefix('-') {
            (Self::Short, inside.to_owned().to_lowercase())
        } else {
            (Self::Plain, s.to_lowercase())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Arg {
    Plain(String),
    Short(char, Vec<String>),
    Long(String, Vec<String>),
}

impl Arg {
    pub fn try_parse(mut args: Args) -> Result<Vec<Self>, impl Error> {
        let mut parsed_args = Vec::new();
        // todo
        while let Ok(s) = args.single::<String>() {
            match ArgType::from(&s) {
                (ArgType::Plain, inside) => parsed_args.push(Self::Plain(inside)),
                (ArgType::Short, inside) => {
                    let mut chars = inside.chars();
                    if let Some(ch) = chars.next() {
                        let mut params = Vec::new();
                        let other = chars.collect::<String>();
                        if !other.is_empty() {
                            params.push(other);
                        }

                        while let Ok(string) = args.parse::<String>() && let (ArgType::Plain,ininside) = ArgType::from(&string)
                        {
                            params.push(ininside);
                            args.advance();
                        }

                        parsed_args.push(Self::Short(ch, params));
                    } else {
                        return Err(InvalidArgument(inside));
                    }
                }
                (ArgType::Long, inside) => {
                    let mut params = Vec::new();

                    while let Ok(string) = args.parse::<String>() && let (ArgType::Plain,ininside) = ArgType::from(&string)
                        {
                            params.push(ininside);
                            args.advance();
                        }

                    parsed_args.push(Self::Long(inside, params));
                }
            }
        }

        Ok(parsed_args)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serenity::framework::standard::Delimiter;

    #[test]
    fn parser() {
        // todo
        assert_eq!(
            Arg::try_parse(Args::new("wzorzec --chojraczka", &[Delimiter::Single(' ')])).ok(),
            Some(vec![
                Arg::Plain("wzorzec".to_owned()),
                Arg::Long("chojraczka".to_owned(), vec![])
            ])
        );
        assert_eq!(
            Arg::try_parse(Args::new(
                "broń dodaj -n rozkurwiator -wC --zasięg B",
                &[Delimiter::Single(' ')]
            ))
            .ok(),
            Some(vec![
                Arg::Plain("broń".to_owned()),
                Arg::Plain("dodaj".to_owned()),
                Arg::Short('n', vec!["rozkurwiator".to_owned()]),
                Arg::Short('w', vec!["c".to_owned()]),
                Arg::Long("zasięg".to_owned(), vec!["b".to_owned()])
            ])
        );
        assert_eq!(
            Arg::try_parse(Args::new("próba -s -nK", &[Delimiter::Single(' ')])).ok(),
            Some(vec![
                Arg::Plain("próba".to_owned()),
                Arg::Short('s', vec![]),
                Arg::Short('n', vec!["k".to_owned()]),
            ])
        );
        assert_eq!(
            Arg::try_parse(Args::new("bitwa -m 1", &[Delimiter::Single(' ')])).ok(),
            Some(vec![
                Arg::Plain("bitwa".to_owned()),
                Arg::Short('m', vec!["1".to_owned()]),
            ])
        );
        assert_eq!(
            Arg::try_parse(Args::new("zanik 2 -z", &[Delimiter::Single(' ')])).ok(),
            Some(vec![
                Arg::Plain("zanik".to_owned()),
                Arg::Plain("2".to_owned()),
                Arg::Short('z', vec![]),
            ])
        );
    }
}
