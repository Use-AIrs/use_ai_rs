use cubecl::Runtime;

/// This trait is used to execute a reduction instruction.
pub trait PipelineExec<R: Runtime> {
    type Operator;
    type OperatorResult;

    fn input(op: Self::Operator) -> Self::OperatorResult;
    fn exec(op: Self::Operator) -> Self::OperatorResult;

}