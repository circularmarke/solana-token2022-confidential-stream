import { StreamCalculator, StreamSchedule } from "./stream_calculator";

export class SolanaConfidentialStreamClient {
  public getClaimableBalance(schedule: StreamSchedule, currentTime: bigint): bigint {
    return StreamCalculator.calculateClaimable(schedule, currentTime);
  }
}
