### LiquidsFi Oracle Contract <!-- omit in toc -->

This repository contains LiquidsFi's Soroban oracle contract (MVP), including key features and the oracle node receptacle contract.

> **⚠ WARNING**
>
> These implementations are in beta, still undergoing development and testing, and have not yet been audited.

## Overview

This repository includes two key smart contracts:

- **Oracle Main Contract:** Handles on-chain consensus and upkeep logic for received data.
- **Receptacle Contract:** Acts as an entry point for each connected oracle node, collecting and queuing data before forwarding it to the main oracle contract.

## The Oracle Main Contract

The **Oracle Main Contract** is the core component, responsible for:

- **Consensus Mechanism:** Ensuring data integrity through decentralized validation.
- **Upkeep Logic:** Processing and maintaining accurate on-chain data.
- **Aggregation of Inputs:** Receiving validated inputs from multiple oracle nodes.

## The Receptacle Contract

The **Receptacle Contract** serves as an intermediary for oracle nodes. Each node submits data to its own receptacle, reducing congestion and improving scalability. This approach prevents latency issues caused by multiple nodes interacting directly with the oracle.

🔗 **Receptacle Contract Repository:** [GitHub](https://github.com/LiquidsFi/liquidsfi-oracle-soroban/tree/main/liquidsfi-oracle-receptacle)

## LiquidsFi Bridge Contract

The **LiquidsFi Bridge Contract** enables cross-chain asset transfers by allowing users to deposit stablecoin liquidity into a pool and send tokens to another blockchain.

### Key Features:

- **Liquidity Pooling:** Deposited tokens are locked in the pool.
- **Cross-Chain Transfer:** Transaction data is sent to the destination chain, where the oracle verifies it before releasing the funds.

🔗 **Bridge Contract Repository:** [GitHub](https://github.com/LiquidsFi/liquidsfi-bridge-soroban)

## Mainnet Deployment

- **Oracle Contract Deployment:** [CADFS4N6Q2JZSNYQ2QKNEFZGFD6NAD3ZXDNEXX7GTVT2KF7UWXNQWBOZ](https://developers.stellar.org/docs/build)
- **Bridge Contract Deployment:** [CB32ILGARL45X7IW6ROE24VPHSVRHDDQQ7GC2L67LYGB4AGZ2LU3565Z](https://stellar.expert/explorer/public/contract/CADFS4N6Q2JZSNYQ2QKNEFZGFD6NAD3ZXDNEXX7GTVT2KF7UWXNQWBOZ)

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

For more details, visit the [LiquidsFi GitHub](https://github.com/LiquidsFi).
