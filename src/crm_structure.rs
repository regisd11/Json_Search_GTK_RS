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
pub struct Crm {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functionalId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverageAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdownIndicatorMode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub businessApplication: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crmEntity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crrBeanMapping: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irbaEligibility: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub irbfEligibility: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdEligibility: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastValuationAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastValuationDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorClaimsAmount: Option<AmountGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sameRankClaimsRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportingDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commercialCoveredRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantorId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverageBasis: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legalValidity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligibility: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitored: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverageRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hedgingRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assetDiversification: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdingPeriod: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priceVolatilityQualification: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custodianId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkResults: Option<Vec<Option<LinkResults>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eclCrmResult: Option<EclCrmResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityDate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialMaturity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residualMaturity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryProductGroup: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverageWithoutHaircutVolatilityAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverageAfterHaircutAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performing: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryApproach: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub familyModel: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intraGroup: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashCollateral: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateralAssimilatedGuarantee: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrowingBase: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retainedDuplicateCrm: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifyingCounterparty: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterpartyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headquarterCounterpartyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SovereignCounterpartyCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kFormula: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pd: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdScale: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalRatingValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalRatingResultType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalRatingAgency: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalDegradationStatus: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalPdRatingModel: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalPdRatingModelVersionModel: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatoryPortfolioWithoutSecuritization: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdUnsecured: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdUnsecuredModel: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdUnsecuredModelVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalRatingCurrencyType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub riskBucket: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifrsLgdUnsecurredModelCodeRwa: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crmLocationCountryCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdNode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externalRatingValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externalRatingAgency: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usedAfterHaircutAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverageAfterHaircutRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdSpecific: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdSpecificModel: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdSpecificModelVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdSpecificModelCodeRwa: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haircutVolatilityRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateralType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processMode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countryGroup: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateralExpertCategory: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdCollateral: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdCollateralModelVersion: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgdSpecificModelCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ifrsCollateralModelRwa: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateralExpertSubCategory: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specificRule: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haircutWithCoverage: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haircutWithoutCoverage: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haircutFloorRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revaluationFrequency: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub haircut: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditedHoldingPeriod: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub higherThreshold: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expertCollegeSubCategoryCode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rwCrmSpe: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashDepositSgIndicator: Option<IntOrFloatOrStringOrBool>,
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
pub struct LinkResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amountType: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturityMismatchHaircuteRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencyMismatchHaircuteRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkEligibility: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ineligibilityReason: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposureEntity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crmEntity: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposureId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crmId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutionMode: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rwRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crmRwRate: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bunchId: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crmUsedAmountValue: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rwSource: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sovereignRating: Option<IntOrFloatOrStringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idLinkBasedOverlapping: Option<IntOrFloatOrStringOrBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EclCrmResult {
    calculationDowngraded: Option<IntOrFloatOrStringOrBool>,
}
