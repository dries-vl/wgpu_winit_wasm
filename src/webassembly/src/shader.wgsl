// Shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

// BEFORE VERTEX FUNCTION:
// Input Assembly: read vertex + index buffer and gather vertex for each index
// Use 'vertex pulling': cache result of vertex function when vertex used more than once

@vertex
fn vertex(VERTEX_IN: VertexInput) -> VertexOutput {
    var VERTEX_OUT: VertexOutput;
    VERTEX_OUT.tex_coords = VERTEX_IN.tex_coords;
    VERTEX_OUT.clip_position = vec4<f32>(VERTEX_IN.position, 1.0);
    return VERTEX_OUT;
}

// BETWEEN VERTEX AND FRAGMENT FUNCTIONS:
// Primitive Assembly: assemble into primitives -> uses index buffer
// Clipping: clip primitives: only pass on parts of primitives that are within view
// Culling: cull back-facing primitives etc.
// Rasterization: convert each primitive into set of fragments
// Depth + Stencil test: discard fragments on depth/stencil value; might also be done after fragment function

@group(0) @binding(0) // group index from set_bind_group(), binding index from BindGroupLayout and BindGroup
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fragment(VERTEX: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, VERTEX.tex_coords);
}

// AFTER FRAGMENT FUNCTION:
// Blending: blend fragment color with color already in frame buffer
// Write final value to frame buffer
