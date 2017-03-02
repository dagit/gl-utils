use gl;
use gl::types::*;
use shader;
use image;
use texture;
use uniform;

pub fn gl_error_str<'a>(err: GLenum) -> &'a str
{
    match err {
        gl::NO_ERROR                      => "GL_NO_ERROR",
        gl::INVALID_ENUM                  => "GL_INVALID_ENUM",
        gl::INVALID_VALUE                 => "GL_INVALID_VALUE",
        gl::INVALID_OPERATION             => "GL_INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY                 => "GL_OUT_OF_MEMORY",
        gl::STACK_UNDERFLOW               => "GL_STACK_UNDERFLOW",
        gl::STACK_OVERFLOW                => "GL_STACK_OVERFLOW",
        _                                 => "Invalid error code"
    }
}

#[macro_export]
macro_rules! check_error {
    () => (
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            return Err(err);
        }
    )
}

#[macro_export]
macro_rules! panic_error {
    () => (
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            use util::*;
            panic!(gl_error_str(err));
        }
    )
}

#[macro_export]
macro_rules! check_error_as_string {
    () => (
        let err = gl::GetError();
        if err != gl::NO_ERROR {
            use util::*;
            return Err(format!("{}", gl_error_str(err)));
        }
    )
}

#[macro_export]
macro_rules! sub { ($_x:expr => $y:expr) => ($y) }

#[macro_export]
macro_rules! implement_vertex_attrib {
    ($struct_name:path, $($field_name:ident),+) => (
        impl $crate::util::VertexAttribFields for $struct_name {
            fn names() -> &'static [&'static str]
            {
                const FIELD_NAMES: &'static [&'static str] =
                    &[$(stringify!($field_name)),+];
                FIELD_NAMES
            }

            fn sizes() -> Vec<::gl::types::GLint>
            {
                vec![$(
                    {
                        fn get_size<T: $crate::util::HasVertexAttribSize>(_: &T) ->
                            ::gl::types::GLint
                        {
                            <T as $crate::util::HasVertexAttribSize>::
                            get_vertex_attrib_size()
                        }

                        let s: &$struct_name = unsafe { ::std::mem::uninitialized() };
                        let s_f = &s.$field_name;
                        ::std::mem::forget(s);
                        get_size(s_f)
                    }
                ),+]
            }
            fn types() -> Vec<::gl::types::GLenum>
            {
                vec![$(
                    {
                        fn get_gl_type<T: $crate::util::HasGLType>(_: &T) ->
                            ::gl::types::GLenum
                        {
                            <T as $crate::util::HasGLType>::get_gl_type()
                        }

                        let s: &$struct_name = unsafe { ::std::mem::uninitialized() };
                        let s_f = &s.$field_name;
                        ::std::mem::forget(s);
                        get_gl_type(s_f)
                    }
                ),+]
            }
            fn normalizeds() -> &'static [::gl::types::GLboolean]
            {
                const N: &'static[::gl::types::GLboolean] =
                    &[$( sub!(stringify!($field_name) => ::gl::FALSE) ),+];
                N
            }
            fn stride() -> ::gl::types::GLsizei
            {
                ::std::mem::size_of::<$struct_name>() as ::gl::types::GLsizei
            }
            fn pointers() -> Vec<* const ::gl::types::GLvoid>
            {
                vec![$(
                    {
                        offset_of!($struct_name => $field_name).get_byte_offset()
                            as * const _
                    }
                ),+]
            }
        }
    )
}

#[macro_export]
macro_rules! uniforms {
    { $( $field_name: ident : $value:expr, )+ } => {
        {
            use $crate::uniform::ToUniform;
            vec![$(
                $crate::uniform::Uniform
                {
                    name:  &stringify!($field_name),
                    value: $value.to_uniform(),
                }
            ),+]
        }
    }
}

pub fn size_of_gl_type(gl_type: GLenum) -> GLsizei
{
    use std::mem::size_of;
    (match gl_type {
        gl::BYTE           => size_of::<GLbyte>(),
        gl::UNSIGNED_BYTE  => size_of::<GLubyte>(),
        gl::SHORT          => size_of::<GLshort>(),
        gl::UNSIGNED_SHORT => size_of::<GLushort>(),
        gl::INT            => size_of::<GLint>(),
        gl::UNSIGNED_INT   => size_of::<GLuint>(),
        gl::FLOAT          => size_of::<GLfloat>(),
        gl::DOUBLE         => size_of::<GLdouble>(),
        _ => unimplemented!()
    }) as GLsizei
}

pub trait HasVertexAttribSize {
    fn get_vertex_attrib_size() -> GLint;
}

impl HasVertexAttribSize for GLfloat {
    fn get_vertex_attrib_size() -> GLint
    {
        1
    }
}

#[macro_export]
macro_rules! has_vertex_attrib_size {
    ($size:expr) => (
        impl<T> HasVertexAttribSize for [T;$size] {
            fn get_vertex_attrib_size() -> GLint
            {
                $size
            }
        }
    )
}

has_vertex_attrib_size!(1);
has_vertex_attrib_size!(2);
has_vertex_attrib_size!(3);
has_vertex_attrib_size!(4);

pub trait HasGLType {
    fn get_gl_type() -> GLenum;
}

impl HasGLType for GLfloat {
    fn get_gl_type() -> GLenum
    {
        gl::FLOAT
    }
}

/// Note: This also defines [[T;$size];$size]
#[macro_export]
macro_rules! has_gl_type {
    ($size:expr) => (
        impl<T: HasGLType> HasGLType for [T;$size] {
            fn get_gl_type() -> GLenum
            {
                <T as HasGLType>::get_gl_type()
            }
        }
    )
}

has_gl_type!(2);
has_gl_type!(3);
has_gl_type!(4);

pub trait VertexAttribFields {
    // the name to look up in the shader program
    fn names() -> &'static[&'static str];
    // index parameter to glVertexAttribPointer
    fn indexes(program: &shader::Program) -> Result<Vec<GLint>,GLenum>
    {
        Self::names()
            .iter()
            .map(|n|
                 program.get_attribute_location(n))
            .collect::<_>()
    }
    // size parameter to glVertexAttribPointer
    fn sizes() -> Vec<GLint>;
    // type parameter to glVertexAttribPointer
    fn types() -> Vec<GLenum>;
    // type parameter to glVertexAttribPointer
    fn normalizeds() -> &'static [GLboolean];
    // stride parameter to glVertexAttribPointer
    fn stride() -> GLsizei;
    // pointer parameter to glVertexAttribPointer
    fn pointers() -> Vec<* const GLvoid>;
}

pub fn setup_vertex_attrib<VAF: VertexAttribFields>
    (shader: &shader::Program) -> Result<(),GLenum>
{
    let locs        = VAF::indexes(shader)?
        .iter()
        .filter_map(|l| match *l {
            -1 => None,
            l  => Some(l as GLuint),
        })
        .collect::<Vec<GLuint>>();
    let sizes       = VAF::sizes();
    let types       = VAF::types();
    let normalizeds = VAF::normalizeds();
    let stride      = VAF::stride();
    let pointers    = VAF::pointers();

    for (i,index) in locs.iter().enumerate() {
        let size       = sizes[i];
        let typ        = types[i];
        let normalized = normalizeds[i];
        let pointer    = pointers[i];
        unsafe {
            gl::VertexAttribPointer(*index, size, typ, normalized, stride, pointer);
            check_error!();
            gl::EnableVertexAttribArray(*index);
            check_error!();
        }
    }
    Ok(())
}

pub trait HasUniformCount {
    fn get_uniform_count(&self) -> GLint;
}

#[macro_export]
macro_rules! has_uniform_count {
    ($ty:ty, $size:expr) => (
        impl HasUniformCount for $ty {
            fn get_uniform_count(&self) -> GLint
            {
                $size
            }
        }
    )
}

impl<T> HasUniformCount for Vec<T> {
    fn get_uniform_count(&self) -> GLint
    {
        self.len() as GLint
    }
}

has_uniform_count!([GLfloat;1],1);
has_uniform_count!([GLfloat;2],1);
has_uniform_count!([GLfloat;3],1);
has_uniform_count!([GLfloat;4],1);
has_uniform_count!([[GLfloat;2];2],1);
has_uniform_count!([[GLfloat;3];3],1);
has_uniform_count!([[GLfloat;4];4],1);
//has_uniform_count!(texture::TextureRef,1);
impl<'a> HasUniformCount for texture::TextureRef<'a> {
    fn get_uniform_count(&self) -> GLint
    {
        1
    }
}

impl<'a> HasUniformCount for uniform::UniformEnum<'a> {
    fn get_uniform_count(&self) -> GLint
    {
        use uniform::UniformEnum::*;
        match self {
            &Vec1(..) => 1,
            &Vec2(..) => 1,
            &Vec3(..) => 1,
            &Vec4(..) => 1,
            &Mat2(..) => 1,
            &Mat3(..) => 1,
            &Mat4(..) => 1,
            &Tex(..)  => 1,
        }
    }
}

pub fn setup_uniforms(uniforms: &[uniform::Uniform], shader: &shader::Program) ->
    Result<(), GLenum>
{
    let mut unit = 0;
    for u in uniforms {
        let index = shader.get_uniform_location(u.name)?;
        let count = u.value.get_uniform_count();
        unsafe {
            use uniform::UniformEnum::*;
            match u.value {
                Vec1(ref v) => gl::Uniform1fv(index, count, v.as_ptr() as * const _),
                Vec2(ref v) => gl::Uniform2fv(index, count, v.as_ptr() as * const _),
                Vec3(ref v) => gl::Uniform3fv(index, count, v.as_ptr() as * const _),
                Vec4(ref v) => gl::Uniform4fv(index, count, v.as_ptr() as * const _),
                Mat2(ref v) => gl::UniformMatrix2fv(index, count,
                                                    gl::FALSE, v.as_ptr() as * const _),
                Mat3(ref v) => gl::UniformMatrix3fv(index, count,
                                                    gl::FALSE, v.as_ptr() as * const _),
                Mat4(ref v) => gl::UniformMatrix4fv(index, count,
                                                    gl::FALSE, v.as_ptr() as * const _),
                Tex(ref t)  => {
                    gl::ActiveTexture(gl::TEXTURE0 + unit);
                    check_error!();
                    t.bind()?;
                    gl::Uniform1i(index, unit as GLint);
                    check_error!();
                    unit += 1;
                }
            }
            check_error!();
        }
    }
    Ok(())
}

pub fn vertex_buffer_data<VAF: VertexAttribFields>
    (data: &[VAF], target: GLenum, usage: GLenum) -> Result<(),GLenum>
{
    unsafe {
        gl::BufferData(target,
                       (data.len() * VAF::stride() as usize) as GLsizeiptr,
                       data.as_ptr() as *const _,
                       usage);
        check_error!();
    }
    Ok(())
}

pub fn build_cubemap<'a>(cubemap_images: &Vec<image::RgbaImage>,) ->
    Result<texture::TextureRef<'a>,GLenum>
{
    // We require them to all be the same size and square
    let size = {
        let mut sz = 0;
        for im in cubemap_images.iter() {
            if sz != 0 && im.dimensions().0 != sz { return Err(gl::INVALID_OPERATION) }
            sz = im.dimensions().0;
            if sz != im.dimensions().1 { return Err(gl::INVALID_OPERATION) }
        }
        if sz == 0 { return Err(gl::INVALID_OPERATION) }
        sz
    };
    let cubemap_texture = {
        texture::TextureRef::unbind_target(gl::TEXTURE_CUBE_MAP)?;
        let result = texture::TextureRef::new(gl::TEXTURE_CUBE_MAP)?;
        result.bind()?;
        for (i, ref im) in cubemap_images.into_iter().enumerate() {
            // TODO: add a bind_point concept? basically a texture
            // that has no drop? So far this is the only place in the
            // code that would use it.  So it seems poorly motivated
            // so far.
            unsafe {
                let bind_point = gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32;
                gl::TexImage2D(bind_point, 0, gl::RGBA as i32,
                               size as i32, size as i32,
                               0, gl::RGBA, gl::UNSIGNED_BYTE,
                               im.as_ptr() as *const _);
                check_error!();
            }
        }
        use texture::*;
        use texture::TexParameter::*;
        result.parameter(MagFilter(TexMagFilter::Linear))?;
        result.parameter(MinFilter(TexMinFilter::Linear))?;
        result.parameter(WrapS(TexWrap::ClampToEdge))?;
        result.parameter(WrapT(TexWrap::ClampToEdge))?;
        result.parameter(WrapR(TexWrap::ClampToEdge))?;
        result.unbind()?;
        result
    };
    Ok(cubemap_texture)
}

pub fn enable(cap: GLenum) -> Result<(), GLenum>
{
    unsafe {
        gl::Enable(cap);
        check_error!();
    }
    Ok(())
}

pub fn disable(cap: GLenum) -> Result<(), GLenum>
{
    unsafe {
        gl::Disable(cap);
        check_error!();
    }
    Ok(())
}

pub fn blend_func(sfactor: GLenum, dfactor: GLenum) -> Result<(), GLenum>
{
    unsafe {
        gl::BlendFunc(sfactor, dfactor);
        check_error!();
    }
    Ok(())
}

pub fn depth_func(func: GLenum) -> Result<(), GLenum>
{
    unsafe {
        gl::DepthFunc(func);
        check_error!();
    }
    Ok(())
}

pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) -> Result<(), GLenum>
{
    unsafe {
        gl::DrawArrays(mode, first, count);
        check_error!();
    }
    Ok(())
}

pub fn clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) ->
    Result<(), GLenum>
{
    unsafe {
        gl::ClearColor(red, green, blue, alpha);
        check_error!();
    }
    Ok(())
}

pub fn clear(mask: GLbitfield) -> Result<(), GLenum>
{
    unsafe {
        gl::Clear(mask);
        check_error!();
    }
    Ok(())
}
