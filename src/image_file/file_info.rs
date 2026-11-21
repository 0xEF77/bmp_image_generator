//! BMP 文件头、信息头与调色板。
//!
//! 这三个结构按 BMP 规范定义，序列化方法都会把字段写成**小端序**字节并追加到
//! `data_stream` 中。它们合起来构成 BMP 文件在像素数据之前的全部元数据：
//!
//! - 文件头 `BitMapFileHeader`：14 字节
//! - 信息头 `BitMapInfoHeader`：40 字节
//! - 调色板 `ColorPallet`：24/32 位图下为 0 字节
//!
//! 因此像素数据的起始偏移量固定为 `14 + 40 = 54` 字节。

/// BMP 文件头，固定 14 字节，记录文件最基础的信息。
///
/// 对应 BMP 规范中的 `BITMAPFILEHEADER`。按字段顺序写入后恰好占
/// `2 + 4 + 2 + 2 + 4 = 14` 字节。
pub(crate) struct BitMapFileHeader {
    //文件头，记录了最基础的文件信息
    /// 文件类型，BMP 固定为 ASCII 字符串 `BM`。
    file_type: &'static str, //文件类型，这里均为“BM”
    /// 文件完整大小，单位为字节，等于 54 加上像素数据大小。
    file_size: u32, //文件完整大小
    /// 保留字段，必须为 0。
    reserved_0: u16, //保留位，默认为0
    /// 保留字段，必须为 0。
    reserved_1: u16, //保留位，默认为0
    /// 从文件开头到像素数据的偏移量，本实现固定为 54。
    pixel_data_offset: u32, //偏移量，记录偏移多少byte到达像素数据
}

/// BMP 信息头，固定 40 字节，用来描述图像的尺寸与像素格式。
///
/// 对应 BMP 规范中的 `BITMAPINFOHEADER`。本实现只生成未压缩位图，
/// 因此 `compression` 恒为 0，`image_size` 等于像素数据占用的字节数。
pub(crate) struct BitMapInfoHeader {
    //BMP图像专属文件头，描述图像信息
    /// 信息头的大小，BMP 规范中固定为 40 字节。
    header_size: u32, //信息头的大小，默认40B
    /// 图像宽度，单位为像素。
    image_width: u32, //图像宽度，单位为pixel
    /// 图像高度，单位为像素。正数表示像素行按自下而上的顺序存储。
    image_height: u32, //图像长度，单位为pixel
    /// 目标设备平面数，属于历史遗留字段，恒为 1。
    planes: u16, //目标设备平面数，历史遗留，默认为1
    /// 位深，表示每个像素占用多少 bit，本实现使用 24。
    bits_per_pixel: u16, //位深，表示每个像素需要多少bits
    /// 压缩方式，0 表示未压缩。
    compression: u32, //压缩信息，默认为0（未压缩）
    /// 像素数据的大小，单位为字节。
    image_size: u32, //整个图像文件的大小,单位为B
    /// 水平分辨率，单位为像素/米，0 表示不指定。
    x_pixel_per_meter: u32, //水平解析度
    /// 垂直分辨率，单位为像素/米，0 表示不指定。
    y_pixel_per_meter: u32, //垂直解析度
    /// 调色板中实际使用的颜色数，24/32 位图不使用调色板，填 0。
    total_colors: u32, // 使用的调色板颜色数
    /// 调色板中对显示重要的颜色数，0 表示全部重要。
    important_colors: u32, //重要的颜色数
}

/// 调色板，24 位及以上位深的 BMP 不使用调色板。
///
/// 这里保留空结构，是为了让整体结构与 BMP 规范保持一致，也让序列化流程
/// 可以在未来支持 8 位及以下的索引色图片时直接补充颜色表。
pub(crate) struct ColorPallet {
    //调色板，24位深及以下使用，其余位深使用RGB
}

//备注：
//一个像素使用1bit存储，为纯黑白图片
//一个像素使用8个bits存储，若为黑白色则为**灰度图**，若为彩色图则为**伪彩色图**
//一个像素使用24个bits存储，为真彩色图

//------------------以下是各种方法实现------------------------------------------
impl BitMapFileHeader {
    /// 根据信息头计算并构造文件头。
    ///
    /// BMP 文件头本身不携带图像尺寸，因此这里从 `info_header` 中取出像素数据
    /// 大小，再加上固定的 54 字节头部，得到文件总大小。
    pub(crate) fn new(info_header: &BitMapInfoHeader) -> BitMapFileHeader {
        let file_size = info_header.image_size + 54;
        BitMapFileHeader {
            file_type: "BM",
            file_size,
            reserved_0: 0,
            reserved_1: 0,
            pixel_data_offset: 54,
        }
    }

    /// 把文件头序列化为小端字节并追加到 `data_stream`。
    ///
    /// 写入顺序为：文件类型 → 文件大小 → 两个保留字段 → 像素数据偏移量。
    pub(crate) fn translator(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
        data_stream.extend_from_slice(self.file_type.as_bytes());
        data_stream.extend_from_slice(&self.file_size.to_le_bytes());
        data_stream.extend_from_slice(&self.reserved_0.to_le_bytes());
        data_stream.extend_from_slice(&self.reserved_1.to_le_bytes());
        data_stream.extend_from_slice(&self.pixel_data_offset.to_le_bytes());

        Ok(())
    }
}

impl BitMapInfoHeader {
    /// 根据图像尺寸和位深构造信息头。
    ///
    /// `img_size` 为 `[宽, 高]`，单位为像素；像素数据大小按
    /// `宽 × 高 × (位深 / 8)` 计算。注意本实现尚未处理 BMP 的行填充规则，
    /// 详见项目 README 的“已知限制”。
    pub(crate) fn new(img_size: [u32; 2], bits_per_pixel: u16) -> BitMapInfoHeader {
        let image_size = (img_size[0] * img_size[1]) * (bits_per_pixel as u32 / 8);
        BitMapInfoHeader {
            header_size: 40,
            image_width: img_size[0],
            image_height: img_size[1],
            planes: 1,
            bits_per_pixel,
            compression: 0,
            image_size,
            x_pixel_per_meter: 0,
            y_pixel_per_meter: 0,
            total_colors: 0,
            important_colors: 0,
        }
    }

    /// 把信息头序列化为小端字节并追加到 `data_stream`。
    ///
    /// 写入顺序与结构体字段顺序一致，共写出 40 字节。
    pub(crate) fn translator(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
        data_stream.extend_from_slice(&self.header_size.to_le_bytes());
        data_stream.extend_from_slice(&self.image_width.to_le_bytes());
        data_stream.extend_from_slice(&self.image_height.to_le_bytes());
        data_stream.extend_from_slice(&self.planes.to_le_bytes());
        data_stream.extend_from_slice(&self.bits_per_pixel.to_le_bytes());
        data_stream.extend_from_slice(&self.compression.to_le_bytes());
        data_stream.extend_from_slice(&self.image_size.to_le_bytes());
        data_stream.extend_from_slice(&self.x_pixel_per_meter.to_le_bytes());
        data_stream.extend_from_slice(&self.y_pixel_per_meter.to_le_bytes());
        data_stream.extend_from_slice(&self.total_colors.to_le_bytes());
        data_stream.extend_from_slice(&self.important_colors.to_le_bytes());

        Ok(())
    }
}

impl ColorPallet {
    /// 创建一个空调色板。
    ///
    /// 24/32 位图的颜色直接由像素数据表示，因此这里没有需要初始化的字段。
    pub(crate) fn new() -> ColorPallet {
        ColorPallet {}
    }

    /// 24/32 位图不使用调色板，因此不写入任何字节。
    ///
    /// 保留该函数是为了让上层可以按“文件头 → 信息头 → 调色板 → 像素数据”
    /// 的顺序统一序列化。
    pub(crate) fn translator(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
        Ok(())
    }
}
