#[macro_export]
macro_rules! impl_view_with_inner {
    ($inner:ident) => {
        #[inline]
        fn render(
            &self,
            printer: &mut $crate::printer::Printer,
        ) {
            self.$inner.render(printer);
        }

        #[inline]
        fn desired_size(&self) -> $crate::vec2::Vec2 {
            self.$inner.desired_size()
        }

        #[inline]
        fn layout(
            &mut self,
            size: $crate::vec2::Vec2,
        ) {
            self.$inner.layout(size);
        }
    };
}

#[macro_export]
macro_rules! impl_scrollable_view_with_inner {
    ($inner:ident) => {
        #[inline]
        fn scroll_vertical_render(
            &self,
            pos: u16,
            printer: &mut Printer,
        ) {
            self.$inner.scroll_vertical_render(pos, printer);
        }
        #[inline]
        fn scroll_horizontal_render(
            &self,
            pos: u16,
            printer: &mut Printer,
        ) {
            self.$inner.scroll_horizontal_render(pos, printer);
        }
        #[inline]
        fn scroll_both_render(
            &self,
            pos: Vec2,
            printer: &mut Printer,
        ) {
            self.$inner.scroll_both_render(pos, printer);
        }
    };
}

#[macro_export]
macro_rules! impl_scrollable_view_for_inner {
    ($ident:ident<$inner:ident $(,$gen:ident)*>) => {
        impl<S, E, $inner $(,$gen)*> ScrollableView<S, E> for $ident<$inner $(,$gen)*> where $inner: ScrollableView<S, E> {
            #[inline]
            fn scroll_vertical_render(
                &self,
                pos: u16,
                printer: &mut Printer,
            ) {
                self.inner.scroll_vertical_render(pos, printer);
            }
            #[inline]
            fn scroll_horizontal_render(
                &self,
                pos: u16,
                printer: &mut Printer,
            ) {
                self.inner.scroll_horizontal_render(pos, printer);
            }
            #[inline]
            fn scroll_both_render(
                &self,
                pos: Vec2,
                printer: &mut Printer,
            ) {
                self.inner.scroll_both_render(pos, printer);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_for_inner {
    ($ident:ident<$inner:ident $(,$gen:ident)*>) => {
        impl<$inner $(,$gen)*> std::ops::Deref for $ident<$inner $(,$gen)*> {
            type Target = $inner;

            #[inline]
            fn deref(&self) -> &$inner {
                &self.inner
            }
        }
        impl<$inner $(,$gen)*> std::ops::DerefMut for $ident<$inner $(,$gen)*> {
            #[inline]
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.inner
            }
        }
    };
}
