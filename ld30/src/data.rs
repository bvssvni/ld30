extern crate wobj = "wavefront-obj";

use piston::AssetStore;
use std::io;

use vertex::Vertex;

#[deriving(PartialEq, Eq, FromPrimitive, Show)]
pub enum Type {
    Slab = 0,
}

static TYPE_LEN: uint = 1;

impl Type {
    pub fn get_obj_file(&self) -> Option<&'static str> {
        match *self {
            Slab => Some("slab.obj"),
        }
    }

    pub fn to_uint(&self) -> uint {
        *self as uint
    }
}

pub struct TypeIterator {
    ty: uint,
}

impl TypeIterator {
    pub fn new() -> TypeIterator {
        TypeIterator {
            ty: 0
        }
    }
}

impl Iterator<Type> for TypeIterator {
    fn next(&mut self) -> Option<Type> {
        match self.ty {
            x if x < TYPE_LEN => {
                self.ty += 1;
                FromPrimitive::from_uint(x)
            },
            _ => None
        }    
    }
}

pub struct Data {
    pub objs: Vec<Option<wobj::obj::ObjSet>>,
}

impl Data {
    pub fn from_asset_store(asset_store: &AssetStore) -> Data {
        Data {
            objs: read_objs(asset_store)
        }
    }
}

pub fn read_objs(asset_store: &AssetStore) -> Vec<Option<wobj::obj::ObjSet>> {
    let mut vec = Vec::new();
    for it in TypeIterator::new() {
        match it.get_obj_file() {
            None => vec.push(None),
            Some(file) => vec.push(Some({
                    let txt = io::File::open(&asset_store.path(file).unwrap()).read_to_string().unwrap();
                    wobj::obj::parse(txt).unwrap()
                }))
        }
    }
    vec
}
 
/// Extracts the coordinates of the corners and stores it in a list.
///
/// Returns the range where the object is stored in the list.
pub fn add_vertices(
    obj: &wobj::obj::Object, 
    vertices: &mut Vec<Vertex>
) -> (uint, uint) {
    let start = vertices.len();
    for v in obj.vertices.iter() {
        vertices.push(Vertex::new([v.x as f32, v.y as f32, v.z as f32]));
    }
    (start, vertices.len())
}


pub fn add_indices(
    geom: &wobj::obj::Geometry, 
    vertex_offset: uint, 
    indices: &mut Vec<u32>
) -> (uint, uint) {
    let start = indices.len();
    for shape in geom.shapes.iter() {
        match *shape {
            wobj::obj::Triangle((a, _), (b, _), (c, _)) => {
                indices.push(a as u32);
                indices.push(b as u32);
                indices.push(c as u32);
            },
            _ => {}
        }
    }
    (start, indices.len())
}

