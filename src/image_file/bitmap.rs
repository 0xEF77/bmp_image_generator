use crate::image_file::pixels_data::ImageData;
use crate::image_file::file_info::{BitMapFileHeader, BitMapInfoHeader, ColorPallet};

pub struct BitMap{
    //BMP图片的数据架构组成，由此写了树状结构的数据架构
    pub bit_map_file_header:BitMapFileHeader,//文件头
    pub bit_map_info_header:BitMapInfoHeader,//信息头
    pub color_pallet:ColorPallet,//调色板
    pub raw_pixel_data:ImageData,//图像像素信息
}