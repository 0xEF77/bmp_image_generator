//! # BMP Image Generator
//!
//! 一个用 Rust 从零生成 BMP 图片的小项目，适合用来学习位图文件格式与二进制文件写入。
//!
//! 本库不依赖任何图像处理库：BMP 的文件头、信息头、调色板和像素数据都由手写结构体
//! 按规范序列化为小端字节码。项目当前会生成一张 1920×1080、24 位色的随机像素图片。
//!
//! ## 模块
//!
//! - [`image_file`]：BMP 文件的数据结构、序列化与写入逻辑
//!
//! ## 简单示例
//!
//! ```no_run
//! use BMP_Image_Generator::image_file::create_image::{self, new_pixel};
//!
//! let mut image = create_image::new([640, 480], 24);
//! let pixels = (0..640 * 480)
//!     .map(|_| new_pixel([0, 0, 0]))
//!     .collect();
//! create_image::write(&mut image.raw_pixel_data, pixels).unwrap();
//!
//! let mut data_stream = Vec::new();
//! create_image::translator(image, &mut data_stream).unwrap();
//! std::fs::write("image.bmp", data_stream).unwrap();
//! ```
//!
//! 如果只是想把图片生成出来，直接运行 `cargo run --release` 即可，程序会在当前目录
//! 写出 `image.bmp`。
pub mod image_file;
