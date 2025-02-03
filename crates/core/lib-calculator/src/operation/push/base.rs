use cubecl::Runtime;

pub trait PipelinePush<R: Runtime> {
    type Operator;
    type OperatorResult;
    fn out(self) -> Self::OperatorResult;
    fn push(self) -> Self::OperatorResult;
}
