import { requery } from "./requery";
import { WalletInfo } from "@/models"
import { InvokeArgs } from "@tauri-apps/api/core";

export default {
    WalletList: () => requery<WalletInfo[]>("query_wallet"),
    WalletCreate: () => requery<null>("create_wallet"),
    WalletBalanceRefresh: () => requery<null>("refresh_balance"),
    WalletAliasUpdate: (args?: InvokeArgs) => requery<Array<WalletInfo>>("change_alias", args),
    WalletDel: (args?: InvokeArgs) => requery<Array<WalletInfo>>("delete_wallet", args)
}
