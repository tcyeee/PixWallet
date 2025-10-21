import { requery } from "./requery";
import { WalletInfo } from "@/models"

export const WalletList = () => requery<WalletInfo[]>("query_wallet");
export const WalletCreate = () => requery<null>("create_wallet");
export const WalletBalanceRefresh = () => requery<null>("refresh_balance");
