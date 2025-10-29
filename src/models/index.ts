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

export type TransferParams = {
    from: string,
    to: string,
    amount: number,
}

export type AccountHistory = {
    public_key: string;
    signature: string;
    slot: number;
    err?: string;
    memo?: string;
    block_time?: number;
    confirmation_status?: Status;
    remark?: string;
    created_at: number;
}
export enum Status {
    PROCESSED = "Processed",
    CONFIRMED = "Confirmed",
    FINALIZED = "Finalized",
}