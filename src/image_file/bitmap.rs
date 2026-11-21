use crate::image_file::file_info::{BitMapFileHeader, BitMapInfoHeader, ColorPallet};
use crate::image_file::pixels_data::ImageData;

/// 一张完整的 BMP 图片。
///
/// BMP 文件在逻辑上可以看作下面四部分的树状组合，序列化时也按同样的顺序写入：
/// 文件头 → 信息头 → 调色板 → 像素数据。
pub struct BitMap {
    //BMP图片的数据架构组成，由此写了树状结构的数据架构
    /// 文件头：记录文件类型、文件总大小以及像素数据的偏移量。
    pub bit_map_file_header: BitMapFileHeader, //文件头
    /// 信息头：记录图像宽高、位深、压缩方式等属性。
    pub bit_map_info_header: BitMapInfoHeader, //信息头
    /// 调色板：24/32 位图不使用，序列化时写出 0 字节。
    pub color_pallet: ColorPallet, //调色板
    /// 图像像素数据：按行保存的像素数组。
    pub raw_pixel_data: ImageData, //图像像素信息
}
