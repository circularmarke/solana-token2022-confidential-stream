# Solana Token-2022 Confidential Stream (STCS)

<p align="center">
  <img src="assets/architecture.svg" alt="Solana Token-2022 Confidential Stream Architecture" width="780" />
</p>

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Network: Solana Mainnet & Devnet](https://img.shields.io/badge/Solana-Anchor_Framework-14F195?logo=solana)](https://solana.com)
[![Standard: Token-2022 Extensions](https://img.shields.io/badge/SPL-Token--2022_Confidential_Transfer-38BDF8.svg)](https://spl.solana.com/token-2022)
[![TypeScript: 5.4](https://img.shields.io/badge/TypeScript-5.4-3178C6?logo=typescript)](https://www.typescriptlang.org)

**Solana Token-2022 Confidential Stream (STCS)** is a privacy-first streaming payroll and token vesting protocol leveraging **Solana SPL Token-2022 confidential transfer extensions (ElGamal encryption and twisted ElGamal proofs)** and non-custodial streaming vaults.

---

## ⚡ Core Capabilities

1. **Confidential Balance Vesting**: Uses Token-2022 ElGamal ciphertexts to conceal continuous streaming balances and employee pay rates from public block explorers.
2. **Deterministic Linear Stream Curve**: Second-by-second linear unlocks with zero front-running vulnerability.
3. **Emergency Clawback & Pausing**: Sponsoring organizations can halt unvested streams upon employment termination while leaving vested funds intact.

---

## 🛠 Smart Contracts & Client SDK

- **`programs/confidential_stream/src/lib.rs`**: Anchor Rust program (`Anchor 0.30+`):
  - `initialize_stream_vault`: Configures Token-2022 mint, beneficiary, rate-per-second, and clawback flags.
  - `claim_vested_confidential`: Calculates unlocked portion and forwards Token-2022 transfer instructions.
- **`src/stream_calculator.ts`**: Pure TypeScript linear vesting curve evaluator.
- **`src/token2022_client.ts`**: High-performance transaction bundle builder using `@solana/web3.js` and `@solana/spl-token`.
- **`tests/stream.test.ts`**: Automated unit test suite.

---

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/circularmarke/solana-token2022-confidential-stream.git
cd solana-token2022-confidential-stream
npm install
npm test
```

---

## 📜 License

Licensed under the [Apache License, Version 2.0](LICENSE).
