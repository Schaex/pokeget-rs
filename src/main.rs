//! Display pokémon sprites in your terminal.

use clap::Parser;
use pokeget::cli::Args;
use pokeget::list::List;
use pokeget::pokemon::{Attributes, Pokemon};
use pokeget::sprites;
use std::process::exit;

fn main() {
    let list = List::read();
    let args = Args::parse();

    if args.pokemon.is_empty() {
        eprintln!("you must specify the pokémon you want to display");
        exit(1);
    }

    let attributes = Attributes::new(&args);
    let pokemon: Vec<Pokemon> = args
        .pokemon
        .into_iter()
        .map(|x| Pokemon::new(x, &list, &attributes))
        .collect();

    let combined = sprites::combine(&pokemon);
    if !args.hide_name {
        let names: Vec<&str> = pokemon.iter().map(|x| x.name.as_ref()).collect();
        eprintln!("{}", names.join(", "));
    }

    println!("{}", showie::render(&combined));
}
