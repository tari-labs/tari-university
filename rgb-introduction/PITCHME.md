## RGB Protocol

- Introduction
- Digital Assets
- Alternatives
- Goals
- Design
- References

---

## Introduction

- The RGB Protocol is an open-source, community driven development of standards and best practices to issue, transmit and store "Bitcoin-base non-bitcoin assets" <span style="font-size:0.2em;">[1](https://github.com/rgb-org/spec)</span>
- The standard is being created to overcome major shortcomings of previous attempts to store digital assets on the blockchain.
- The protocol is currently based on Bitcoin, and is meant to provide an acceptable level of scalability, confidentiality and standardization. <span style="font-size:0.2em;">[2](https://github.com/rgb-org/spec/blob/master/00-introduction.md) </span>

---

## Digital Assets

- Growing interest in a digital proxy for securities, utilities or collectibles. <span style="font-size:0.2em;">[2](https://github.com/rgb-org/spec/blob/master/00-introduction.md)</span>
- The current methods of issuing and transferring these assets is slow, expensive and inefficient. <span style="font-size:0.2em;">[2](https://github.com/rgb-org/spec/blob/master/00-introduction.md)</span>
- There is now more demand for digital assets. <span style="font-size:0.2em;">[3](https://medium.com/coinmonks/a-nonfungible-token-stampede-is-coming-224fe44a9d3b)</span>

+++

### Bitcoin-based (non-bitcoin) Assets

- There are still many uses for the current methods of digital assets.
- Putting digital assets on the blockchain presents its own list of problems:
  - Complex
  - New and not well understood technology
  - Regulatory issues, such as pseudo-anonymity / complete openness. 

+++

### So Why Do It?

**Asset Independence**

- The asset is completely independent from the centralized issuer right after the issuance moment.
- Decentralized mechanisms enabling autonomous, trustless, and censorship-resistant mechanisms to enforce rights/benefits connected to the assets.
  -  The "self-enforced-right" proposition is more interesting but still not realistic: applications of this kind are still far from any actual implementation. <span style="font-size:0.2em;">[2](https://github.com/rgb-org/spec/blob/master/00-introduction.md)</span> 

+++

**Asset Blindness**

- To a certain degree, the issuer of the digital asset is "blind" to the exact asset when they act on it.
- There may be legal ramifications such as KYC/AML legislation.

+++

**Asset Auditability**

- A blockchain based system would be fair towards end users.
- It would make it difficult to inflate supply / change attributes / forge tokens / censor users.
- This could move towards relaxing the current, expensive account/auditing requirements.

+++

**Asset Standardization**

- Much easier to adopt an open standard on a wide scale, especially in the case of Bitcoin based protocols.
- There can be standardized libraries to interact with this protocol.
- With standardization comes liquidity, frameworks, APIs etc.
  
---

## Alternatives

- Ethereum based ERC20 protocol, currently the most used standard.
- Meta-protocols on Bitcoin, are not standardized.
- Colored Coins, further exacerbate the fungibility issue of bitcoin and being on-chain leads to bloating.

---

## Goals For RGB

- A new better proposal for blockchain-based asset management standard.
- Meet the market requirements for standardization, auditability, blindness, independence.
- Try and stay in Layer 2 oscalability / fungibility strategies.
- Leverage as much as possible from the current Bitcoin infrastructure.
- Particular focus is paid to Lightning Network.
- The protocol proposal will contain an additional, specific and native “layer 2” solution based on the "Proofmarshal" idea

---

## Design

**Contract Engine**

- RGB enabled wallets must include software capable of analyzing contract conditions and comply with them.
- Predefined behaviours can be defined using "blueprints" which allows RGB to be implemented in a modular fashion.
- Together with some very specific, "popular", contract blueprints (which should cover most of the use-cases), there's one more general purpose blueprint that embeds a meta-script executor, which allows very complex contract to be created.

+++

**Extended URI**

- To keep within a standardized approach and implement the protocol within a Client Side Validation paradigm
  - Use address passing.
  - When a payee generates an address he transmits the coordinates of the selected Publishing Server.
  - He generates a number called the "dark-tag" and passes it to the payer along with the address and publishing information.
  - This feature can be developed by extend the BOLT Invoice Protocol. <span style="font-size:0.2em;">[4](https://github.com/lightningnetwork/lightning-rfc/blob/master/11-payment-encoding.md)</span>

+++

**Publisher Servers**

- The scheme requires additional agreed-upon (at the transaction level) third parties which store the chain of encrypted proofs and accept related queries.
- In the context of an issued asset, these "publisher" servers could be maintained by the issuer itself.
- They can be maintained individually by the receivers, or by one or many independent third parties selected by the sender from a set defined by the receivers.
- The proofs could be stored and communicated via decentralized systems, but there is a complexity gain with that. 
- The Proofmarshal Integration requires a centralized third party, which could be effectively leveraged for the Lightning Network Integration.

+++

**Lightning Network Integration**

- Because the protocol is UTXO-based, it will be possible to link one or more assets to a Lightning Network channel.
- The channel becomes colored, exchanging state updates which are compliant to the asset scheme.
- There are strong guarantees that asset distribution will be preserved in the case of non-consensual closures.
- It can leverage off of the scalability and privacy features of the Lightning Network.
- LN-enabled atomic swaps and LN-based path discovery for decentralized asset exchange.

+++

**Proofmarshal Integration**

- The protocol will also include another L2 strategy for scalability / fungibility
- An asset-specific implementation of the “Proofmarshal” concept, based on “Single-Use-Seals” and “Proof-of-Publication-Ledgers”.
  
+++

**Proofmarshal Benefits**

- A Publisher Server might also act as “sealer” whereby they can obfuscate transactions but not manipulate/forge/falsify
  - Committing multiple proofs from different anonymous users to a single UTXO. 
- This could decouple the anti-double-spending function of the Bitcoin blockchain from the specific asset transactions
  - Making it possible to "seal" a huge number of them spending a single Bitcoin UTXO.

---
## References

- \[1\] RGB Protocol Specifications for Bitcoin-Based Digital Assets. 2018. [https://github.com/rgb-org/spec/blob/master/README.md](https://github.com/rgb-org/spec/blob/master/README.md)
- \[2\] RGB Protocol Specification #00: Introduction. 2018. [https://github.com/rgb-org/spec/blob/master/00-introduction.md](https://github.com/rgb-org/spec/blob/master/00-introduction.md)
- \[3\] A NonFungible Token Stampede Is Coming. Dan Emmons. 2018. [https://medium.com/coinmonks/a-nonfungible-token-stampede-is-coming-224fe44a9d3b](https://medium.com/coinmonks/a-nonfungible-token-stampede-is-coming-224fe44a9d3b)
- \[4\] BOLT #11: Invoice Protocol for Lightning Payments. 2018. [https://github.com/lightningnetwork/lightning-rfc/blob/master/11-payment-encoding.md](https://github.com/lightningnetwork/lightning-rfc/blob/master/11-payment-encoding.md)