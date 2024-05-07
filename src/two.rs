use image::GenericImage;
use image::{DynamicImage, GenericImageView};
use rect_packer::{Config, Packer, Rect};
use walkdir::WalkDir;
// Enum representing supported image formats
enum ImageFormat {
    Jpg,
    Png,
}

// Struct to hold image data and its dimensions
#[derive(Debug)]
struct ImageData {
    data: DynamicImage,
    width: u32,
    height: u32,
}

// Struct for A4 dimensions (you may need to adjust based on units)
struct A4 {
    width: i32,
    height: i32,
}

fn load_images(dir_path: &str) -> Vec<ImageData> {
    let mut images = Vec::new();

    // Iterate through files in the directory
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if it's a file and has a supported extension
        if path.is_file()
            && (path.extension().unwrap_or_default() == "jpg"
                || path.extension().unwrap_or_default() == "png")
        {
            // Decode the image using the "image" crate
            let img = image::open(path).unwrap();
            let (width, height) = img.dimensions();

            // Store image data in the vector
            images.push(ImageData {
                data: img,
                width,
                height,
            });
        }
    }

    images
}

fn pack_images(images: &[ImageData], a4: &A4) -> Vec<Rect> {
    let mut packer = Packer::new(Config {
        width: a4.width,
        height: a4.height,
        border_padding: 10,
        rectangle_padding: 20,
    });
    let mut rects = Vec::new();
    for image in images {
        // let rect = Rect::new(0, 0, image.width as i32, image.height as i32);
        if let Some(packed_rect) = packer.pack(image.width as i32, image.height as i32, false) {
            rects.push(packed_rect);
        } else {
            // Handle the case where image doesn't fit
            // println!("Image doesn't fit on the canvas: {:?}", image);
        }
    }
    rects
}

fn create_composed_image(
    images: &[ImageData],
    rects: &[Rect],
    a4: &A4,
) -> Result<DynamicImage, image::ImageError> {
    // Create a new image with A4 dimensions
    let mut composed_image = DynamicImage::new_rgb8(a4.width as u32, a4.height as u32);

    // Iterate over images and their corresponding rectangles
    for (image, rect) in images.iter().zip(rects) {
        // Copy the image data onto the composed image at the specified position
        composed_image.copy_from(&image.data, rect.x as u32, rect.y as u32)?;
    }

    Ok(composed_image)
}

fn main() {
    // Specify directory path and A4 dimensions
    // let dir_path = "~/Downloads/screenshots/earthquake";
    let dir_path = "/Users/abhishek/Downloads/screenshots/earthquake";

    let a4 = A4 {
        width: 2480,
        height: 3508,
    }; // Example A4 dimensions in pixels

    // Load images
    let images = load_images(dir_path);

    // Pack images onto A4 canvas
    let rects = pack_images(&images, &a4);

    // Create composed image
    let composed_image = create_composed_image(&images, &rects, &a4);

    // Save the composed image
    composed_image
        .expect("COULD NOT SAVE IMAGE")
        .save("composed.png")
        .unwrap();
}
