use gl;
use gl::types::*;
use util::*;
use std::rc::Rc;

#[derive(Debug,Clone)]
pub struct VAORef {
    handle: Rc<VAOHandle>,
}

#[derive(Debug)]
struct VAOHandle {
    id: GLuint,
}

impl VAOHandle {
    fn new() -> Result<Self, GLenum>
    {
        unsafe {
            let mut vao = VAOHandle { id: 0 };
            gl::GenVertexArrays(1, &mut vao.id);
            check_error!();
            Ok(vao)
        }
    }
}

impl VAORef {
    pub fn new() -> Result<Self, GLenum>
    {
        let h = VAOHandle::new()?;
        Ok(VAORef { handle: Rc::new(h) })
    }

    pub fn bind(&self) -> Result<(), GLenum>
    {
        unsafe {
            gl::BindVertexArray(self.handle.id);
            check_error!();
            Ok(())
        }
    }

    pub fn unbind() -> Result<(), GLenum>
    {
        unsafe {
            gl::BindVertexArray(0);
            check_error!();
            Ok(())
        }
    }
}
impl Drop for VAOHandle {
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
            // We should use check_error here, but the type of drop
            // won't allow it. We have to panic instead.
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                panic!("VAO DeleteBuffers returned: {}", gl_error_str(err));
            }
        }
    }
}
