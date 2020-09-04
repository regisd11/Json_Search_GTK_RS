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
pub struct PoolCall {
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
    pub eadAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawnPartAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadOnBalanceFinancingAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadOffBalanceFinancingAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eadOffBalanceCommitmentsBySignatureAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interestAccrualsAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidInterestsAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feeAccrualsAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvisionDrawnPartAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalCommitmentAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annualOnBalanceProvisionAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityOnBalanceProvisionAmount: Option<AmountGenericCall>,
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
    pub averageAnnualSAlesAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificProvisionUndrawnPartAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annualOffBalanceProvisionPartAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityOffBalanceProvisionAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualValueRisk: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpaidFeesAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditRiskEquivalentAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtmAmountOfGivenOrLentSecurities: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtmAmountOfReceivedOrBorrowedSecurities: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashAmountReceivedAsCollateralOfSecuritiesGiven: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashAmountGivenAsCollateralOfSecuritiesReceived: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifatAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interestAccrualsCashGivenAsCollateralAmount: Option<AmountGenericCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greatGuarantor: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmountGenericCall {
    currencyCode: String,
    value: Option<IntOrFloatOrStringOrBool>,
}
