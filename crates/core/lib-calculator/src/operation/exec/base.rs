use cubecl::Runtime;

pub trait PipelineExec<R: Runtime> {
    type Operator;
    type OperatorResult;

    fn exec(self) -> Self::OperatorResult;
    fn out(self) -> Self::OperatorResult;
}
