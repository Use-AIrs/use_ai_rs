use crate::error::Result;
use crate::operator::data_raw_builder::DataRaw;

pub struct Model {
    id: usize,
}

pub struct MdataRaw {
    data_raw: DataRaw,
    model: Model,
}

pub struct Mdata {}

pub struct Data {
    id: usize,
    model: Model,
    data: Mdata,
}

//Here we take a value to prepare data for a specific model
pub fn data_builder_interface(mraw: MdataRaw) -> Result<Data> {
    let raw = mraw.data_raw;
    let model = mraw.model;

    match model.id {
        1 => todo!(),
        _ => todo!(),
    }
}