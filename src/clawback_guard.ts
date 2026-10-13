export interface ClawbackPolicy {
  streamId: string;
  isClawbackAuthorized: boolean;
  unvestedAmount: bigint;
}

export class StreamClawbackGuard {
  public static evaluateClawback(isTerminated: boolean, totalAllocated: bigint, vestedAmount: bigint): ClawbackPolicy {
    const unvested = totalAllocated > vestedAmount ? totalAllocated - vestedAmount : 0n;
    return {
      streamId: "stream-policy",
      isClawbackAuthorized: isTerminated && unvested > 0n,
      unvestedAmount: unvested,
    };
  }
}
