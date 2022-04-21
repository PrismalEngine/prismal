struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) tex_coords: vec2<f32>,
};
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    out.tex_coords = in.tex_coords;
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
    out.color = vec4<f32>(inverse_gamma_correct(in.color.rgb, 2.2), 1.0);
    return out;
}