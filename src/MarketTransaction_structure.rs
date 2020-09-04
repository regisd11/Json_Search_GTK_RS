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
pub struct MarketTransaction {
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
    pub legalEntityCode: Option<IntOrFloatOrStringOrBool>,
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
    pub nominalAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markToMarketBorrowingAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markToMarketLendingAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashBorrowingAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashLendingAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearerActivity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountingClassification: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub securitizationInvestor: Option<SecuritizationInvestor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defaultFlag: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgRelationshipCcp: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicationEmitterCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvision: Option<SpecificProvision>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecuritizationInvestor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bankRoleCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resecuritizationIndicator: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seniority: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketOperationId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketOperationType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub securitizationProgramId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originationRatings: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ratings: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpecificProvision {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annualProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balanceSheet: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisionBase: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountingStandard: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposureId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuerApplication: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentExposureId: Option<IntOrFloatOrStringOrBool>,
}
