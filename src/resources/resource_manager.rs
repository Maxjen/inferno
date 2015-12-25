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

pub type Result<T> = result::Result<T, ResourceManagerError>;

const ATLAS_DIMENSION: usize = 2048;

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
        let h = string_id_table.upgrade().unwrap();
        h.borrow_mut().insert(id, name.to_string());
        let result = StringId { id: id, string_id_table: string_id_table };
        result
    }

    pub fn get_name(&self) -> String {
        let h = self.string_id_table.upgrade().unwrap();
        let result = match h.borrow_mut().get(&self.id) {
            Some(name) => name.clone(),
            None => "null".to_string()
        };
        result
    }
}

#[derive(Clone)]
pub struct Texture {
    pub name: StringId,
    pub atlas: Rc<RefCell<TextureAtlas>>,
    pub uv_min: (f32, f32),
    pub uv_max: (f32, f32),
    pub pixel_dimension: f32,
}

struct TextureInternal {
    atlas: Weak<RefCell<TextureAtlas>>,
    uv_min: (f32, f32),
    uv_max: (f32, f32),
    pixel_dimension: f32,
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
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            string_id_table: Rc::new(RefCell::new(HashMap::<u64, String>::new())),
            current_atlas: Rc::new(RefCell::new(TextureAtlas::new(ATLAS_DIMENSION, ATLAS_DIMENSION, 4))),
            textures: HashMap::<u64, TextureInternal>::new(),
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
            if let Some(region) = atlas.get_region(width as usize, height as usize) {
                atlas.set_region(region.0, region.1, region.2, region.3, &img.raw_pixels(), width as usize * 4);
                *uv_min = (region.0 as f32 / ATLAS_DIMENSION as f32, region.1 as f32 / ATLAS_DIMENSION as f32);
                *uv_max = (uv_min.0 as f32 + width as f32 * pixel_dimension, uv_min.1 as f32 + height as f32 * pixel_dimension);
                return true;
            }
            return false;
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
