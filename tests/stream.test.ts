import { describe, it } from "node:test";
import assert from "node:assert";
import { StreamCalculator, StreamClawbackGuard, SolanaConfidentialStreamClient, StreamSchedule } from "../src/index";

describe("Solana Token-2022 Confidential Stream Test Suite", () => {
  it("should calculate correct linear vested amounts across time steps", () => {
    const schedule: StreamSchedule = {
      ratePerSecond: 10n,
      startTime: 1000n,
      endTime: 2000n,
      totalClaimed: 0n,
    };

    // Before start
    assert.strictEqual(StreamCalculator.calculateClaimable(schedule, 900n), 0n);

    // Mid stream: 500s elapsed * 10 = 5000
    assert.strictEqual(StreamCalculator.calculateClaimable(schedule, 1500n), 5000n);

    // After end: 1000s elapsed * 10 = 10000
    assert.strictEqual(StreamCalculator.calculateClaimable(schedule, 3000n), 10000n);
  });

  it("should account for previously claimed amounts", () => {
    const schedule: StreamSchedule = {
      ratePerSecond: 10n,
      startTime: 1000n,
      endTime: 2000n,
      totalClaimed: 4000n,
    };

    assert.strictEqual(StreamCalculator.calculateClaimable(schedule, 1500n), 1000n);
  });

  it("should permit clawback of unvested funds only upon termination", () => {
    const activeResult = StreamClawbackGuard.evaluateClawback(false, 10000n, 4000n);
    assert.strictEqual(activeResult.isClawbackAuthorized, false);

    const termResult = StreamClawbackGuard.evaluateClawback(true, 10000n, 4000n);
    assert.strictEqual(termResult.isClawbackAuthorized, true);
    assert.strictEqual(termResult.unvestedAmount, 6000n);
  });
});
