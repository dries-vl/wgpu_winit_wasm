// Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(VERTEX_IN: VertexInput) -> VertexOutput {
    var VERTEX_OUT: VertexOutput;
    VERTEX_OUT.color = VERTEX_IN.color;
    VERTEX_OUT.clip_position = vec4<f32>(VERTEX_IN.position, 1.0);
    return VERTEX_OUT;
}

@fragment
fn fs_main(VERTEX: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(VERTEX.color, 1.0);
}
