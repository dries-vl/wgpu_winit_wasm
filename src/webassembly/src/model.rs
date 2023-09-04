pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)] // need bytemuck to cast to &[u8] for buffer
pub struct BasicVertex {                                                  // Pod = plain old data = can convert to u8
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex for BasicVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BasicVertex>() as wgpu::BufferAddress, // shader skips over this many bytes for next vertex
            step_mode: wgpu::VertexStepMode::Vertex, // per-vertex <=> per-instance data
            attributes: &[ // fields of vertex object
                wgpu::VertexAttribute {
                    offset: 0, // first in memory -> no jump required to get
                    shader_location: 0, // corresponds to @location(0) some_name: vec3<f32>
                    format: wgpu::VertexFormat::Float32x3, // corresponding shader format; FYI: MAX SIZE IS 32x4
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress, // offset as in jump over the previous elements
                    shader_location: 1, // corresponds to @location(1) some_name: vec3<f32>
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

pub const VERTICES: &[BasicVertex] = &[
    BasicVertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // 0
    BasicVertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // 1
    BasicVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // 2
    BasicVertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // 3
    BasicVertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // 4
];

pub const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
