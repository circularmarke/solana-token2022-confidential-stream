export interface StreamSchedule {
  ratePerSecond: bigint;
  startTime: bigint;
  endTime: bigint;
  totalClaimed: bigint;
}

export class StreamCalculator {
  public static calculateClaimable(schedule: StreamSchedule, currentTime: bigint): bigint {
    if (currentTime <= schedule.startTime) {
      return 0n;
    }

    const effectiveTime = currentTime > schedule.endTime ? schedule.endTime : currentTime;
    const elapsed = effectiveTime - schedule.startTime;
    const totalVested = elapsed * schedule.ratePerSecond;

    return totalVested > schedule.totalClaimed ? totalVested - schedule.totalClaimed : 0n;
  }
}
