export function formatSol(lamport: number | undefined): string {
    if (!lamport) return "0 SOL";
    return (lamport / 1_000_000_000).toFixed(2) + " SOL";
}