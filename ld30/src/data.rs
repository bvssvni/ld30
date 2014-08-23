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
 
pub fn add_vertices(obj: wobj::obj::Object, list: &mut Vec<Vertex>) -> (uint, uint) {
    let start = list.len();
    for v in obj.vertices.iter() {
        list.push(Vertex::new([v.x as f32, v.y as f32, v.z as f32]));
    }
    (start, list.len())
}

