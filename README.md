# LiquidsFi Oracle Contract <!-- omit in toc -->

This repository contains example smart contracts for key Soroban features and concepts. The examples illustrate how to use the features, in their simplest form.

This repository contains LiquidsFi's soroban oracle contract (MVP) with key features and oracle node receptacle contract.

> [!WARNING]  
> These implementations is a beta version, and it is still undergoing development, testing and it is yet to be audited.

The repository contains a main oracle contracts and a receptacle subcontract:

## The Oracle Main Contract

This is the main contract in the repository, it contains onchain concensus mechanism and upkeep logic for data received.

## The Receptacle Contract

The receptacle contract is the entry-point for each node connected to the oracle. It is where data received onchain are dumped. This ensures scalability since if multiple nodes are sending data to the oracle, it will cause latency, so each node has its own receptacle contract where data are collected and queued.

The receptacle contract can be found at: [Receptacle](https://github.com/LiquidsFi/liquidsfi-oracle-soroban/tree/main/liquidsfi-oracle-receptacle)

## Mainnet Deployment:

- Oracle Contract Deployment: [CADFS4N6Q2JZSNYQ2QKNEFZGFD6NAD3ZXDNEXX7GTVT2KF7UWXNQWBOZ](https://developers.stellar.org/docs/build)
- Bridge Contract Deployment: [CB32ILGARL45X7IW6ROE24VPHSVRHDDQQ7GC2L67LYGB4AGZ2LU3565Z](https://stellar.expert/explorer/public/contract/CADFS4N6Q2JZSNYQ2QKNEFZGFD6NAD3ZXDNEXX7GTVT2KF7UWXNQWBOZ)
