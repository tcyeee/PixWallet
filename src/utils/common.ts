export function formatSol(lamport: number | undefined): string {
    if (!lamport) return "0 SOL";
    if (lamport < 10000) return lamport + " lamport"
    return (lamport / 1_000_000_000).toFixed(2) + " SOL";
}


/**
 * 格式化时间戳
 * @param timestamp 时间戳，单位可以是秒或毫秒
 * @param isSeconds 是否为秒，默认为 true
 * @returns 格式化后的时间字符串 "YYYY-MM-DD HH:mm:ss"
 */
export function formatTimestamp(timestamp: number, isSeconds: boolean = true): string {
    // 如果是秒级时间戳，转换为毫秒
    const date = new Date(isSeconds ? timestamp * 1000 : timestamp);

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0'); // 月份从0开始
    const day = String(date.getDate()).padStart(2, '0');

    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
}