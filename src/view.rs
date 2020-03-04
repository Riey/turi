use crate::{
    printer::Printer,
    vec2::Vec2,
};

pub trait View {
    fn render(
        &self,
        printer: &mut Printer,
    );
    fn layout(
        &mut self,
        size: Vec2,
    );
    fn desired_size(&self) -> Vec2;
}

impl View for Box<dyn View> {
    #[inline]
    fn desired_size(&self) -> Vec2 {
        (**self).desired_size()
    }

    #[inline]
    fn layout(
        &mut self,
        size: Vec2,
    ) {
        (**self).layout(size)
    }

    #[inline]
    fn render(
        &self,
        printer: &mut Printer,
    ) {
        (**self).render(printer);
    }
}
