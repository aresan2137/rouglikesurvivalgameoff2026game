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

@group(0) @binding(0) var<uniform> data0: UboData0;

@group(1) @binding(0) var<uniform> data1: UboData1;

@group(2) @binding(0) var<uniform> data2: DUboData2;
@group(2) @binding(1) var t_base: texture_2d<f32>;
@group(2) @binding(2) var s_base: sampler;

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

    if (data2.is_gui == 1u) {
        out.pos = data0.proj * data2.model * vec4f(in.position, 1.0);
    } else {
        out.pos = data0.proj * data1.view * data2.model * vec4f(in.position, 1.0);
    }

    out.uv = in.uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let tex = textureSample(t_base, s_base, in.uv);

    if tex.a <= 0.05 {
        discard;
    }

    return tex; 
}