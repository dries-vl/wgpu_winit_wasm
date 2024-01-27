use crate::state::texture;

#[derive(Debug)]
pub enum ResourceError {
    TextureError(texture::TextureError),
}

pub async fn load_string(file_name: &str) -> String {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let txt = std::fs::read_to_string(path).unwrap();

    txt
}

pub async fn load_binary(file_name: &str) -> Vec<u8> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let data = std::fs::read(path).unwrap();

    data
}

pub async fn load_texture(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<texture::Texture, texture::TextureError> {
    let data = load_binary(file_name).await;
    texture::Texture::from_bytes(device, queue, &data, file_name)
}
