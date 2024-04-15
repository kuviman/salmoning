use super::*;

#[derive(Deref)]
pub struct Pointer<'a, T>(pub &'a T);
impl<T> PartialEq for Pointer<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 as *const T == other.0 as *const T
    }
}
impl<T> Eq for Pointer<'_, T> {}
impl<T> Hash for Pointer<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 as *const T).hash(state);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DrawMode {
    TriangleFan,
    Triangles,
    TriangleStrip,
}

impl Into<ugli::DrawMode> for DrawMode {
    fn into(self) -> ugli::DrawMode {
        match self {
            DrawMode::TriangleFan => ugli::DrawMode::TriangleFan,
            DrawMode::Triangles => ugli::DrawMode::Triangles,
            DrawMode::TriangleStrip => ugli::DrawMode::TriangleStrip,
        }
    }
}
