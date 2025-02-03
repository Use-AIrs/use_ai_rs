mod error;
mod gbdt;

pub trait Operation: Sized {
    type Ctx;
    type Operator;
    type Output;

    fn exec(ctx: Self::Ctx, operator: Self::Operator) -> Self::Output;
}