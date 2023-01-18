

use std::mem::{size_of, size_of_val};
use beryllium::*;
use ogl33::*;

use crate::gl_methods::*;

#[derive(Default)]
pub struct Game{}
impl Game {
    pub fn setup_gl(&self, window: &GlWindow) {
        unsafe {
            load_gl_with(|f_name| window.get_proc_address(f_name));
            glClearColor(0.0, 0.0, 0.0, 0.0);
            let shaderProgram = create_shader_program();

            //=========== Set Vertex Object
            {
                let mut vertex_array_object: GLuint = 0;
                glGenVertexArrays(1, &mut vertex_array_object);
                assert_ne!(vertex_array_object, 0);
                glBindVertexArray(vertex_array_object);

                //===========

                let mut vertex_buffer_object: GLuint = 0;
                glGenBuffers(1, &mut vertex_buffer_object);
                assert_ne!(vertex_buffer_object, 0);
                glBindBuffer(GL_ARRAY_BUFFER, vertex_buffer_object);


                //===========

                glBufferData(
                    GL_ARRAY_BUFFER,
                    size_of_val(&VERTICES) as isize,
                    VERTICES.as_ptr().cast(),
                    GL_STATIC_DRAW,
                );
            }
            //===========

            glVertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                GL_FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            glEnableVertexAttribArray(0);



            window.set_swap_interval(SwapInterval::Vsync);
        }
    }


    pub fn setup(&self) {

    }

    pub fn update_game(&self, delta_time: f64) {
        unsafe {
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
    }
}

type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] =
  [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];