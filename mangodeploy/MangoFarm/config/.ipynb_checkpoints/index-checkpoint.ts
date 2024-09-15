import dotenv from 'dotenv';

dotenv.config();

// solana farm vault seed
export const SOLANA_FARM_VAULT = 'solana_farm_vault';

// solana farm user pda seed
export const SOLANA_FARM_USER_PDA = 'solana_farm_user_pda';

// dev key
export const DEV_KEY = process.env.DEV_KEY as string;

// solana farm key
export const SOLANA_FARM_KEY = process.env.SOLANA_FARM_KEY as string;
