#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env, Symbol};

mod errors;
mod events;
mod storage;
mod types;

pub use errors::*;
pub use events::*;
pub use storage::*;
pub use types::*;

#[contract]
pub struct PaymentsContract;

#[contractimpl]
impl PaymentsContract {
    /// Initialize the contract with an admin address, accepted token address,
    /// platform fee (in basis points, 0-10000), and platform wallet address.
    /// This can only be called once. If already initialized, this is a no-op.
    pub fn initialize(
        env: Env,
        admin: Address,
        token: Address,
        platform_fee_bps: u32,
        platform_wallet: Address,
    ) -> Result<(), PaymentError> {
        if storage::is_initialized(&env) {
            return Ok(());
        }

        if platform_fee_bps > 10_000 {
            return Err(PaymentError::InvalidFeeBps);
        }

        storage::set_admin(&env, &admin);
        storage::set_accepted_token(&env, &token);
        storage::set_platform_fee_bps(&env, platform_fee_bps);
        storage::set_platform_wallet(&env, &platform_wallet);

        Ok(())
    }

    /// Get a payment record by payment ID.
    pub fn get_payment(env: Env, payment_id: u64) -> Result<PaymentRecord, PaymentError> {
        storage::get_payment(&env, payment_id)
    }

    /// Get the total revenue for an event.
    pub fn get_event_revenue(env: Env, event_id: Symbol) -> i128 {
        storage::get_event_revenue(&env, &event_id)
    }

    /// Get a ticket record by ticket ID.
    pub fn get_ticket(env: Env, ticket_id: u64) -> Result<Ticket, PaymentError> {
        storage::get_ticket(&env, ticket_id)
    }

    /// Get all ticket IDs owned by a wallet.
    pub fn get_owner_tickets(env: Env, owner: Address) -> soroban_sdk::Vec<u64> {
        storage::get_owner_tickets(&env, &owner)
    }

    /// Pay for a ticket. Transfers tokens from payer to contract escrow.
    pub fn pay_for_ticket(
        env: Env,
        payer: Address,
        event_id: Symbol,
        amount: i128,
    ) -> Result<u64, PaymentError> {
        payer.require_auth();

        if amount <= 0 {
            return Err(PaymentError::InvalidAmount);
        }

        let token_address = storage::get_accepted_token(&env)?;
        let contract_address = env.current_contract_address();

        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&payer, &contract_address, &amount);

        let payment_id = storage::get_next_payment_id(&env);
        let paid_at = env.ledger().timestamp();

        let payment = PaymentRecord {
            payment_id,
            event_id: event_id.clone(),
            payer: payer.clone(),
            amount,
            token: token_address.clone(),
            status: PaymentStatus::Held,
            paid_at,
        };

        storage::save_payment(&env, &payment);
        storage::add_event_payment(&env, &event_id, payment_id);
        storage::add_event_revenue(&env, &event_id, amount);

        events::emit_payment_received(&env, payment_id, event_id, payer, amount);

        let ticket_id = storage::get_next_ticket_id(&env);
        let ticket = Ticket {
            ticket_id,
            event_id: payment.event_id.clone(),
            owner: payment.payer.clone(),
            payment_id,
        };
        storage::save_ticket(&env, &ticket);
        storage::add_owner_ticket(&env, &payment.payer, ticket_id);
        events::emit_ticket_issued(&env, ticket_id, payment.event_id, payment.payer);

        Ok(payment_id)
    }

    pub fn refund(env: Env, admin: Address, payment_id: u64) -> Result<(), PaymentError> {
        let stored_admin = storage::get_admin(&env)?;
        if admin != stored_admin {
            return Err(PaymentError::Unauthorized);
        }
        admin.require_auth();

        let mut payment = storage::get_payment(&env, payment_id)?;

        if payment.status == PaymentStatus::Refunded {
            return Err(PaymentError::PaymentAlreadyRefunded);
        }
        if payment.status != PaymentStatus::Held {
            return Err(PaymentError::PaymentAlreadyProcessed);
        }

        let token_client = token::Client::new(&env, &payment.token);
        token_client.transfer(
            &env.current_contract_address(),
            &payment.payer,
            &payment.amount,
        );

        payment.status = PaymentStatus::Refunded;
        storage::update_payment(&env, &payment)?;

        let revenue = storage::get_event_revenue(&env, &payment.event_id);
        storage::set_event_revenue(&env, &payment.event_id, revenue - payment.amount);

        events::emit_payment_refunded(
            &env,
            payment_id,
            payment.event_id,
            payment.payer,
            payment.amount,
        );

        Ok(())
    }

    pub fn withdraw(env: Env, organizer: Address, event_id: Symbol) -> Result<(), PaymentError> {
        organizer.require_auth();

        let revenue = storage::get_event_revenue(&env, &event_id);
        if revenue <= 0 {
            return Err(PaymentError::NoRevenue);
        }

        let token_address = storage::get_accepted_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);
        let payment_ids = storage::get_event_payments(&env, &event_id);

        let mut total: i128 = 0;
        let mut payments_to_release: soroban_sdk::Vec<PaymentRecord> = soroban_sdk::Vec::new(&env);

        for i in 0..payment_ids.len() {
            let pid = payment_ids.get(i).unwrap();
            let payment = storage::get_payment(&env, pid)?;
            if payment.status == PaymentStatus::Held {
                total += payment.amount;
                payments_to_release.push_back(payment);
            }
        }

        if total <= 0 {
            return Err(PaymentError::NoRevenue);
        }

        // Calculate platform fee
        let fee_bps = storage::get_platform_fee_bps(&env) as i128;
        let fee_amount = total * fee_bps / 10_000;
        let organizer_amount = total - fee_amount;

        // Transfer organizer share
        token_client.transfer(
            &env.current_contract_address(),
            &organizer,
            &organizer_amount,
        );

        // Accumulate platform revenue if there is a fee
        if fee_amount > 0 {
            storage::add_platform_revenue(&env, &event_id, fee_amount);
            events::emit_platform_fee_collected(
                &env,
                event_id.clone(),
                fee_amount,
                organizer_amount,
            );
        }

        for i in 0..payments_to_release.len() {
            let mut payment = payments_to_release.get(i).unwrap();
            payment.status = PaymentStatus::Released;
            storage::update_payment(&env, &payment)?;
        }

        storage::set_event_revenue(&env, &event_id, 0);

        events::emit_revenue_withdrawn(&env, event_id, organizer, organizer_amount);

        Ok(())
    }

    pub fn get_event_payments(env: Env, event_id: Symbol) -> soroban_sdk::Vec<u64> {
        storage::get_event_payments(&env, &event_id)
    }

    /// Withdraw revenue for an event. Deducts platform fee and sends the rest
    /// to the specified address. Platform fees are accumulated for later
    /// withdrawal by the admin.
    pub fn withdraw_revenue(env: Env, event_id: Symbol, to: Address) -> Result<(), PaymentError> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        let revenue = storage::get_event_revenue(&env, &event_id);
        if revenue <= 0 {
            return Err(PaymentError::InvalidAmount);
        }

        // Calculate platform fee
        let fee_bps = storage::get_platform_fee_bps(&env) as i128;
        let fee_amount = revenue * fee_bps / 10_000;
        let organizer_amount = revenue - fee_amount;

        let token_address = storage::get_accepted_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);

        // Transfer organizer share
        token_client.transfer(&env.current_contract_address(), &to, &organizer_amount);

        // Accumulate platform revenue if there is a fee
        if fee_amount > 0 {
            storage::add_platform_revenue(&env, &event_id, fee_amount);
            events::emit_platform_fee_collected(
                &env,
                event_id.clone(),
                fee_amount,
                organizer_amount,
            );
        }

        // Update revenue tracking
        storage::reset_event_revenue(&env, &event_id);

        // Record withdrawal history
        let record = WithdrawalRecord {
            amount: organizer_amount,
            timestamp: env.ledger().timestamp(),
            organizer: to.clone(),
        };
        storage::add_withdrawal_record(&env, &event_id, &record);

        Ok(())
    }

    /// Get all withdrawal history for an event.
    pub fn get_withdrawal_history(
        env: Env,
        event_id: Symbol,
    ) -> soroban_sdk::Vec<WithdrawalRecord> {
        storage::get_withdrawal_history(&env, &event_id)
    }

    /// Update the platform fee (admin only). Fee is in basis points (0-10000).
    pub fn set_platform_fee(env: Env, fee_bps: u32, wallet: Address) -> Result<(), PaymentError> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        if fee_bps > 10_000 {
            return Err(PaymentError::InvalidFeeBps);
        }

        let old_bps = storage::get_platform_fee_bps(&env);
        storage::set_platform_fee_bps(&env, fee_bps);
        storage::set_platform_wallet(&env, &wallet);

        events::emit_platform_fee_updated(&env, old_bps, fee_bps);

        Ok(())
    }

    /// Get the current platform fee in basis points.
    pub fn get_platform_fee_bps(env: Env) -> u32 {
        storage::get_platform_fee_bps(&env)
    }

    /// Get the accumulated platform revenue for an event.
    pub fn get_platform_revenue(env: Env, event_id: Symbol) -> i128 {
        storage::get_platform_revenue(&env, &event_id)
    }

    /// Withdraw accumulated platform fees for an event (admin only).
    /// Sends fees to the configured platform wallet.
    pub fn withdraw_platform_revenue(env: Env, event_id: Symbol) -> Result<(), PaymentError> {
        let admin = storage::get_admin(&env)?;
        admin.require_auth();

        let platform_revenue = storage::get_platform_revenue(&env, &event_id);
        if platform_revenue <= 0 {
            return Err(PaymentError::NoPlatformRevenue);
        }

        let platform_wallet = storage::get_platform_wallet(&env)?;
        let token_address = storage::get_accepted_token(&env)?;
        let token_client = token::Client::new(&env, &token_address);

        token_client.transfer(
            &env.current_contract_address(),
            &platform_wallet,
            &platform_revenue,
        );

        storage::reset_platform_revenue(&env, &event_id);

        events::emit_platform_revenue_withdrawn(&env, event_id, platform_revenue, platform_wallet);

        Ok(())
    }
}

#[cfg(test)]
mod test;
