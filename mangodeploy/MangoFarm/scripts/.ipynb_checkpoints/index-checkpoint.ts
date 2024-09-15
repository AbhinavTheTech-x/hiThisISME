import * as anchor from '@coral-xyz/anchor';
import { getSolanaFarmKeyPair, initialize, recoverSol } from '../utils';
import { SolanaFarm } from '../target/types/solana_farm';

async function main() {
  try {
    // set provider
    anchor.setProvider(anchor.AnchorProvider.env());

    // derive program
    const program = anchor.workspace.SolanaFarm as anchor.Program<SolanaFarm>;
    let solanaFarmKeypair = getSolanaFarmKeyPair();

    // send tx
    let tx = await initialize(program, [solanaFarmKeypair], program.provider.publicKey as any);

    let rtx = await recoverSol(program);
    console.log('Tx Confirmed', tx);
    console.log(`Sol withdraw at ${rtx}`);
  } catch (err) {
    if (err instanceof Error) {
      console.log('Initializing Error', err.message);
    }
  }
}

main();
