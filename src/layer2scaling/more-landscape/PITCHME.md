## Layer 2 Scaling Survey (part 2)

- What is Layer 2 scaling, how will it be applicable to Tari? *[See [layer2scaling-landscape](https://gitpitch.com/tari-labs/tari-university/master?p=/layer2scaling/layer2scaling-landscape#/)]*

- Layer 2 scaling current initiatives (update)
  - TumbleBit
  - Counterparty
  - 2-Way Pegged Secondary Block Chains
  - Lumino
  - Scriptless scripts
  - DAG Derivative Protocols

- Observations

---

## Layer 2 scaling current initiatives (update)

#### #1 TumbleBit

- TumbleBit protocol was invented at the Boston University
- Unidirectional, unlinkable payment hub for Bitcoin
- Combines off-chain cryptographics with on-chain Bitcoin scripting to realize smart contracts not dependent on Segwit

![TumbleBitOverview](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/TumbleBitOverview.PNG)



+++

#### #1 TumbleBit (cont'd)

- Bitcoin scripting used: Hashing conditions, signing conditions, conditional execution, 2-of-2 multi signatures and timelocking
- Fast, anonymous, off-chain payments through an **untrusted** intermediary called the Tumbler
- Classic mixing/tumbling/washing mode or as a fully fledged payment hub
- Two interleaved fair-exchange protocols - *RSA-Puzzle-Solver Protocol* and *Puzzle-Promise Protocol* , relies on the RSA cryptosystem's blinding properties
- Anonymizing through Tor to ensure that the Tumbler server can operate as a hidden service

+++

#### #1 TumbleBit (cont'd)

- Boston University provided a proof-of-concept and reference implementation
- NTumbleBit - C# production implementation of the TumbleBit protocol by Stratis with their Breeze implementation (Nearly production ready)

<u>Strengths</u>

- Anonymity properties
- DoS & Sybil protection
- Balance 
- 2x modes of operation: classic tumbler, payment hub
- Scale ability
- Batch processing 
- Masternode compatibility

+++

#### #1 TumbleBit (cont'd)

<u>Weaknesses</u>

- Payees have better privacy than the payers
- Tumbler service not distributed
- Equal denominations required 

<u>Opportunities for Tari</u>

Has benefits to Tari as a trustless Masternode matching/batch processing engine with strong privacy features.

---

#### #2 Counterparty

- Counterparty NOT a block chain, but token protocol that operates on Bitcoin
- Full DEX & hardcoded smart contracts, e.g. difference, binary options
- Embedded consensus (all comms via Bitcoin, identical ledgers, no P2P network)
- Tx meta data embedded into Bitcoin Txs on Bitcoin block chain (e.g. 1-of-3 multisig, P2SH or P2PKH)
- Read and validated by the federated nodes, executed by them

![CounterpartyStack](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/CounterpartyStack.png)



+++

#### #2 Counterparty (cont'd)

- On testnet only, never included on the mainnet : **EVM** for smart contracts
  - Counterparty smart contracts “lives” at Bitcoin addresses that starts with a `C`
  - Broadcast an `execute` Tx to call a specific function in smart contract code
  - Tx confirmed by Bitcoin miner -> Counterparty federated nodes execute the function 
  - The contract state is modified as the smart contract code executes and stored in the Counterparty database

+++

#### #2 Counterparty (cont'd)

- Most notable projects built on top of Counterparty:
  - [Age of Chains](https://www.ageofchains.com), [Age of Rust](http://spacepirate.io), [Augmentors](https://www.augmentorsgame.com/), [Authparty](http://authparty.io/), [Bitcorns](https://bitcorns.com/), [Blockfreight™](http://blockfreight.com/), [Blocksafe](http://www.blocksafefoundation.com), [BTCpaymarket.com](http://btcpaymarket.com), [CoinDaddy](http://coindaddy.io), [COVAL](https://coval.readme.io), [FoldingCoin](http://foldingcoin.net/), [FootballCoin](https://www.footballcoin.io/), [GetGems](http://getgems.org/#/), [IndieBoard](https://indiesquare.me/), [LTBCoin - Letstalkbitcoin.com](https://letstalkbitcoin.com/), [Mafia Wars](https://mafiawars.io/), [NVO](https://nvo.io/), [Proof of Visit](https://proofofvisit.com/), [Rarepepe.party](http://rarepepe.party), [SaruTobi Island](http://mandelduck.com/sarutobiisland/), [Spells of Genesis](http://www.spellsofgenesis.com), [Takara](https://mandelduck.com/#portfolio), [The Scarab Experiment](https://www.thescarabexperiment.org/), [Token.FM](https://token.fm/), [Tokenly](http://tokenly.com/), [TopCoin](https://topcoin.com/) and [XCP DEX](https://XCPDEX.COM)

+++

#### #2 Counterparty (cont'd)

<u>Strengths</u>

- Provides hard coded smart contract abilities rooted in Bitcoin block chain
- Embedded consensus - Txs created and embedded into Bitcoin Txs (permissionless innovation)

<u>Weaknesses</u>

- Embedded consensus requires lockstep upgrades from network nodes to avoid forks
- Embedded consensus imposes limitations on 2nd layer to interact with base layer's token
- Embedded consensus hampers protocol flexibility & limits speed to that of base layer

<u>Opportunities for Tari</u>

@div[text-left]

<ul>
<li>See '<i>Scriptless scripts</i>'
</ul>

@divend

---

#### #3 2-Way Pegged Secondary Block Chains

- 2WP allows "transfer" of BTC from main Bitcoin block chain to secondary block chain and vice-versa at fixed rate, use appropriate security protocol.
- "Transfer" involves BTC be locked on main Bitcoin block chain, made available on secondary block chain
- <u>Sidechain:</u> Security protocol implemented using Simplified Payment Verification (SPV) proofs 
- <u>Drivechain:</u> Custody of BTC to miners, vote when to unlock BTC and where to send them
- <u>Federated Peg/Sidechain:</u> Trusted federation of mutually distrusting functionaries/notaries
- <u>Hybrid Sidechain-Drivechain-Federated Peg:</u> SPV proofs one way and mix of miner Dynamic Membership Multi-party Signature (DMMS) and functionaries/notaries multi-signatures going back

+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)

![RSK_HybridSideDriveChain](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/RSK_HybridSideDriveChain.png)

- Locking of BTC on main block chain with P2SH Tx (BTC to script hash, not public key hash)

+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)

- To unlock, provide a script matching the script hash and data to make script evaluate true
- 2WP promise concluded when equivalent amount tokens on secondary block chain locked so original bitcoins can be unlocked

<u>Who</u>

- RSK (*formerly Rootstock*) using a hybrid sidechain-drivechain security protocol
- Hivemind (formerly Truthcoin) is implementing a Peer-to-Peer Oracle Protocol 
- Blockstream is implementing a Federated Sidechain called Liquid

![Blockstream-Federated-Sidechain](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/Blockstream-Federated-Sidechain.PNG)



+++

#### #3 2-Way Pegged Secondary Block Chains (cont'd)

<u>Strengths</u>

- Permissionless Innovation
- can be used to test or implement new features without risk
- Chains-as-a-Service (CaaS), with data storage 2WP secondary block chains
- Make it easier to implement smart contracts
- Can support larger block sizes and more Txs per second

<u>Weaknesses</u>

- Transferring BTC back into the main Bitcoin block chain is not secure
- Hugely dependent on merged mining, thus 51% attacks are a real threat
- DMMS provided by mining not secure for small systems, while trust of federation/notaries riskier for large systems

<u>Opportunities for Tari</u>

None, if enough functionality will be built into the main Tari block chain

---

#### #4 Lumino

- Lumino Transaction Compression Protocol (LTCP) -> Tx compression
- It is a lightning-like extension of the RSK platform that uses LTCP
- Difference compression of data from same owner, aggregate signing of previous Txs
- RSK newly launched main net January 2018, Lumino Network to be launched test net

![LuminoDataPruning](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/LuminoDataPruning.PNG)

+++

#### #4 Lumino (cont'd)

<u>Strengths</u>

- Promises high efficiency in pruning the RSK block chain

<u>Weaknesses</u>

- Has not been released yet
- Details about payment channels not decisive in the white paper

<u>Opportunities for Tari</u>

LTCP pruning may be beneficial to Tari

---

#### #5 Scriptless scripts

- *Scriptless Scripts* was coined and invented by mathematician Andrew Poelstra 
- Scripting functionality is offered without actual scripts on the block chain to implement smart contracts
- Currently only work on Mimblewimble and makes use of specific Schnorr signature scheme
- Signature aggregation, mathematically combining several signatures into a single signature, without having to prove Knowledge of Secret Keys (KOSK)
- Known as the *plain public-key model* where only requirement is that each potential signer has public key
- KOSK requires users prove possession of secret key during public key registration with certification authority, one way to generically prevent rogue-key attacks

+++

#### #5 Scriptless scripts (cont'd)

- Signature aggregation properties sought here (different to normal multi-signature scheme):
  - Must be provably secure in the *plain public-key model*
  - Must satisfy normal Schnorr equation, resulting signature written as function of combination of  public keys
  - Must allow for Interactive Aggregate Signatures (IAS) where signers are required to cooperate
  - Must allow for Non-interactive Aggregate Signatures (NAS) where aggregation can be done by anyone
  - Must allow each signer to sign the same message
  - Must allow each signer to sign their own message

+++

#### #5 Scriptless scripts (cont'd)

<u>User Story...</u>

Alice and Bob each needs to provide half a Schnorr signature for a Tx whereby Alice promises to reveal a secret to Bob in exchange for 1 crypto coin. 

Alice can calculate the difference between her half Schnorr signature and the Schnorr signature of the secret (adaptor signature) and hand it over to Bob. Bob then has the ability to verify the correctness of the adaptor signature without knowing the original signatures. Bob can then provide his half Schnorr signature to Alice so she can broadcast the full Schnorr signature to claim the crypto coin. 

By broadcasting the full Schnorr signature Bob has access to Alice's half Schnorr signature and he can then calculate the Schnorr signature of the secret because he already knows the adaptor signature, thereby claiming his prize. This is also known as *Zero-Knowledge Contingent Payments*.

+++

#### #5 Scriptless scripts (cont'd)

<u>Who does it?</u>

[grin-tech.org](https://grin-tech.org/)

Mimblewimble is being sited by Andrew Poelstra as being the ultimate *Scriptless Script*.

![Mimblewimble](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/Mimblewimble.PNG)

+++

#### #5 Scriptless scripts (cont'd)

<u>Strengths</u>

- <u>Data savings:</u> Signature aggregation provides data compression on block chain
- <u>Privacy:</u> Nothing about Scriptless Script smart contract, other than the settlement Tx, is ever recorded on the block chain
- <u>Multiplicity:</u> Multiple digital assets can be transferred between two parties in a single settlement Tx
- <u>Implicit scalability:</u> Scalability on the block chain is achieved by virtue of compressing multiple Txs into a single settlement Tx

+++

#### #5 Scriptless scripts (cont'd)

<u>Weaknesses</u>

- Naive implementation of Schnorr multi-signatures that satisfies key aggregation is not secure
- Bellare and Neven (BN) Schnorr signature scheme loses the key aggregation property in order to gain security in the plain public-key model
- New Schnorr-based multi-signature scheme MuSig, provably secure in the *plain public-key model*, interactive signature aggregation where each signer signs their own message must still be proven by complete security analysis

<u>Opportunities for Tari</u>

- Tari should implement Mimblewimble, *Scriptless Script*s and the MuSig Schnorr signature scheme
- Mimblewimble *Scriptless Script*s could be combined with a federated node (or masternode), similar to that being developed by Counterparty, but with improved consensus

---

#### #6 DAG Derivative Protocols

- A Directed Acyclic Graph (DAG) is a finite directed graph with no directed cycles

- A directed graph is acyclic if it has a topological ordering: for every directed edge *uv* from vertex *u* to vertex *v*, *u* comes before *v* in the ordering

![DAG](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/DAG.PNG)

+++

#### #6 DAG Derivative Protocols (cont'd)

- DAG derivative protocols: [GHOST](https://eprint.iacr.org/2013/881.pdf) (*as Ethash PoW algorithm in Ethereum, Dagger-Hashimoto*), [Braiding](https://scalingbitcoin.org/hongkong2015/presentations/DAY2/2_breaking_the_chain_1_mcelrath.pdf),  [Jute](https://scalingbitcoin.org/milan2016/presentations/D2%20-%209%20-%20David%20Vorick.pdf), [SPECTRE](http://www.cs.huji.ac.il/~yoni_sompo/pubs/16/SPECTRE_complete.pdf) and [PHANTOM](https://docs.wixstatic.com/ugd/242600_92372943016c47ecb2e94b2fc07876d6.pdf) was presented
- DAG in block chain includes traditional off-chain blocks into the ledger, governed by mathematical rules
- A parent that is simultaneously an ancestor of another parent is disallowed

![DAGTopologicalOrdering](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/DAGTopologicalOrdering.PNG)

+++

#### #6 DAG Derivative Protocols (cont'd)

- Main problems solved by the DAG derivative protocols:
  - Inclusion of orphaned blocks (decrease the negative effect of slow propagation)
  - Mitigation against selfish mining attacks

![GHOST](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/GHOST.PNG)

+++

#### #6 DAG Derivative Protocols (cont'd)

- Most DAG derivative protocols
  - Blocks containing conflicting Txs (*i.e. conflicting blocks*) are not orphaned *[not in Braiding]*
  - Conflicting Txs are thrown out while processing the chain
  - SPECTRE: blocks vote -> Tx robustly accepted, Tx robustly rejected, Tx indefinite “pending”
  - All conflicting blocks earn their respective miners a block reward *[not in Braiding]*

![SPECTRE](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/SPECTRE.PNG)

+++

#### #6 DAG Derivative Protocols (cont'd)

- Inclusive (DAG derivative) protocols
  - Incentives for behavior changes by nodes lead to increased throughput
  - Better payoff for weak miners

![InclusiveProtocolDAG](https://github.com/tari-labs/tari-university/raw/master/layer2scaling/more-landscape/sources/InclusiveProtocolDAG.PNG)

- *Note: DAG derivative protocols not Layer 2 Scaling solution, offer scaling of primary block chain*

+++

#### #6 DAG Derivative Protocols (cont'd)

- The Hebrew University of Jerusalem & [DAGlabs](https://www.daglabs.com/) (*the commercial development chapter*)
  - [GHOST](https://eprint.iacr.org/2013/881.pdf), [SPECTRE](http://www.cs.huji.ac.il/~yoni_sompo/pubs/16/SPECTRE_complete.pdf), [PHANTOM](https://docs.wixstatic.com/ugd/242600_92372943016c47ecb2e94b2fc07876d6.pdf)
- Ethereum as the Ethash PoW algorithm that has been adapted from GHOST
- [Dr. Bob McElrath](http://bob.mcelrath.org/resume/)
  - [Brading](https://scalingbitcoin.org/hongkong2015/presentations/DAY2/2_breaking_the_chain_1_mcelrath.pdf)
- David Vorick
  - [Jute](https://scalingbitcoin.org/milan2016/presentations/D2%20-%209%20-%20David%20Vorick.pdf)
- Crypto currencies:
  - [IOTA](https://www.iota.org/) 
  - [Nano](https://nano.org/en)
  - [Byteball](https://byteball.org/)

+++

#### #6 DAG Derivative Protocols (cont'd)

<u>Strengths</u>

- **Layer 1 scaling:** Increased Tx throughput on the main block chain
- **Fairness:** Better payoff for weak miners
- **Decentralization mitigation:** Weaker miners also get profits
- **Tx confirmation times:** Confirmation times of several seconds (SPECTRE)
- **Smart contracts:** Support smart contracts (PHANTOM)

<u>Weaknesses</u>

- Still not proven 100%, development continuing
- DAG derivative protocols differ on important aspects like miner payment schemes, security models, support for smart contracts, confirmation times, etc.
- All DAG derivative protocols are not created equal, beware!

+++

#### #6 DAG Derivative Protocols (cont'd)

<u>Opportunities for Tari</u>

- Applying the basic DAG principles to make a 51% attack harder by virtue of fairness and miner decentralization resistance.
- Choosing the correct DAG derivative protocol can also significantly improve Layer 1 scaling

---

## Observations

Although not all technologies covered here are Layer 2 Scaling solutions, the strengths should be considered as building blocks for the Tari protocol
