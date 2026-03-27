#[cfg(test)]
mod tests {
    use crate::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

    #[test]
    fn test_contract_version_initialization() {
        let env = Env::default();
        let admin = Address::generate(&env);

        // Initialize the contract
        EventContract::initialize(
            env.clone(),
            admin.clone(),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Check initial version (should default to 1)
        let version = EventContract::contract_version(env.clone());
        assert_eq!(version, 1);
    }

    #[test]
    fn test_migration_v1_to_v2() {
        let env = Env::default();
        let admin = Address::generate(&env);

        // Initialize the contract
        EventContract::initialize(
            env.clone(),
            admin.clone(),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Verify current version
        let current_version = EventContract::contract_version(env.clone());
        assert_eq!(current_version, 1);

        // Perform migration
        let new_version = EventContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(new_version, 2);

        // Verify version after migration
        let updated_version = EventContract::contract_version(env.clone());
        assert_eq!(updated_version, 2);
    }

    #[test]
    fn test_migration_unauthorized() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let unauthorized = Address::generate(&env);

        // Initialize the contract
        EventContract::initialize(
            env.clone(),
            admin.clone(),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Attempt migration by unauthorized user
        let result = EventContract::migrate(env.clone(), unauthorized);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EventError::Unauthorized);
    }

    #[test]
    fn test_storage_compatibility_after_migration() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let ticket_contract = Address::generate(&env);
        let payments_contract = Address::generate(&env);

        // Initialize the contract
        EventContract::initialize(
            env.clone(),
            admin.clone(),
            ticket_contract.clone(),
            payments_contract.clone(),
        )
        .unwrap();

        // Perform migration
        EventContract::migrate(env.clone(), admin.clone()).unwrap();

        // Verify that storage data is still accessible
        let admin_after = storage::get_admin(&env).unwrap();
        assert_eq!(admin_after, admin);

        let ticket_contract_after = storage::get_ticket_contract(&env).unwrap();
        assert_eq!(ticket_contract_after, ticket_contract);

        let payments_contract_after = storage::get_payments_contract(&env).unwrap();
        assert_eq!(payments_contract_after, payments_contract);
    }

    #[test]
    fn test_multiple_migrations() {
        let env = Env::default();
        let admin = Address::generate(&env);

        // Initialize the contract
        EventContract::initialize(
            env.clone(),
            admin.clone(),
            Address::generate(&env),
            Address::generate(&env),
        )
        .unwrap();

        // Perform first migration (v1 -> v2)
        let v2 = EventContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(v2, 2);

        // Perform second migration (v2 -> v3)
        let v3 = EventContract::migrate(env.clone(), admin.clone()).unwrap();
        assert_eq!(v3, 3);

        // Verify final version
        let final_version = EventContract::contract_version(env);
        assert_eq!(final_version, 3);
    }

    #[test]
    fn test_version_compatibility_check() {
        let env = Env::default();

        // Verify version is compatible
        let result = storage::verify_version(&env);
        assert!(result.is_ok());
    }
}
