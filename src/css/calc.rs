pub trait Calc {
    type Output;

    fn calc(
        self,
        parent: Self::Output,
    ) -> Self::Output;
}
