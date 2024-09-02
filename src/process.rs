use crate::decrypt;
use csv::{Reader, StringRecord, Writer};
use serde::{Deserialize, Serialize};
use std::error::Error;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
pub fn process_csv(input: &str, output: &str, keys: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    // We nest this call in its own scope because of lifetimes.
    let headers = reader.headers()?.clone();
    let vec_index = find_vec_index(&headers, keys_to_vec(keys));
    for record in reader.records() {
        let mut data: Vec<_> = Vec::with_capacity(128);
        let iter_record = record?;
        for (ind, r) in iter_record.iter().enumerate() {
            if vec_index.contains(&ind) && !r.is_empty() {
                let r = key_vi_decrypt(r);
                data.push(r);
                continue;
            }
            data.push(r.to_string());
        }
        records.push(data);
    }
    write_csv(&headers, &records, output).expect("this is error");
    Ok(())
}
// 将data  数据写入csv文件
pub fn write_csv(
    header: &StringRecord,
    records: &Vec<Vec<String>>,
    output: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(output)?;
    wtr.write_record(header)?;
    for r in records {
        wtr.write_record(r)?;
    }
    wtr.flush()?;
    Ok(())
}
pub fn keys_to_vec(keys: &str) -> Vec<&str> {
    let vec: Vec<_> = keys.split(',').collect();
    if vec.len() == 1 {
        return vec![keys];
    };
    vec
}
/*

*/
pub fn find_vec_index(vec: &StringRecord, key_vec: Vec<&str>) -> Vec<usize> {
    let mut index_vec: Vec<usize> = Vec::new();
    for (i, key) in vec.iter().enumerate() {
        if key_vec.contains(&key) {
            index_vec.push(i);
        }
    }
    index_vec
}
pub fn key_vi_decrypt(r: &str) -> String {
    let key = [
        168, 138, 224, 55, 189, 151, 237, 194, 153, 242, 114, 191, 103, 176, 13, 33,
    ];
    let iv = [
        44, 226, 76, 85, 129, 205, 196, 230, 36, 224, 130, 148, 138, 221, 2, 168,
    ];
    //let (key, iv) = generate_key_and_iv();
    let r = decrypt(r, &key, &iv).expect("TODO: panic message");
    r
}
