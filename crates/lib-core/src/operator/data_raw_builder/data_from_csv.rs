#![allow(dead_code, unused_imports, path_statements)]

use csv::ReaderBuilder;
use ndarray::{stack, Array1, Array2, Axis};
use rayon::prelude::*;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::operator::data_raw_builder::{DataRaw, DataRawBuilder};

#[derive(Debug)]
pub struct CsvRaw {
    pub header: Option<Array1<String>>,
    pub data: Array2<String>,
}

pub fn csv_data() -> Result<CsvRaw> {
    let file_path = "./ibm_sample_data/DemandPlan_v1.csv";

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .map_err(|e| Error::IoError(e.to_string()))?;

    let csv_headers = rdr
        .headers()
        .map_err(|e| Error::CsvBuildError(e.to_string()))?;
    let header_array = Array1::from(
        csv_headers
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
    );

    let mut records = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| Error::CsvBuildError(e.to_string()))?;
        records.push(record);
    }

    println!("Successfully loaded {} records.", records.len());

    let processed: Vec<Array1<String>> = records
        .par_iter()
        .map(|record| {
            let fields: Vec<String> = record.iter().map(|s| s.to_string()).collect();
            Array1::from(fields)
        })
        .collect();

    let combined: Array2<String> = stack(
        Axis(0),
        &processed.iter().map(|arr| arr.view()).collect::<Vec<_>>(),
    )
    .map_err(|e| Error::CsvBuildError(e.to_string()))?;

    Ok(CsvRaw {
        header: Some(header_array),
        data: combined,
    })
}

impl DataRawBuilder for CsvRaw {
    fn build(self) -> Result<DataRaw> {
        let (rows, cols) = self.data.dim();

        if rows == 0 {
            return Err(Error::DataBuilderError("CsvRaw got no content".to_string()));
        }

        let header = self.header;

        let data_part = self.data.slice(ndarray::s![1.., ..]).to_owned();
        let (data_rows, data_cols) = data_part.dim();
        let mut encoded_columns: Vec<Vec<f64>> = Vec::with_capacity(data_cols);
        let mut category_maps: HashMap<usize, HashMap<String, usize>> = HashMap::new();
        let max_categories = 100;

        for col_idx in 0..data_cols {
            let column_strings = data_part.slice(ndarray::s![.., col_idx]).to_owned();

            let all_floats = column_strings
                .iter()
                .map(|cell| {
                    cell.trim()
                        .parse::<f64>()
                        .map_err(|parse_err| Error::DataBuilderError(parse_err.to_string()))
                })
                .collect::<Result<Vec<f64>>>();

            if let Ok(parsed_nums) = all_floats {
                encoded_columns.push(parsed_nums);
            } else {
                let mut string_to_id = HashMap::new();
                let mut current_id = 0_usize;
                let mut encoded_values = Vec::with_capacity(data_rows);
                let mut skip_column = false;

                for cell in column_strings.iter() {
                    let cell_str = cell.trim();

                    if !string_to_id.contains_key(cell_str) {
                        string_to_id.insert(cell_str.to_string(), current_id);
                        current_id += 1;

                        if current_id > max_categories {
                            skip_column = true;
                            break;
                        }
                    }

                    if !skip_column {
                        let id = *string_to_id.get(cell_str).unwrap();
                        encoded_values.push(id as f64);
                    }
                }

                if skip_column {
                    eprintln!(
                        "Colum {} was skiped (More then {} categories))",
                        col_idx, max_categories
                    );
                    continue;
                }

                encoded_columns.push(encoded_values);
                category_maps.insert(col_idx, string_to_id);
            }
        }
        let final_num_cols = encoded_columns.len();

        if final_num_cols == 0 {
            let empty = Array2::<f64>::default((0, 0));

            return Ok(DataRaw {
                header,
                data: Ok(empty),
                map: None,
            });
        }

        let mut final_data = Array2::<f64>::zeros((data_rows, final_num_cols));
        for (col, col_vec) in encoded_columns.into_iter().enumerate() {
            for (row, val) in col_vec.into_iter().enumerate() {
                final_data[[row, col]] = val;
            }
        }
        let data_raw = DataRaw {
            header,
            data: Ok(final_data),
            map: if category_maps.is_empty() {
                None
            } else {
                Some(category_maps)
            },
        };
        Ok(data_raw)
    }
}