use super::credit_contract_structure::CreditContract;
use rayon::prelude::*;
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn cc_load(ids_searched: &String, file_src: &String) -> Result<Vec<CreditContract>, Error> {
    let data_path = Path::new(&file_src);
    let data = File::open(data_path).unwrap();
    let reader = BufReader::new(data);
    let array: Vec<CreditContract> = serde_json::from_reader(reader)?;
    let mut to_search: Vec<&str> = ids_searched.split(";").collect();
    to_search.sort();
    to_search.dedup();
    let mut out_array = Vec::<CreditContract>::new();

    for expo in to_search.iter() {
        let tmp = array
            .par_iter()
            .find_first(|elem| &elem.id == &expo.to_string());
        if tmp.is_some() {
            let tmp2 = tmp.unwrap().clone();
            out_array.push(tmp2)
        }
    }
    Ok(out_array)
}
