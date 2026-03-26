use soroban_sdk::{contractevent, Address, Env, Symbol};

#[contractevent(data_format = "vec", topics = ["payment"])]
pub struct PaymentReceived {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: Address,
    pub amount: i128,
}

#[contractevent(data_format = "vec", topics = ["refund"])]
pub struct PaymentRefunded {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: Address,
    pub amount: i128,
}

#[contractevent(data_format = "vec", topics = ["ticket_issued"])]
pub struct TicketIssued {
    pub ticket_id: u64,
    pub event_id: Symbol,
    pub owner: Address,
}

pub fn emit_payment_received(
    env: &Env,
    payment_id: u64,
    event_id: Symbol,
    payer: Address,
    amount: i128,
) {
    PaymentReceived {
        payment_id,
        event_id,
        payer,
        amount,
    }
    .publish(env);
}

#[contractevent(data_format = "vec", topics = ["withdrawal"])]
pub struct RevenueWithdrawn {
    pub event_id: Symbol,
    pub organizer: Address,
    pub amount: i128,
}

pub fn emit_revenue_withdrawn(env: &Env, event_id: Symbol, organizer: Address, amount: i128) {
    RevenueWithdrawn {
        event_id,
        organizer,
        amount,
    }
    .publish(env);
}

pub fn emit_payment_refunded(
    env: &Env,
    payment_id: u64,
    event_id: Symbol,
    payer: Address,
    amount: i128,
) {
    PaymentRefunded {
        payment_id,
        event_id,
        payer,
        amount,
    }
    .publish(env);
}

pub fn emit_ticket_issued(env: &Env, ticket_id: u64, event_id: Symbol, owner: Address) {
    TicketIssued {
        ticket_id,
        event_id,
        owner,
    }
    .publish(env);
}

#[contractevent(data_format = "vec", topics = ["platform_fee"])]
pub struct PlatformFeeCollected {
    pub event_id: Symbol,
    pub fee_amount: i128,
    pub organizer_amount: i128,
}

pub fn emit_platform_fee_collected(
    env: &Env,
    event_id: Symbol,
    fee_amount: i128,
    organizer_amount: i128,
) {
    PlatformFeeCollected {
        event_id,
        fee_amount,
        organizer_amount,
    }
    .publish(env);
}

#[contractevent(data_format = "vec", topics = ["platform_fee_updated"])]
pub struct PlatformFeeUpdated {
    pub old_bps: u32,
    pub new_bps: u32,
}

pub fn emit_platform_fee_updated(env: &Env, old_bps: u32, new_bps: u32) {
    PlatformFeeUpdated { old_bps, new_bps }.publish(env);
}

#[contractevent(data_format = "vec", topics = ["platform_withdrawal"])]
pub struct PlatformRevenueWithdrawn {
    pub event_id: Symbol,
    pub amount: i128,
    pub to: Address,
}

pub fn emit_platform_revenue_withdrawn(env: &Env, event_id: Symbol, amount: i128, to: Address) {
    PlatformRevenueWithdrawn {
        event_id,
        amount,
        to,
    }
    .publish(env);
}
