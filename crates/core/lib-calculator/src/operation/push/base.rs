use cubecl::Runtime;

pub trait PipelinePush<R: Runtime> {
    type Operator;
    type OperatorResult;
    fn push(op: Self::OperatorResult) -> Self::Operator;
    fn output(op: Self::OperatorResult) -> Self::Operator;
}