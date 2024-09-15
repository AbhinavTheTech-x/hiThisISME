import * as anchor from '@coral-xyz/anchor';
import { SolanaFarm } from '../target/types/solana_farm';
import chai, { expect } from 'chai';
import chaiBN from 'chai-bn';
import chaiAsPromised from 'chai-as-promised';
import {
  buyEggs,
  getAccounts,
  getDevKeyPair,
  getEarnedSFarm,
  getLamports,
  getMyEggs,
  getPDA,
  getSolanaFarmKeyPair,
  getSolanaFarmState,
  getUserAccountState,
  hatchEggs,
  initialize,
  recoverSol,
  requestAirdropMulti,
  sellEggs
} from '../utils';
import { SOLANA_FARM_USER_PDA, SOLANA_FARM_VAULT } from '../config';

chai.use(chaiBN(anchor.BN));
chai.use(chaiAsPromised);

const LAMPORTS = 1 * anchor.web3.LAMPORTS_PER_SOL;

describe('solana-farm', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaFarm as anchor.Program<SolanaFarm>;
  const programProvider = program.provider as anchor.AnchorProvider;

  // data account
  const solanaFarmKeypair = getSolanaFarmKeyPair();
  const dev = getDevKeyPair();

  const { alice, bob, referral } = getAccounts();

  before('#setup', async () => {
    let solanaFarmVault = getPDA(program.programId, SOLANA_FARM_VAULT);
    await requestAirdropMulti(
      programProvider,
      [alice.publicKey, bob.publicKey, referral.publicKey, dev.publicKey, solanaFarmVault],
      LAMPORTS
    );
  });

  describe('#initialize', () => {
    it('try initialize with invalid solana farm', async () => {
      await expect(
        initialize(program, [anchor.web3.Keypair.generate()], program.provider.publicKey as any)
      ).to.be.rejected;
    });

    it('try initializing with invalid owner', async () => {
      await expect(initialize(program, [solanaFarmKeypair, alice], alice.publicKey)).to.be.rejected;
    });

    it('buy before initialization', async () => {
      let aliceDepositAmount = new anchor.BN(2e8);
      await expect(
        buyEggs(
          program,
          aliceDepositAmount,
          [alice],
          solanaFarmKeypair.publicKey,
          dev.publicKey,
          referral.publicKey
        )
      ).to.be.rejected;
    });

    it('initialize tx should confirm', async () => {
      await expect(initialize(program, [solanaFarmKeypair], program.provider.publicKey as any)).to
        .be.fulfilled;
    });

    it('check state', async () => {
      const solanaFarmState = await getSolanaFarmState(program, solanaFarmKeypair.publicKey);
      expect(solanaFarmState.initialized).to.be.true;
      expect(solanaFarmState.marketEggs.eq(new anchor.BN('108000000000'))).to.be.true;
    });

    it('re-initialization should fail', async () => {
      await expect(initialize(program, [solanaFarmKeypair], program.provider.publicKey as any)).to
        .be.rejected;
    });
  });

  describe('#buyEggs', () => {
    let aliceSolBalanceBefore: number;
    let devSolBalanceBefore: number;

    it('signer is not owner of PDA should fail', async () => {
      let aliceDepositAmount = new anchor.BN(2e8);

      await expect(
        buyEggs(
          program,
          aliceDepositAmount,
          [alice],
          solanaFarmKeypair.publicKey,
          dev.publicKey,
          referral.publicKey,
          true
        )
      ).to.be.rejected;
    });

    it('alice buy with 0.2 sol', async () => {
      // take snap shot
      aliceSolBalanceBefore = await getLamports(programProvider, alice.publicKey);
      devSolBalanceBefore = await getLamports(programProvider, dev.publicKey);

      await expect(
        buyEggs(
          program,
          new anchor.BN(2e8),
          [alice],
          solanaFarmKeypair.publicKey,
          dev.publicKey,
          referral.publicKey
        )
      ).to.fulfilled;
    });

    it('getMyEggs', async () => {
      let eggs = await getMyEggs(program, alice.publicKey);
      expect(eggs.toNumber()).to.be.gt(0);
    });

    it('getReferralEggs', async () => {
      let eggs = await getMyEggs(program, referral.publicKey);
      expect(eggs.toNumber()).to.be.gt(0);
    });

    it('check alice balance', async () => {
      let aliceBalanceAfter = await getLamports(programProvider, alice.publicKey);
      let aliceDepositAmount = 2_00_000_000;
      let priortiyFee = aliceSolBalanceBefore - aliceBalanceAfter - aliceDepositAmount;
      expect(aliceBalanceAfter).to.be.equal(
        aliceSolBalanceBefore - aliceDepositAmount - priortiyFee
      );
    });

    it('check dev balance will receive 8% fees on buy', async () => {
      let newDevBalance = await getLamports(programProvider, dev.publicKey);
      let devFee = newDevBalance - devSolBalanceBefore;
      let expectedDevFee = (2_00_000_000 * 8) / 100;
      expect(devFee).to.be.eql(expectedDevFee);
    });

    it('check user pda state', async () => {
      let userPda = await getUserAccountState(program, alice.publicKey);
      let refPda = getPDA(program.programId, SOLANA_FARM_USER_PDA, referral.publicKey);

      expect(userPda.claimedEggs.toNumber()).to.be.eql(0);
      expect(userPda.hatcheryMiners.toNumber()).to.be.equal(15333);
      expect(userPda.referral.toString()).to.be.equal(refPda.toString());
      expect(userPda.isInitialized).to.be.true;
      expect(userPda.lastHatch.toNumber()).to.be.gt(0);
    });

    it('check ref pda state', async () => {
      let refPda = await getUserAccountState(program, referral.publicKey);
      expect(refPda.claimedEggs.toNumber()).to.be.eql(2070000000);
    });

    it('check solana farm state', async () => {
      let solanaFarm = await getSolanaFarmState(program, solanaFarmKeypair.publicKey);
      expect(solanaFarm.marketEggs.toNumber()).to.be.equal(111312000000);
    });

    it('check sol rewards', async () => {
      let rewards = await getEarnedSFarm(program, solanaFarmKeypair.publicKey, alice.publicKey);
      expect(rewards.toNumber()).to.be.gt(0);
    });

    it('bob should buy with 0.5 sol without referral', async () => {
      await expect(
        buyEggs(program, new anchor.BN(5e8), [bob], solanaFarmKeypair.publicKey, dev.publicKey)
      ).to.fulfilled;
    });
  });

  describe('#hatchEggs', () => {
    it('alice should hatch eggs', async () => {
      await expect(hatchEggs(program, [alice], solanaFarmKeypair.publicKey, referral.publicKey)).to
        .be.fulfilled;
    });

    it('bob should hatch eggs with same referral revert', async () => {
      await expect(hatchEggs(program, [bob], solanaFarmKeypair.publicKey, bob.publicKey)).to.be
        .rejected;
    });

    it('bob should hatch eggs', async () => {
      await expect(hatchEggs(program, [bob], solanaFarmKeypair.publicKey)).to.be.fulfilled;
    });
  });

  describe('#sellEggs', () => {
    let devBalancebefore: number;
    it('alice sell eggs but passing different dev pubkey', async () => {
      const solanaFarmVault = getPDA(program.programId, SOLANA_FARM_VAULT);
      await expect(sellEggs(program, [alice], solanaFarmVault, solanaFarmKeypair.publicKey)).to.be
        .rejected;
    });

    it('alice sell eggs', async () => {
      devBalancebefore = await getLamports(programProvider, dev.publicKey);
      await expect(sellEggs(program, [alice], dev.publicKey, solanaFarmKeypair.publicKey)).to.be
        .fulfilled;
    });

    it('referral sell eggs', async () => {
      await expect(sellEggs(program, [referral], dev.publicKey, solanaFarmKeypair.publicKey)).to.be
        .fulfilled;
    });

    it('self referral not allowed', async () => {
      await expect(
        buyEggs(
          program,
          new anchor.BN(5e8),
          [referral],
          solanaFarmKeypair.publicKey,
          dev.publicKey,
          referral.publicKey
        )
      ).to.rejected;
    });

    it('check dev fee would receive 8% on sell', async () => {
      let devBalanceAfter = await getLamports(programProvider, dev.publicKey);
      expect(devBalanceAfter).to.be.gt(devBalancebefore);
    });
  });

  describe('#recoverSol', () => {
    let ownerSolBalanceBefore: number;
    it('should call by only owner', async () => {
      await expect(recoverSol(program, alice)).to.be.rejected;
    });

    it('owner withdraw some sol', async () => {
      ownerSolBalanceBefore = await getLamports(programProvider, programProvider.publicKey);
      await expect(recoverSol(program)).to.be.fulfilled;
    });

    it('check dev balance after calling recover sol', async () => {
      let ownerAfterBalance = await getLamports(programProvider, programProvider.publicKey);
      expect(ownerAfterBalance).to.be.gt(ownerSolBalanceBefore);
    });
  });
});
