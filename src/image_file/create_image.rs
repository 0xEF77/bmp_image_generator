use crate::image_file::pixels_data::{ImageData, Pixel};
use crate::image_file::file_info::{BitMapFileHeader, BitMapInfoHeader, ColorPallet};
use crate::image_file::bitmap::BitMap;

///创建新的空白图像文件
/// 
pub fn new(size:[u32; 2], bits_per_pixel:u16) -> BitMap {
    
    if !(bits_per_pixel == 24 || bits_per_pixel == 32) {
        panic!("该程序暂不支持像素位深8bits以下的图像数据.");
    }

    let info_header = BitMapInfoHeader::new(size, bits_per_pixel);
    let file_header = BitMapFileHeader::new(&info_header);
    let color_pallet = ColorPallet::new();
    let image_data = ImageData::new(size);

    BitMap {
        bit_map_file_header: file_header,
        bit_map_info_header: info_header,
        color_pallet,
        raw_pixel_data: image_data,
    }
}

///创建图像里的各个像素元数据
pub fn new_pixel(pixel:[u8;3]) -> Pixel {
    Pixel::new(pixel[0], pixel[1], pixel[2])
}

///向图像文件内写入像素数据
pub fn write(image: &mut ImageData, raw: Vec<Pixel>) ->std::io::Result<()>{
    ImageData::write(image, raw)?;

    Ok(())
}

///将所有图像元数据序列化为小端字节码
pub fn translator(bmp: BitMap, data_stream: &mut Vec<u8>) -> std::io::Result<()>{
    bmp.bit_map_file_header.translator(data_stream)?;
    bmp.bit_map_info_header.translator(data_stream)?;
    bmp.color_pallet.translator(data_stream)?;
    bmp.raw_pixel_data.translate(data_stream)?;
    
    Ok(())
}