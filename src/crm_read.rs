#![allow(non_snake_case)]

use super::credit_contract_structure::CreditContract;
use super::crm_structure::Crm;
use super::MarketTransaction_structure::MarketTransaction;
use super::SecuritizationProgram_structure::SecuritizationProgram;
use super::SecurityPosition_structure::SecurityPosition;
use rayon::prelude::*;
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn crm_load(ids_searched: &String, file_src: &String) -> Result<Vec<Crm>, Error> {
    let data_path = Path::new(&file_src);
    let data = File::open(data_path).unwrap();
    let reader = BufReader::new(data);
    let mut to_search: Vec<&str> = ids_searched.split(";").collect();
    to_search.sort();
    to_search.dedup();
    println!("{:?}", to_search);
    let mut out_array = Vec::<Crm>::new();
    let array: Vec<Crm> = serde_json::from_reader(reader)?;
    for expo in to_search.iter() {
        let tmp = array
            .par_iter()
            .find_first(|elem| &elem.id == &expo.to_string());
        if tmp.is_some() {
            let tmp2 = tmp.unwrap().clone();
            out_array.push(tmp2);
        }
    }
    Ok(out_array)
}

pub fn crm_cc(source: &Vec<CreditContract>) -> Result<String, Error> {
    let mut crm_list = std::string::String::new();
    for elem in source.iter() {
        for crm_id in elem.crms.iter() {
            for id in crm_id.iter() {
                crm_list.push_str(&id);
                crm_list.push_str(";");
            }
        }
    }
    let len = crm_list.len();
    crm_list.truncate(len - 1);
    Ok(crm_list)
}

pub fn crm_mkt(source: &Vec<MarketTransaction>) -> Result<String, Error> {
    let mut crm_list = std::string::String::new();
    for elem in source.iter() {
        for crm_id in elem.crms.iter() {
            for id in crm_id.iter() {
                crm_list.push_str(&id);
                crm_list.push_str(";");
            }
        }
    }
    let len = crm_list.len();
    crm_list.truncate(len - 1);
    Ok(crm_list)
}

pub fn crm_pos_sec(source: &Vec<SecurityPosition>) -> Result<String, Error> {
    let mut crm_list = std::string::String::new();
    for elem in source.iter() {
        for crm_id in elem.crms.iter() {
            for id in crm_id.iter() {
                crm_list.push_str(&id);
                crm_list.push_str(";");
            }
        }
    }
    let len = crm_list.len();
    crm_list.truncate(len - 1);
    Ok(crm_list)
}

pub fn crm_pgm_sec(source: &Vec<SecuritizationProgram>) -> Result<String, Error> {
    let mut crm_list = std::string::String::new();
    for SecuritizationProgram in source.iter() {
        for SecuritizationOriginator in SecuritizationProgram.securitizationOriginators.iter() {
            for SecuritizationOriginator in SecuritizationOriginator.iter() {
                for crms in SecuritizationOriginator.crms.iter() {
                    for crm in crms {
                        crm_list.push_str(crm);
                        crm_list.push_str(";");
                    }
                }
            }
        }
    }
    let len = crm_list.len();
    crm_list.truncate(len - 1);
    Ok(crm_list)
}
