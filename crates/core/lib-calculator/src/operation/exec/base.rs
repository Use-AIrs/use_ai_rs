use cubecl::Runtime;

pub trait PipelineExec<R: Runtime> {
    type Operator;
    type OperatorResult;

    fn input(op: Self::Operator) -> Self::OperatorResult;
    fn exec(op: Self::Operator) -> Self::OperatorResult;

}