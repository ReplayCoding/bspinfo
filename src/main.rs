use binrw::BinRead;
use lzma_rs::lzma_decompress;
use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek},
};
use zip::ZipArchive;

#[allow(non_camel_case_types)]
#[derive(Debug, num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
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

#[allow(unused)]
#[derive(BinRead, Debug)]
struct BspHeader {
    ident: u32,
    version: u32,
    lumps: [LumpInfo; HEADER_LUMPS],
    map_revision: u32,
}

#[allow(unused)]
#[derive(BinRead, Debug)]
struct LumpInfo {
    fileofs: u32,
    filelen: u32,
    version: u32,
    uncompressed_size: u32,
}

struct BspFile<'a, R> {
    header: BspHeader,
    reader: &'a mut R,
}

impl<'a, R: Read + Seek> BspFile<'a, R> {
    fn new(reader: &'a mut R) -> binrw::BinResult<BspFile<R>> {
        Ok(Self {
            header: BspHeader::read_le(reader)?,
            reader,
        })
    }

    fn version(&self) -> u32 {
        self.header.version
    }

    fn map_revision(&self) -> u32 {
        self.header.map_revision
    }

    fn lumps(&self) -> &[LumpInfo; HEADER_LUMPS] {
        &self.header.lumps
    }

    fn get_lump(&mut self, lump: LumpType) -> Option<Vec<u8>> {
        let lump = self.header.lumps.get(lump as usize)?;

        if lump.fileofs == 0 || lump.filelen == 0 {
            return None;
        }

        self.reader
            .seek(io::SeekFrom::Start(lump.fileofs.into()))
            .ok()?;
        // Compressed
        Some(if lump.uncompressed_size != 0 {
            let mut buf: Vec<u8> = vec![];
            buf.resize(lump.uncompressed_size as usize, 0);

            lzma_decompress(&mut BufReader::new(&mut self.reader), &mut buf).ok()?;

            buf
        }
        // Uncompressed
        else {
            let mut buf: Vec<u8> = vec![];
            buf.resize(lump.filelen as usize, 0);

            self.reader.read_exact(&mut buf).ok()?;

            buf
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: bspinfo <mapname.bsp> [pakfile output dir]");
        return;
    }

    let mut reader = File::open(&args[1]).unwrap();
    let mut bsp = BspFile::new(&mut reader).unwrap();

    println!("BSP Version: {}", bsp.version());
    println!("Revision: {}", bsp.map_revision());

    println!("\nFiles:");
    if let Some(pak) = bsp.get_lump(LumpType::PAKFILE) {
        let mut pakreader = &mut Cursor::new(pak);
        let mut zip = ZipArchive::new(&mut pakreader).unwrap();

        for i in 0..zip.len() {
            let file = zip.by_index_raw(i).unwrap();
            println!("{}: crc32 = {}", file.name(), file.crc32());
        }
    };

    println!("\nEntities:");
    if let Some(entities) = bsp.get_lump(LumpType::ENTITIES) {
        std::io::copy(&mut Cursor::new(entities), &mut std::io::stdout()).unwrap();
    };
}
