use cubecl::Runtime;

pub trait PipelinePush<R: Runtime> {
    type Operator;
    type OperatorResult;
    fn push(self) -> Self::OperatorResult;
    fn out(self) -> Self::OperatorResult;
}