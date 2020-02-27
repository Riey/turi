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
