#![deny(clippy::all)]

mod miner; // Import your existing miner.rs file

use ethers::types::Address;
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

      // Validate and convert batch_size
    let batch_size = match config.batch_size {
      Some(size) => {
        // Ensure batch size is not zero and within reasonable limits
        if size == 0 {
          return Err(Error::from_reason("Batch size must be greater than zero".to_string()));
        }
        if size > 1_000_000 {
          return Err(Error::from_reason("Batch size too large, must be less than 1,000,000".to_string()));
        }
        Some(size as usize)
      },
      None => None
    };
    
    // Validate and convert thread_count
    let thread_count = match config.thread_count {
      Some(threads) => {
        // Ensure thread count is not zero
        if threads == 0 {
          return Err(Error::from_reason("Thread count must be greater than zero".to_string()));
        }
        
        // Ensure thread count doesn't exceed available CPUs
        let available_cpus = num_cpus::get();
        if threads as usize > available_cpus {
          println!("Warning: Requested {} threads but only {} CPUs available, limiting to available CPUs", 
                  threads, available_cpus);
          Some(available_cpus)
        } else {
          Some(threads as usize)
        }
      },
      None => None
    };

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