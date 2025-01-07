use lib_core::operator::data_raw_builder::{data_from_csv::csv_data, DataRawBuilder};

fn main() {
    let csv = csv_data();
    let csv_raw = csv.unwrap();
    println!("{:?}", csv_raw);
    println!();

    let data_raw_result = csv_raw.build();

    match data_raw_result {
        Ok(data_raw) => {
            println!();
            println!("Looks good");
            println!();

            match data_raw.data {
                Ok(arr2) => {
                    println!("arr2 = {:?}", arr2);
                    println!();
                }
                Err(e) => {
                    println!("Num error: {:?}", e);
                    println!();
                }
            }
            println!("Header: {:?}", data_raw.header);
            println!();
            println!("Map: {:?}", data_raw.map);
            println!();
        }
        _ => {
            println!("Num error");
            println!();
        }
    }
}