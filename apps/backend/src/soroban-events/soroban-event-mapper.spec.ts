import { mapSorobanEvent } from './soroban-event-mapper';
import { CanonicalEventType, EventCategory } from '../common/event-catalog';

describe('mapSorobanEvent', () => {
  it('maps ProjectCreatedEvent to project.created', () => {
    const result = mapSorobanEvent('ProjectCreatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.PROJECT_CREATED,
      category: EventCategory.PROJECT,
    });
  });

  it('maps DepositEvent to contribution.deposited', () => {
    const result = mapSorobanEvent('DepositEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.CONTRIBUTION_DEPOSITED,
      category: EventCategory.CONTRIBUTION,
    });
  });

  it('maps BurnEvent to token.burned', () => {
    const result = mapSorobanEvent('BurnEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_BURNED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps RoundCreatedEvent to pool.round_created', () => {
    const result = mapSorobanEvent('RoundCreatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.POOL_ROUND_CREATED,
      category: EventCategory.POOL,
    });
  });

  it('maps UpgradedEvent to admin.upgraded', () => {
    const result = mapSorobanEvent('UpgradedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.ADMIN_UPGRADED,
      category: EventCategory.ADMIN,
    });
  });

  it('maps VestingCreatedEvent to token.vesting_created', () => {
    const result = mapSorobanEvent('VestingCreatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_VESTING_CREATED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps PriceUpdatedEvent to price.updated', () => {
    const result = mapSorobanEvent('PriceUpdatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.PRICE_UPDATED,
      category: EventCategory.PRICE,
    });
  });

  // Regression test for issue #1231: `lumenpulse-curation` publishes its
  // events via `#[contractevent]` like every other contract in this
  // workspace, so the real on-chain topic is the PascalCase struct name
  // (`ProjectProposedEvent`), never a short symbol like `proposed`. A
  // prior version of this mapper keyed a separate `CURATION_EVENT_MAP` by
  // the short symbol, which never matched anything the contract actually
  // emitted — these events were silently unmapped from day one.
  it('maps ProjectProposedEvent (lumenpulse-curation) to project.proposed', () => {
    const result = mapSorobanEvent('ProjectProposedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.PROJECT_PROPOSED,
      category: EventCategory.PROJECT,
    });
  });

  it('maps ProjectVerifiedEvent to project.verified', () => {
    const result = mapSorobanEvent('ProjectVerifiedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.PROJECT_VERIFIED,
      category: EventCategory.PROJECT,
    });
  });

  it('no longer maps the short curation symbol names (issue #1231)', () => {
    expect(mapSorobanEvent('proposed')).toBeNull();
    expect(mapSorobanEvent('voted')).toBeNull();
    expect(mapSorobanEvent('verified')).toBeNull();
    expect(mapSorobanEvent('rejected')).toBeNull();
    expect(mapSorobanEvent('expired')).toBeNull();
  });

  it('maps ProposalExpiredEvent (treasury + contributor_registry + lumenpulse-curation) to governance.proposal_expired', () => {
    const result = mapSorobanEvent('ProposalExpiredEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.GOVERNANCE_PROPOSAL_EXPIRED,
      category: EventCategory.GOVERNANCE,
    });
  });

  it('maps MilestoneDecisionEvent to milestone.decision_recorded', () => {
    const result = mapSorobanEvent('MilestoneDecisionEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.MILESTONE_DECISION_RECORDED,
      category: EventCategory.MILESTONE,
    });
  });

  it('maps EmrgMigrProposedEvent to contribution.emergency_migration_proposed', () => {
    const result = mapSorobanEvent('EmrgMigrProposedEvent');
    expect(result).toEqual({
      canonicalType:
        CanonicalEventType.CONTRIBUTION_EMERGENCY_MIGRATION_PROPOSED,
      category: EventCategory.CONTRIBUTION,
    });
  });

  it('maps PoolInitializedEvent (liquidity_pool / stable_swap_pool) to liquidity.pool_initialized', () => {
    const result = mapSorobanEvent('PoolInitializedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.LIQUIDITY_POOL_INITIALIZED,
      category: EventCategory.LIQUIDITY,
    });
  });

  it('maps SwapEvent to liquidity.swapped', () => {
    const result = mapSorobanEvent('SwapEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.LIQUIDITY_SWAPPED,
      category: EventCategory.LIQUIDITY,
    });
  });

  it('maps OperationQueuedEvent (upgradable-contract timelock) to admin.operation_queued', () => {
    const result = mapSorobanEvent('OperationQueuedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.ADMIN_OPERATION_QUEUED,
      category: EventCategory.ADMIN,
    });
  });

  it('maps ScopePauseChangedEvent to admin.scope_pause_changed', () => {
    const result = mapSorobanEvent('ScopePauseChangedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.ADMIN_SCOPE_PAUSE_CHANGED,
      category: EventCategory.ADMIN,
    });
  });

  it('maps AttestationSuspendedEvent to reputation.attestation_suspended', () => {
    const result = mapSorobanEvent('AttestationSuspendedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.REPUTATION_ATTESTATION_SUSPENDED,
      category: EventCategory.REPUTATION,
    });
  });

  it('maps PriceInvalidatedEvent to price.invalidated', () => {
    const result = mapSorobanEvent('PriceInvalidatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.PRICE_INVALIDATED,
      category: EventCategory.PRICE,
    });
  });

  it('maps BadgeGrantedEvent to reputation.badge_granted', () => {
    const result = mapSorobanEvent('BadgeGrantedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.REPUTATION_BADGE_GRANTED,
      category: EventCategory.REPUTATION,
    });
  });

  it('returns null for null input', () => {
    expect(mapSorobanEvent(null)).toBeNull();
  });

  it('returns null for unknown event type', () => {
    expect(mapSorobanEvent('UnknownEvent')).toBeNull();
  });

  it('maps ModuleRegisteredEvent to module.registered', () => {
    const result = mapSorobanEvent('ModuleRegisteredEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.MODULE_REGISTERED,
      category: EventCategory.MODULE,
    });
  });

  it('maps StreamCreatedEvent to token.stream_created', () => {
    const result = mapSorobanEvent('StreamCreatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_STREAM_CREATED,
      category: EventCategory.TOKEN,
    });
  });

  // ── New coverage added under issue #1231 (event emission audit) ──────────

  it('maps MintEvent (lumen_token) to token.minted', () => {
    const result = mapSorobanEvent('MintEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_MINTED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps TransferEvent (lumen_token transfer + transfer_from) to token.transferred', () => {
    const result = mapSorobanEvent('TransferEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_TRANSFERRED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps AllowanceChangedEvent to token.allowance_changed', () => {
    const result = mapSorobanEvent('AllowanceChangedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_ALLOWANCE_CHANGED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps AccountStateChangedEvent (lumen_token freeze/unfreeze) to token.account_state_changed', () => {
    const result = mapSorobanEvent('AccountStateChangedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_ACCOUNT_STATE_CHANGED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps StreamCancelledEvent (treasury) to token.stream_cancelled', () => {
    const result = mapSorobanEvent('StreamCancelledEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.TOKEN_STREAM_CANCELLED,
      category: EventCategory.TOKEN,
    });
  });

  it('maps EmergencyStopEvent (treasury) to admin.emergency_stop', () => {
    const result = mapSorobanEvent('EmergencyStopEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.ADMIN_EMERGENCY_STOP,
      category: EventCategory.ADMIN,
    });
  });

  it('maps ConfigUpdatedEvent (project_registry) to admin.config_updated', () => {
    const result = mapSorobanEvent('ConfigUpdatedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.ADMIN_CONFIG_UPDATED,
      category: EventCategory.ADMIN,
    });
  });

  it('maps ContributorDeregisteredEvent to reputation.contributor_deregistered', () => {
    const result = mapSorobanEvent('ContributorDeregisteredEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.REPUTATION_CONTRIBUTOR_DEREGISTERED,
      category: EventCategory.REPUTATION,
    });
  });

  it('maps AdminRotationProposedEvent (upgradable-contract) to admin.rotation_proposed', () => {
    const result = mapSorobanEvent('AdminRotationProposedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.ADMIN_ROTATION_PROPOSED,
      category: EventCategory.ADMIN,
    });
  });

  it('maps YieldInvestedEvent (crowdfund_vault) to contribution.yield_invested', () => {
    const result = mapSorobanEvent('YieldInvestedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.CONTRIBUTION_YIELD_INVESTED,
      category: EventCategory.CONTRIBUTION,
    });
  });

  it('maps SubscriberChangedEvent (crowdfund_vault) to contribution.subscriber_changed', () => {
    const result = mapSorobanEvent('SubscriberChangedEvent');
    expect(result).toEqual({
      canonicalType: CanonicalEventType.CONTRIBUTION_SUBSCRIBER_CHANGED,
      category: EventCategory.CONTRIBUTION,
    });
  });
});
