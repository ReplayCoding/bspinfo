use binrw::BinRead;
use std::fs::File;

#[allow(non_camel_case_types)]
#[derive(Debug, num_enum::TryFromPrimitive)]
#[repr(u32)]
enum LumpType {
    ENTITIES = 0,
    PLANES = 1,
    TEXTURE_DATA = 2,
    VERTICES = 3,
    VISIBILITY = 4,
    NODES = 5,
    TEXTURE_INFO = 6,
    FACES = 7,
    LIGHTING = 8,
    OCCLUSION = 9,
    LEAVES = 10,
    FACE_IDS = 11,
    EDGES = 12,
    SURFEDGES = 13,
    MODELS = 14,
    WORLD_LIGHTS = 15,
    LEAF_FACES = 16,
    LEAF_BRUSHES = 17,
    BRUSHES = 18,
    BRUSH_SIDES = 19,
    AREAS = 20,
    AREA_PORTALS = 21,
    UNUSED_22 = 22,
    UNUSED_23 = 23,
    UNUSED_24 = 24,
    UNUSED_25 = 25,
    DISPLACEMENT_INFO = 26,
    ORIGINAL_FACES = 27,
    PHYSICS_DISPLACEMENT = 28,
    PHYSICS_COLLIDE = 29,
    VERTEX_NORMALS = 30,
    VERTEX_NORMAL_INDICES = 31,
    DISPLACEMENT_LIGHTMAP_ALPHAS = 32,
    DISPLACEMENT_VERTICES = 33,
    DISPLACEMENT_LIGHTMAP_SAMPLE_POSITIONS = 34,
    GAME_LUMP = 35,
    LEAF_WATER_DATA = 36,
    PRIMITIVES = 37,
    PRIMITIVE_VERTICES = 38,
    PRIMITIVE_INDICES = 39,
    PAKFILE = 40,
    CLIP_PORTAL_VERTICES = 41,
    CUBEMAPS = 42,
    TEXTURE_DATA_STRING_DATA = 43,
    TEXTURE_DATA_STRING_TABLE = 44,
    OVERLAYS = 45,
    LEAF_MIN_DIST_TO_WATER = 46,
    FACE_MACRO_TEXTURE_INFO = 47,
    DISPLACEMENT_TRIS = 48,
    PHYSICS_COLLIDE_SURFACE = 49,
    WATER_OVERLAYS = 50,
    LEAF_AMBIENT_INDEX_HDR = 51,
    LEAF_AMBIENT_INDEX = 52,
    LIGHTING_HDR = 53,
    WORLD_LIGHTS_HDR = 54,
    LEAF_AMBIENT_LIGHTING_HDR = 55,
    LEAF_AMBIENT_LIGHTING = 56,
    XZIP_PAKFILE = 57,
    FACES_HDR = 58,
    MAP_FLAGS = 59,
    OVERLAY_FADES = 60,
    UNUSED_61 = 61,
    PHYSICS_LEVEL = 62,
    UNUSED_63 = 63,
}

const HEADER_LUMPS: usize = 64;

#[derive(BinRead, Debug)]
struct BspHeader {
    ident: u32,
    version: u32,
    lumps: [LumpInfo; HEADER_LUMPS],
    map_revision: u32,
}

#[derive(BinRead, Debug)]
struct LumpInfo {
    fileofs: u32,
    filelen: u32,
    version: u32,
    uncompressed_size: u32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: bspinfo <mapname.bsp>");
        return;
    }

    let mut file = File::open(&args[1]).unwrap();
    let bsp = BspHeader::read_le(&mut file).unwrap();

    println!("BSP Version: {}", bsp.version);
    println!("Revision: {}", bsp.map_revision);
    for (idx, lump) in bsp.lumps.iter().enumerate() {
        let mut lump_name = format!("{}", idx);
        if let Ok(lump_type) = LumpType::try_from(idx as u32) {
            lump_name = format!("{:?}", lump_type);
        };

        let size = if lump.uncompressed_size != 0 {
            lump.uncompressed_size
        } else {
            lump.filelen
        };

        println!("{} (v{}): size = {}", lump_name, lump.version, size);
    }
}
