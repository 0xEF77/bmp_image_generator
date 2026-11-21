     pub struct ImageData{
         raw: Vec<Pixel>,
    }
    //像素元数据，考虑到后期拓展性直接用了可变数组

    pub struct Pixel{
        pub blue: u8,
        pub green: u8,
        pub red: u8,
    }
    //定义了单个像素的色彩组成

    //----------以下是方法实现---------------
    impl Pixel{
        
        pub(crate) fn new(blue: u8, green: u8, red: u8) -> Pixel{
            Pixel{blue, green, red }
        }
        
        pub(crate) fn translator(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
            data_stream.extend_from_slice(&self.blue.to_le_bytes());
            data_stream.extend_from_slice(&self.green.to_le_bytes());
            data_stream.extend_from_slice(&self.red.to_le_bytes());
            
            Ok(())
        }
    }
    
    impl ImageData{
        pub(crate) fn new(size:[u32; 2]) ->ImageData{
            ImageData{
                raw: Vec::with_capacity(size[0]as usize * size[1]as usize),
            }
        }

        pub(crate) fn write(&mut self, raw: Vec<Pixel>)-> std::io::Result<()>{
            self.raw = raw;
            
            Ok(())
        }
        
        pub(crate) fn translate(self, data_stream: &mut Vec<u8>) -> std::io::Result<()> {
            self.raw.into_iter()
                .for_each(
                    |pixel| pixel.translator(data_stream)
                    .expect("像素信息写入错误！ code：src/image_file/pixels_data::line 42  err_function:ImageData::translator")
                );
            
            Ok(())
        }
    }
