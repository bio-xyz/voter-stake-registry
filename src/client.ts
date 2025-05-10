import { Program, Provider } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { VoterStakeRegistry } from './voter_stake_registry';
import VoterStakeRegistryIDL from './voter_stake_registry.json';

export const VSR_ID = new PublicKey(
  '9SJqwCQ5AJkFtC7zxfFsF6Y5dm22XzN3JEhn3N14v23t',
);

export class VsrClient {
  constructor(
    public program: Program<VoterStakeRegistry>,
    public devnet?: boolean,
  ) {}

  static async connect(
    provider: Provider,
    devnet?: boolean,
  ): Promise<VsrClient> {
    // alternatively we could fetch from chain
    // const idl = await Program.fetchIdl(VSR_ID, provider);
    const idl = VoterStakeRegistryIDL;

    return new VsrClient(
      new Program<VoterStakeRegistry>(
        idl as VoterStakeRegistry,
        provider,
      ),
      devnet,
    );
  }
}
