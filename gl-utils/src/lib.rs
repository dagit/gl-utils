extern crate gl;
extern crate image;

#[macro_use]
pub mod util;
pub mod shader;
pub mod texture;
pub mod uniform;
pub mod vbo;
pub mod vao;
pub mod ebo;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
