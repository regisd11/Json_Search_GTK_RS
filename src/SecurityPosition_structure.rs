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
    #[serde(skip_serializing_if = "Option::is_none")]
    currencyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convertedCurrencyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    convertedValue: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityPosition {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functionalId: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradeDate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityDateProvided: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productCode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioningStageProvided: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fundingCurrencyCodeProvided: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdRating: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdRatingSpecific: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balanceSheetProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seniority: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectiveMaturityWithNetting: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectiveMaturityWithoutNetting: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearingHouseLegalEntityCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amountType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuerLegalEntityCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structureEntityCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryPortfolioProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionCategory: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountingStructure: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculationDataPresenceIndicator: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibleFlag: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalModelMethodologyIndicator: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryApproachProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfallSeniority: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlyingType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountingClassification: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub securitizationInvestor: Option<SecuritizationInvestor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultFlag: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvision: Option<SpecificProvision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub securityPositionRatings: Option<SecurityPositionRating>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecuritizationInvestor {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bankRoleCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    granularity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resecuritizationIndicator: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seniority: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketOperationId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marketOperationType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    securitizationProgramId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    originationRatings: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ratings: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpecificProvision {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annualProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maturityProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    balanceSheet: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisionBase: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accountingStandard: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposureId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    issuerApplication: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parentExposureId: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityPositionRating {
    #[serde(skip_serializing_if = "Option::is_none")]
    securitizationInvestorId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agency: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ratingValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<IntOrFloatOrStringOrBool>,
}
