# SKALE Rust Gasless Miner on a Node.js package

This Node.js package provides lightning-fast gas price mining for SKALE Chains transactions using a Rust native addon. It leverages proof-of-work algorithms to find optimized gas prices for your transactions.

## Features

- **High Performance**: Written in Rust for maximum speed
- **Multi-threaded**: Automatically utilizes all available CPU cores
- **Cross-platform**: Works on Windows, macOS, and Linux
- **TypeScript Support**: Full TypeScript declarations included

## Installation

```bash
npm install @manuelbarbas/pow-rust
```

## Usage

### JavaScript

```javascript
const { GasMiner } = require('@manuelbarbas/pow-rust');

async function mineGasPrice() {
  try {
    console.log('Creating GasMiner instance...');
    const miner = new GasMiner();
    
    const num_cpus = miner.getCpuCount();

    console.log(`CPU cores available: ${num_cpus}`);
    

    // Example Ethereum address (replace with a valid one for your tests)
    const fromAddress = '0x742d35Cc6634C0532925a3b844Bc454e4438f44e';
    
    // Parameters
    const nonce = 42;
    const gas = 21000;
    
    // Optional configuration
    const config = {
      batchSize: 4096,  // Optional batch size
      threadCount: num_cpus    // Optional thread count
    };
    
    console.log(`Mining gas for transaction with nonce=${nonce}, gas=${gas}...`);
    console.log(`Using address: ${fromAddress}`);
    console.time('Mining duration');
    
    const result = await miner.mineGasForTransaction(nonce, gas, fromAddress, config);
    
    console.timeEnd('Mining duration');
    console.log('Mining result:', result);
    console.log(`Found gas price: ${result.gasPrice}`);
    console.log(`Mining took ${result.duration.toFixed(2)} seconds`);
    
  } catch (error) {
    console.error('Error in gas mining test:', error);
  }
}

mineGasPrice().catch(console.error);
```

### TypeScript

```typescript
import { GasMiner, MiningConfig, MiningResult } from '@manuelbarbas/pow-rust';

async function mineGasPrice(): Promise<void> {
 try {
    console.log('Creating GasMiner instance...');
    const miner = new GasMiner();
    
    const num_cpus = miner.getCpuCount();

    console.log(`CPU cores available: ${num_cpus}`);
        
    // Example Ethereum address (replace with a valid one for your tests)
    const fromAddress = '0x742d35Cc6634C0532925a3b844Bc454e4438f44e';
    
    // Parameters
    const nonce = 42;
    const gas = 21000;
    
    // Optional configuration
    const config: MiningConfig = {
      batchSize: 4096,     // Optional batch size
      threadCount: num_cpus       // Optional thread count
    };
    
    console.log(`Mining gas for transaction with nonce=${nonce}, gas=${gas}...`);
    console.log(`Using address: ${fromAddress}`);
    console.time('Mining duration');
    
    const result: MiningResult = await miner.mineGasForTransaction(
      nonce, 
      gas, 
      fromAddress, 
      config
    );
    
    console.timeEnd('Mining duration');
    console.log('Mining result:', result);
    console.log(`Found gas price: ${result.gasPrice}`);
    console.log(`Mining took ${result.duration.toFixed(2)} seconds`);
    
  } catch (error) {
    console.error('Error in gas mining test:', error);
  }
}

mineGasPrice().catch(console.error);

```