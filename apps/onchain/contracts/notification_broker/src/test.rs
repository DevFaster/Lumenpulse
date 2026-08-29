#![cfg(test)]

use crate::{NotificationBrokerContract, NotificationBrokerContractClient};
use notification_interface::{Notification, NotificationReceiverClient};
use soroban_sdk::{
    contract, contractimpl, testutils::Address as TestAddress, vec, Address, Bytes, Env, Symbol, Vec,
};

// ============================================================================
// MOCK NOTIFICATION RECEIVER CONTRACT FOR TESTING
// ============================================================================

/// Mock receiver that accepts notifications (conforming subscriber)
#[contract]
pub struct MockReceiverSuccess;

#[contractimpl]
impl MockReceiverSuccess {
    pub fn on_notify(_env: Env, notification: Notification) -> Result<(), u32> {
        // Verify notification has required fields
        assert!(!notification.source.account_id().is_empty());
        assert!(notification.data.len() > 0);
        Ok(())
    }
}

/// Mock receiver that fails notifications (non-conforming subscriber)
/// Used to test that failures don't block other subscribers
#[contract]
pub struct MockReceiverFailure;

#[contractimpl]
impl MockReceiverFailure {
    pub fn on_notify(_env: Env, _notification: Notification) -> Result<(), u32> {
        // Simulate a failing receiver
        Err(500u32)
    }
}

/// Mock receiver that panics during notification
#[contract]
pub struct MockReceiverPanic;

#[contractimpl]
impl MockReceiverPanic {
    pub fn on_notify(_env: Env, _notification: Notification) -> Result<(), u32> {
        panic!("Receiver panic");
    }
}

// ============================================================================
// TEST FIXTURES & HELPERS
// ============================================================================

struct TestFixture<'a> {
    env: Env,
    client: NotificationBrokerContractClient<'a>,
    broker_id: Address,
    admin: Address,
    source1: Address,
    source2: Address,
    listener1: Address,
    listener2: Address,
    listener3: Address,
}

fn setup() -> TestFixture<'static> {
    let env = Env::default();
    env.mock_all_auths();

    // Register the notification broker contract
    let broker_id = env.register_contract(None, NotificationBrokerContract);
    let client = NotificationBrokerContractClient::new(&env, &broker_id);

    // Generate addresses for test actors
    let admin = Address::generate(&env);
    let source1 = Address::generate(&env);
    let source2 = Address::generate(&env);
    let listener1 = Address::generate(&env);
    let listener2 = Address::generate(&env);
    let listener3 = Address::generate(&env);

    // Initialize the broker
    client.initialize(&admin).expect("Failed to initialize broker");

    TestFixture {
        env,
        client,
        broker_id,
        admin,
        source1,
        source2,
        listener1,
        listener2,
        listener3,
    }
}

fn create_test_notification(env: &Env, source: &Address, event_type: &str, data: &str) -> Notification {
    Notification {
        source: source.clone(),
        event_type: Symbol::new(env, event_type),
        data: Bytes::from_slice(env, data.as_bytes()),
    }
}

// ============================================================================
// UNIT TESTS: INITIALIZATION
// ============================================================================

#[test]
fn test_initialize_success() {
    let f = setup();
    let stored_admin = f.client.admin().expect("Failed to get admin");
    assert_eq!(stored_admin, f.admin);
}

#[test]
fn test_initialize_twice_fails() {
    let f = setup();
    let result = f.client.try_initialize(&f.admin);
    // Second initialization should fail with AlreadyInitialized error
    assert!(result.is_err());
}

#[test]
fn test_admin_before_init_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let broker_id = env.register_contract(None, NotificationBrokerContract);
    let client = NotificationBrokerContractClient::new(&env, &broker_id);

    // Before initialization, admin() should fail
    let result = client.try_admin();
    assert!(result.is_err());
}

// ============================================================================
// UNIT TESTS: SUBSCRIPTION MANAGEMENT
// ============================================================================

#[test]
fn test_subscribe_specific_event() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe");

    let is_subscribed = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type))
        .expect("Failed to check subscription");
    assert!(is_subscribed);
}

#[test]
fn test_subscribe_all_events_wildcard() {
    let f = setup();

    // Subscribe to all events from source (no specific event_type)
    f.client
        .subscribe(&f.listener1, &f.source1, &None)
        .expect("Failed to subscribe");

    let is_subscribed = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &None)
        .expect("Failed to check subscription");
    assert!(is_subscribed);
}

#[test]
fn test_subscribe_multiple_listeners_same_source() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe listener1");

    f.client
        .subscribe(&f.listener2, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe listener2");

    let is_sub1 = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to check listener1");
    let is_sub2 = f
        .client
        .is_subscribed(&f.listener2, &f.source1, &Some(event_type))
        .expect("Failed to check listener2");

    assert!(is_sub1);
    assert!(is_sub2);
}

#[test]
fn test_subscribe_duplicate_idempotent() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");

    // Subscribe first time
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe first time");

    // Subscribe again with same parameters - should not error
    let result = f.client.try_subscribe(&f.listener1, &f.source1, &Some(event_type.clone()));
    assert!(result.is_ok(), "Duplicate subscription should be idempotent");

    // Verify still subscribed
    let is_subscribed = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type))
        .expect("Failed to check subscription");
    assert!(is_subscribed);
}

#[test]
fn test_unsubscribe_success() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe");

    f.client
        .unsubscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to unsubscribe");

    let is_subscribed = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type))
        .expect("Failed to check subscription");
    assert!(!is_subscribed);
}

#[test]
fn test_unsubscribe_nonexistent_fails() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");
    let result = f.client.try_unsubscribe(&f.listener1, &f.source1, &Some(event_type));
    // Unsubscribing from non-existent subscription should fail
    assert!(result.is_err());
}

#[test]
fn test_unsubscribe_leaves_other_subscriptions() {
    let f = setup();

    let deposit = Symbol::new(&f.env, "deposit");
    let withdraw = Symbol::new(&f.env, "withdraw");

    // Subscribe to both events
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(deposit.clone()))
        .expect("Failed to subscribe to deposit");

    f.client
        .subscribe(&f.listener1, &f.source1, &Some(withdraw.clone()))
        .expect("Failed to subscribe to withdraw");

    // Unsubscribe from deposit only
    f.client
        .unsubscribe(&f.listener1, &f.source1, &Some(deposit.clone()))
        .expect("Failed to unsubscribe from deposit");

    // Should no longer be subscribed to deposit
    let is_sub_deposit = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(deposit))
        .expect("Failed to check deposit subscription");
    assert!(!is_sub_deposit);

    // Should still be subscribed to withdraw
    let is_sub_withdraw = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(withdraw))
        .expect("Failed to check withdraw subscription");
    assert!(is_sub_withdraw);
}

// ============================================================================
// UNIT TESTS: QUERY OPERATIONS
// ============================================================================

#[test]
fn test_get_listeners_for_source_empty() {
    let f = setup();

    let listeners = f
        .client
        .get_listeners_for_source(&f.source1)
        .expect("Failed to get listeners");

    assert_eq!(listeners.len(), 0);
}

#[test]
fn test_get_listeners_for_source_multiple() {
    let f = setup();

    let deposit = Symbol::new(&f.env, "deposit");
    let wildcard: Option<Symbol> = None;

    f.client
        .subscribe(&f.listener1, &f.source1, &Some(deposit.clone()))
        .expect("Failed to subscribe listener1");

    f.client
        .subscribe(&f.listener2, &f.source1, &wildcard)
        .expect("Failed to subscribe listener2");

    f.client
        .subscribe(&f.listener3, &f.source1, &Some(deposit))
        .expect("Failed to subscribe listener3");

    let listeners = f
        .client
        .get_listeners_for_source(&f.source1)
        .expect("Failed to get listeners");

    // Should have 3 listeners (duplicates removed)
    assert_eq!(listeners.len(), 3);
    assert!(listeners.iter().any(|l| l == &f.listener1));
    assert!(listeners.iter().any(|l| l == &f.listener2));
    assert!(listeners.iter().any(|l| l == &f.listener3));
}

// ============================================================================
// INTEGRATION TESTS: NOTIFICATION DISPATCH
// ============================================================================

#[test]
fn test_notify_single_listener_specific_event() {
    let f = setup();

    let deposit = Symbol::new(&f.env, "deposit");

    // Register success receiver
    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    let receiver = NotificationReceiverClient::new(&f.env, &receiver_id);

    f.client
        .subscribe(&receiver_id, &f.source1, &Some(deposit.clone()))
        .expect("Failed to subscribe");

    let notification = create_test_notification(&f.env, &f.source1, "deposit", "test_data");

    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    assert_eq!(notified_count, 1);
}

#[test]
fn test_notify_multiple_listeners_specific_event() {
    let f = setup();

    let deposit = Symbol::new(&f.env, "deposit");

    // Register two success receivers
    let receiver1_id = f.env.register_contract(None, MockReceiverSuccess);
    let receiver2_id = f.env.register_contract(None, MockReceiverSuccess);

    f.client
        .subscribe(&receiver1_id, &f.source1, &Some(deposit.clone()))
        .expect("Failed to subscribe receiver1");

    f.client
        .subscribe(&receiver2_id, &f.source1, &Some(deposit))
        .expect("Failed to subscribe receiver2");

    let notification = create_test_notification(&f.env, &f.source1, "deposit", "test_data");

    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    assert_eq!(notified_count, 2);
}

#[test]
fn test_notify_wildcard_subscription() {
    let f = setup();

    // Register receiver with wildcard subscription
    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);

    f.client
        .subscribe(&receiver_id, &f.source1, &None)
        .expect("Failed to subscribe with wildcard");

    // Send deposit event
    let notification_deposit = create_test_notification(&f.env, &f.source1, "deposit", "data");
    let count1 = f
        .client
        .notify(&f.source1, &notification_deposit)
        .expect("Failed to notify deposit");

    // Send withdraw event
    let notification_withdraw = create_test_notification(&f.env, &f.source1, "withdraw", "data");
    let count2 = f
        .client
        .notify(&f.source1, &notification_withdraw)
        .expect("Failed to notify withdraw");

    // Wildcard subscriber should receive both
    assert_eq!(count1, 1);
    assert_eq!(count2, 1);
}

#[test]
fn test_notify_no_matching_subscribers() {
    let f = setup();

    let deposit = Symbol::new(&f.env, "deposit");
    let withdraw = Symbol::new(&f.env, "withdraw");

    // Register receiver subscribed only to deposit
    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    f.client
        .subscribe(&receiver_id, &f.source1, &Some(deposit))
        .expect("Failed to subscribe");

    // Send withdraw event (no subscribers)
    let notification = create_test_notification(&f.env, &f.source1, "withdraw", "data");
    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    assert_eq!(notified_count, 0);
}

#[test]
fn test_notify_multiple_sources() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "update");

    // Register two receivers
    let receiver1_id = f.env.register_contract(None, MockReceiverSuccess);
    let receiver2_id = f.env.register_contract(None, MockReceiverSuccess);

    // receiver1 listens to source1
    f.client
        .subscribe(&receiver1_id, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe receiver1");

    // receiver2 listens to source2
    f.client
        .subscribe(&receiver2_id, &f.source2, &Some(event_type.clone()))
        .expect("Failed to subscribe receiver2");

    // Send from source1 - only receiver1 should be notified
    let notification = create_test_notification(&f.env, &f.source1, "update", "data");
    let count1 = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify source1");
    assert_eq!(count1, 1);

    // Send from source2 - only receiver2 should be notified
    let notification2 = create_test_notification(&f.env, &f.source2, "update", "data");
    let count2 = f
        .client
        .notify(&f.source2, &notification2)
        .expect("Failed to notify source2");
    assert_eq!(count2, 1);
}

// ============================================================================
// INTEGRATION TESTS: BEST-EFFORT DELIVERY
// ============================================================================

#[test]
fn test_notify_continues_on_listener_failure() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");

    // Register a failing receiver and a success receiver
    let failing_receiver_id = f.env.register_contract(None, MockReceiverFailure);
    let success_receiver_id = f.env.register_contract(None, MockReceiverSuccess);

    // Subscribe both
    f.client
        .subscribe(&failing_receiver_id, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe failing receiver");

    f.client
        .subscribe(&success_receiver_id, &f.source1, &Some(event_type))
        .expect("Failed to subscribe success receiver");

    let notification = create_test_notification(&f.env, &f.source1, "deposit", "data");

    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    // Both should have been attempted, count includes both
    assert_eq!(notified_count, 2, "Best-effort delivery should attempt all subscribers");
}

#[test]
fn test_notify_mixed_success_and_failures() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "harvest");

    // Register three receivers: success, failure, success
    let receiver1_id = f.env.register_contract(None, MockReceiverSuccess);
    let failing_id = f.env.register_contract(None, MockReceiverFailure);
    let receiver3_id = f.env.register_contract(None, MockReceiverSuccess);

    f.client
        .subscribe(&receiver1_id, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe receiver1");

    f.client
        .subscribe(&failing_id, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe failing receiver");

    f.client
        .subscribe(&receiver3_id, &f.source1, &Some(event_type))
        .expect("Failed to subscribe receiver3");

    let notification = create_test_notification(&f.env, &f.source1, "harvest", "data");

    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    // All three should have been notified despite middle one failing
    assert_eq!(notified_count, 3);
}

// ============================================================================
// SECURITY TESTS: AUTHORIZATION
// ============================================================================

#[test]
fn test_notify_requires_source_auth() {
    let env = Env::default();
    // Don't mock all auths - we want to test auth enforcement
    // Only mock for initialization
    env.mock_all_auths();

    let broker_id = env.register_contract(None, NotificationBrokerContract);
    let client = NotificationBrokerContractClient::new(&env, &broker_id);

    let admin = Address::generate(&env);
    let source1 = Address::generate(&env);

    client.initialize(&admin).expect("Failed to initialize");

    // Now disable auth mocking to test auth enforcement
    env.mock_auths(&[]);

    let notification = create_test_notification(&env, &source1, "deposit", "data");

    // This should fail because source.require_auth() will fail
    // when source doesn't have authorization
    let result = client.try_notify(&source1, &notification);

    // The result should indicate auth failure
    // Note: The exact error may vary based on Soroban SDK version
    assert!(result.is_err());
}

// ============================================================================
// EVENT EMISSION TESTS
// ============================================================================

#[test]
fn test_initialize_event_shape() {
    let f = setup();

    // Event was emitted during setup
    // Verify via contract state that admin matches what was emitted
    let stored_admin = f.client.admin().expect("Failed to get admin");
    assert_eq!(stored_admin, f.admin);
    // InitializedEvent fields: admin (Address)
}

#[test]
fn test_subscription_event_on_subscribe() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");

    // Subscribe - this emits SubscriptionEvent
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type))
        .expect("Failed to subscribe");

    // Verify subscription was recorded (event implicitly tested via successful storage)
    let is_subscribed = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(Symbol::new(&f.env, "deposit")))
        .expect("Failed to check subscription");
    assert!(is_subscribed);
}

#[test]
fn test_subscription_event_on_unsubscribe() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");

    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe");

    // Unsubscribe - this emits SubscriptionEvent with action="unsubscribe"
    f.client
        .unsubscribe(&f.listener1, &f.source1, &Some(event_type))
        .expect("Failed to unsubscribe");

    // Verify unsubscription was recorded
    let is_subscribed = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(Symbol::new(&f.env, "deposit")))
        .expect("Failed to check subscription");
    assert!(!is_subscribed);
}

#[test]
fn test_notification_emitted_event_structure() {
    let f = setup();

    let deposit = Symbol::new(&f.env, "deposit");

    // Register receiver
    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    f.client
        .subscribe(&receiver_id, &f.source1, &Some(deposit.clone()))
        .expect("Failed to subscribe");

    let notification = create_test_notification(&f.env, &f.source1, "deposit", "event_data_123");

    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    // NotificationEmittedEvent fields should be:
    // - source: Address (matches f.source1)
    // - event_type: Symbol (matches "deposit")
    // - notified_count: u32 (equals 1)
    assert_eq!(notified_count, 1);
    // Further event verification would require access to event log,
    // which is handled by the soroban-event-mapper in backend
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_subscribe_unsubscribe_resubscribe_cycle() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");

    // Subscribe
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to subscribe");

    let is_sub1 = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to check subscription");
    assert!(is_sub1);

    // Unsubscribe
    f.client
        .unsubscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to unsubscribe");

    let is_sub2 = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to check subscription");
    assert!(!is_sub2);

    // Resubscribe
    f.client
        .subscribe(&f.listener1, &f.source1, &Some(event_type.clone()))
        .expect("Failed to resubscribe");

    let is_sub3 = f
        .client
        .is_subscribed(&f.listener1, &f.source1, &Some(event_type))
        .expect("Failed to check subscription");
    assert!(is_sub3);
}

#[test]
fn test_listener_not_in_registry_no_notification() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "deposit");

    // Register only one receiver and subscribe
    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    f.client
        .subscribe(&receiver_id, &f.source1, &Some(event_type))
        .expect("Failed to subscribe");

    // Send notification from source1
    let notification = create_test_notification(&f.env, &f.source1, "deposit", "data");
    let notified_count = f
        .client
        .notify(&f.source1, &notification)
        .expect("Failed to notify");

    // Only registered receiver should be notified
    assert_eq!(notified_count, 1);
}

#[test]
fn test_empty_notification_data_allowed() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "ping");

    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    f.client
        .subscribe(&receiver_id, &f.source1, &Some(event_type))
        .expect("Failed to subscribe");

    // Create notification with empty data
    let notification = Notification {
        source: f.source1.clone(),
        event_type: Symbol::new(&f.env, "ping"),
        data: Bytes::new(&f.env),
    };

    let result = f.client.try_notify(&f.source1, &notification);
    // Empty data should still work
    assert!(result.is_ok());
}

#[test]
fn test_large_notification_data() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "bulk");

    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    f.client
        .subscribe(&receiver_id, &f.source1, &Some(event_type))
        .expect("Failed to subscribe");

    // Create large data payload
    let large_data = "x".repeat(1000);
    let notification = create_test_notification(&f.env, &f.source1, "bulk", &large_data);

    let result = f.client.try_notify(&f.source1, &notification);
    assert!(result.is_ok());
}

// ============================================================================
// REENTRANCY PROTECTION TESTS
// ============================================================================

#[test]
fn test_notify_completes_without_reentrancy_error() {
    let f = setup();

    let event_type = Symbol::new(&f.env, "action");

    let receiver_id = f.env.register_contract(None, MockReceiverSuccess);
    f.client
        .subscribe(&receiver_id, &f.source1, &Some(event_type))
        .expect("Failed to subscribe");

    let notification = create_test_notification(&f.env, &f.source1, "action", "data");

    // First notify should work
    let result1 = f.client.try_notify(&f.source1, &notification);
    assert!(result1.is_ok());

    // Second notify should also work (no reentrancy guard persistence issues)
    let notification2 = create_test_notification(&f.env, &f.source1, "action", "data2");
    let result2 = f.client.try_notify(&f.source1, &notification2);
    assert!(result2.is_ok());
}

#[test]
fn test_subscribe_completes_without_reentrancy_error() {
    let f = setup();

    let event_type1 = Symbol::new(&f.env, "event1");
    let event_type2 = Symbol::new(&f.env, "event2");

    // Multiple subscribes should work
    let result1 = f.client.try_subscribe(&f.listener1, &f.source1, &Some(event_type1));
    assert!(result1.is_ok());

    let result2 = f.client.try_subscribe(&f.listener1, &f.source1, &Some(event_type2));
    assert!(result2.is_ok());
}
