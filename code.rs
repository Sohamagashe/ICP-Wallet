use ic_cdk::export::Principal;
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

type Balance = u64;

#[derive(Default)]
struct Wallet {
    owner: Principal,
    balances: HashMap<Principal, Balance>,
}

// Initialize the wallet struct safely
static mut WALLET: Option<Wallet> = None;

/// Initializes the canister and sets the owner
#[init]
fn init_wallet() {
    let caller = ic_cdk::caller(); // Get the caller who initializes the canister
    unsafe {
        if WALLET.is_none() {
            WALLET = Some(Wallet {
                owner: caller,
                balances: HashMap::new(),
            });
            ic_cdk::println!("Wallet canister initialized by {}", caller);
        } else {
            ic_cdk::println!("Wallet is already initialized");
        }
    }
}

/// Sends tokens to another Principal
#[update]
async fn send_tokens(to: Principal, amount: u64) -> Result<String, String> {
    let caller = ic_cdk::caller();
    unsafe {
        let wallet = WALLET.as_mut().unwrap();
        if wallet.owner != caller {
            return Err("Unauthorized access".to_string());
        }
        let sender_balance = wallet.balances.entry(caller).or_insert(0);
        if *sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        *sender_balance -= amount;
        *wallet.balances.entry(to).or_insert(0) += amount;
        Ok(format!("Sent {} tokens to {}", amount, to))
    }
}

/// Receives tokens and updates balance
#[update]
async fn receive_tokens(from: Principal, amount: u64) -> Result<String, String> {
    unsafe {
        let wallet = WALLET.as_mut().unwrap();
        *wallet.balances.entry(from).or_insert(0) += amount;
        Ok(format!("Received {} tokens from {}", amount, from))
    }
}

/// Fetches the current balance of the caller
#[query]
fn get_balance() -> Balance {
    let caller = ic_cdk::caller();
    unsafe {
        let wallet = WALLET.as_ref().unwrap();
        *wallet.balances.get(&caller).unwrap_or(&0)
    }
}
