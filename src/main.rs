use clap::Parser;
use image::GenericImageView;
use std::path::PathBuf;

/// A simple tool to generate ASCII art from images
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input image file
    #[arg(required = true)]
    input: PathBuf,

    /// Width of the output ASCII art
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    /// Invert the character mapping (useful for dark backgrounds)
    #[arg(short, long, default_value_t = false)]
    invert: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load the image
    let img = image::open(&args.input)?;

    // Calculate new dimensions
    let (width, height) = img.dimensions();
    let aspect_ratio = height as f64 / width as f64;
    
    // Terminal characters are typically about twice as tall as they are wide.
    // We adjust the aspect ratio to account for this.
    let font_aspect_ratio = 0.5; 
    let new_width = args.width;
    let new_height = (new_width as f64 * aspect_ratio * font_aspect_ratio) as u32;

    // Resize the image
    let scaled = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

    // ASCII characters sorted from dark to light
    let ascii_chars = "@%#*+=-:. ";
    let rev_ascii_chars = " .:-=+*#%@";

    let chars = if args.invert { rev_ascii_chars } else { ascii_chars };
    let scale = (chars.len() - 1) as f32;

    for y in 0..new_height {
        for x in 0..new_width {
            let pixel = scaled.get_pixel(x, y);
            // Convert to grayscale using luminance formula
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            let gray = 0.299 * r + 0.587 * g + 0.114 * b;
            
            // Map grayscale value (0-255) to index in ascii_chars
            let idx = (gray / 255.0 * scale).round() as usize;
            print!("{}", chars.chars().nth(idx).unwrap_or(' '));
        }
        println!();
    }

    Ok(())
}
