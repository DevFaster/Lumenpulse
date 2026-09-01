import {
  CanonicalEventType,
  getCategory,
  EventCategory,
} from '../common/event-catalog';

// Keyed by the Soroban event's first topic — for every contract event in
// this workspace defined via `#[contractevent]` without an explicit
// `#[event_name]` override, that topic is auto-generated from the Rust
// struct name (e.g. `ProjectCreatedEvent`). Several structs share a name
// across contracts (e.g. `AdminChangedEvent`, `UpgradedEvent`,
// `PoolInitializedEvent`) — that's intentional: they represent the same
// canonical concept, and the emitting contract address (`contractId`,
// captured separately by the indexer) disambiguates which contract fired
// it. Do not key this map by lowercase/short symbol names unless a
// contract actually publishes one (see issue #1231 — a prior
// `CURATION_EVENT_MAP` here assumed `lumenpulse-curation` used short
// symbol topics like `proposed`/`voted`; it never did, so those events
// were silently unmapped since they were introduced).
const RAW_EVENT_MAP: Record<string, CanonicalEventType> = {
  InitializedEvent: CanonicalEventType.ADMIN_STORAGE_MIGRATED,
  ProjectCreatedEvent: CanonicalEventType.PROJECT_CREATED,
  DepositEvent: CanonicalEventType.CONTRIBUTION_DEPOSITED,
  MilestoneApprovedEvent: CanonicalEventType.MILESTONE_APPROVED,
  MilestoneDecisionEvent: CanonicalEventType.MILESTONE_DECISION_RECORDED,
  WithdrawEvent: CanonicalEventType.CONTRIBUTION_PAID_OUT,
  ContributorRegisteredEvent: CanonicalEventType.REPUTATION_UPDATED,
  ReputationUpdatedEvent: CanonicalEventType.REPUTATION_UPDATED,
  ContractPauseEvent: CanonicalEventType.ADMIN_PAUSED,
  ContractUnpauseEvent: CanonicalEventType.ADMIN_UNPAUSED,
  UpgradedEvent: CanonicalEventType.ADMIN_UPGRADED,
  AdminChangedEvent: CanonicalEventType.ADMIN_CHANGED,
  ProjectCanceledEvent: CanonicalEventType.PROJECT_CANCELED,
  ContributionRefundedEvent: CanonicalEventType.CONTRIBUTION_REFUNDED,
  ContributorPayoutEvent: CanonicalEventType.CONTRIBUTION_PAID_OUT,
  ProjectExpiredEvent: CanonicalEventType.PROJECT_EXPIRED,
  ContributionClawedBackEvent: CanonicalEventType.CONTRIBUTION_CLAWED_BACK,
  ProtocolFeeDeductedEvent: CanonicalEventType.FEE_DEDUCTED,
  MilestoneVoteStartedEvent: CanonicalEventType.MILESTONE_VOTE_STARTED,
  FeeConfigChangedEvent: CanonicalEventType.ADMIN_FEE_CONFIG_CHANGED,
  ConfigUpdatedEvent: CanonicalEventType.ADMIN_CONFIG_UPDATED,
  VoteCastEvent: CanonicalEventType.MILESTONE_VOTE_CAST,
  MilestoneApprovedByVoteEvent: CanonicalEventType.MILESTONE_APPROVED_BY_VOTE,
  MilestoneDisputedEvent: CanonicalEventType.MILESTONE_DISPUTED,
  MilestoneDisputeResolvedEvent: CanonicalEventType.MILESTONE_DISPUTE_RESOLVED,
  StorageMigratedEvent: CanonicalEventType.ADMIN_STORAGE_MIGRATED,
  RoundCreatedEvent: CanonicalEventType.POOL_ROUND_CREATED,
  PoolFundedEvent: CanonicalEventType.POOL_FUNDED,
  RewardPoolFundedEvent: CanonicalEventType.POOL_REWARD_FUNDED,
  ProjectApprovedEvent: CanonicalEventType.POOL_PROJECT_APPROVED,
  ProjectRemovedEvent: CanonicalEventType.POOL_PROJECT_REMOVED,
  ContributionRecordedEvent: CanonicalEventType.POOL_CONTRIBUTION_RECORDED,
  RoundFinalizedEvent: CanonicalEventType.POOL_ROUND_FINALIZED,
  RoundCapUpdatedEvent: CanonicalEventType.POOL_ROUND_CAP_UPDATED,
  MatchDistributedEvent: CanonicalEventType.POOL_MATCH_DISTRIBUTED,
  AllMatchesDistributedEvent: CanonicalEventType.POOL_ALL_MATCHES_DISTRIBUTED,
  PoolInitializedEvent: CanonicalEventType.LIQUIDITY_POOL_INITIALIZED,
  LiquidityAddedEvent: CanonicalEventType.LIQUIDITY_ADDED,
  LiquidityRemovedEvent: CanonicalEventType.LIQUIDITY_REMOVED,
  SwapEvent: CanonicalEventType.LIQUIDITY_SWAPPED,
  BurnEvent: CanonicalEventType.TOKEN_BURNED,
  MintEvent: CanonicalEventType.TOKEN_MINTED,
  TransferEvent: CanonicalEventType.TOKEN_TRANSFERRED,
  AllowanceChangedEvent: CanonicalEventType.TOKEN_ALLOWANCE_CHANGED,
  AccountStateChangedEvent: CanonicalEventType.TOKEN_ACCOUNT_STATE_CHANGED,
  VestingCreatedEvent: CanonicalEventType.TOKEN_VESTING_CREATED,
  TokensClaimedEvent: CanonicalEventType.TOKEN_CLAIMED,
  StreamCreatedEvent: CanonicalEventType.TOKEN_STREAM_CREATED,
  CliffStreamCreatedEvent: CanonicalEventType.TOKEN_STREAM_CREATED,
  BeneficiaryRotatedEvent: CanonicalEventType.TOKEN_STREAM_BENEFICIARY_ROTATED,
  StreamCancelledEvent: CanonicalEventType.TOKEN_STREAM_CANCELLED,
  DelegateApprovedEvent: CanonicalEventType.TOKEN_DELEGATE_APPROVED,
  DelegateRevokedEvent: CanonicalEventType.TOKEN_DELEGATE_REVOKED,
  DelegatedClaimEvent: CanonicalEventType.TOKEN_DELEGATED_CLAIM,
  PriceUpdatedEvent: CanonicalEventType.PRICE_UPDATED,
  OracleUpdatedEvent: CanonicalEventType.PRICE_ORACLE_UPDATED,
  PriceInvalidatedEvent: CanonicalEventType.PRICE_INVALIDATED,
  StalenessWindowUpdatedEvent:
    CanonicalEventType.PRICE_STALENESS_WINDOW_UPDATED,
  ProposalCreatedEvent: CanonicalEventType.GOVERNANCE_PROPOSAL_CREATED,
  SignatureCollectedEvent: CanonicalEventType.GOVERNANCE_SIGNATURE_COLLECTED,
  ProposalExecutedEvent: CanonicalEventType.GOVERNANCE_PROPOSAL_EXECUTED,
  ProposalCancelledEvent: CanonicalEventType.GOVERNANCE_PROPOSAL_CANCELLED,
  ProposalExpiredEvent: CanonicalEventType.GOVERNANCE_PROPOSAL_EXPIRED,
  MultisigConfiguredEvent: CanonicalEventType.GOVERNANCE_MULTISIG_CONFIGURED,
  GaslessRegistrationEvent: CanonicalEventType.REPUTATION_UPDATED,
  BadgeGrantedEvent: CanonicalEventType.REPUTATION_BADGE_GRANTED,
  BadgeRevokedEvent: CanonicalEventType.REPUTATION_BADGE_REVOKED,
  ReputationPenaltyAppliedEvent: CanonicalEventType.REPUTATION_PENALTY_APPLIED,
  ContributorProfileChangedEvent: CanonicalEventType.REPUTATION_PROFILE_CHANGED,
  ContributorDeregisteredEvent:
    CanonicalEventType.REPUTATION_CONTRIBUTOR_DEREGISTERED,
  AttestationSuspendedEvent:
    CanonicalEventType.REPUTATION_ATTESTATION_SUSPENDED,
  AttestationRevokedEvent: CanonicalEventType.REPUTATION_ATTESTATION_REVOKED,
  AttestationRestoredEvent: CanonicalEventType.REPUTATION_ATTESTATION_RESTORED,
  ModuleRegisteredEvent: CanonicalEventType.MODULE_REGISTERED,
  ModuleUpdatedEvent: CanonicalEventType.MODULE_UPDATED,
  ModuleDeactivatedEvent: CanonicalEventType.MODULE_DEACTIVATED,
  ModuleActivatedEvent: CanonicalEventType.MODULE_ACTIVATED,
  ModuleAdminTransferredEvent: CanonicalEventType.ADMIN_TRANSFERRED,
  AdminTransferredEvent: CanonicalEventType.ADMIN_TRANSFERRED,
  AdminRotationProposedEvent: CanonicalEventType.ADMIN_ROTATION_PROPOSED,
  AdminRotationCancelledEvent: CanonicalEventType.ADMIN_ROTATION_CANCELLED,
  ScopePauseChangedEvent: CanonicalEventType.ADMIN_SCOPE_PAUSE_CHANGED,
  FlagSetEvent: CanonicalEventType.ADMIN_FEATURE_FLAG_SET,
  OperationQueuedEvent: CanonicalEventType.ADMIN_OPERATION_QUEUED,
  OperationCancelledEvent: CanonicalEventType.ADMIN_OPERATION_CANCELLED,
  OperationExecutedEvent: CanonicalEventType.ADMIN_OPERATION_EXECUTED,
  EmergencyStopEvent: CanonicalEventType.ADMIN_EMERGENCY_STOP,
  ProjectRegisteredEvent: CanonicalEventType.PROJECT_CREATED,
  ProjectProposedEvent: CanonicalEventType.PROJECT_PROPOSED,
  ProjectVerifiedEvent: CanonicalEventType.PROJECT_VERIFIED,
  ProjectRejectedEvent: CanonicalEventType.PROJECT_REJECTED,
  ProjectArchivedEvent: CanonicalEventType.PROJECT_ARCHIVED,
  ProjectDelistedEvent: CanonicalEventType.PROJECT_DELISTED,
  VerificationOverriddenEvent: CanonicalEventType.ADMIN_VERIFICATION_OVERRIDDEN,
  SubscriberChangedEvent: CanonicalEventType.CONTRIBUTION_SUBSCRIBER_CHANGED,
  TreasuryAllocatedEvent: CanonicalEventType.CONTRIBUTION_ALLOCATED_TO_TREASURY,
  EmrgMigrProposedEvent:
    CanonicalEventType.CONTRIBUTION_EMERGENCY_MIGRATION_PROPOSED,
  EmrgMigrExecutedEvent:
    CanonicalEventType.CONTRIBUTION_EMERGENCY_MIGRATION_EXECUTED,
  EmergencyMigrationVetoedEvent:
    CanonicalEventType.CONTRIBUTION_EMERGENCY_MIGRATION_VETOED,
  YieldProviderSetEvent: CanonicalEventType.CONTRIBUTION_YIELD_PROVIDER_SET,
  YieldInvestedEvent: CanonicalEventType.CONTRIBUTION_YIELD_INVESTED,
  YieldDivestedEvent: CanonicalEventType.CONTRIBUTION_YIELD_DIVESTED,
};

export interface CanonicalMapping {
  canonicalType: CanonicalEventType;
  category: EventCategory;
}

export function mapSorobanEvent(
  eventType: string | null,
): CanonicalMapping | null {
  if (!eventType) return null;

  const mapped = RAW_EVENT_MAP[eventType];
  if (!mapped) return null;

  return { canonicalType: mapped, category: getCategory(mapped) };
}
