import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Parakeet } from '../target/types/parakeet';

describe('parakeet', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Parakeet as Program<Parakeet>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
