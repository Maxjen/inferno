use std::hash::{Hash, SipHasher, Hasher};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use super::texture_atlas::TextureAtlas;
use image;
use image::GenericImage;
use std::path::Path;
use std::result;
use std::fmt;
use std::error::Error;
use glium::texture::TextureCreationError;
use rusttype::{FontCollection, Pixels, point, PositionedGlyph};

pub type Result<T> = result::Result<T, ResourceManagerError>;

const ATLAS_DIMENSION: usize = 2048;
const FONT_ATLAS_DIMENSION: usize = 1024;

#[derive(Clone)]
pub struct StringId {
    pub id: u64,
    string_id_table: Weak<RefCell<HashMap<u64, String>>>,
}

impl StringId {
    pub fn new(name: &str, string_id_table: Weak<RefCell<HashMap<u64, String>>>) -> Self {
        let mut s = SipHasher::new();
        name.hash(&mut s);
        let id = s.finish();
        let h = string_id_table.upgrade().expect("String id table doesn't exist!");
        let old = h.borrow_mut().insert(id, name.to_string());
        if let Some(old) = old {
            let new = name.to_string();
            if old != new {
                println!("Collision between {} and {} occured!", old, new);
            }
        }
        let result = StringId { id: id, string_id_table: string_id_table };
        result
    }

    pub fn get_name(&self) -> String {
        let string_id_table = self.string_id_table.upgrade().expect("String id table doesn't exist!");
        if let Some(name) = string_id_table.borrow_mut().get(&self.id) {
            return name.clone();
        }
        unreachable!();
    }
}

struct TextureInternal {
    atlas: Weak<RefCell<TextureAtlas>>,
    uv_min: (f32, f32),
    uv_max: (f32, f32),
    pixel_dimension: f32,
}

#[derive(Clone)]
pub struct Texture {
    pub name: StringId,
    pub atlas: Rc<RefCell<TextureAtlas>>,
    pub uv_min: (f32, f32),
    pub uv_max: (f32, f32),
    pub pixel_dimension: f32,
}

pub struct Glyph {
    pub uv_min: (f32, f32),
    pub uv_max: (f32, f32),
    pub width: f32,
    pub height: f32,
    pub offset: (f32, f32),
    pub advance_x: f32,
}

impl Glyph {
    pub fn new() -> Glyph {
        Glyph {
            uv_min: (0.0, 0.0),
            uv_max: (0.0, 0.0),
            width: 0.0,
            height: 0.0,
            offset: (0.0, 0.0),
            advance_x: 0.0,
        }
    }
}

struct FontInternal {
    atlas: Weak<RefCell<TextureAtlas>>,
    glyphs: Weak<RefCell<HashMap<char, Glyph>>>,
    kernings: Weak<RefCell<HashMap<(char, char), f32>>>,
}

#[derive(Clone)]
pub struct Font {
    pub name: StringId,
    pub path: String,
    pub atlas: Rc<RefCell<TextureAtlas>>,
    pub glyphs: Rc<RefCell<HashMap<char, Glyph>>>,
    pub kernings: Rc<RefCell<HashMap<(char, char), f32>>>,
    pub size: f32,
}

impl Font {
    pub fn load_glyphs(&self) {
        use std::path::Path;
        use std::fs::File;
        use std::io::prelude::*;
        use std::f32;

        let path = Path::new(&self.path);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(err) => panic!("couldn't open {}: {}", display, Error::description(&err)),
            Ok(file) => file,
        };

        let mut font_data = Vec::new();
        match file.read_to_end(&mut font_data) {
            Err(err) => panic!("couldn't read {}: {}", display, Error::description(&err)),
            Ok(_) => {}
        };

        //let font_data = include_bytes!("Gudea-Regular.ttf");
        let collection = FontCollection::from_bytes(font_data);
        let font = collection.into_font().unwrap();

        let scale = Pixels(self.size);

        /*let v_metrics = font.v_metrics(scale);
        println!("v_metrics {} {}", v_metrics.ascent, v_metrics.descent);*/

        for i in 0..129 {
            let c = (i as u8) as char;
            let g = match font.glyph(c) {
                Some(g) => g,
                None => continue,
            };
            let g = g.scaled(scale);
            let exact_bounding_box = match g.exact_bounding_box() {
                Some(exact_bounding_box) => exact_bounding_box,
                None => continue,
            };
            if c == 'P' {
                println!("exact");
                println!("{:?}", exact_bounding_box);
            }
            let pos_x: f32 = -(exact_bounding_box.min.x as f32).floor();
            let pos_y: f32 = (exact_bounding_box.max.y as f32).ceil();

            let g2 = g.clone();
            let g2 = g2.positioned(point(0.0, 0.0));
            let pbb = match g2.pixel_bounding_box() {
                Some(pbb) => pbb,
                None => continue,
            };
            if c == 'P' {
                println!("pixel");
                println!("{:?}", pbb);
            }

            let g = g.positioned(point(pos_x, pos_y));
            let pixel_bounding_box = match g.pixel_bounding_box() {
                Some(pixel_bounding_box) => pixel_bounding_box,
                None => continue,
            };
            if c == 'P' {
                println!("pixel2");
                println!("{:?}\n", pixel_bounding_box);
                let h_metrics = g.h_metrics();
                println!("left {}, advance {}", h_metrics.left_side_bearing, h_metrics.advance_width);
            }
            let width = pixel_bounding_box.max.x as usize;
            let height = pixel_bounding_box.max.y as usize;
            let mut buf: Vec<u8> = vec![0; (width + 1) * (height + 1)];
            g.draw(|x, y, v| {
                buf[x as usize + y as usize * width] = (v * 255.0) as u8;
            });
            let mut atlas = self.atlas.borrow_mut();
            let region = match atlas.get_region(width, height) {
                Some(region) => region,
                None => continue,
            };
            atlas.set_region(region.0, region.1, region.2, region.3, &buf, width as usize);

            let mut glyph = Glyph::new();
            glyph.uv_min = (region.0 as f32 / FONT_ATLAS_DIMENSION as f32, region.1 as f32 / FONT_ATLAS_DIMENSION as f32);
            glyph.uv_max = ((region.0 as f32 + width as f32) / FONT_ATLAS_DIMENSION as f32, (region.1 as f32 + height as f32)  / FONT_ATLAS_DIMENSION as f32);
            glyph.width = region.2 as f32;
            glyph.height = region.3 as f32;
            glyph.offset = (-pos_x, pos_y);
            glyph.advance_x = g.h_metrics().advance_width;
            self.glyphs.borrow_mut().insert(c, glyph);

            for c_other in self.glyphs.borrow().keys() {
                let kerning1 = font.pair_kerning(scale, c, *c_other);
                let kerning2 = font.pair_kerning(scale, *c_other, c);
                if kerning1 != 0.0 {
                    print!("{:.10} ", kerning1);
                    self.kernings.borrow_mut().insert((c, *c_other), kerning1);
                }
                if kerning2 != 0.0 {
                    print!("{:.10} ", kerning2);
                    self.kernings.borrow_mut().insert((*c_other, c), kerning2);
                }
            }
        }
        let mut atlas = self.atlas.borrow_mut();
        atlas.save_to_png(0);
    }
}

#[derive(Debug)]
pub enum ResourceManagerError {
    Image(image::ImageError),
    Texture(TextureCreationError),
    TooLarge,
}

impl fmt::Display for ResourceManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResourceManagerError::Image(ref err) => write!(f, "Image error: {}", err),
            ResourceManagerError::Texture(_) => write!(f, "Texture creation error!"),
            ResourceManagerError::TooLarge => write!(f, "TooLarge error: image dimension too large!"),
        }
    }
}

impl Error for ResourceManagerError {
    fn description(&self) -> &str {
        match *self {
            ResourceManagerError::Image(ref err) => err.description(),
            ResourceManagerError::Texture(_) => &"Texture creation Error",
            ResourceManagerError::TooLarge => &"Image dimension too large!",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ResourceManagerError::Image(ref err) => Some(err),
            ResourceManagerError::Texture(_) => None,
            _ => None,
        }
    }
}

impl From<image::ImageError> for ResourceManagerError {
    fn from(err: image::ImageError) -> ResourceManagerError {
        ResourceManagerError::Image(err)
    }
}

impl From<TextureCreationError> for ResourceManagerError {
    fn from(err: TextureCreationError) -> ResourceManagerError {
        ResourceManagerError::Texture(err)
    }
}

pub struct ResourceManager {
    string_id_table: Rc<RefCell<HashMap<u64, String>>>,
    current_atlas: Rc<RefCell<TextureAtlas>>,
    textures: HashMap<u64, TextureInternal>,
    fonts: HashMap<u64, FontInternal>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            string_id_table: Rc::new(RefCell::new(HashMap::<u64, String>::new())),
            current_atlas: Rc::new(RefCell::new(TextureAtlas::new(ATLAS_DIMENSION, ATLAS_DIMENSION, 4))),
            textures: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn intern_string(&self, string: &str) -> StringId {
        StringId::new(string, Rc::downgrade(&self.string_id_table))
    }

    pub fn get_texture(&self, string: &str) -> Option<Texture> {
        unimplemented!();
    }

    pub fn create_texture(&mut self, string: &str) -> Result<Texture> {
        let texture_name = self.intern_string(string);

        if let Some(texture_internal) = self.textures.get(&texture_name.id) {
            if let Some(atlas) = texture_internal.atlas.upgrade() {
                return Ok(Texture {
                    name: texture_name,
                    atlas: atlas,
                    uv_min: texture_internal.uv_min,
                    uv_max: texture_internal.uv_max,
                    pixel_dimension: texture_internal.pixel_dimension,
                })
            }
        }
        self.textures.remove(&texture_name.id);

        let img = try!(image::open(&Path::new(string)));
        let (width, height) = img.dimensions();
        let mut uv_min: (f32, f32) = (0.0, 0.0);
        let mut uv_max: (f32, f32) = (0.0, 0.0);
        let pixel_dimension = 1.0f32 / ATLAS_DIMENSION as f32;

        let attempt_set_region = |this: &mut ResourceManager, uv_min: &mut (f32, f32), uv_max: &mut (f32, f32)| -> bool {
            let mut atlas = this.current_atlas.borrow_mut();
            let region = match atlas.get_region(width as usize, height as usize) {
                Some(region) => region,
                None => return false,
            };
            atlas.set_region(region.0, region.1, region.2, region.3, &img.raw_pixels(), width as usize * 4);
            *uv_min = (region.0 as f32 / ATLAS_DIMENSION as f32, region.1 as f32 / ATLAS_DIMENSION as f32);
            *uv_max = (uv_min.0 as f32 + width as f32 * pixel_dimension, uv_min.1 as f32 + height as f32 * pixel_dimension);
            return true;
            /*if let Some(region) = atlas.get_region(width as usize, height as usize) {
                atlas.set_region(region.0, region.1, region.2, region.3, &img.raw_pixels(), width as usize * 4);
                *uv_min = (region.0 as f32 / ATLAS_DIMENSION as f32, region.1 as f32 / ATLAS_DIMENSION as f32);
                *uv_max = (uv_min.0 as f32 + width as f32 * pixel_dimension, uv_min.1 as f32 + height as f32 * pixel_dimension);
                return true;
            }
            return false;*/
        };

        let mut success;
        success = attempt_set_region(self, &mut uv_min, &mut uv_max);
        if !success {
            self.current_atlas = Rc::new(RefCell::new(TextureAtlas::new(ATLAS_DIMENSION, ATLAS_DIMENSION, 4)));
            success = attempt_set_region(self, &mut uv_min, &mut uv_max);
        }
        if !success {
            return Err(ResourceManagerError::TooLarge);
        }

        self.textures.insert(texture_name.id, TextureInternal {
            atlas: Rc::downgrade(&self.current_atlas),
            uv_min: (uv_min.0, uv_min.1),
            uv_max: (uv_max.0, uv_max.1),
            pixel_dimension: pixel_dimension,
        });

        Ok(Texture {
            name: texture_name,
            atlas: self.current_atlas.clone(),
            uv_min: (uv_min.0, uv_min.1),
            uv_max: (uv_max.0, uv_max.1),
            pixel_dimension: pixel_dimension,
        })
    }

    pub fn create_font(&mut self, string: &str, size: u32) -> Result<Font> {
        let font_name_string = format!("{}{}", string, size);
        let font_name = self.intern_string(&font_name_string);

        if let Some(font_internal) = self.fonts.get(&font_name.id) {
            if let Some(atlas) = font_internal.atlas.upgrade() {
                if let Some(glyphs) = font_internal.glyphs.upgrade() {
                    if let Some(kernings) = font_internal.kernings.upgrade() {
                        return Ok(Font {
                            name: font_name,
                            path: string.to_string(),
                            atlas: atlas,
                            glyphs: glyphs,
                            kernings: kernings,
                            size: size as f32,
                        })
                    }
                }
            }
        }
        self.fonts.remove(&font_name.id);

        let font_atlas = Rc::new(RefCell::new(TextureAtlas::new(FONT_ATLAS_DIMENSION, FONT_ATLAS_DIMENSION, 1)));
        let glyphs = Rc::new(RefCell::new(HashMap::new()));
        let kernings = Rc::new(RefCell::new(HashMap::new()));

        self.fonts.insert(font_name.id, FontInternal {
            atlas: Rc::downgrade(&font_atlas),
            glyphs: Rc::downgrade(&glyphs),
            kernings: Rc::downgrade(&kernings),
        });

        Ok(Font {
            name: font_name,
            path: string.to_string(),
            atlas: font_atlas,
            glyphs: glyphs,
            kernings: kernings,
            size: size as f32,
        })
    }

    /*pub fn upload_atlas(&mut self, frame: &glium::backend::glutin_backend::GlutinFacade, id: usize) -> Result<()> {
        let pixels = self.texture_atlases.get(id).unwrap().0.get_pixels();
        let image = glium::texture::RawImage2d::from_raw_rgba(pixels, (ATLAS_DIMENSION as u32, ATLAS_DIMENSION as u32));
        let texture = glium::texture::Texture2d::new(frame, image).unwrap();
        self.texture_atlases.get_mut(id).unwrap().1 = Some(texture);
        Ok(())
    }

    pub fn get_atlas_texture(&self, id: usize) -> Option<&Texture2d> {
        self.texture_atlases.get(id).unwrap().1.as_ref()
    }

    pub fn save_atlases_to_png(&self) {
        for (i, atlas) in self.texture_atlases.iter().enumerate() {
            atlas.0.save_to_png(i);
        }
    }*/
}
