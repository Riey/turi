pub trait Combine {
    fn combine(
        self,
        other: Self,
    ) -> Self;
}
