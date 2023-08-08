extern crate clap;
extern crate qrlink;

use clap::{App, Arg};
use std::{fs, io, io::Write};
use std::path::Path;

fn main() {
    let options = App::new("qrlink")
        .version("0.0.1")
        .about("Encode a link as a scannable QR code")
        .author("davidk")
        .usage("qrlink [ https://example.com | --ask ] [ --imagefile (output_name.png) --scale 10 | --svg | --svgfile (output_name.svg) ]")
        .arg(
            Arg::with_name("link")
                .long("link")
                .short("l")
                .takes_value(true)
                .required_unless("ask")
                .index(1)
                .display_order(1)
                .help("URL (or text) to convert to a QR code"),
        )
        .arg(
            Arg::with_name("scale")
                .long("scale")
                .takes_value(true)
                .default_value("10")
                .display_order(2)
                .help("QR code scaling factor. Applies to imagefile output only."),
        )
        .arg(
            Arg::with_name("quiet_zone")
                .long("quietzone")
                .takes_value(true)
                .display_order(3)
                .default_value("2")
                .help("QR code: The size of the quiet zone/border to apply to the final QR code"),
        )
        .arg(
            Arg::with_name("image_file")
                .long("imagefile")
                .takes_value(true)
                .display_order(4)
                .help("The name of the file to save to (e.g. --imagefile qr.png). Formats: [png, jpg, bmp]"),
        )
        .arg(
            Arg::with_name("svg")
                .long("svg")
                .takes_value(false)
                .display_order(5)
                .help("Emit the QR code as an SVG (to standard output)")
        )
        .arg(
            Arg::with_name("svg_file")
                .long("svgfile")
                .takes_value(true)
                .display_order(6)
                .help("Save the QR code to a file (SVG formatted)")
        )
        .arg(
            Arg::with_name("ask")
                .long("ask")
                .short("a")
                .takes_value(false)
                .display_order(7)
                .help("Ask for URL/link instead of getting it through the command-line")
        )
        .get_matches();

    // Note: avoid turbofish/generic on parse() through upfront declaration
    let scale: i32 = options.value_of("scale").unwrap_or("10").parse().unwrap();
    let quiet_zone: i32 = options
        .value_of("quiet_zone")
        .unwrap_or("10")
        .parse()
        .unwrap();
    let image_file: String = options
        .value_of("image_file")
        .unwrap_or("")
        .parse()
        .unwrap();

    let mut link = String::new();

    if options.is_present("ask") {

        print!("Enter URL to convert: ");

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut link)
            .expect("Failed to read URL/link ...");
        link = link.trim().to_string();
    } else {
        link = options.value_of("link").unwrap().to_string();
    }
    
    let config = qrlink::Url::new(Some(&link));

    let encoding = match qrlink::encode(&config) {
        Ok(e) => e,
        Err(e) => {
            println!("There was a problem generating the QR code.\n{}", e);
            return;
        }
    };

    if options.is_present("svg_file") {
        println!("Generating QR code ..");
        let file_name = options.value_of("svg_file").unwrap();

        println!("Writing out to SVG file: {} ..", file_name);
        let svg_data = qrlink::make_svg(&encoding);
        fs::write(file_name, svg_data).expect("Unable to write file");
    } else if options.is_present("image_file") {
        // Validate that image_file extension supplied is compatible with upstream library export formats
        match Path::new(&image_file).extension() {
            None => {
                println!("Error: No extension found for image file. Try --imagefile [ qr.jpeg | qr.png ] instead.");
                return;
            }
            Some(p) => {
                let ext: &str = p.to_str().unwrap();
                match ext {
                    "png" | "jpeg" | "jpg" => {}
                    _ => {
                        println!("Unrecognized file extension: {}. Try --imagefile [ qr.png | qr.jpeg | qr.jpg ] instead.", ext);
                        return;
                    }
                }
            }
        };

        println!("Generating QR code ..");

        println!("Parameters: scale {} + quiet zone: {} ", scale, quiet_zone);

        let image = qrlink::make_image(&encoding, scale, quiet_zone);
        match qrlink::save_image(&image, image_file.to_string()) {
            Ok(_) => {
                println!("QR code has been saved to file {}", image_file);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                println!("Unable to write QR image to file in requested format. Supported extensions are .jpeg and .png. Try --imagefile qr.jpeg or --imagefile qr.png");
                fs::remove_file(image_file).unwrap();
            }
        };
    } else if options.is_present("svg") {
        println!("{}", qrlink::make_svg(&encoding));
    } else {
        qrlink::console_qr(&encoding, quiet_zone);
    }
}
