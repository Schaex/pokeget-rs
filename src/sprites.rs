use image::{DynamicImage, GenericImage};

use crate::pokemon::Pokemon;

/// Combines several pokémon sprites into one by stitching them horizontally.
pub fn combine(pokemon: &[Pokemon]) -> DynamicImage {
    let mut width: u32 = 0;
    let mut height: u32 = 0;

    for single_pokemon in pokemon {
        width += single_pokemon.sprite.width() + 1;
        if single_pokemon.sprite.height() > height {
            height = single_pokemon.sprite.height();
        }
    }

    let mut combined = DynamicImage::new_rgba8(width - 1, height);
    let mut shift = 0;

    for pokemon in pokemon {
        combined
            .copy_from(&pokemon.sprite, shift, height - pokemon.sprite.height())
            .unwrap();
        shift += pokemon.sprite.width() + 1;
    }

    combined
}
