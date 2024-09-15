import * as anchor from '@coral-xyz/anchor';
import { getMangoFarmKeyPair, initialize, recoverSol } from '../utils';
import { MangoFarm } from '../target/types/mango_farm';

async function main() {
  try {
    // set provider
    anchor.setProvider(anchor.AnchorProvider.env());

    // derive program
    const program = anchor.workspace.MangoFarm as anchor.Program<MangoFarm>;
    let mangoFarmKeypair = getMangoFarmKeyPair();

    // send tx
    let tx = await initialize(program, [mangoFarmKeypair], program.provider.publicKey as any);

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
