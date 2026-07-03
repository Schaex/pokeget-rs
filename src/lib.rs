use rust_embed::RustEmbed;

pub mod cli;
pub mod list;
pub mod pokemon;
pub mod sprites;

#[derive(RustEmbed)]
#[folder = "data/schaex_merged_sprites"]
pub struct Data;
