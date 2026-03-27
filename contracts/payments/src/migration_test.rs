#[cfg(test)]
mod tests {
    use crate::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

    #[test]
    fn test_contract_version_initialization() {
        let env = Env::default();
        let admin = Address::random(&env);
        let token = Address::random(&env);
        let event_contract = Address::random(&env);

        // Initialize the contract
        PaymentsContract::initialize(env.clone(), admin.clone(), token, event_contract).unwrap();

        // Check initial version (should default to 1)
        let version = PaymentsContract::contract_version(env.clone());
        assert_eq!(version, 1);
    }

    #[test]
    fn test_migration_v1_to_v2() {
        let env = Env::default();
        let admin = Address::random(&env);
        let token = Address::random(&env);
        let event_contract = Address::random(&env);

        // Initialize the contract
        PaymentsContract::initialize(env.clone(), admin.clone(), token, event_contract).unwrap();

        // Verify current version
        let current_version = PaymentsContract::contract_version(env.clone());
        assert_eq!(current_version, 1);

        // Perform migration
        let new_version = PaymentsContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(new_version, 2);

        // Verify version after migration
        let updated_version = PaymentsContract::contract_version(env.clone());
        assert_eq!(updated_version, 2);
    }

    #[test]
    fn test_migration_unauthorized() {
        let env = Env::default();
        let admin = Address::random(&env);
        let unauthorized = Address::random(&env);
        let token = Address::random(&env);
        let event_contract = Address::random(&env);

        // Initialize the contract
        PaymentsContract::initialize(env.clone(), admin.clone(), token, event_contract).unwrap();

        // Attempt migration by unauthorized user
        let result = PaymentsContract::migrate(env.clone(), unauthorized);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PaymentError::Unauthorized);
    }

    #[test]
    fn test_storage_compatibility_after_migration() {
        let env = Env::default();
        let admin = Address::random(&env);
        let token = Address::random(&env);
        let event_contract = Address::random(&env);

        // Initialize the contract
        PaymentsContract::initialize(env.clone(), admin.clone(), token.clone(), event_contract.clone())
            .unwrap();

        // Perform migration
        PaymentsContract::migrate(env.clone(), admin.clone()).unwrap();

        // Verify that storage data is still accessible
        let admin_after = storage::get_admin(&env).unwrap();
        assert_eq!(admin_after, admin);

        let token_after = storage::get_accepted_token(&env).unwrap();
        assert_eq!(token_after, token);

        let event_contract_after = storage::get_event_contract(&env).unwrap();
        assert_eq!(event_contract_after, event_contract);
    }

    #[test]
    fn test_multiple_migrations() {
        let env = Env::default();
        let admin = Address::random(&env);
        let token = Address::random(&env);
        let event_contract = Address::random(&env);

        // Initialize the contract
        PaymentsContract::initialize(env.clone(), admin.clone(), token, event_contract).unwrap();

        // Perform first migration (v1 -> v2)
        let v2 = PaymentsContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(v2, 2);

        // Perform second migration (v2 -> v3)
        let v3 = PaymentsContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(v3, 3);

        // Verify final version
        let final_version = PaymentsContract::contract_version(env);
        assert_eq!(final_version, 3);
    }

    #[test]
    fn test_version_compatibility_check() {
        let env = Env::default();

        // Verify version is compatible
        let result = storage::verify_version(&env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_payment_operations_after_migration() {
        let env = Env::default();
        let admin = Address::random(&env);
        let token = Address::random(&env);
        let event_contract = Address::random(&env);

        // Initialize the contract
        PaymentsContract::initialize(env.clone(), admin.clone(), token, event_contract).unwrap();

        // Perform migration
        PaymentsContract::migrate(env.clone(), admin).unwrap();

        // Verify that payment data is still accessible
        let event_id = Symbol::new(&env, "test_event");
        let payments = PaymentsContract::get_event_payments(env.clone(), event_id.clone());
        assert_eq!(payments.len(), 0);

        let revenue = PaymentsContract::get_event_revenue(env, event_id);
        assert_eq!(revenue, 0);
    }
}
