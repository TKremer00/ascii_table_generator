use ascii_table_generator;
use js_sys::Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;

// fn main() {
//     let data = vec![
//         vec!["Vip level".to_string(), "Vip points".to_string()],
//         vec!["0 > 1".to_string(), "100".to_string()],
//         vec!["0 > 2".to_string(), "100\n200\n300".to_string()],
//     ];

//     println!("{}", ascii_table_generator::make_ascii_table(data));
// }

fn convert_table_data_from_array(table_data: Array) -> Vec<Vec<String>> {
    let mut rust_vec = Vec::new();
    for i in 0..table_data.length() {
        let columns: Array = table_data.get(i).into();
        let mut rust_columns = Vec::new();
        for j in 0..columns.length() {
            let element = columns.get(j);
            if let Some(string) = element.as_string() {
                rust_columns.push(string);
            } else {
                rust_columns.push(String::new());
            }
        }
        rust_vec.push(rust_columns);
    }

    rust_vec
}

#[wasm_bindgen(js_name = getTable)]
pub fn get_table(table_data: Array) -> String {
    ascii_table_generator::make_ascii_table(convert_table_data_from_array(table_data)).to_string()
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
