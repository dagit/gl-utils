use gl;
use gl::types::*;
use util::*;
use std::rc::Rc;

struct ShaderHandle {
    id: GLuint,
}

impl ShaderHandle {
    fn new(target: GLenum) -> Result<Self, GLenum>
    {
        unsafe {
            let mut h = ShaderHandle { id: 0 };
            h.id = gl::CreateShader(target);
            check_error!();
            Ok(h)
        }
    }
}

impl Drop for ShaderHandle {
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteShader(self.id);
            // We should use check_error here, but the type of drop
            // won't allow it. We have to panic instead.
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                panic!("DeleteShader returned: {}", gl_error_str(err));
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct Program {
    handle: Rc<ProgramHandle>,
}

#[derive(Debug)]
struct ProgramHandle {
    id: GLuint,
}

impl ProgramHandle {
    fn new() -> Result<Rc<Self>, GLenum>
    {
        unsafe {
            let p = gl::CreateProgram();
            check_error!();
            Ok(Rc::new(ProgramHandle { id: p }))
        }
    }
}

impl Drop for ProgramHandle {
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteProgram(self.id);
            // We should use check_error here, but the type of drop
            // won't allow it. We have to panic instead.
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                panic!("DeleteProgram returned: {}", gl_error_str(err));
            }
        }
    }
}

pub struct ShaderSrc<'a> {
    pub vertex:   &'a str,
    pub fragment: &'a str,
}

fn check_shader_log(id: GLuint, pname: GLenum) -> Result<(), String>
{
    let mut success = 0;
    unsafe {
        gl::GetShaderiv(id, pname, &mut success);
        check_error_as_string!();
        if success == gl::FALSE as i32 {
            let mut log_size = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut log_size);
            check_error_as_string!();
            let mut log: Vec<u8> = Vec::with_capacity(log_size as usize);
            gl::GetShaderInfoLog(id, log_size, &mut log_size,
                                 log.as_mut_ptr() as *mut GLchar);
            check_error_as_string!();
            log.set_len(log_size as usize);
            return Err(String::from_utf8_unchecked(log))
        }
    }
    Ok(())
}

fn check_program_log(id: GLuint, pname: GLenum) -> Result<(), String>
{
    let mut success = 0;
    unsafe {
        gl::GetProgramiv(id, pname, &mut success);
        check_error_as_string!();
        if success == gl::FALSE as i32 {
            let mut log_size = 0;
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut log_size);
            check_error_as_string!();
            let mut log: Vec<u8> = Vec::with_capacity(log_size as usize);
            gl::GetProgramInfoLog(id, log_size, &mut log_size,
                                  log.as_mut_ptr() as *mut GLchar);
            check_error_as_string!();
            log.set_len(log_size as usize);
            return Err(String::from_utf8_unchecked(log))
        }
    }
    Ok(())
}

impl Program {
    pub fn new(shaders: ShaderSrc) -> Result<Program, String>
    {
        unsafe {
            use std::ffi::CString;
            use std::ptr;
            let vertex     = ShaderHandle::new(gl::VERTEX_SHADER)
                .map_err(|e|format!("{}",e))?;
            let vertex_str = CString::new(shaders.vertex)
                .map_err(|e|format!("{}",e))?;
            gl::ShaderSource(vertex.id, 1, &vertex_str.as_ptr(), ptr::null());
            check_error_as_string!();
            gl::CompileShader(vertex.id);
            check_error_as_string!();
            check_shader_log(vertex.id, gl::COMPILE_STATUS)?;
            let fragment     = ShaderHandle::new(gl::FRAGMENT_SHADER)
                .map_err(|e|format!("{}",e))?;
            let fragment_str = CString::new(shaders.fragment)
                .map_err(|e|format!("{}",e))?;
            gl::ShaderSource(fragment.id, 1, &fragment_str.as_ptr(), ptr::null());
            check_error_as_string!();
            gl::CompileShader(fragment.id);
            check_error_as_string!();
            check_shader_log(fragment.id, gl::COMPILE_STATUS)?;
            let h = ProgramHandle::new().map_err(|e| format!("{}",e))?;
            check_error_as_string!();
            gl::AttachShader(h.id, vertex.id);
            check_error_as_string!();
            gl::AttachShader(h.id, fragment.id);
            check_error_as_string!();
            gl::LinkProgram(h.id);
            check_error_as_string!();
            check_program_log(h.id, gl::LINK_STATUS)?;
            Ok(Program { handle: h })
        }
    }

    pub fn use_program(&self) -> Result<(), GLenum>
    {
        unsafe {
            gl::UseProgram(self.handle.id);
            check_error!();
            Ok(())
        }
    }

    pub fn get_uniform_location<'a>(&self, name: &'a str) -> Result<GLint, GLenum>
    {
        use std::ffi::CString;
        let name_str = CString::new(name)
            .map_err(|_| gl::INVALID_OPERATION)?;
        unsafe {
            let loc = gl::GetUniformLocation(self.handle.id, name_str.as_ptr());
            check_error!();
            Ok(loc)
        }
    }

    pub fn get_attribute_location<'a>(&self, name: &'a str) -> Result<GLint, GLenum>
    {
        use std::ffi::CString;
        let name_str = CString::new(name)
            .map_err(|_| gl::INVALID_OPERATION)?;
        unsafe {
            let loc = gl::GetAttribLocation(self.handle.id, name_str.as_ptr());
            check_error!();
            Ok(loc)
        }
    }
}
