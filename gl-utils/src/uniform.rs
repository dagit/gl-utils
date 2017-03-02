use texture;

use gl::types::*;

pub struct Uniform<'a> {
    pub name:  &'static str,
    pub value: UniformEnum<'a>,
}

pub enum UniformEnum<'a> {
    Vec1(&'a[GLfloat;1]),
    Vec2(&'a[GLfloat;2]),
    Vec3(&'a[GLfloat;3]),
    Vec4(&'a[GLfloat;4]),
    Mat2(&'a[[GLfloat;2];2]),
    Mat3(&'a[[GLfloat;3];3]),
    Mat4(&'a[[GLfloat;4];4]),
    Tex(&'a texture::TextureRef<'a>),
}

macro_rules! implement_to_uniform {
    ($ty:ty, $field:ident) => {
        impl ToUniform for $ty {
            fn to_uniform(&self) -> UniformEnum
            {
                UniformEnum::$field(self)
            }
        }
    }
}

pub trait ToUniform {
    fn to_uniform(&self) -> UniformEnum;
}

implement_to_uniform!([GLfloat;1],Vec1);
implement_to_uniform!([GLfloat;2],Vec2);
implement_to_uniform!([GLfloat;3],Vec3);
implement_to_uniform!([GLfloat;4],Vec4);
implement_to_uniform!([[GLfloat;2];2],Mat2);
implement_to_uniform!([[GLfloat;3];3],Mat3);
implement_to_uniform!([[GLfloat;4];4],Mat4);
//implement_to_uniform!(texture::TextureRef,Tex);
impl<'a> ToUniform for texture::TextureRef<'a> {
    fn to_uniform(&self) -> UniformEnum
    {
        UniformEnum::Tex(self)
    }
}
