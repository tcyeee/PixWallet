export function formatSol(lamport: number | undefined): string {
    if (!lamport) return "0 SOL";
    if (lamport < 10000) return lamport + " lamport"
    return (lamport / 1_000_000_000).toFixed(2) + " SOL";
}