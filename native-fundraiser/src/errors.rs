pub enum FundraiserError {
    TargetNotMet,
    TargetMet,
    ContributionTooBig,
    ContributionTooSmall,
    MaximumContributionsReached,
    FundraiserNotEnded,
    FundraiserEnded,
    InvalidAmount
}