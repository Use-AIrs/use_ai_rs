use crate::operation::{Context, Operator};
use cubecl::Runtime;

mod error;
mod gbdt;

pub trait Operation<R: Runtime>: Sized {
    type Ctx: Context;
    type Op: Operator<R>;
    type Output;

    fn exec(ctx: Self::Ctx, operator: Self::Op) -> Self::Output;
}