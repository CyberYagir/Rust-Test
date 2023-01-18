use std::string::ToString;
use ogl33::*;

pub mod vert_shader;
pub mod frag_shader;

pub struct ShaderData{
    pub shader_text: String,
    pub shader_type: GLenum
}

pub fn get_shaders_data() -> Vec<ShaderData> {
   let shaders = vec![
        ShaderData {
            shader_text: vert_shader::VERT_SHADER.to_string(),
            shader_type: GL_VERTEX_SHADER,
        },
        ShaderData {
            shader_text: frag_shader::FRAG_SHADER.to_string(),
            shader_type: GL_FRAGMENT_SHADER,
        },
    ];

    return shaders;
}