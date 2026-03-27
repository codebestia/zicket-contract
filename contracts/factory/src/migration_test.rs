#[cfg(test)]
mod tests {
    use crate::*;
    use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

    #[test]
    fn test_contract_version_initialization() {
        let env = Env::default();
        let admin = Address::generate(&env);

        // Initialize the contract
        FactoryContract::initialize(
            env.clone(),
            admin.clone(),
            BytesN::from_array(&env, &[0u8; 32]),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Check initial version (should default to 1)
        let version = FactoryContract::contract_version(env.clone());
        assert_eq!(version, 1);
    }

    #[test]
    fn test_migration_v1_to_v2() {
        let env = Env::default();
        let admin = Address::generate(&env);

        // Initialize the contract
        FactoryContract::initialize(
            env.clone(),
            admin.clone(),
            BytesN::from_array(&env, &[0u8; 32]),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Verify current version
        let current_version = FactoryContract::contract_version(env.clone());
        assert_eq!(current_version, 1);

        // Perform migration
        let new_version = FactoryContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(new_version, 2);

        // Verify version after migration
        let updated_version = FactoryContract::contract_version(env.clone());
        assert_eq!(updated_version, 2);
    }

    #[test]
    fn test_migration_unauthorized() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let unauthorized = Address::generate(&env);

        // Initialize the contract
        FactoryContract::initialize(
            env.clone(),
            admin.clone(),
            BytesN::from_array(&env, &[0u8; 32]),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Attempt migration by unauthorized user
        let result = FactoryContract::migrate(env.clone(), unauthorized);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), FactoryError::Unauthorized);
    }

    #[test]
    fn test_storage_compatibility_after_migration() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let event_wasm = BytesN::from_array(&env, &[0u8; 32]);
        let ticket_contract = Address::generate(&env);
        let payments_contract = Address::generate(&env);

        // Initialize the contract
        FactoryContract::initialize(
            env.clone(),
            admin.clone(),
            event_wasm.clone(),
            ticket_contract.clone(),
            payments_contract.clone(),
        )
        .unwrap();

        // Perform migration
        FactoryContract::migrate(env.clone(), admin.clone()).unwrap();

        // Verify that storage data is still accessible
        let admin_after = storage::get_admin(&env).unwrap();
        assert_eq!(admin_after, admin);

        let wasm_after = storage::get_event_wasm_hash(&env).unwrap();
        assert_eq!(wasm_after, event_wasm);

        let ticket_contract_after = storage::get_ticket_contract(&env).unwrap();
        assert_eq!(ticket_contract_after, ticket_contract);

        let payments_contract_after = storage::get_payments_contract(&env).unwrap();
        assert_eq!(payments_contract_after, payments_contract);
    }

    #[test]
    fn test_event_deployment_after_migration() {
        let env = Env::default();
        let admin = Address::generate(&env);

        // Initialize the contract
        FactoryContract::initialize(
            env.clone(),
            admin.clone(),
            BytesN::from_array(&env, &[0u8; 32]),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Perform migration
        FactoryContract::migrate(env.clone(), admin.clone()).unwrap();

        // Verify that event data is still accessible after migration
        let all_events = FactoryContract::get_all_events(env.clone());
        assert_eq!(all_events.len(), 0);

        let organizer = Address::generate(&env);
        let organizer_events = FactoryContract::get_organizer_events(env, organizer);
        assert_eq!(organizer_events.len(), 0);
    }
}
