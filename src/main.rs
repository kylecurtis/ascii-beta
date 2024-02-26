use clap::{Arg, ArgMatches, Command};
mod image_processor;

fn main() {
    let matches: ArgMatches = Command::new("Image to ASCII Converter")
        .version("1.0")
        .author("Kyle Curtis")
        .about("Converts various media types to ASCII art")
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .value_name("FILE")
                .help("Sets the input image file")
                .required(false),
        )
        .arg(
            Arg::new("scale")
                .short('s')
                .long("scale")
                .value_name("SCALE")
                .help("Sets the scale percentage (1-100)")
                .required(false),
        )
        .arg(
            Arg::new("set")
                .short('c')
                .long("set")
                .value_name("CHARSET")
                .help("Selects the character set preset (default, set1, set2)")
                .required(false)
                .default_value("default"),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .help("Inverts the character set brightness")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let image_path = matches.get_one::<String>("image").unwrap();
    let scale = matches
        .get_one::<String>("scale")
        .unwrap()
        .parse::<u32>()
        .expect("Scale must be a number");
    let character_set = matches.get_one::<String>("set").unwrap();
    let invert = matches.get_flag("invert");

    match image_processor::process_image(image_path, scale, character_set, invert) {
        Ok(ascii_art) => println!("{}", ascii_art),
        Err(e) => eprintln!("Error processing image: {}", e),
    }
}
