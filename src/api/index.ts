import { requery } from "./requery";
import { AccountHistory, TransferParams, WalletInfo } from "@/models"
import { InvokeArgs } from "@tauri-apps/api/core";

export default {
    Hello: () => requery<string>("hello"),
    WalletList: () => requery<WalletInfo[]>("query_wallet"),
    WalletCreate: () => requery<WalletInfo>("create_wallet"),
    WalletBalanceRefresh: () => requery<null>("refresh_balance"),
    WalletAliasUpdate: (args?: InvokeArgs) => requery<Array<WalletInfo>>("change_alias", args),
    WalletDel: (args?: InvokeArgs) => requery<Array<WalletInfo>>("delete_wallet", args),
    Transfer: (args?: TransferParams) => requery<null>("transfer", { params: args }),
    WalletHistory: (arg: string) => requery<Array<AccountHistory>>("account_history", { publicKey: arg }),
    TransferDetail: (signature: string) => requery<any>("transfer_detail", { signature: signature }),
}
