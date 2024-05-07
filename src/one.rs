// // Extern crate for the 'image' library
// extern crate image;

// // Use statements for required modules
// use image::GenericImageView;

// use std::fs;
// use std::path::Path;
// // Define the image format enum
// enum ImageFormat {
//     Jpg,
//     Png,
// }

// // Define the ImageData struct to hold image data and dimensions
// struct ImageData {
//     format: ImageFormat,
//     width: u32,
//     height: u32,
//     data: Vec<u8>,
// }

// // Trait to represent objects that can be composed on a canvas
// trait Composable {
//     fn width(&self) -> u32;
//     fn height(&self) -> u32;
//     fn compose(&self, canvas: &mut Vec<u8>, x: u32, y: u32);
// }
// impl Composable for ImageData {
//     fn width(&self) -> u32 {
//         self.width
//     }

//     fn height(&self) -> u32 {
//         self.height
//     }

//     fn compose(&self, canvas: &mut Vec<u8>, x: u32, y: u32) {
//         let canvas_image = image::RgbImage::from_raw(A4_WIDTH, A4_HEIGHT, canvas.clone()).unwrap();
//         let image_data = match self.format {
//             ImageFormat::Jpg => {
//                 image::load_from_memory_with_format(&self.data, image::ImageFormat::Jpeg).unwrap()
//             }
//             ImageFormat::Png => {
//                 image::load_from_memory_with_format(&self.data, image::ImageFormat::Png).unwrap()
//             }
//         };

//         let image_data = image_data.to_rgb8();
//         let (image_width, image_height) = image_data.dimensions();

//         let mut composed_image = canvas_image;
//         // composed_image.copy_from(&image_data, x, y).unwrap();
//         composed_image.copy_from_slice(&image_data);

//         *canvas = composed_image.into_raw();
//     }
// }

// // Load images from the given directory
// fn load_images(dir_path: &Path) -> Vec<ImageData> {
//     let mut images = Vec::new();

//     if let Ok(entries) = fs::read_dir(dir_path) {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 let path = entry.path();
//                 if path.is_file() {
//                     match path.extension().and_then(|ext| ext.to_str()) {
//                         Some("jpg") | Some("jpeg") => {
//                             if let Ok(img) = image::open(&path) {
//                                 let (width, height) = img.dimensions();
//                                 let data = img.into_raw();
//                                 images.push(ImageData {
//                                     format: ImageFormat::Jpg,
//                                     width,
//                                     height,
//                                     data,
//                                 });
//                             }
//                         }
//                         Some("png") => {
//                             if let Ok(img) = image::open(&path).and_then(|img| img.to_rgb8()) {
//                                 let (width, height) = img.dimensions();
//                                 let data = img.into_raw();
//                                 images.push(ImageData {
//                                     format: ImageFormat::Png,
//                                     width,
//                                     height,
//                                     data,
//                                 });
//                             }
//                         }
//                         _ => continue,
//                     }
//                 }
//             }
//         }
//     }

//     images
// }

// // Constants for A4 paper dimensions (in pixels at 96 DPI)
// const A4_WIDTH: u32 = 8 * 96;
// const A4_HEIGHT: u32 = 11 * 96;

// // Compose the images on an A4 canvas
// fn compose_images(images: &[ImageData]) -> Vec<u8> {
//     let mut canvas = vec![0; (A4_WIDTH * A4_HEIGHT * 3) as usize];

//     let mut x = 0;
//     let mut y = 0;
//     let mut max_height = 0;

//     for image in images {
//         if x + image.width() > A4_WIDTH {
//             x = 0;
//             y += max_height;
//             max_height = 0;
//         }

//         image.compose(&mut canvas, x, y);

//         x += image.width();
//         max_height = max_height.max(image.height());
//     }

//     canvas
// }

// fn main() {
//     // Load images from a directory (replace with your directory path)
//     let dir_path = Path::new("/path/to/images");
//     let images = load_images(dir_path);

//     // Compose the images on an A4 canvas
//     let canvas_data = compose_images(&images);

//     // Do something with the canvas_data, e.g., save it to a file
//     // ...
// }
