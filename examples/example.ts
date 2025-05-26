import { AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { Connection, Keypair } from '@solana/web3.js';
import { VsrClient } from '../src';

async function main() {
  const options = AnchorProvider.defaultOptions();
  const connection = new Connection('https://api.devnet.solana.com', options);
  const wallet = new Wallet(Keypair.generate());
  const provider = new AnchorProvider(connection, wallet, options);
  const client = await VsrClient.connect(provider, true);
}

main();
