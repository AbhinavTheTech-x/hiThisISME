import * as anchor from '@coral-xyz/anchor';
import { Program, web3, AnchorProvider, BN } from '@coral-xyz/anchor';
import { MangoFarm } from '../target/types/mango_farm';
import { MANGO_FARM_KEY, MANGO_FARM_USER_PDA, MANGO_FARM_VAULT, DEV_KEY } from '../config';

export const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

export const getDevKeyPair = () => {
  const devSecret = anchor.utils.bytes.bs58.decode(DEV_KEY);
  return anchor.web3.Keypair.fromSecretKey(devSecret);
};

export const getMangoFarmKeyPair = () => {
  const mangoFarmSecret = anchor.utils.bytes.bs58.decode(MANGO_FARM_KEY);
  return anchor.web3.Keypair.fromSecretKey(mangoFarmSecret);
};

export const getAccounts = () => {
  return {
    bob: web3.Keypair.generate(),
    alice: web3.Keypair.generate(),
    referral: web3.Keypair.generate()
  };
};

export const getPDA = (programId: web3.PublicKey, seed: string, pubKey?: web3.PublicKey) => {
  let seedBuffer = pubKey ? [Buffer.from(seed), pubKey.toBuffer()] : [Buffer.from(seed)];
  const [pda] = web3.PublicKey.findProgramAddressSync(seedBuffer, programId);
  return pda;
};

export const getPDAMulti = (
  programId: web3.PublicKey,
  user: web3.PublicKey,
  referral?: web3.PublicKey
) => {
  let mangoFarmVault = getPDA(programId, MANGO_FARM_VAULT);
  let userPda = getPDA(programId, MANGO_FARM_USER_PDA, user);
  let refPda = referral ? getPDA(programId, MANGO_FARM_USER_PDA, referral) : null;

  return {
    userPda,
    refPda,
    mangoFarmVault
  };
};

export const getLamports = async (programProvider: AnchorProvider, user: web3.PublicKey) => {
  return await programProvider.connection.getBalance(user);
};

export const requestAirdropMulti = async (
  programProvider: AnchorProvider,
  senders: web3.PublicKey[],
  lamports: number
) => {
  let unResolvedPromises = [] as Promise<string>[];
  senders.forEach((fa) => unResolvedPromises.push(requestAirdrop(fa, lamports, programProvider)));
  await Promise.all(unResolvedPromises);
};

export const requestAirdrop = async (
  fundAccount: web3.PublicKey,
  lamports: number,
  programProvider: AnchorProvider
): Promise<string> => {
  const airdropTx = await programProvider.connection.requestAirdrop(fundAccount, lamports);
  const { blockhash, lastValidBlockHeight } = await programProvider.connection.getLatestBlockhash();

  await programProvider.connection.confirmTransaction(
    {
      blockhash,
      lastValidBlockHeight,
      signature: airdropTx
    },
    'finalized'
  );
  return airdropTx;
};

export const initialize = async (
  program: Program<MangoFarm>,
  signers: web3.Keypair[],
  owner: web3.PublicKey
): Promise<string> => {
  return await program.methods
    .initialize()
    .accounts({
      mangoFarm: signers[0].publicKey,
      owner,
      // its by default
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers(signers)
    .rpc({ skipPreflight: true, commitment: 'finalized' });
};

export const initializeUser = async (
  program: Program<MangoFarm>,
  user: web3.Keypair,
  userPda: web3.PublicKey,
  userAccount: web3.PublicKey
) => {
  const tx = program.methods.initializeUser().accounts({
    userPda,
    user: user.publicKey,
    userAccount,
    systemProgram: anchor.web3.SystemProgram.programId
  });
  return await tx.instruction();
};

export const buyEggs = async (
  program: Program<MangoFarm>,
  solValue: BN,
  signers: web3.Keypair[],
  mangoFarmPubkey: web3.PublicKey,
  devSystemAc: web3.PublicKey,
  referral?: web3.PublicKey,
  sendWithFakePda?: boolean
) => {
  let user = signers[0];
  // user pda
  let { mangoFarmVault, userPda, refPda } = getPDAMulti(
    program.programId,
    user.publicKey,
    referral
  );

  let itx = [] as web3.TransactionInstruction[];

  // initialize the user pda
  const userPdaInfo = await program.provider.connection.getAccountInfo(userPda);
  if (userPdaInfo === null) {
    const userPdaItx = await initializeUser(program, user, userPda, user.publicKey);
    itx.push(userPdaItx);
  }

  // check for ref pda
  if (refPda !== null) {
    const refPdaInfo = await program.provider.connection.getAccountInfo(refPda);
    if (refPdaInfo === null) {
      const refPdaItx = await initializeUser(program, user, refPda, referral);
      itx.push(refPdaItx);
    }
  }

  userPda = sendWithFakePda === true ? refPda : userPda;

  let tx = program.methods.buyEggs(solValue).accounts({
    mangoFarm: mangoFarmPubkey,
    mangoFarmVault,
    userPda,
    refPda,
    user: user.publicKey,
    dev: devSystemAc,
    systemProgram: anchor.web3.SystemProgram.programId
  });

  itx.push(await tx.instruction());
  let buyTransaction = new web3.Transaction().add(...itx);

  return await web3.sendAndConfirmTransaction(
    program.provider.connection,
    buyTransaction,
    signers,
    {
      commitment: 'finalized',
      skipPreflight: true
    }
  );
};

export const hatchEggs = async (
  program: Program<MangoFarm>,
  signers: web3.Keypair[],
  mangoFarm: web3.PublicKey,
  refId?: web3.PublicKey
) => {
  let user = signers[0].publicKey;
  let { userPda, refPda } = getPDAMulti(program.programId, user, refId);
  let tx = await program.methods
    .hatchEggs()
    .accounts({
      mangoFarm,
      userPda,
      refPda,
      user,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers(signers)
    .transaction();

  return await web3.sendAndConfirmTransaction(program.provider.connection, tx, signers, {
    commitment: 'max',
    skipPreflight: true
  });
};

export const sellEggs = async (
  program: Program<MangoFarm>,
  signers: web3.Keypair[],
  devSystem: web3.PublicKey,
  mangoFarm: web3.PublicKey
) => {
  // accounts
  let user = signers[0].publicKey;

  let { mangoFarmVault, userPda } = getPDAMulti(program.programId, user);
  let tx = await program.methods
    .sellEggs()
    .accounts({
      mangoFarm,
      mangoFarmVault,
      userPda,
      user,
      dev: devSystem,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers(signers)
    .transaction();

  return await web3.sendAndConfirmTransaction(program.provider.connection, tx, signers, {
    commitment: 'max',
    skipPreflight: true
  });
};

export const recoverSol = async (program: Program<MangoFarm>, signer?: web3.Keypair) => {
  const mangoFarmVault = getPDA(program.programId, MANGO_FARM_VAULT);
  let signers = signer ? [signer] : [];

  return await program.methods
    .recoverSol()
    .accounts({
      mangoFarmVault,
      owner: signer?.publicKey ?? program.provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers(signers)
    .rpc({
      skipPreflight: true,
      commitment: 'finalized'
    });
};

// solana farm state
export const getMangoFarmState = async (
  program: Program<MangoFarm>,
  mangoFarm: web3.PublicKey
) => {
  return await program.account.mangoFarm.fetch(mangoFarm);
};

// returns user pda state
export const getUserAccountState = async (program: Program<MangoFarm>, user: web3.PublicKey) => {
  let userPda = getPDA(program.programId, MANGO_FARM_USER_PDA, user);
  return (await program.account.user.fetch(userPda)).userState;
};

export const getEarnedSFarm = async (
  program: Program<MangoFarm>,
  mangoFarm: web3.PublicKey,
  user: web3.PublicKey
) => {
  let userPda = getPDA(program.programId, MANGO_FARM_USER_PDA, user);
  let mangoFarmVault = getPDA(program.programId, MANGO_FARM_VAULT);

  return (await program.methods
    .getAccumulatedSol()
    .accounts({
      userPda,
      mangoFarm,
      mangoFarmVault
    })
    .view({ skipPreflight: true })) as BN;
};

export const getMyEggs = async (program: Program<MangoFarm>, user: web3.PublicKey) => {
  let userPda = getPDA(program.programId, MANGO_FARM_USER_PDA, user);

  return (await program.methods
    .getMyEggs()
    .accounts({
      userPda
    })
    .view({ skipPreflight: true })) as BN;
};
