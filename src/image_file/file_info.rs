    pub(crate) struct BitMapFileHeader {
        //文件头，记录了最基础的文件信息
        file_type:&'static str,//文件类型，这里均为“BM”
        file_size:u32,//文件完整大小
        reserved_0:u16,//保留位，默认为0
        reserved_1:u16,//保留位，默认为0
        pixel_data_offset:u32,//偏移量，记录偏移多少byte到达像素数据
    }


    pub(crate) struct BitMapInfoHeader {
        //BMP图像专属文件头，描述图像信息
        header_size:u32,//信息头的大小，默认40B
        image_width:u32,//图像宽度，单位为pixel
        image_height:u32,//图像长度，单位为pixel
        planes:u16,//目标设备平面数，历史遗留，默认为1
        bits_per_pixel:u16,//位深，表示每个像素需要多少bits
        compression:u32,//压缩信息，默认为0（未压缩）
        image_size:u32,//整个图像文件的大小,单位为B
        x_pixel_per_meter:u32,//水平解析度
        y_pixel_per_meter:u32,//垂直解析度
        total_colors:u32,// 使用的调色板颜色数
        important_colors:u32,//重要的颜色数
    }

    pub(crate) struct ColorPallet {
        //调色板，24位深及以下使用，其余位深使用RGB
    }

    //备注：
    //一个像素使用1bit存储，为纯黑白图片
    //一个像素使用8个bits存储，若为黑白色则为**灰度图**，若为彩色图则为**伪彩色图**
    //一个像素使用24个bits存储，为真彩色图


    //------------------以下是各种方法实现------------------------------------------
    impl BitMapFileHeader {
        pub(crate) fn new(info_header: &BitMapInfoHeader)-> BitMapFileHeader {
            let file_size = info_header.image_size + 54;
            BitMapFileHeader {
                file_type: "BM",
                file_size,
                reserved_0: 0,
                reserved_1: 0,
                pixel_data_offset: 54,
            }
        }
        
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
        pub(crate) fn new(img_size: [u32; 2], bits_per_pixel:u16)-> BitMapInfoHeader {
            let image_size = (img_size[0] * img_size[1]) * (bits_per_pixel as u32 / 8);
            BitMapInfoHeader{
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
        pub(crate) fn new() -> ColorPallet  {
            ColorPallet{}
        }
        
        pub(crate) fn translator(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
            Ok(())
        }
    }


