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
pub struct Pool {
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
    pub productTypeCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuerApplication: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryApproachProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryPortfolioProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poolType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterpartyType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performingProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performingIndicatorAfterCrm: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localCcfUndrawnFacility: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub localCcfSignature: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rwAverage: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accountingClassification: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioningStageProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawnPartAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadOnBalanceFinancingAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadOffBalanceFinancingAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadOffBalanceCommitmentsBySignatureAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interestAccrualsAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidInterestsAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeAccrualsAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvisionDrawnPartAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalCommitmentAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annualOnBalanceProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityOnBalanceProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualMaturityProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdWithoutCrm: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdWithCrm: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdWithGuaranteesAndCollateral: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdWithCrmModelProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdWithCrmModelVersionProvided: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub averageAnnualSAlesAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvisionUndrawnPartAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annualOffBalanceProvisionPartAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityOffBalanceProvisionAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualValueRisk: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidFeesAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditRiskEquivalentAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtmAmountOfGivenOrLentSecurities: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtmAmountOfReceivedOrBorrowedSecurities: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashAmountReceivedAsCollateralOfSecuritiesGiven: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashAmountGivenAsCollateralOfSecuritiesReceived: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifatAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interestAccrualsCashGivenAsCollateralAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greatGuarantor: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TrancheResult: Option<Vec<TrancheResult>>,
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
