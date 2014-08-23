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
    pub obj_data: Vec<Option<wobj::obj::ObjSet>>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub index_ranges: Vec<IndexRange>,
    pub objs: Vec<ObjectRange>,
    pub obj_sets: Vec<ObjSetRange>
}

impl Data {
    pub fn from_asset_store(asset_store: &AssetStore) -> Data {
        let obj_data = read_obj_data(asset_store);
        let vertices = Vec::new();
        let indices = Vec::new();
        let index_ranges = Vec::new();
        let objs = Vec::new();
        let obj_sets = Vec::new();
        Data {
            obj_data: obj_data,
            vertices: vertices,
            indices: indices,
            index_ranges: index_ranges,
            objs: objs,
            obj_sets: obj_sets
        }
    }
}

pub fn read_obj_data(asset_store: &AssetStore) -> Vec<Option<wobj::obj::ObjSet>> {
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

/// Stores [start, end) ranges to vertices.
pub struct VertexRange(uint, uint);

impl VertexRange {
    /// Extracts the coordinates of the corners and stores it in a list.
    ///
    /// Returns the range where the object is stored in the list.
    pub fn add_vertices(
        obj: &wobj::obj::Object, 
        vertices: &mut Vec<Vertex>
    ) -> VertexRange {
        let start = vertices.len();
        for v in obj.vertices.iter() {
            vertices.push(Vertex::new([v.x as f32, v.y as f32, v.z as f32]));
        }
        VertexRange(start, vertices.len())
    }
}

/// Stores [start, end) ranges to indices.
pub struct IndexRange(uint, uint);

impl IndexRange {
    /// Extracts the indices from the triangles and stores it in a list.
    ///
    /// Returns the range where the geometry is stored in the list.
    pub fn add_indices(
        geom: &wobj::obj::Geometry, 
        VertexRange(offset, _): VertexRange, 
        indices: &mut Vec<u32>
    ) -> IndexRange {
        let start = indices.len();
        for shape in geom.shapes.iter() {
            match *shape {
                // Extract triangles and offset them relative to the
                // position in the vertex mesh.
                wobj::obj::Triangle((a, _), (b, _), (c, _)) => {
                    indices.push((a + offset) as u32);
                    indices.push((b + offset) as u32);
                    indices.push((c + offset) as u32);
                },
                _ => {}
            }
        }
        IndexRange(start, indices.len())
    }
}

/// Stores [start, end) ranges for objects.
///
/// This points to a range of index ranges.
pub struct ObjectRange(uint, uint);

impl ObjectRange {
    pub fn add_object(
        obj: &wobj::obj::Object,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        index_ranges: &mut Vec<IndexRange>
    ) -> ObjectRange {
        let vertex_range = VertexRange::add_vertices(obj, vertices);
        let start = index_ranges.len();
        for geom in obj.geometry.iter() {
            index_ranges.push(IndexRange::add_indices(geom, vertex_range, indices))
        }
        ObjectRange(start, index_ranges.len())
    }
}

/// Stores [start, end) for object sets (one OBJ file).
///
/// This points to a range of object ranges.
pub struct ObjSetRange(uint, uint);

impl ObjSetRange {
    pub fn add_obj_set(
        obj_set: &wobj::obj::ObjSet,
        vertices: &mut Vec<Vertex>,
        indices: &mut Vec<u32>,
        index_ranges: &mut Vec<IndexRange>,
        objs: &mut Vec<ObjectRange>
    ) -> ObjSetRange {
        let start = objs.len();
        for obj in obj_set.objects.iter() {
            objs.push(
                ObjectRange::add_object(
                    obj, 
                    vertices,
                    indices,
                    index_ranges
                )
            );
        }
        ObjSetRange(start, objs.len())
    }
}

