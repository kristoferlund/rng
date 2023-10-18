use std::{cell::RefCell, time::Duration};

use candid::Principal;
use ic_cdk::{init, post_upgrade};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

thread_local! {
  static RNG: RefCell<Option<ChaCha20Rng>> = RefCell::new(None);
}

fn generate_nonce() -> Result<[u8; 10], String> {
    let mut buf = [0u8; 10];
    RNG.with(|rng| rng.borrow_mut().as_mut().unwrap().fill_bytes(&mut buf));
    Ok(buf)
}

#[ic_cdk::query]
fn rng() -> String {
    let nonce1 = generate_nonce().unwrap();
    let nonce2 = generate_nonce().unwrap();
    format!(
        "nonce1: {}, nonce2: {}",
        hex::encode(nonce1),
        hex::encode(nonce2)
    )
}

fn init_rng() {
    ic_cdk_timers::set_timer(Duration::ZERO, || {
        ic_cdk::spawn(async {
            ic_cdk::println!("Initializing RNG");
            let (seed,): ([u8; 32],) =
                ic_cdk::call(Principal::management_canister(), "raw_rand", ())
                    .await
                    .unwrap();
            RNG.with(|rng| *rng.borrow_mut() = Some(ChaCha20Rng::from_seed(seed)));
            ic_cdk::println!("RNG initialized");
        })
    });
}

#[init]
fn init() {
    init_rng();
}

#[post_upgrade]
fn upgrade() {
    init_rng();
}
