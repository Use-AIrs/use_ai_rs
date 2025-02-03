mod gbdt_gen;
mod gbdt_trainer;

use crate::model::Operation;
use crate::operation::{Context, Operator};
use crate::MetaData;
use cubecl::prelude::*;
use cubecl::server::Handle;
use cubecl::Runtime;
use lib_proc_macros::{ctx, operator};

#[derive(Debug)]
#[operator]
pub struct GbdtOperator {
    pub target: (MetaData, Handle),
    pub table: (MetaData, Handle),
    pub buffer: (MetaData, Handle),
}

#[ctx]
pub struct GbdtRules {
    pub n_trees: u32,
    pub learning_rate: f32,
    pub max_depth: u32,
    pub sub_sample: Option<f32>,
}

pub fn gbdt<R: Runtime, In: Numeric, Out: Numeric, Op: Operation<R>>(
    op: GbdtOperator,
    ctx: GbdtRules,
) {
    todo!()
}
