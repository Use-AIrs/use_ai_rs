use crate::model::Operation;

pub struct ExecMean;

impl Operation for ExecMean {
    type Ctx = ();
    type Operator = ();
    type Output = ();

    fn exec(ctx: Self::Ctx, operator: Self::Operator) -> Self::Output {
        todo!()
    }
}