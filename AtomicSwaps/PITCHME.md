## Atomic swaps

- Introduction
- What is Atomic swaps
- Hashed Timelock Contracts
- Atomic vs Etomic
- Currently implementations
- Observations

---

## Introduction

- Currency swap
- Centralized exchanged
- Security risks
  - To date : $1.9 Billion 
  - This year: $800 Million

---

## What is Atomic swaps

- Decentralised exchange

- Allows the exchange of currencies in a trustless environment
- If one party defaults or fails the transaction the tranaction is not completed and neither party can "run off" with the other party's money

---

## Hashed Timelock Contracts

- Hashed Timelock Contracts (HTLC)  is one of the most important technologies required for atomic swaps. 
- This is a payment class that uses haslocks and timelocks to require certain public knowledge before doing a payment, otherwise the payment is reversed. 
- HTLCs are also crucial in the lighting network.

---

## Hashed Timelock Contracts (cont'd)

Example:

#### ![Characters](https://raw.githubusercontent.com/tari-labs/tari-university/SW-atamicSwaps/AtomicSwaps/sources/Characters.png)

---

## Hashed Timelock Contracts (cont'd)

#### ![Char-1](https://raw.githubusercontent.com/tari-labs/tari-university/SW-atamicSwaps/AtomicSwaps/sources/Char-1.png)

---

## Hashed Timelock Contracts (cont'd)

#### ![Char-2](https://raw.githubusercontent.com/tari-labs/tari-university/SW-atamicSwaps/AtomicSwaps/sources/Char-2.png)


---

## Hashed Timelock Contracts (cont'd)

#### ![Char-3](https://raw.githubusercontent.com/tari-labs/tari-university/SW-atamicSwaps/AtomicSwaps/sources/Char-3.png)

---

## Atomic vs Etomic

- For an atomic swap transaction to happen, both currencies must use the same hashing function as this is crucial for HTLC to function. 
- Etomic swaps was created in an attemp to make atomic swaps happend between bitcoin coins and ethereum based tokens (ERC20).

---

## Examples of current atomic swaps

- Manual
- Atomic wallet
- BarterDEX
  - Etomic and Atomic
  - Electron server

---

## Observations

Atomic swaps could make it very easiy for the user to obtain tari tokens. 

In the current stance of things BarterDEX looks like the most complete atomic Swap library. 