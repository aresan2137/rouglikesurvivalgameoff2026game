struct UboData0 {
    proj: mat4x4f
}

struct UboData1 {
    view: mat4x4f
}

struct DUboData2 {
    model: mat4x4f,
    is_gui: u32
}

const MAP_WIDTH: u32 = 200u;
const MAP_HEIGHT: u32 = 200u;

const ATLAS_TILES_X: f32 = 8.0;
const ATLAS_TILES_Y: f32 = 1.0;

@group(0) @binding(0) var<uniform> data0: UboData0;

@group(1) @binding(0) var<uniform> data1: UboData1;

@group(2) @binding(0) var<uniform> data2: DUboData2;
@group(2) @binding(1) var t_base: texture_2d<f32>;
@group(2) @binding(2) var s_base: sampler;

@group(3) @binding(0) var<storage, read> map_data: array<u32>;

struct VertexData {
    @location(0) position: vec3f,
    @location(1) uv: vec2f,
    @location(2) normal: vec3f
}

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f
}

@vertex
fn vs_main(in: VertexData) -> VertexOutput {
    var out: VertexOutput;

    out.pos = data0.proj * data1.view * data2.model * vec4f(in.position, 1.0);

    out.uv = in.uv;

    return out;
}

fn get_tile(x: u32, y: u32) -> u32 {
    if x >= MAP_WIDTH || y >= MAP_HEIGHT {
        return 0u;
    }

    let index = x + y * MAP_WIDTH;
    let u32_index = index / 4u;
    let byte_offset = (index % 4u) * 8u;

    let packed_val = map_data[u32_index];
    return (packed_val >> byte_offset) & 0xFFu;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let map_pos = in.uv * vec2f(f32(MAP_WIDTH), f32(MAP_HEIGHT));

    let tile_x = u32(clamp(map_pos.x, 0.0, f32(MAP_WIDTH - 1u)));
    let tile_y = u32(clamp(map_pos.y, 0.0, f32(MAP_HEIGHT - 1u)));

    let tile_id = get_tile(tile_x, tile_y);

    let tile_uv = fract(map_pos);

    let sprite_idx = f32(tile_id);
    let atlas_x = sprite_idx % ATLAS_TILES_X;
    let atlas_y = floor(sprite_idx / ATLAS_TILES_X);

    let tile_size_uv = vec2f(1.0 / ATLAS_TILES_X, 1.0 / ATLAS_TILES_Y);
    let final_uv = (vec2f(atlas_x, atlas_y) + tile_uv) * tile_size_uv;

    let tex = textureSample(t_base, s_base, final_uv);

    return tex;
}