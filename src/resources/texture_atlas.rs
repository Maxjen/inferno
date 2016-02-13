use std::usize;
use std::ptr;
use std::path::Path;
use image;
use glium;
use glium::texture::Texture2d;

pub struct TextureAtlas {
    width: usize,
    height: usize,
    depth: usize,
    skyline: Vec<(usize, usize, usize)>,
    used: usize,
    data: Vec<u8>,

    texture: Option<Texture2d>,
    needs_upload: bool,
}

impl TextureAtlas {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let mut result = TextureAtlas {
            width: width,
            height: height,
            depth: depth,
            skyline: Vec::new(),
            used: 0,
            data: Vec::with_capacity(width * height * depth),
            texture: None,
            needs_upload: true,
            //data: vec![0; width * height * depth]
        };
        result.skyline.push((1, 1, width - 2));
        for _ in 0..result.data.capacity() {
            result.data.push(0);
        }

        /*result.skyline.push((1, 1, 5));
        result.skyline.push((6, 3, 5));
        result.skyline.push((11, 3, 5));
        result.skyline.push((16, 2, 5));
        result.skyline.push((21, 3, 5));*/
        result
    }

    pub fn set_region(&mut self, x: usize, y: usize, width: usize, height: usize, data: &Vec<u8>, stride: usize) {
        // TODO: checks

        for i in 0..height {
            unsafe {
                ptr::copy_nonoverlapping(&data[i * stride], &mut self.data[((y + i) * self.width + x) * self.depth], width * self.depth);
            }
        }
        self.needs_upload = true;
    }

    fn fit(&self, index: usize, width: usize, height: usize) -> Option<usize> {
        let (x, mut y, _) = self.skyline[index];
        let mut width_left: i32 = width as i32;
        let mut i = index;

        if x + width > self.width - 1 {
            //return Err("Not enough atlas width left!");
            return None;
        }
        while width_left > 0 {
            let (_, cur_y, cur_width) = self.skyline[i];
            if cur_y > y {
                y = cur_y;
            }

            if y + height > self.height - 1 {
                //return Err("Not enough atlas height left!")
                return None;
            }

            width_left -= cur_width as i32;
            i += 1;
        }
        //Ok(y)
        Some(y)
    }

    fn merge(&mut self) {
        let mut to_remove = Vec::<usize>::new();
        let mut last = 0;

        for i in 1..self.skyline.len() {
            if self.skyline[i].1 == self.skyline[last].1 {
                self.skyline[last].2 += self.skyline[i].2;
                to_remove.push(i);
            }
            else {
                last = i;
            }
        }

        for i in to_remove.iter().rev() {
            self.skyline.remove(*i);
        }

        /*for &(x, y, width) in &self.skyline {
            println!("{} {} {}", x, y, width);
        }*/
    }

    pub fn get_region(&mut self, width: usize, height: usize) -> Option<(usize, usize, usize, usize)> {
        let mut region = (0, 0, width, height);
        let (mut best_height, mut best_width, mut best_index) = (usize::MAX, usize::MAX, usize::MAX);

        let mut i = 0;
        for &(cur_x, _, cur_width) in &self.skyline {
            if let Some(y) = self.fit(i, width, height) {
                if (y + height < best_height) || (y + height == best_height && cur_width < best_width) {
                    best_height = y + height;
                    best_index = i;
                    best_width = cur_width;
                    region.0 = cur_x;
                    region.1 = y;
                }
            }
            i += 1;
        }

        if best_index == usize::MAX {
            //return Err("Couldn't fit region into atlas!");
            return None;
        }

        self.skyline.insert(best_index, (region.0, region.1 + height, width));

        let mut to_remove = Vec::<usize>::new();

        for i in best_index + 1..self.skyline.len() {
            if self.skyline[i].0 < region.0 + region.2 {
                let shrink = region.0 + region.2 - self.skyline[i].0;
                if shrink >= self.skyline[i].2 {
                    to_remove.push(i);
                }
                else {
                    self.skyline[i].0 += shrink;
                    self.skyline[i].2 -= shrink;
                    break;
                }
            }
            else {
                break;
            }
        }

        for i in to_remove.iter().rev() {
            self.skyline.remove(*i);
        }

        self.merge();
        self.used += width * height;
        Some(region)
    }

    pub fn clear(&mut self) {
        self.skyline.clear();
        self.skyline.push((1, 1, self.width - 2));
        self.used = 0;

        for i in &mut self.data {
            *i = 0;
        }
    }

    pub fn upload(&mut self, frame: &glium::backend::glutin_backend::GlutinFacade) {
        use std::borrow::Cow;

        if self.needs_upload {
            if self.depth == 4 {
                let image = glium::texture::RawImage2d::from_raw_rgba(self.data.clone(), (self.width as u32, self.height as u32));
                let texture = glium::texture::Texture2d::new(frame, image).unwrap();
                self.texture = Some(texture);
            } else if self.depth == 1 {
                let mut buf = Vec::<glium::texture::RawImage1d<u8>>::new();
                for i in 0..self.height {
                    let row_data = Cow::Borrowed(&self.data[i * self.width..(i + 1) * self.width]);
                    let row = glium::texture::RawImage1d {
                        data: row_data,
                        width: self.width as u32,
                        format: glium::texture::ClientFormat::U8,
                    };
                    buf.push(row);
                }
                let image = glium::texture::RawImage2d::from_vec_raw1d(&buf);
                let texture = glium::texture::Texture2d::new(frame, image).unwrap();
                self.texture = Some(texture);
            }
            self.needs_upload = false;
        }
    }

    pub fn get_texture(&self) -> Option<&Texture2d> {
        self.texture.as_ref()
    }

    /*pub fn get_pixels(&self) -> Vec<u8> {
        return self.data.clone();
    }*/

    pub fn save_to_png(&self, i: usize) {
        if self.depth == 4 {
            let name = format!("atlas{}.png", i);
            image::save_buffer(&Path::new(&*name), &self.data, self.width as u32, self.height as u32, image::RGBA(8)).unwrap();
        }
        else if self.depth == 1 {
            let name = format!("font_atlas{}.png", i);
            image::save_buffer(&Path::new(&*name), &self.data, self.width as u32, self.height as u32, image::Gray(8)).unwrap();
        }
    }

    pub fn print_skyline(&self) {
        println!("{:?}", self.skyline);
    }
}
