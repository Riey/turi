#[macro_export]
macro_rules! impl_deref_for_generic_inner {
    ($ty:ident => $ident:ident) => {
        impl<T> std::ops::Deref for $ty<T> {
            type Target = T;

            fn deref(&self) -> &T {
                &self.$ident
            }
        }

        impl<T> std::ops::DerefMut for $ty<T> {
            fn deref_mut(&mut self) -> &mut T {
                &mut self.$ident
            }
        }
    };
}
