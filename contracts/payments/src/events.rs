use crate::types::PaymentPrivacy;
use soroban_sdk::{contractevent, Address, Env, Symbol};

#[contractevent(data_format = "vec", topics = ["payment"])]
pub struct PaymentReceived {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: Address,
    pub amount: i128,
    pub token: Address,
    pub paid_at: u64,
    pub privacy_level: PaymentPrivacy,
}

/// Payment event emitted for anonymous payments (no payer exposed).
#[contractevent(data_format = "vec", topics = ["payment_anonymous"])]
pub struct PaymentReceivedAnonymous {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub amount: i128,
    pub token: Address,
    pub paid_at: u64,
}

/// Payment event emitted for private payments (privacy_level included, no payer).
#[contractevent(data_format = "vec", topics = ["payment_private"])]
pub struct PaymentReceivedPrivate {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub amount: i128,
    pub token: Address,
    pub paid_at: u64,
    pub privacy_level: PaymentPrivacy,
}

#[contractevent(data_format = "vec", topics = ["refund"])]
pub struct PaymentRefunded {
    pub payment_id: u64,
    pub event_id: Symbol,
    pub payer: Address,
    pub amount: i128,
    pub refunded_at: u64,
}

#[contractevent(data_format = "vec", topics = ["ticket_issued"])]
pub struct TicketIssued {
    pub ticket_id: u64,
    pub event_id: Symbol,
    pub owner: Address,
    pub payment_id: u64,
}

#[contractevent(data_format = "vec", topics = ["withdrawal"])]
pub struct RevenueWithdrawn {
    pub event_id: Symbol,
    pub organizer: Address,
    pub amount: i128,
    pub withdrawn_at: u64,
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
    privacy_level: PaymentPrivacy,
) {
    match privacy_level {
        PaymentPrivacy::Anonymous => {
            PaymentReceivedAnonymous {
                payment_id,
                event_id,
                amount,
                token,
                paid_at,
            }
            .publish(env);
        }
        PaymentPrivacy::Private => {
            PaymentReceivedPrivate {
                payment_id,
                event_id,
                amount,
                token,
                paid_at,
                privacy_level,
            }
            .publish(env);
        }
        PaymentPrivacy::Standard => {
            PaymentReceived {
                payment_id,
                event_id,
                payer,
                amount,
                token,
                paid_at,
                privacy_level,
            }
            .publish(env);
        }
    }
}

pub fn emit_revenue_withdrawn(env: &Env, event_id: Symbol, organizer: Address, amount: i128) {
    RevenueWithdrawn {
        event_id,
        organizer,
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
) {
    PaymentRefunded {
        payment_id,
        event_id,
        payer,
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
) {
    TicketIssued {
        ticket_id,
        event_id,
        owner,
        payment_id,
    }
    .publish(env);
}
