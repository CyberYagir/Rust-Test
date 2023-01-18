
use ogl33::*;

use crate::shaders_list;

pub unsafe fn create_shader_program() -> GLuint {
    let shader_program = glCreateProgram();
    assert_ne!(shader_program, 0);

    let mut shaders: Vec<GLuint> = vec![];

    for shaderData in shaders_list::get_shaders_data() {
        shaders.push(compile_shader(shaderData.shader_type, shaderData.shader_text.as_str(), &shader_program))
    }



    glLinkProgram(shader_program);
    {
        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            get_shader_error(shader_program);
        }
    }

    for shader in shaders {
        glDeleteShader(shader);
    }
    glUseProgram(shader_program);

    return shader_program;
}

pub unsafe fn get_shader_error(value: GLuint) {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let mut log_len = 0_i32;
    glGetProgramInfoLog(
        value,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
    );
    v.set_len(log_len.try_into().unwrap());
    panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
}

pub unsafe fn compile_shader(shader_data: GLenum, shader_text: &str, shader_program: &GLuint) -> GLuint {
    let shader: GLuint = glCreateShader(shader_data);
    assert_ne!(shader, 0);

    glShaderSource(
        shader,
        1,
        &(shader_text.as_bytes().as_ptr().cast()),
        &(shader_text.len().try_into().unwrap()),
    );
    glCompileShader(shader);

    //===========
    let mut success = 0;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);

    if success == 0 {
        get_shader_error(shader);
    }


    glAttachShader(*shader_program, shader);
    return shader;
}
