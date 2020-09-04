#![allow(non_snake_case)]

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum IntOrFloatOrStringOrBool {
    Integer(u64),
    String(String),
    Float(f64),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmountGeneric {
    currencyCode: Option<IntOrFloatOrStringOrBool>,
    value: Option<IntOrFloatOrStringOrBool>,
    convertedCurrencyCode: Option<IntOrFloatOrStringOrBool>,
    convertedValue: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecuritizationProgram {
    pub id: String,
    pub granularity: Option<IntOrFloatOrStringOrBool>,
    pub banRoleCode: Option<IntOrFloatOrStringOrBool>,
    pub securitizationOriginators: Option<Vec<SecuritizationOriginator>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecuritizationOriginator {
    pub id: Option<IntOrFloatOrStringOrBool>,
    pub securitizationProgramId: Option<IntOrFloatOrStringOrBool>,
    pub lossPosition: Option<IntOrFloatOrStringOrBool>,
    pub seniority: Option<IntOrFloatOrStringOrBool>,
    pub tradeDate: Option<IntOrFloatOrStringOrBool>,
    pub maturityDateProvided: Option<IntOrFloatOrStringOrBool>,
    pub crms: Option<Vec<String>>,
    pub idVersion: Option<IntOrFloatOrStringOrBool>,
}
