use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The pokémon to display, use "random" to get a random pokémon, use a region to get a random pokémon from that region
    pub pokemon: Vec<String>,

    /// Whether to hide the pokémon's name which appears above it
    #[arg(long, default_value_t = false)]
    pub hide_name: bool,

    /// The form of the pokémon
    #[arg(short, long, default_value = "")]
    pub form: String,

    /// Display the pokémon as its mega form
    #[arg(short, long, default_value_t = false)]
    pub mega: bool,

    /// Display the pokémon as its mega X form
    #[arg(long, default_value_t = false)]
    pub mega_x: bool,

    /// Display the pokémon as its mega Y form
    #[arg(long, default_value_t = false)]
    pub mega_y: bool,

    /// Display the pokémon as shiny
    #[arg(short, long, default_value_t = false)]
    pub shiny: bool,

    /// Display the alolan variant of the pokémon
    #[arg(short, long, default_value_t = false)]
    pub alolan: bool,

    /// Display the gigantamax variant of the pokémon
    #[arg(short, long, default_value_t = false)]
    pub gmax: bool,

    /// Display the hisui variant of the pokémon
    #[arg(long, default_value_t = false)]
    pub hisui: bool,

    /// Display the noble variant of the pokémon, this option often times only works in tandom with --hisui
    #[arg(short, long, default_value_t = false)]
    pub noble: bool,

    /// Display the galarian variant of the pokémon
    #[arg(long, default_value_t = false)]
    pub galar: bool,

    /// Display the paldean variant of the pokémon
    #[arg(long, default_value_t = false)]
    pub paldea: bool,

    /// Display the female variant of the pokémon if it exists. This doesn't apply to Nidoran. Here, use the suffixes '-F' and '-M' for the female and male forms, respectively
    #[arg(long, default_value_t = false)]
    pub female: bool,
}
