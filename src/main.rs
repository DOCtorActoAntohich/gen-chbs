use chbs::config::BasicConfig;
use chbs::probability::Probability;
use chbs::scheme::{Scheme, ToScheme};
use chbs::word::WordList;
use clap::Parser;

#[derive(Parser)]
struct Args {
    /// How many phrases to generate.
    #[arg(long, default_value_t = 1)]
    phrases: u32,
    /// Words per phrase
    #[arg(long, default_value_t = 2)]
    words: u8,
    /// What goes between
    #[arg(long, default_value_t = Separator::Dash)]
    separator: Separator,
    #[arg(long, default_value_t = Capitalize::None)]
    capitalize: Capitalize,
    #[arg(long, default_value_t = WordListOpt::GeneralShort)]
    word_list: WordListOpt,
}

fn main() {
    let args = Args::parse();

    let scheme = generation_scheme(&args);

    (0..args.phrases).for_each(|_| println!("{}", scheme.generate()));
}

fn generation_scheme(args: &Args) -> Scheme {
    let word_list = match args.word_list {
        WordListOpt::GeneralShort => WordList::builtin_eff_general_short(),
        WordListOpt::Short => WordList::builtin_eff_short(),
        WordListOpt::Large => WordList::builtin_eff_large(),
    };

    let capitalize_first = match args.capitalize {
        Capitalize::All => Probability::Always,
        Capitalize::FirstLetter => Probability::Always,
        Capitalize::None => Probability::Never,
    };
    let capitalize_words = match args.capitalize {
        Capitalize::All => Probability::Always,
        Capitalize::FirstLetter => Probability::Never,
        Capitalize::None => Probability::Never,
    };

    BasicConfig {
        words: args.words.into(),
        word_provider: word_list.sampler(),
        separator: args.separator.as_arg(),
        capitalize_first,
        capitalize_words,
    }
    .to_scheme()
}

#[derive(Debug, Clone, Copy, derive_more::FromStr, derive_more::Display)]
enum Separator {
    Dash,
    Space,
    Underscore,
}

impl From<Separator> for String {
    fn from(separator: Separator) -> Self {
        separator.to_string()
    }
}

impl Separator {
    pub fn as_arg(self) -> String {
        match self {
            Separator::Dash => "-",
            Separator::Space => " ",
            Separator::Underscore => "_",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone, Copy, derive_more::FromStr, derive_more::Display)]
#[from_str(rename_all = "kebab-case")]
#[display(rename_all = "kebab-case")]
enum Capitalize {
    All,
    FirstLetter,
    None,
}

#[derive(Debug, Clone, Copy, derive_more::FromStr, derive_more::Display)]
#[from_str(rename_all = "kebab-case")]
#[display(rename_all = "kebab-case")]
enum WordListOpt {
    GeneralShort,
    Short,
    Large,
}
