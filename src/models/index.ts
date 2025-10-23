export type WalletInfo = {
    public_key: string;
    alias: string;
    network: string;
    balance: number;
}

export enum MsgType {
    /* 网络状态信息 */
    PING = "PING",
    /* 全部账户查询完毕 */
    BALANCE_REFRESH_END = "BALANCE_REFRESH_END",
    /* 单个账户更新通知 */
    BALANCE_CHANGE = "BALANCE_CHANGE",
}

export enum NetworkHealth {
    GOOD = "GOOD",
    POOR = "POOR",
    LOST = "LOST",
}

export type NetworkStatus = {
    status: string,
    ping: Number,
}