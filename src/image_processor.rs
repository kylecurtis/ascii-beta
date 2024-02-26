use image::{imageops::FilterType, DynamicImage, ImageError};
use terminal_size::{terminal_size, Height, Width};

pub fn process_image(
    path: &str,
    scale: u32,
    character_set: &str,
    invert: bool,
) -> Result<String, ImageError> {
    // Returns the ASCII art as a string or an error

    // Open the image file.
    // The '?' handles any errors by returning them.
    let img: DynamicImage = image::open(path)?;

    // Define a character aspect ratio to adjust the image's dimensions based on terminal display characteristics.
    let char_aspect_ratio: f32 = 0.5; // A typical value; might need adjustment for different terminals

    // Get terminal dimensions
    let term_dimensions: Option<(f32, f32)> =
        terminal_size().map(|(Width(w), Height(h))| (w as f32, h as f32));
    let (term_width, term_height) = term_dimensions.unwrap_or((80.0, 24.0));

    // Calculate baseline scale to fit the image within terminal dimensions
    let scale_factor: f32 = scale as f32 / 100.0;
    let adjusted_width_factor: f32 = 1.0 / char_aspect_ratio;

    let new_width = (img.width() as f32 * scale_factor * adjusted_width_factor).round() as u32;
    let new_height = (img.height() as f32 * scale_factor).round() as u32;

    let img: DynamicImage = img.resize_exact(new_width, new_height, FilterType::Triangle);

    Ok(image_to_ascii(&img, character_set, invert))
}

fn image_to_ascii(img: &DynamicImage, character_set: &str, invert: bool) -> String {
    let img: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = img.to_luma8();
    let mut ascii_art: String = String::new();

    // Example: Selecting character sets based on a preset name
    let characters = match character_set {
        // SET1 (WIP Set)
        // =================================================================================================
        "set1" => vec!['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '],

        // SET2 (WIP Set)
        // =================================================================================================
        "set2" => vec!['█', '▓', '▒', '░', '#', '.', ' '],

        // DEFAULT SET (Standard Set)
        // =================================================================================================
        _ => vec![
            '$', '@', 'B', '%', '8', '&', 'W', 'M', '#', '*', 'o', 'a', 'h', 'k', 'b', 'd', 'p',
            'q', 'w', 'm', 'Z', 'O', '0', 'Q', 'L', 'C', 'J', 'U', 'Y', 'X', 'z', 'c', 'v', 'u',
            'n', 'x', 'r', 'j', 'f', 't', '/', '\\', '|', '(', ')', '1', '{', '}', '[', ']', '?',
            '-', '_', '+', '~', '<', '>', 'i', '!', 'l', 'I', ';', ':', ',', '"', '^', '`', '\'',
            '.', ' ',
        ],
    };

    // Invert the character set if requested
    let characters = if invert {
        characters.into_iter().rev().collect()
    } else {
        characters
    };

    let interval = 255 / (characters.len() - 1) as u8;

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let index = (pixel[0] as usize) / interval as usize;
            let index = index.min(characters.len() - 1); // Ensure index stays within bounds
            ascii_art.push(characters[index]);
        }
        ascii_art.push('\n');
    }

    ascii_art
}
