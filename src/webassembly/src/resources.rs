use std::io::{BufReader, Cursor};

use cfg_if::cfg_if;
use wasm_bindgen_futures::JsFuture;
use wgpu::util::DeviceExt;

use crate::{model, texture};

#[derive(Debug)]
pub enum ResourceError {
    TextureError(texture::TextureError),
    LoadError(tobj::LoadError),
}

#[cfg(target_arch = "wasm32")]
// serve files as a local webserver and make http requests to get them
fn format_url(file_name: &str) -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let mut origin = location.origin().unwrap();
    if !origin.ends_with("wgpu_winit/src/webassembly/res") {
        origin = format!("{}/wgpu_winit/src/webassembly/res", origin);
    }
    format!("{}/{}", origin, file_name)
}

pub async fn load_string(file_name: &str) -> String {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let window = web_sys::window().unwrap();
            let fetch_promise = window.fetch_with_str(&url);

            let js_future = JsFuture::from(fetch_promise);
            let result = js_future.await.unwrap();

            use wasm_bindgen::JsCast;
            let response: web_sys::Response = result.dyn_into().unwrap();
            let text_promise = response.text().unwrap();

            let js_future = JsFuture::from(text_promise);
            let txt: String = js_future.await.unwrap().as_string().unwrap();

        } else {
            let path = std::path::Path::new(env!("OUT_DIR"))
                .join("res")
                .join(file_name);
            let txt = std::fs::read_to_string(path)?;
        }
    }

    txt
}

pub async fn load_binary(file_name: &str) -> Vec<u8> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let window = web_sys::window().unwrap();
            let fetch_promise = window.fetch_with_str(&url);

            let js_future = JsFuture::from(fetch_promise);
            let result = js_future.await.unwrap();

            use wasm_bindgen::JsCast;
            let response: web_sys::Response = result.dyn_into().unwrap();
            let array_buffer_promise = response.array_buffer().unwrap();

            let js_future = JsFuture::from(array_buffer_promise);
            let array_buffer: js_sys::ArrayBuffer = js_future.await.unwrap().dyn_into().unwrap();

            let uint8_array = js_sys::Uint8Array::new(&array_buffer);
            let mut data = vec![0; uint8_array.length() as usize];
            uint8_array.copy_to(&mut data);

        } else {
            let path = std::path::Path::new(env!("OUT_DIR"))
                .join("res")
                .join(file_name);
            let data = std::fs::read(path)?;
        }
    }

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

pub async fn load_model(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout) -> Result<model::Model, ResourceError>
{
    let obj_text = load_string(file_name).await;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    let (models, obj_materials) = tobj::load_obj_buf_async(
        &mut obj_reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |p| async move {
            let mat_text = load_string(&p).await;
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )
        .await
        .map_err(|e| ResourceError::LoadError(e))?;

    let mut materials = Vec::new();
    for m in obj_materials.map_err(|e| ResourceError::LoadError(e))? {
        let diffuse_texture = load_texture(&m.diffuse_texture, device, queue)
            .await
            .map_err(|e| ResourceError::TextureError(e)).unwrap();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: None,
        });

        materials.push(model::Material {
            name: m.name,
            diffuse_texture,
            bind_group,
        })
    }

    let meshes = models
        .into_iter()
        .map(|m| {
            let vertices = (0..m.mesh.positions.len() / 3)
                .map(|i| model::ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                    normal: [
                        m.mesh.normals[i * 3],
                        m.mesh.normals[i * 3 + 1],
                        m.mesh.normals[i * 3 + 2],
                    ],
                })
                .collect::<Vec<_>>();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", file_name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", file_name)),
                contents: bytemuck::cast_slice(&m.mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            model::Mesh {
                name: file_name.to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: m.mesh.indices.len() as u32,
                material: m.mesh.material_id.unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    Ok(model::Model { meshes, materials })
}
