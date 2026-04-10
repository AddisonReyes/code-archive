use clap::{Arg, Command};

fn main() {
    let matches = Command::new("cli-app")
        .version("1.0")
        .author("Addison Reyes")
        .about("A simple CLI app")
        .subcommand(
            Command::new("resize")
                .about("Resizes an image")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Sets the input file path")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file path")
                        .required(true),
                )
                .arg(
                    Arg::new("width")
                        .short('w')
                        .long("width")
                        .value_name("WIDTH")
                        .help("Sets the new width")
                        .required(true),
                )
                .arg(
                    Arg::new("height")
                        .short('H')
                        .long("height")
                        .value_name("HEIGHHT")
                        .help("Sets the new height")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("convert")
                .about("Converts an image")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Sets the input file path")
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Sets the output file path")
                        .required(true),
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .value_name("FORMAT")
                        .help("Sets the output format (e.g. png, jpg)")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("resize", sub_maches)) => {
            let input = sub_maches
                .get_one::<String>("input")
                .expect("Input file is required");

            let output = sub_maches
                .get_one::<String>("output")
                .expect("Output file is required");

            let width = sub_maches
                .get_one::<String>("width")
                .expect("Width is required");

            let height = sub_maches
                .get_one::<String>("height")
                .expect("Height is required");

            println!("Resizing {input} to {width}x{height} and saving to {output}");
        }

        Some(("convert", sub_maches)) => {
            let input = sub_maches
                .get_one::<String>("input")
                .expect("Input file is required");

            let output = sub_maches
                .get_one::<String>("output")
                .expect("Output file is required");

            let format = sub_maches
                .get_one::<String>("format")
                .expect("Format is required");

            println!("Converting {input} to \'{format}\' and saving to {output}");
        }

        _ => println!("No subcommand was used"),
    }
}
