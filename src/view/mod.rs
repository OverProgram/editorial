use crate::utils::AABB;

pub trait View {
    fn size(&self) -> AABB;

    fn render(&self);
}
