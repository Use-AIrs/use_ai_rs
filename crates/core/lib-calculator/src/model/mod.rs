use crate::operation::Operator;
use cubecl::Runtime;

mod error;
mod gbdt;

pub trait Operation<R: Runtime>: Sized {
    type Ctx;
    type Op: Operator<R>;
    type Output;

    fn exec(ctx: Self::Ctx, operator: Self::Op) -> Self::Output;
}
