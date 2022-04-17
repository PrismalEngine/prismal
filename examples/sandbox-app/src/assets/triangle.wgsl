struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
};
struct VertexInput {
    @builtin(vertex_index) index: u32,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    var positions = array<vec2<f32>, 6>(
        // Triangle 00
        vec2(-1.0,  1.0), // Top-Left
        vec2(-1.0, -1.0), // Bottom-Left
        vec2( 1.0,  1.0), // Top-Right
        // Triangle 01
        vec2( 1.0,  1.0), // Top-Right
        vec2(-1.0, -1.0), // Bottom-Left
        vec2( 1.0, -1.0), // Bottom-Right
    );
    var colors = array<vec3<f32>, 6>(
        // Triangle 00
        vec3(1.0, 1.0, 0.0), // Top-Left: Yellow
        vec3(0.0, 0.0, 1.0), // Bottom-Left: Blue
        vec3(1.0, 1.0, 1.0), // Top-Right: White
        // Triangle 01
        vec3(1.0, 1.0, 1.0), // Top-Right: White
        vec3(0.0, 0.0, 1.0), // Bottom-Left: Blue
        vec3(1.0, 0.0, 0.0), // Bottom-Right: Red
    );
    out.pos = vec4<f32>(positions[in.index], 0.0, 1.0);
    out.color = colors[in.index];

    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
};

fn inverse_gamma_correct(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3(gamma));
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;
    out.color = vec4<f32>(inverse_gamma_correct(in.color, 2.2), 1.0);
    return out;
}