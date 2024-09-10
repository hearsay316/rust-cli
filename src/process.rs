use crate::decrypt;
use csv::{Reader, StringRecord, Writer};
use std::error::Error;
pub fn process_csv(input: &str, output: &str, keys: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    // 由于生命周期，我们将此调用嵌套在其自己的范围内。
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
    println!("生成成功");
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
// 将keys str 转换成vec
pub fn keys_to_vec(keys: &str) -> Vec<&str> {
    let vec: Vec<_> = keys.split(',').collect();
    if vec.len() == 1 {
        return vec![keys];
    };
    vec
}
// 查找vec 中 对应的index
pub fn find_vec_index(vec: &StringRecord, key_vec: Vec<&str>) -> Vec<usize> {
    let mut index_vec: Vec<usize> = Vec::new();
    for (i, key) in vec.iter().enumerate() {
        if key_vec.contains(&key) {
            index_vec.push(i);
        }
    }
    index_vec
}
// 解密
pub fn key_vi_decrypt(r: &str) -> String {
    // 解密需要的key 和 iv 不能是动态生成的
    //let (key, iv) = generate_key_and_iv();
    let r = decrypt(r, None, "utf-8").expect("TODO: panic message");
    r
}
