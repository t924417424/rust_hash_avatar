use std::path::Path;

use rand::Rng;
pub struct Generator {
    size: u32,
    padding: u32,
    block_num: u32,
    img: Option<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
}

struct PointInfo {
    x: u32,
    y: u32,
}
impl Generator {
    /// 创建随机图像生成器
    /// # 默认参数
    /// ```
    /// Generator {
    ///     size: 360,
    ///     padding: 30,
    ///     block_num: 10,
    ///     img: None,
    /// }
    /// ```
    pub fn new() -> Self {
        Generator {
            size: 360,
            padding: 30,
            block_num: 10,
            img: None,
        }
    }
    /// 设置画板大小
    pub fn set_img_size(&mut self, size: u32) -> Self {
        self.size = size;
        Generator {
            size: self.size,
            padding: self.padding,
            block_num: self.block_num,
            img: None,
        }
    }
    /// 设置图像边距
    pub fn set_padding(&mut self, padding: u32) -> Self {
        self.padding = padding;
        Generator {
            size: self.size,
            padding: self.padding,
            block_num: self.block_num,
            img: None,
        }
    }
    /// 设置每一行的色块数量
    pub fn set_block_num(&mut self, block_num: u32) -> Self {
        self.block_num = block_num;
        Generator {
            size: self.size,
            padding: self.padding,
            block_num: self.block_num,
            img: None,
        }
    }
    /// 获取随机RGBA色值
    fn rand_rgba() -> image::Rgba<u8> {
        let mut color: [u8; 4] = [255; 4];
        color[0] = rand::thread_rng().gen_range(0..255);
        color[1] = rand::thread_rng().gen_range(0..255);
        color[2] = rand::thread_rng().gen_range(0..255);
        image::Rgba(color)
    }
    fn generate_random_point_list(&self) -> Box<Vec<PointInfo>> {
        let mut result: Vec<PointInfo> = Vec::new();
        for x in 0..=self.block_num / 2 {
            for y in 0..self.block_num {
                if Self::rand_fill() {
                    result.push(PointInfo { x: x, y: y });
                }
            }
        }
        Box::new(result)
    }
    fn rand_fill() -> bool {
        rand::thread_rng().gen_ratio(50, 100)
    }
    /// 创建图像并画出色块
    pub fn create(&mut self) -> Self {
        let block_len = (self.size - 2 * self.padding) / self.block_num;
        let color = Generator::rand_rgba();
        let points = self.generate_random_point_list();
        let mut img: image::ImageBuffer<image::Luma<u8>, _> =
            image::ImageBuffer::new(self.size, self.size);
        img.fill(238);
        let mut img = image::DynamicImage::ImageLuma8(img).into_rgba8();
        for point in points.iter() {
            let p_x = point.x * block_len + self.padding;
            let p_y = point.y * block_len + self.padding;
            //println!("{:#?}",point);
            for x in p_x..=(p_x + block_len) {
                for y in p_y..=(p_y + block_len) {
                    img.put_pixel(x, y, color);
                    img.put_pixel(self.size - x, self.size - (self.size - y), color);
                }
            }
        }
        Generator {
            size: self.size,
            padding: self.padding,
            block_num: self.block_num,
            img: Some(img),
        }
    }
    /// 保存图片到png文件
    pub fn save_to_png<P>(&self, path: P) -> Result<(), &'static str>
    where
        P: AsRef<Path>,
    {
        return match &self.img {
            Some(img) => match img.save(path) {
                Ok(_) => Ok(()),
                Err(_e) => Err("保存失败"),
            },
            None => Err("资源不存在！"),
        };
    }
}

impl Drop for Generator {
    fn drop(&mut self) {
        println!("{}", "drop");
    }
}
