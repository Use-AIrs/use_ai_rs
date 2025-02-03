use cubecl::prelude::*;

/// This trait moves the operator after a reduction in the state it needs to be in for the next reduction.
pub trait PipelinePush<R: Runtime> {
    type Operator;
    type OperatorResult;
    fn push(op: Self::OperatorResult) -> Self::Operator;
    fn output(op: Self::OperatorResult) -> Self::Operator;
}
