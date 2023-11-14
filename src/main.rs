use qrcode_generator::{QRCodeError, QrCodeEcc};
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    content: String,
    filename: Option<PathBuf>,
}

const WHITE: &str = "\x1b[47m  ";
const BLACK: &str = "\x1b[40m  ";
const CLEAR: &str = "\x1b[0m";

fn main() -> Result<(), QRCodeError> {
    let args = Args::parse();
    match args.filename {
        Some(filename) => {
            qrcode_generator::to_png_to_file(&args.content, QrCodeEcc::Medium, 1024, filename)
        }
        None => {
            let mat = qrcode_generator::to_matrix(&args.content, QrCodeEcc::Medium)?;
            for row in mat {
                let row = row
                    .into_iter()
                    .map(|bool| if bool { WHITE } else { BLACK })
                    .collect::<String>();
                println!("{row}{CLEAR}")
            }
            Ok(())
        }
    }
}
