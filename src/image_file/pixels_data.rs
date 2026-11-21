//! 像素与像素数据的定义与序列化。
//!
//! BMP 的 24 位像素按 **蓝、绿、红**（BGR）的顺序存储，而不是常见的 RGB，
//! 因此 [`Pixel`] 的字段顺序也是 `blue → green → red`，序列化时必须保持该顺序。
//!
//! [`ImageData`] 负责保存整张图片的像素数组，最终由它把所有像素依次写入字节流。

/// 整张图像的像素数据容器。
///
/// 目前内部就是 `Vec<Pixel>`，按“从左到右、从上到下”的顺序保存每个像素。
/// 使用动态数组是为了后续能够支持不同尺寸的图像。
pub struct ImageData {
    raw: Vec<Pixel>,
}
//像素元数据，考虑到后期拓展性直接用了可变数组

/// 单个像素的颜色分量。
///
/// 三个字段的顺序与 BMP 文件的存储顺序一致：蓝、绿、红（BGR）。
pub struct Pixel {
    /// 蓝色分量。
    pub blue: u8,
    /// 绿色分量。
    pub green: u8,
    /// 红色分量。
    pub red: u8,
}
//定义了单个像素的色彩组成

//----------以下是方法实现---------------
impl Pixel {
    /// 用 BGR 三个分量创建一个像素。
    pub(crate) fn new(blue: u8, green: u8, red: u8) -> Pixel {
        Pixel { blue, green, red }
    }

    /// 把单个像素按 蓝 → 绿 → 红 的顺序写成 3 个字节。
    pub(crate) fn translator(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
        data_stream.extend_from_slice(&self.blue.to_le_bytes());
        data_stream.extend_from_slice(&self.green.to_le_bytes());
        data_stream.extend_from_slice(&self.red.to_le_bytes());

        Ok(())
    }
}

impl ImageData {
    /// 创建一个空的像素容器，并按图像尺寸预留内存。
    ///
    /// `size` 为 `[宽, 高]`，单位为像素。这里只分配容量，不填充任何像素。
    pub(crate) fn new(size: [u32; 2]) -> ImageData {
        ImageData {
            raw: Vec::with_capacity(size[0] as usize * size[1] as usize),
        }
    }

    /// 用传入的像素数组替换容器中已有的像素数据。
    pub(crate) fn write(&mut self, raw: Vec<Pixel>) -> std::io::Result<()> {
        self.raw = raw;

        Ok(())
    }

    /// 按保存顺序把全部像素写入 `data_stream`。
    ///
    /// # Panics
    ///
    /// 如果某个像素写入字节流失败，会直接 panic。当前实现中写入目标是一个
    /// `Vec<u8>`，实际上不会失败，因此该分支只用于防御式处理。
    pub(crate) fn translate(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
        self.raw.into_iter()
                .for_each(
                    |pixel| pixel.translator(data_stream)
                    .expect("像素信息写入错误！ code：src/image_file/pixels_data::line 42  err_function:ImageData::translator")
                );

        Ok(())
    }
}
