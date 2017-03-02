use gl;
use gl::types::*;
use util::*;
use std::rc::Rc;

#[derive(Debug,Clone)]
pub struct EBORef {
    handle: Rc<EBOHandle>,
}

#[derive(Debug)]
struct EBOHandle {
    id: GLuint,
}

impl EBOHandle {
    fn new() -> Result<Self, GLenum>
    {
        unsafe {
            let mut ebo = EBOHandle { id: 0 };
            gl::GenBuffers(1, &mut ebo.id);
            check_error!();
            Ok(ebo)
        }
    }
}

impl EBORef {
    pub fn new() -> Result<Self, GLenum>
    {
        let h = EBOHandle::new()?;
        Ok(EBORef { handle: Rc::new(h) })
    }

    pub fn bind(&self, target: GLenum) -> Result<(), GLenum>
    {
        unsafe {
            gl::BindBuffer(target, self.handle.id);
            check_error!();
            Ok(())
        }
    }

    pub fn unbind(target: GLenum) -> Result<(), GLenum>
    {
        unsafe {
            gl::BindBuffer(target, 0);
            check_error!();
            Ok(())
        }
    }
}
impl Drop for EBOHandle {
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
            // We should use check_error here, but the type of drop
            // won't allow it. We have to panic instead.
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                panic!("EBO DeleteBuffers returned: {}", gl_error_str(err));
            }
        }
    }
}
