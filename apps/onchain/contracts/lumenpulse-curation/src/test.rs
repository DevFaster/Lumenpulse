use crate::storage::LEDGER_THRESHOLD;
use crate::{
    CommunityCurationContract, CommunityCurationContractClient, ProjectMetadata, ProjectStatus,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::StellarAssetClient,
    Address, Env, String,
};

fn setup<'a>(env: &Env) -> (CommunityCurationContractClient<'a>, Address, Address) {
    let admin = Address::generate(env);
    let proposer = Address::generate(env);
    let contributor_registry = Address::generate(env);

    let deposit_token_admin = Address::generate(env);
    let token_contract = env.register_stellar_asset_contract_v2(deposit_token_admin);
    let token_address = token_contract.address();
    StellarAssetClient::new(env, &token_address).mint(&proposer, &1_000_000_000);

    let contract_id = env.register(CommunityCurationContract, ());
    let client = CommunityCurationContractClient::new(env, &contract_id);
    client.initialize(&admin, &token_address, &contributor_registry);

    (client, admin, proposer)
}

fn sample_metadata(env: &Env) -> ProjectMetadata {
    ProjectMetadata {
        // NOTE: `emit_project_proposed` (events.rs) currently copies `name`
        // into a fixed 32-byte stack buffer via `String::copy_into_slice`
        // (panics unless the string is *exactly* 32 bytes) and then builds a
        // `Symbol` from it (which only accepts `[A-Za-z0-9_]` — no spaces or
        // punctuation). Both are apparent pre-existing bugs unrelated to
        // TTL/storage: any realistic human-readable project name would
        // panic `propose_project` outright. Using an exactly-32-byte,
        // Symbol-safe literal here works around it so this TTL test can
        // exercise `propose_project`; flagged separately, not fixed, since
        // it's out of scope for issue #1226.
        name: String::from_str(env, "TTL_Boundary_Test_Project_Name12"),
        description: String::from_str(env, "A project proposed for TTL testing."),
        url: String::from_str(env, "https://example.com"),
        funding_address: Address::generate(env),
    }
}

/// Advances the ledger sequence past `LEDGER_THRESHOLD` repeatedly and
/// exercises reads and writes at each step, so a persistent or instance
/// entry that was never re-bumped would fail to be found and this test
/// would fail.
#[test]
fn test_ttl_extended_after_read_write() {
    let env = Env::default();
    env.mock_all_auths();
    let (client, _admin, proposer) = setup(&env);

    let metadata = sample_metadata(&env);
    let project_id = client.propose_project(&proposer, &metadata);

    // Advance past LEDGER_THRESHOLD once: a read should re-bump both the
    // instance (Admin/DepositToken/ContributorRegistry/NextProjectId) and
    // the per-project persistent `Proposal` entry.
    env.ledger().set_sequence_number(LEDGER_THRESHOLD + 1);
    let proposal = client.get_proposal_state(&project_id).unwrap();
    assert_eq!(proposal.status, ProjectStatus::Pending);
    assert!(!client.is_verified(&project_id));

    // Advance again — this only survives if the prior read actually
    // extended the TTL rather than leaving it to expire.
    env.ledger().set_sequence_number(2 * LEDGER_THRESHOLD + 2);
    let proposal = client.get_proposal_state(&project_id).unwrap();
    assert_eq!(proposal.status, ProjectStatus::Pending);

    // An admin write after a long gap must also succeed, and must itself
    // keep protecting the entries it touches (Admin instance key,
    // Proposal persistent key).
    env.ledger().set_sequence_number(3 * LEDGER_THRESHOLD + 3);
    client.admin_reject(&project_id);

    env.ledger().set_sequence_number(4 * LEDGER_THRESHOLD + 4);
    let proposal = client.get_proposal_state(&project_id).unwrap();
    assert_eq!(proposal.status, ProjectStatus::Rejected);

    // A second proposal after the long gap must also succeed — proves the
    // instance-tier NextProjectId counter and DepositToken/Admin survived.
    let second_id = client.propose_project(&proposer, &sample_metadata(&env));
    assert_eq!(second_id, project_id + 1);
}
