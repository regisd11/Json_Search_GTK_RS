#![allow(non_snake_case)]

use super::SecuritizationProgram_structure::SecuritizationProgram;
use rayon::prelude::*;
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn secpgm_load(
    ids_searched: &String,
    file_src: &String,
) -> Result<Vec<SecuritizationProgram>, Error> {
    let data_path = Path::new(&file_src);
    let data = File::open(data_path).unwrap();
    let reader = BufReader::new(data);
    let array: Vec<SecuritizationProgram> = serde_json::from_reader(reader)?;
    let to_search: Vec<&str> = ids_searched.split(";").collect();
    let mut out_array = Vec::<SecuritizationProgram>::new();

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
