import dotenv from 'dotenv';

dotenv.config();

// solana farm vault seed
export const MANGO_FARM_VAULT = 'mango_farm_vault';

// solana farm user pda seed
export const MANGO_FARM_USER_PDA = 'mango_farm_user_pda';

// dev key
export const DEV_KEY = process.env.DEV_KEY as string;

// solana farm key
export const MANGO_FARM_KEY = process.env.MANGO_FARM_KEY as string;
