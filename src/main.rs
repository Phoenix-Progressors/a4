use image::codecs::gif::GifDecoder;
use image::codecs::jpeg::JpegDecoder;
use image::codecs::png::PngDecoder;
use image::GenericImageView;
use pdf::backend::RenderingMode;
use pdf::content::operations::Drawing;
use pdf::content::Content;
use pdf::encoding::builtin::Name;
use pdf::file::File;
use std::env;
use std::fs;
use std::path::Path;

fn create_pdf(images: &[String], output_filename: &str) {
    let (mut pdf, mut page) = File::new().generate_pages();
    let max_width = 595.0; // A4 width in points
    let max_height = 842.0; // A4 height in points

    let mut x_offset = 0.0;
    let mut y_offset = max_height;
    let mut current_row_height = 0.0;

    for img_path in images {
        let img_data = fs::read(img_path).unwrap();
        let img = match &img_data[..] {
            data @ &[0x89, 0x50, 0x4E, 0x47, ..] => {
                let decoder = PngDecoder::new(data).unwrap();
                let dimensions = decoder.dimensions();
                decoder.decode().unwrap().into_rgba8()
            }
            data @ &[0xFF, 0xD8, ..] => {
                let decoder = JpegDecoder::new(data).unwrap();
                let dimensions = decoder.dimensions();
                decoder.decode().unwrap().into_rgba8()
            }
            data @ &[0x47, 0x49, 0x46, ..] => {
                let decoder = GifDecoder::new(data).unwrap();
                let dimensions = decoder.dimensions();
                decoder.decode_frame(0).unwrap().into_rgba8()
            }
            _ => continue,
        };

        let (img_width, img_height) = img.dimensions();
        let aspect_ratio = img_width as f64 / img_height as f64;

        // Scale image to fit A4 width if necessary
        let (mut img_width, mut img_height) = (img_width as f64, img_height as f64);
        if img_width > max_width {
            img_width = max_width;
            img_height = img_width / aspect_ratio;
        }
        if img_height > max_height {
            img_height = max_height;
            img_width = img_height * aspect_ratio;
        }

        // Check if the image can be placed in the current row
        if x_offset + img_width > max_width {
            // Move to next row
            x_offset = 0.0;
            y_offset -= current_row_height;
            current_row_height = 0.0;
        }

        // Check if we need to start a new page
        if y_offset - img_height < 0.0 {
            pdf = pdf.add_page(page).unwrap();
            page = pdf.get_page_mut(pdf.num_pages() - 1).unwrap();
            y_offset = max_height;
            x_offset = 0.0;
            current_row_height = 0.0;
        }

        // Draw image on the canvas
        let content = Content::new()
            .add_image(img, Some(Name::from_name("img", true)))
            .add_instruction(Drawing::image(
                "img",
                x_offset,
                y_offset - img_height,
                img_width,
                img_height,
            ));
        page.content_mut().extend_from_slice(&content);

        x_offset += img_width;
        current_row_height = current_row_height.max(img_height);
    }

    pdf.render_next(RenderingMode::Fill, output_filename)
        .unwrap();
}

fn create_pdf_from_directory(directory: &str, output_filename: &str) {
    let mut images: Vec<String> = fs::read_dir(directory)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("jpg")
                || path.extension().and_then(|ext| ext.to_str()) == Some("png")
                || path.extension().and_then(|ext| ext.to_str()) == Some("gif")
            {
                Some(path.to_str().unwrap().to_owned())
            } else {
                None
            }
        })
        .collect();

    images.sort();
    create_pdf(&images, output_filename);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} --directory=<directory> --output_filename=<output_filename>",
            args[0]
        );
        return;
    }

    let mut directory = String::new();
    let mut output_filename = String::new();

    for arg in &args[1..] {
        if arg.starts_with("--directory=") {
            directory = arg[12..].to_owned();
        } else if arg.starts_with("--output_filename=") {
            output_filename = arg[18..].to_owned();
        }
    }

    create_pdf_from_directory(&directory, &output_filename);
}
