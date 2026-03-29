use privacy_utils::{mask_address, MaskedAddress, PrivacyLevel};
use soroban_sdk::{contractevent, Address, BytesN, Env, Symbol};

#[contractevent(data_format = "vec", topics = ["payment"])]
pub struct PaymentReceived {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: MaskedAddress,
    pub amount: i128,
    pub token: Address,
    pub paid_at: u64,
}

#[contractevent(data_format = "vec", topics = ["receipt_requested"])]
pub struct PaymentReceiptRequested {
    pub payment_id: u64,
    pub email_hash: Option<BytesN<32>>,
}

pub fn emit_payment_receipt_requested(env: &Env, payment_id: u64, email_hash: Option<BytesN<32>>) {
    PaymentReceiptRequested {
        payment_id,
        email_hash,
    }
    .publish(env);
}

#[contractevent(data_format = "vec", topics = ["refund"])]
pub struct PaymentRefunded {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: MaskedAddress,
    pub amount: i128,
    pub refunded_at: u64,
}

#[contractevent(data_format = "vec", topics = ["ticket_issued"])]
pub struct TicketIssued {
    pub ticket_id: u64,
    pub event_id: Symbol,
    pub owner: MaskedAddress,
    pub payment_id: u64,
}

#[contractevent(data_format = "vec", topics = ["withdrawal"])]
pub struct RevenueWithdrawn {
    pub event_id: Symbol,
    pub organizer: MaskedAddress,
    pub amount: i128,
    pub withdrawn_at: u64,
}

#[contractevent(data_format = "vec", topics = ["escrow_released"])]
pub struct EscrowAutoReleased {
    pub event_id: Symbol,
    pub organizer: Address,
    pub amount: i128,
}

#[allow(clippy::too_many_arguments)]
pub fn emit_payment_received(
    env: &Env,
    payment_id: u64,
    event_id: Symbol,
    payer: Address,
    amount: i128,
    token: Address,
    paid_at: u64,
    level: &PrivacyLevel,
) {
    PaymentReceived {
        payment_id,
        event_id,
        payer: mask_address(env, &payer, level.clone()),
        amount,
        token,
        paid_at,
    }
    .publish(env);
}

pub fn emit_revenue_withdrawn(
    env: &Env,
    event_id: Symbol,
    organizer: Address,
    amount: i128,
    level: &PrivacyLevel,
) {
    RevenueWithdrawn {
        event_id,
        organizer: mask_address(env, &organizer, level.clone()),
        amount,
        withdrawn_at: env.ledger().timestamp(),
    }
    .publish(env);
}

pub fn emit_payment_refunded(
    env: &Env,
    payment_id: u64,
    event_id: Symbol,
    payer: Address,
    amount: i128,
    level: &PrivacyLevel,
) {
    PaymentRefunded {
        payment_id,
        event_id,
        payer: mask_address(env, &payer, level.clone()),
        amount,
        refunded_at: env.ledger().timestamp(),
    }
    .publish(env);
}

pub fn emit_ticket_issued(
    env: &Env,
    ticket_id: u64,
    event_id: Symbol,
    owner: Address,
    payment_id: u64,
    level: &PrivacyLevel,
) {
    TicketIssued {
        ticket_id,
        event_id,
        owner: mask_address(env, &owner, level.clone()),
        payment_id,
    }
    .publish(env);
}

pub fn emit_escrow_auto_released(env: &Env, event_id: Symbol, organizer: Address, amount: i128) {
    EscrowAutoReleased {
        event_id,
        organizer,
        amount,
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
