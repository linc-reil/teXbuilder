use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BiblatexCommand {
    Biblatex,
    Biber,
    None,
}

impl ToString for BiblatexCommand {
    fn to_string(&self) -> String {
        match self {
            &Self::Biber => String::from("biber"),
            &Self::Biblatex => String::from("biblatex"),
            &Self::None => String::from("none"),
        }
    }
}
