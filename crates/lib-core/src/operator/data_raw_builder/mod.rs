pub mod data_from_csv;

use ndarray::{Array1, Array2};
use std::collections::HashMap;

use crate::error::Result;

#[derive(Debug)]
pub struct DataRaw {
    pub header: Option<Array1<String>>,
    pub data: Result<Array2<f64>>,
    pub map: Option<HashMap<usize, HashMap<String, usize>>>,
}

pub trait DataRawBuilder {
    fn build(self) -> Result<DataRaw>;
}
