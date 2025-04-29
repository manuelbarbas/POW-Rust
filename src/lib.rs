#![deny(clippy::all)]

mod miner; // Import your existing miner.rs file

use ethers::types::{Address, U256};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::str::FromStr;

#[napi(object)]
pub struct MiningResult {
  pub duration: f64,
  pub gas_price: String, // U256 serialized as string
}

#[napi(object)]
pub struct MiningConfig {
  pub batch_size: Option<u32>,
  pub thread_count: Option<u32>,
}

#[napi]
pub struct GasMiner;

#[napi]
impl GasMiner {
  #[napi(constructor)]
  pub fn new() -> Self {
    GasMiner
  }

  #[napi]
  pub fn get_cpu_count(&self) -> u32 {
    num_cpus::get() as u32
  }

  #[napi]
  pub async fn mine_gas_for_transaction(
    &self,
    nonce: i64,
    gas: i64,
    from: String,
    config: Option<MiningConfig>
  ) -> Result<MiningResult> {
    // Convert i64 to u64 (with validation)
    let nonce_u64 = if nonce >= 0 {
      nonce as u64
    } else {
      return Err(Error::from_reason("Nonce must be a non-negative integer".to_string()));
    };

    let gas_u64 = if gas >= 0 {
      gas as u64
    } else {
      return Err(Error::from_reason("Gas must be a non-negative integer".to_string()));
    };

    // Parse the Ethereum address from hex string
    let address = match Address::from_str(&from) {
      Ok(addr) => addr,
      Err(e) => return Err(Error::from_reason(format!("Invalid address: {}", e))),
    };

    // Convert config
    let config = config.unwrap_or_else(|| MiningConfig {
      batch_size: None,
      thread_count: None
    });

    let batch_size = config.batch_size.map(|s| s as usize);
    let thread_count = config.thread_count.map(|t| t as usize);

    // Call your existing mining function with the converted u64 values
    let result = miner::mine_gas_for_transaction(nonce_u64, gas_u64, address, batch_size, thread_count)
      .await
      .map_err(|e| Error::from_reason(format!("Mining failed: {}", e)))?;

    
    // Convert the result to JavaScript-friendly format
    Ok(MiningResult {
      duration: result.duration,
      gas_price: format!("{}", result.gas_price), 
    })
  }
}