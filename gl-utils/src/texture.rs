use gl;
use gl::types::*;
use util::*;
use std::rc::Rc;

use std::marker::PhantomData;

#[derive(Debug)]
pub struct TextureHandle<'a> {
    id: GLuint,
    phantom: PhantomData<&'a GLuint>,
}

impl<'a> TextureHandle<'a> {
    fn new() -> Result<Self, GLenum>
    {
        unsafe {
            let mut th = TextureHandle { id: 0, phantom: PhantomData };
            gl::GenTextures(1, &mut th.id);
            check_error!();
            Ok(th)
        }
    }
}

impl<'a> Drop for TextureHandle<'a> {
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
            // We should use check_error here, but the type of drop
            // won't allow it. We have to panic instead.
            let err = gl::GetError();
            if err != gl::NO_ERROR {
                panic!("DeleteTextures returned: {}", gl_error_str(err));
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct TextureRef<'a> {
    handle: Rc<TextureHandle<'a>>,
    target: GLenum,
}

impl<'a> TextureRef<'a> {
    pub fn new(target: GLenum) -> Result<Self, GLenum>
    {
        let h = TextureHandle::new()?;
        Ok(TextureRef { handle: Rc::new(h), target: target })
    }

    pub fn bind(&self) -> Result<(), GLenum>
    {
        unsafe {
            gl::BindTexture(self.target, self.handle.id);
            check_error!();
            Ok(())
        }
    }

    pub fn unbind(&self) -> Result<(), GLenum>
    {
        unsafe {
            gl::BindTexture(self.target, 0);
            check_error!();
            Ok(())
        }
    }

    pub fn unbind_target(target: GLenum) -> Result<(), GLenum>
    {
        unsafe {
            gl::BindTexture(target, 0);
            check_error!();
            Ok(())
        }
    }

    pub fn image_2d<T>(&self, level: GLint, internalformat: GLint,
                       width: GLsizei, height: GLsizei, border: GLint,
                       format: GLenum, type_: GLenum, pixels: *const T)
                       -> Result<(),GLenum>
    {
        unsafe {
            gl::TexImage2D(self.target, level, internalformat, width, height,
                           border, format, type_, pixels as *const _);
            check_error!();
            Ok(())
        }
    }

    pub fn parameter(&self, param: TexParameter) -> Result<(), GLenum>
    {
        let (pname, pvalue) = param.to_gl();
        self.tex_parameter_i(pname, pvalue)
    }

    pub fn tex_parameter_i(&self, pname: GLenum, param: GLint) -> Result<(),GLenum>
    {
        unsafe {
            gl::TexParameteri(self.target, pname, param);
            check_error!();
            Ok(())
        }
    }

    pub fn generate_mipmap(&self) -> Result<(),GLenum>
    {
        unsafe {
            gl::GenerateMipmap(self.target);
            check_error!();
            Ok(())
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub enum TexParameter {
    MinFilter(TexMinFilter),
    MagFilter(TexMagFilter),
    WrapS(TexWrap),
    WrapT(TexWrap),
    WrapR(TexWrap),
}

#[derive(Debug,Copy,Clone)]
pub enum TexMinFilter {
    Nearest              = gl::NEAREST as isize,
    Linear               = gl::LINEAR  as isize,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST as isize,
    LinearMipmapNearest  = gl::LINEAR_MIPMAP_NEAREST  as isize,
    NearestMipmapLinear  = gl::NEAREST_MIPMAP_LINEAR  as isize,
    LinearMipmapLinear   = gl::LINEAR_MIPMAP_LINEAR   as isize,
}

#[derive(Debug,Copy,Clone)]
pub enum TexMagFilter {
    Nearest = gl::NEAREST as isize,
    Linear  = gl::LINEAR  as isize,
}

#[derive(Debug,Copy,Clone)]
pub enum TexWrap {
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    Mirrored    = gl::MIRRORED_REPEAT as isize,
    Repeat      = gl::REPEAT as isize,
}

impl TexParameter {
    fn to_gl(&self) -> (GLenum, GLint)
    {
        match *self {
            TexParameter::MinFilter(ref f) => (gl::TEXTURE_MIN_FILTER, f.to_gl()),
            TexParameter::MagFilter(ref f) => (gl::TEXTURE_MAG_FILTER, f.to_gl()),
            TexParameter::WrapS    (ref w) => (gl::TEXTURE_WRAP_S, w.to_gl()),
            TexParameter::WrapT    (ref w) => (gl::TEXTURE_WRAP_T, w.to_gl()),
            TexParameter::WrapR    (ref w) => (gl::TEXTURE_WRAP_R, w.to_gl()),
        }
    }
}

impl TexMinFilter {
    fn to_gl(&self) -> GLint
    {
        *self as GLint
    }
}

impl TexMagFilter {
    fn to_gl(&self) -> GLint
    {
        *self as GLint
    }
}

impl TexWrap {
    fn to_gl(&self) -> GLint
    {
        *self as GLint
    }
}
