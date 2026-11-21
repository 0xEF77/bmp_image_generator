use std::io::{BufWriter, Write};
use std::fs::File;
use BMP_Image_Generator::image_file::create_image;
use BMP_Image_Generator::image_file::pixels_data::Pixel;
use fastrand::u8 as fastrand_u8;

fn main() -> std::io::Result<()> {
    let image_size:[u32; 2] = [1920, 1080];//[x, y]
    let mut raw = Vec::with_capacity(image_size[0]as usize * image_size[1]as usize);//创建图像元数据的动态数组
    let mut data_stream:Vec<u8> = Vec::new();//创建数据流容器


    let mut image = create_image::new(image_size, 24);//创建空图像

    fill_raw_data(&mut raw, &image_size)?;//填充像素数据

    create_image::write(&mut image.raw_pixel_data, raw)?;//向图像文件写入像素数据

    create_image::translator(image, &mut data_stream)?;//将图像的所有数据序列化为小端字节码

    let mut buf_writer = BufWriter::new(File::create("image.bmp")?);//创建图像文件

    buf_writer.write_all(&data_stream)?;
    buf_writer.flush()?;//将所有字节码写入图像中

    Ok(())
}

fn fill_raw_data(raw: &mut Vec<Pixel>, size:&[u32; 2])-> std::io::Result<()> {
    for _i in 0..size[0]*size[1] {
        let pixel = [fastrand_u8(..), fastrand_u8(..), fastrand_u8(..)];
        let pixel = create_image::new_pixel(pixel);
        raw.push(pixel);
    }

    Ok(())
}