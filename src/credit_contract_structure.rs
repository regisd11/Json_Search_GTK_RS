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
pub struct CreditContract {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functionalId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structureEntityCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccfKrValidity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comodityFinancedType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countryExportingCommodity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditContractType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditNatureRiskCountry: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectiveMaturityProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fileId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inPool: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityDateProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offShore: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioningStageProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualityBankLegalProtection: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryApproachProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryPortfolioProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolving: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slottingKrValidity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sustainabilityProductionExportCapacity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termLoanDrawingPeriod: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradeDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originationAsOfDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionCategory: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thirdParties: Option<Vec<ThirdParties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facilityProducts: Option<Vec<FacilityProducts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facilityAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accruedFeesNotMaturedAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undrawnAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountedResidualAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvision: Option<SpecificProvision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contractlines: Option<Vec<ContractLines>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TrancheResult: Option<Vec<TrancheResult>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacilityProducts {
    productCode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContractLines {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functionnalId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nature: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structureEntityCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountingClassification: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effectiveMaturityProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityDateProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub productCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioningStageProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryApproachProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryPortfolioProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradeDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originationAsOfDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccfRateProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fundingCurrencyCodeProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgContractionAccount: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgContractionPosition: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionNature: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undrawn: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ead: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thirdParties: Option<Vec<ThirdParties>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seniority: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualValueRisk: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposureAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accruedInterestSign: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accruedInterestAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accruedFeesNotMaturedAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualAmountNetOfImpairment: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rentPendingSettlement: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidRentAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgContractionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidInterestAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidFeesAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairAdjustmentSign: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairAdjustmentAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<Vec<CclSchedules>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TrancheResult: Option<Vec<TrancheResult>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThirdParties {
    #[serde(skip_serializing_if = "Option::is_none")]
    legalEntityCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    principalCounterparty: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CclSchedules {
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signAmount: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduleAmount: Option<AmountGeneric>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrancheResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amountType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grossAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eadAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lgd: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rwRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expectedLossAmount: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rwaAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pd: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performing: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposureEntity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exposureId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regulatoryPortfolioWithoutSecuritization: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rwExposureAfterProvisionRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rwaBeforeCrmAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capitalRequirementAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kFormula: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lgdModel: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lgdModelVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pdCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lgdCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coefficientRateValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s1ProvisionRateCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s2ProvisionRateCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pdMultiScenarioCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lgdMultiScenarioCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiScenarioCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sectorCoefficientRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expertCoefficientValue: Option<IntOrFloatOrStringOrBool>,
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
