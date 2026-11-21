//! 面向调用者的 BMP 创建与序列化入口。
//!
//! 本模块把底层结构的细节包装成几个简单函数，典型的使用流程是：
//!
//! 1. [`new`]：创建一张指定尺寸和位深的空白图像；
//! 2. [`new_pixel`] + [`write()`]：生成像素并填入图像；
//! 3. [`translator`]：把整张图像序列化为字节流，由调用者写入文件。
//!
//! 完整示例见 crate 文档。

use crate::image_file::bitmap::BitMap;
use crate::image_file::file_info::{BitMapFileHeader, BitMapInfoHeader, ColorPallet};
use crate::image_file::pixels_data::{ImageData, Pixel};

/// 创建一张空白图像。
///
/// `size` 为 `[宽, 高]`（单位：像素），`bits_per_pixel` 目前只支持 24 和 32。
/// 返回的 [`BitMap`] 中文件头和信息头已经填好，调色板为空，像素数据为空，
/// 需要再调用 [`write()`] 填充像素。
///
/// # Panics
///
/// 当 `bits_per_pixel` 不是 24 或 32 时会 panic。
pub fn new(size: [u32; 2], bits_per_pixel: u16) -> BitMap {
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

/// 用长度为 3 的数组创建一个像素。
///
/// 数组按 BMP 的存储顺序解释为 `[蓝, 绿, 红]`，并直接传给 `Pixel::new`。
pub fn new_pixel(pixel: [u8; 3]) -> Pixel {
    Pixel::new(pixel[0], pixel[1], pixel[2])
}

/// 把像素数据写入图像。
///
/// 容器中已有的像素会被 `raw` 覆盖；调用者需要保证 `raw` 中的像素数量与
/// 图像的宽高一致。
pub fn write(image: &mut ImageData, raw: Vec<Pixel>) -> std::io::Result<()> {
    ImageData::write(image, raw)?;

    Ok(())
}

/// 把整张图像序列化为小端字节码，并追加到 `data_stream`。
///
/// 写入顺序严格遵循 BMP 规范：文件头 → 信息头 → 调色板 → 像素数据。
/// 得到的字节流可以直接写入 `.bmp` 文件。
pub fn translator(bmp: BitMap, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
    bmp.bit_map_file_header.translator(data_stream)?;
    bmp.bit_map_info_header.translator(data_stream)?;
    bmp.color_pallet.translator(data_stream)?;
    bmp.raw_pixel_data.translate(data_stream)?;

    Ok(())
}
