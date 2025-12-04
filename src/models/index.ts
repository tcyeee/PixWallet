export type HistoryQuery = {
    public_key: string;
    page: number;
    page_size: number;
}

export type WalletInfo = {
    public_key: string;
    alias: string;
    network: string;
    balance: number;
}

export interface WalletHistoryResp {
    list: AccountHistory[];
    total: number;
}

export enum MsgType {
    /* 显示在界面正上方 */
    ALERT = "ALERT",
    /* 显示在界面右上角 */
    NOTICE = "NOTICE",
    /* 网络状态信息 */
    PING = "PING",
    /* 交易完毕 */
    TRANSFER_END = "TRANSFER_END",
    /* 交易信息 */
    TRANSFER_INFO = "TRANSFER_INFO",
    /* 全部账户查询完毕 */
    BALANCE_REFRESH_END = "BALANCE_REFRESH_END",
    /* 刷新历史记录 */
    REFRESH_HISTORY = "REFRESH_HISTORY",
    /* 刷新单个钱包 */
    REFRESH_WALLET = "REFRESH_WALLET",
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
    new_flag: boolean;
}
export enum Status {
    PROCESSED = "Processed",
    CONFIRMED = "Confirmed",
    FINALIZED = "Finalized",
}

export type NoticeParams = {
    level: NoticeEnum,
    content: string,
}

export enum NoticeEnum {
    SUCCESS = "SUCCESS",
    WARNING = "WARNING",
    ERROR = "ERROR",
    INFO = "INFO",
}