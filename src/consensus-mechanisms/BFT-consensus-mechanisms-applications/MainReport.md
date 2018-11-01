# Applications of Byzantine Consensus Mechanisms 

## Abstract 

This report investigates the most promising Byzantine Consensus Mechanisms to achieve optimal consensus performance and safety against ill-behaved participants.

This paper focuses on analyzing these consensus protocols and their feasibility and efficiency in meeting the characteristics of scalability, decentralization and security. 

## Contents

- [Introduction](#introduction)
- [Terminology](#terminology)
  -[Consensus](#consensus)
  -[Binary Consensus](#binary-consensus)
  -[Byzantine Fault Tolerance](#byzantine-fault-tolerance)
  -[Practical Byzantine Fault Tolerant Variants](#practical-byzantine-fault-tolerant-variants)
  -[Deterministic and Non-Deterministic Protocols](#deterministic-and-non-deterministic-protocols)
  -[Scalability-performance trade off](#scalability-performance-trade-off)
  -[Many Forms of Timing Assumptions (Degrees of Synchrony)](#many-forms-of-timing-assumptions-(degrees-of-synchrony)
    -[Synchrony](#synchrony)
    -[Partial Synchrony](#partial-synchrony)
    -[Unknown-△T Model](#unknown-△t-model)
    -[Eventually Synchronous](#eventually-synchronous)
    -[Weak Synchrony](#weak-synchrony)
    -[Random Synchrony](#random-synchrony)
    -[Asynchrony](#asynchrony)
      -[Counting rounds in asynchronous networks](#counting-rounds-in-asynchronous-networks)
      -[The Problem with Timing Assumptions](#the-problem-with-timing-assumptions)
  -[Denial of Service Attack](#denial-of-service-attack)
  -[The FLP Impossibility](#the-flp-impossibility)
  -[Randomized Agreement](#randomized-agreement)
  -[Gossip Protocol](#gossip-protocol) 
- [A brief survey of Byzantine Fault Tolerant Consensus Mechanisms](#a-brief-survey-of-byzantine-fault-tolerant-consensus-mechanisms)
- [Permissioned Byzantine Fault Tolerant Protocols](#permissioned-byzantine-fault-tolerant-protocols)
  - [Hyperledger Fabric (HLF)](#hyperledger-fabric-(hlf))
  - [Tendermint](#tendermint)
- [Permissionless Byzantine Fault Tolerant Protocols](#permissionless-byzantine-fault-tolerant-protocols)
  - [Paxos](#paxos)
  - [Chandra Toueg](#chandra-toueg)
  - [Raft](#raft)
  - [HashGraph](#hashgraph)
  - [SINTRA](#sintra)
  - [HoneyBadgerBFT](#honeybadgerbft)
  - [Stellar Consensus Protocol](#stellar-consensus-protocol)
  - [LinBFT](#linbft)
  - [Algorand](#algorand)
  - [Thunderella](#thunderella)
  - [Avalanche](#avalanche)
  - [PARSEC](#parsec)
  -   [Democratic BFT](#democratic-bft)
- [Summary](#summary)
- [References](#references)

## Introduction 

When considering the how Tari will potentially build its second layer, an analysis of the most promising Byzantine Consensus Mechanisms and their applications was sought. 

Important to consider is the 'scalability trilemma'; a phrase referred to by Vitalik Buterin, which takes into account the potential trade-offs regarding decentralization, security and scalability. [[19]]

**Decentralization** : a core principle on which majority of the systems are build, taking into account censorship-resistance and ensuring that everyone, without prejudice, is permitted to partake in the decentralized system. 

**Scalability** : encompasses the ability of the network to process transactions. Thus, if a public block chain is deemed to be efficient, effective and usable, it should be designed to handle millions of users on the network. 

**Security** : refers to the immutability of the ledger and takes into account threats of 51% attacks, Sybil attacks and DDoS attacks etc. 

Through the recent development of this ecosystem, most block chains have focused on two of the three factors, namely decentralization and security; at the expense of scalability. The primary reason for this is that nodes must reach consensus before transactions can be processed. [[19]]

This report sees the examination of proposals considering Byzantine Fault Tolerant (BFT) consensus mechanisms and considers their feasibility and efficiency in meeting the characteristics of scalability, decentralization and security. In each instance the protocol assumptions, reference implementations and discernment on whether the protocol may be used for Tari as a means to maintain the distributed asset state will be assessed. 

## Terminology

In order to gain a full understanding of the field of consensus mechanism, specifically BFT consensus mechanisms, certain terms and concepts need to be defined and fleshed out.  

### Consensus

Distributed agents (these could be computers, generals co-ordinating an attack, or sensors in a nuclear plant)
that communicate via a network (be it digital, courier or mechanical) need to agree on facts in order to act
as a coordinated whole.

When all non-faulty agents agree on a given fact, then we say that the network is in consensus.

Consensus is achieved when all non-faulty agents, agree on a prescribed fact. 

There are a host of formal requirements which a consensus protocol may adhere to; these include:

- **Agreement:** Where all correct processes agree on the same fact 
- **Weak Validity:** Where for all correct processes, the output must be the input for some correct process
- **Strong Validity:** Where if all correct processes receive the same input value, they must all output that value
- **Termination:** All processes must eventually decide on an output value [[21]]

### Binary Consensus

There is a unique case of the consensus problem, referred to as the binary consensus restricts the input and hence the output domain to a single binary digit {0,1}. 

When the input domain is large; relative to the number of processes, for instance an input set of all the natural numbers, it can be shown that consensus is impossible in a synchronous message passing model. [[21]]

### Byzantine Fault Tolerance

Byzantine failures are considered the most general and most difficult class of failures among the failure modes. The so-called fail-stop failure mode occupies the simplest end of the spectrum. Whereas fail-stop failure mode simply means that the only way to fail is a node crash, detected by other nodes, Byzantine failures imply no restrictions, which means that the failed node can generate arbitrary data, pretending to be a correct one. Thus, Byzantine failures can confuse failure detection systems, which makes fault tolerance difficult. [[39]] 

Several papers in the literature contextualize the problem using generals at different camps, situated outside the enemy castle, needing to decide whether or not to attack. A consensus algorithm that would fail, would perhaps see one general attack while all the others stay back, resulting in the vulnerability of first general 

One key property of a block chain system is that the nodes do not trust each other, meaning that some may behave in Byzantine manners. The consensus protocol must therefore tolerate Byzantine failures. 

A network is Byzantine Fault Tolerant when it can provide service and reach a consensus despite faults or failures of the system. The processes use a protocol for consensus or atomic broadcast to agree on a common sequence of operations to execute. [[20]]

The literature on distributed consensus is vast, and there are many variants of previously proposed protocols being developed for block chains. They can be largely classified along a spectrum. One extreme consists of purely computation based protocols which use proof of computation to randomly select a node which single-handedly decides the next operation. The other extreme is purely communication based protocols in which nodes have equal votes and go through multiple rounds of communication to reach consensus, Practical Byzantine Fault Tolerance (PBFT) being the prime example, which is a replication algorithm designed to be BFT.  [[10]]

For systems with _n_ processors, of which _f_ are Byzantine, it has been shown that _no algorithm exists_ that solves the consensus problem for _f > n/3_.[[21]]

### Practical Byzantine Fault Tolerant Variants 

PoW suffers from non-finality, that is a block appended to a block chain is not confirmed until it is extended by many other blocks. Even then, its existence in the block chain is only probabilistic. For example, eclipse attacks on Bitcoin exploit this probabilistic guarantee to allow double spending. In contrast, the original PBFT protocol is deterministic. [[10]]

### Deterministic and Non-Deterministic Protocols

Deterministic, bounded Byzantine agreement relies on consensus being finalized for each epoch before moving to the next one ensuring that there is some safety about a consensus reference point prior to continuing. If instead you allow an unbounded number of consensus agreements within the same epoch, then there is no overall consensus reference point with which to declare finality and thus safety is compromised. [[8]]

For non-deterministic or probabilistic protocols, the probability that an honest node is undecided after _r_ rounds approaches zero as r approaches infinity.

Non-deterministic protocols which solve consensus under the purely asynchronous case potentially rely on random oracles and generally incur high message complexity overhead, as they depend on reliable broadcasting for all communication.

Protocols like HoneyBadger BFT fall into this class of nondeterministic protocols under asynchrony. Normally, they require three instances of reliable broadcast for a single round of communication. [[34]]

### Scalability-performance trade off 

As briefly mentioned in the [Introduction](#introduction), the scalability of BFT protocols considering the number of participants is highly limited and the performance of most protocols deteriorates as the number of involved replicas increases. This effect is especially problematic for BFT deployment in permissionless block chains. [[7]]

The problem of BFT scalability is twofold: a high throughput as well as a large consensus group with good reconfigurability that can tolerate a high number of failures are both desirable properties in BFT protocols, but are often in direct conflict. 

Bitcoin mining, for example supports thousands of participants, offers good reconfigurability, i.e. nodes can join or leave the network at any time, and can tolerate a high number of failures, however they are only able to process a severely limited number of transactions per second. Most BFT protocols achieve a significantly higher throughput, but are limited to small groups of participants of less than 20 nodes and the group reconfiguration is not easily achievable. 

Several approaches have been employed to remedy these problems, e.g. threshold cryptography, creating new consensus groups for every round, or limiting the number of necessary messages to reach consensus. [[9]]

### *Many Forms of Timing Assumptions (Degrees of Synchrony)* 

#### Synchrony

Here, the time for nodes to wait and receive information is predefined. If a node has not received an input within the redefined time structure,  there is a problem. [[5]]

In synchronous systems it is assumed that all communications proceed in rounds. In one round a process may send all the messages it requires while receiving all messages from other processes. In this manner no message from one round may influence any messages sent within the same round [[21]]

A △T-synchronous network guarantees that every message sent is delivered after at most a delay of △T (where △T is a measure of real time) [[6]] Synchronous protocols come to a consensus every x seconds. [[5]]

#### Partial Synchrony 

Here, the network retains some form of a predefined timing structure, however it can operate within knowing the assumption of how fast nodes can exchange messages over the network. Instead of pushing out a block every x seconds, in a partially synchronous block chain would gauge the limit, with messages always being sent and received within the unknown deadline. 

Partially synchronous protocols come to a consensus in an unknown, but finite period. [[5]]

##### Unknown-△T Model 

The protocol is unable to use the delay bound as a parameter. [[6]] 

##### Eventually Synchronous  

The message delay bound △ is only guaranteed to hold after some (unknown instant, called the "Global Stabilization Time". [[6]]

#### Weak Synchrony 

Most existing Byzantine fault tolerant systems, even those called 'robust' assume some variation of *weak synchrony*, where messages are guaranteed to be delivered after a certain bound △T, but △T may be time-varying or unknown to the protocol designer. 

However, the liveness properties of weakly synchronous protocols can fail completely when the expected timing assumptions are violated (e.g., due to a malicious network adversary). 

Even when the weak synchrony assumptions are satisfied in practice, weakly synchronous protocols degrade significantly in throughput when the underlying network is unpredictable. Unfortunately, weakly asynchronous protocols require timeout parameters that are difficult to tune, especially in cryptocurrency application settings; and when the chosen timeout values are either too long to too short, throughput can be hampered. 

In terms of feasibility, both weak and partially synchronous protocols are equivalent- a protocol that succeeds in one setting can be systematically adapted for another. In terms of concrete performance, however, adjusting for weak synchrony means gradually increasing the timeout parameter over time (e.g. by an exponential back-off policy). This results in delays when recovering from transient network partition. Protocols typically manifest these assumptions in the form of a timeout event. For example, if parties detect that no progress has been made within a certain interval, then they take a corrective action such as electing a new leader. Asynchronous protocols do not rely on timers, and make progress whenever messages are delivered, regardless of actual clock time. [[6]]

#### Random Synchrony

Messages are delivered with random delays, such that the average delay is finite. There may be periods of arbitrarily long days (this is a weaker assumption than weak synchrony, and only a bit stronger than full asynchrony, where the only guarantee is that messages are eventually delivered). It is impossible to tell whether an instance has failed by completely stopping or if there is just a delay in message delivery. [[1]]

#### Asynchrony 

In an asynchronous network, the adversary can deliver messages in any order and at any time, however the message must eventually be delivered between correct nodes. Nodes in an asynchronous network effectively have no use for real time clocks, and can only take actions based on the ordering of messages they receive. [[6]]. The speed is determined by the speed at which the network communicates-instead of a fixed limit of x seconds. 

An asynchronous protocol requires a different means to decide when all nodes are able to come to a consensus.  

As will be discussed in [The FLP Impossibility](#the-flp-impossibility), FLP result rules out the possibility of the deterministic asynchronous protocols for atomic broadcast and many other tasks. A deterministic protocol must therefore make some stronger timing assumptions. [[6]]

##### Counting rounds in asynchronous networks

Although the guarantee of eventual delivery is decoupled from notions of 'real time', it is nonetheless desirable to characterize the running time of asynchronous protocols. The standard approach is for the adversary to assign each message a virtual round number, subject to the condition that every (*r*-1) message between correct nodes must be delivered before any (*r*+1) message is sent. 

### The Problem with Timing Assumptions

The problem with both synchronous and partially synchronous assumptions is that "the protocols based on timing assumptions are unsuitable for decentralized, cryptocurrency settings, where network links can be unreliable, network speeds change rapidly, and network delays may even be adversarially induced."[[6]]

#### Denial of Service Attack 

Basing a protocol on timings, exposes the network to Denial of Service (DoS) attacks. A synchronous protocol will be deemed unsafe if a DoS slows down the network sufficiently. Even though a partially synchronous protocol would be safe, it would be unable to operate, as the messages would be exposed to interference. 

An asynchronous protocol would be able to function under a DoS attack, however it is difficult to reach consensus, as it is impossible to know if the network is under attack, or if a particular message is delayed by the protocol itself. 

### The FLP Impossibility

The paper, 'Impossibility of Distributed Consensus with One Faulty Process' by Fischer et al. [[22]], mapped out what is possible to achieve with distributed processes in an asynchronous environment. 

The result, referred to as the FLP result, which raised the problem of consensus, that is, getting a distributed network of processors to agree on a common value. This problem was known to be solvable in a synchronous setting, where processes could proceed in simultaneous steps. The synchronous solution was seen as resilient to faults, where processors crash and take no further part in the computation. Synchronous models allow failures to be detected by waiting one entire step length for a reply from a processor, and presuming that it has crashed if no reply is received. 

This kind of failure detection is not possible in an asynchronous setting, as there are no bounds on the amount of time a processor might take to complete its work and then respond. The FLP result shows that in an asynchronous setting, where only one processor might crash, there is no distributed algorithm that solves the consensus problem. [[23]]

### Randomized Agreement 

The consensus problem involves an asynchronous system of processes, some of which may be unreliable. The problem is for the reliable processes to agree on a binary value. Every protocol for this problem has the possibility of nontermination. [[22]] While the vast majority of PBFT protocols steer clear of this impossibility result by making timing assumptions, randomness (and, in particular, cryptography) provides an alternative route. Asynchronous BFT protocols have been used for a variety of tasks such as binary agreement (ABA), reliable broadcast (RBC) and more. [[6]]

## A brief survey of Byzantine Fault Tolerant Consensus Mechanisms

Many peer-to-peer online Real-time strategy games use a modified Lockstep protocol as a consensus protocol in order to manage game state between players in a game. Each game action results in a game state delta broadcast to all other  players in the game along with a hash of the total game state. Each player validates the change by applying the delta to their own game state and comparing the game state hashes. If the hashes do not agree then a vote is cast, and those players whose game state is in the minority are disconnected and removed from the game (known as a desync.) [[21]]

## Permissioned Byzantine Fault Tolerant Protocols 

Byzantine agreement schemes are considered well suited for permissioned block chains, where the identity of the participants is known. Examples include Hyperledger and Tendermint. Here the Federated Consensus Algorithm is implemented. [[9]] 

### Hyperledger Fabric (HLF)

HLF began as a project under the LinX Foundation in early 2016 [[13]], with the aim of creating an open-source cross-industry standard platform for distributed ledgers. Hyperledger Fabric is an implementation of a distributed ledger platform for running smart contracts, leveraging familiar and proven technologies, with a modular architecture allowing pluggable implementations of various functions. The distributed ledger protocol of the fabric is run on the peers. [[11]]

The block chain's hash chain is computed based on the executed transactions and resulting persistent state. The replicated execution of chaincode is used for validating the transactions. They assume that among *n* validating peers, at most *f<n/3* (where *f* is the number of faulty nodes and *n* is the number of nodes present in the network) may behave arbitrarily, while others will execute correctly, thus adapting to concept BFT consensus. Since HLF follows PBFT, the chaincode transactions must be deterministic in nature, otherwise different peers might have different persistent state. The SIEVE protocol is used to filter out the non-deterministic transactions, thus assuring a unique persistent state among peers. [[11]]

While being redesigned for a v1.0 release, the format's goal was to achieve extensibility. This version allowed for modules such as membership and consensus mechanism to be exchanged. Being permissioned, this consensus mechanism is mainly responsible for receiving the transaction request from the clients and establishing a total execution order. So far, these pluggable consensus modules include a centralized, single orderer for testing purposes and a crash-tolerant ordering service based on Apache Kafka. [[9]]

### Tendermint

Tendermint Core is a BFT Proof-of-Stake (PoS) protocol which is composed of two protocols in one: a consensus algorithm and a peer-to-peer networking protocol. Jae Kwon and Ethan Buchman, inspired by the design goal behind [Raft](#raft), specified Tendermint as an easy to understand, developer-friendly algorithm while doing algorithmically complex systems engineering. [[34]]

Tendermint is modeled as a deterministic protocol, live under partial synchrony, which achieves throughput within the bounds of the latency of the network and individual processes themselves. 

Tendermint rotates through the validator set, in a weighted round-robin fashion: where the higher the stake (i.e. voting power) that a validator possesses, the greater their weighting, the proportionally more times they will be elected as leaders. Thus, if one validator has the same amount of voting power as another validator, they will both be elected by the protocol an equal amount of times. [[34]] 

Critics have argued that Tendermint is not decentralized, and one can distinguish and target leadership, launching a DDoS attack against them, sniffling the progression of the chain. Although Sentry Architecture (containing sentry nodes, see [Sentry Nodes](#sentry-nodes)) in Tendermint has been implemented, the argument on the degree of decentralization is still questionable. 

#### Sentry Nodes 

Sentry Nodes are guardians of a validator node and provide the validator nodes with access to the rest of the network. Sentry nodes are well connected to other full nodes on the network. Sentry nodes may be dynamic, but should maintain persistent connections to some evolving random subset of each other. They should always expect to have direct incoming connections from the validator node and its backup(s). They do not report the validator node's address in the Peer Exchange Reactor (PEX) and they may be more strict about the quality of peers they keep.

Sentry nodes belonging to validators that trust each other may wish to maintain persistent connections via Virtual Private Network (VPN) with one another, but only report each other sparingly in the PEX.

## Permissionless Byzantine Fault Tolerant Protocols  

BFT protocols face several limitations when utilized in permissionless block chains. They do not scale well with the number of participants, resulting in performance deterioration for the targeted network sizes. In addition, they are not well established in this setting, thus they are prone to security issues, e.g. Sybil attacks. Currently, there are approaches that attempt to circumvent or solve this problem. [[9]]

### Paxos 

The Paxos family of protocols includes a spectrum of trade-offs between the number of processors, number of message delays before learning the agreed value, the activity level of individual participants, number of messages sent,  and types of failures. Although the FLP theorem states that there is no deterministic fault-tolerant consensus protocol that can guarantee progress in an asynchronous network, Paxos guarantees safety (consistency), and the conditions that could prevent it from making progress are difficult to provoke [[29]].

Paxos achieves consensus as long as there are *f* failures, where _f < (n-1)/2_. These failures cannot be Byzantine (otherwise the BFT proof would be violated). Thus it is assumed that messages are never corrupted, and that nodes do not collude to subvert the system. 

Paxos proceeds through a set of negotiation rounds, with one node having 'Leadership' status. Progress will stall if the leader becomes unreliable, until a new leader is elected, or if suddenly an old leader comes back online and a dispute between two leader nodes arises.

### Chandra-Toueg

The Chandra–Toueg consensus algorithm was published by Tushar Deepak Chandra and Sam Toueg in 1996. It relies on a special node that acts as a failure detector. In essence, it pings other nodes to make sure they're still responsive.

This implies that the detector stays online and that the detector must continuously be made aware when new nodes join the network.

The algorithm itself is similar to the Paxos algorithm, which also relies on failure detectors and as such requires *f<n/2*, where n is the total number of processes. [[27]]

### Raft

Raft is a consensus algorithm designed as an alternative to Paxos. It was meant to be more understandable than Paxos by means of separation of logic, but it is also formally proven safe and offers some additional features [[28]].  

Raft achieves consensus via an elected leader. Each follower has a timeout in which it expects the heartbeat from the leader. It is thus a synchronous protocol. If the leader fails, an election is held to find a new leader. This entails nodes nominating themselves on a first-come, first-served basis. Hung votes require the election to be scrapped and restarted. This suggests that a high degree of cooperation is required by nodes and that malicious nodes could easily collude to  disrupt a leader and then prevent a new leader from being elected. Raft is a simple algorithm but is clearly unsuitable for consensus in cryptocurrency applications.

While Paxos and Raft and many other well-known protocols tolerate crash faults, Byzantine fault tolerant protocols beginning with PBFT, tolerate even arbitrary corrupted nodes. Many subsequent protocols offer improved performance, often through optimistic execution that provides excellent performance when there are no faults, clients do not contend much, and the network is well behaved, and at least some progress otherwise. 

In general, BFT systems are evaluated in deployment scenarios where latency and CPU are the bottleneck, thus the most effective protocols reduce the number of rounds and minimize expensive cryptographic operations.

Clement et al. [[40]] initiated a recent line of work by advocating improvement of the worst-case performance, providing service quality guarantees even when the system is under attack, even if this comes at the expense of performance in the optimistic case. However, although the "Robust BFT protocols in this vein gracefully tolerate comprised nodes, they still rely on timing assumptions about the underlying network".  Thus focus shifted to asynchronous networks. [[6]] 

### HashGraph

The Hashgraph consensus algorithm [[30]], was released in 2016.  It claims Byzantine fault tolerance under complete **asynchrony** assumptions, no leaders, no round robin, no proof-of-work, eventual consensus with probability one, and high speed in the absence of faults. 

It is based on the gossip protocol, which is a fairly efficient distribution strategy that entails nodes randomly sharing information with each other, similar to how human beings gossip with each other.

Nodes jointly build a hash graph reflecting all of the gossip events. This allows Byzantine agreement to be achieved through virtual voting. Alice does not send Bob a vote over the Internet. Instead, Bob calculates what vote Alice would have sent, based on his knowledge of what Alice knows.  

HashGraph uses digital signatures to prevent undetectable changes to transmitted messages.

HashGraph does not violate the FLP theorem, since it is _non-deterministic_. 

The Hash graph has some similarities to a block chain. To quote the white paper: "The HashGraph consensus algorithm is equivalent to a block chain in which the 'chain' is constantly branching, without any pruning, where no blocks are ever stale, and where each miner is allowed to mine many new blocks per second, without proof-of-work" [[30]].

Because each node keeps track of the hash graph, there is no need to have voting rounds in HashGraph; each node already knows what all of its peers will vote for and thus consensus is reached purely by analyzing the graph. 

### The HashGraph Algorithm: The Gossip Protocol 

The gossip protocol works like this:

- Alice selects a random peer node, say Bob, and sends him _everything she knows_. She then selects another random node and repeats the process indefinitely.

- Bob, on receiving Alice's information, marks this as a gossip event and fills in any gaps in his knowledge from Alice's information. Once done, he continues gossiping with his updated information.

In this way, information spreads throughout the network in an exponential fashion. [[30]]

![Figure 1 - HashGraph](../assets/gossip.png 'The history of any gossip protocol can be represented by a directed graph, 
where each member is a column of vertices. Each transfer event is shown as a new vertex with two e dges linking the 
immediately-preceding gossip events.')

The gossip history can be represented as a directed graph, as in Figure 1. 

HashGraph introduces a few important concepts that are used repeatedly in later BFT consensus algorithms: famous witnesses, and strongly seeing.

#### Ancestors

If an event (_x1_) comes before another event (_x2_), and they are connected by a line; the older event is an _ancestor_ of that event.

If both events were created by the _same node_, then _x1_ is a _self-ancestor_ of _x2_. 

**Note**: The gossip protocol defines an event as being a (self-)ancestor of itself!

#### Seeing

If an event _x1_ is an ancestor of _x2_, then we say that _x1_ **sees** _x2_ as long as the node is not aware of any forks from _x2_.

So in the absence of forks, all events will _see_ all of their ancestors.

```text
     +-----> y
     |
x +--+
     |
     |
     +-----> z
```

In the example above, x is an ancestor to both y and z. However, because there is no ancestor relationship between y and z, the _seeing_ condition fails, and so y cannot see x, and z cannot see x.

It may be the case that it takes time before nodes in the protocol detect the fork. For instance Bob may create z and y; but share z with Alice and y with Charlie. Both Alice and Charlie will eventually learn about the deception, but until that point, Alice will believe that y sees x, and Charlie will believe that z sees x. 

This is where the concept of _strongly seeing_ comes in. 

#### Strongly seeing

If a node examines its hash graph and notices that an event z _sees_ an event x, and not only that, but it can draw an ancestor relationship (usually via multiple routes) through a super-majority of peer nodes, and that a different event from each node also sees x; then it is said that according to this node, that z _strongly sees_ x.

The following example comes from [[30]]:

![Strongly seeing example](../assets/strongly-seeing.png)

#### The Construct of Gossiping

The main consensus algorithm loop consists of every node (Alice), selecting a random peer node (Bob) and sharing their graph history. Now Alice and Bob have the same graph history.

Alice and Bob both create a new event with the new knowledge they have just learnt from their peer.

Alice repeats this process continuously.

#### Internal consensus

After a sync, a node will determine the order for as many events as possible, using three procedures.
The algorithm uses constant _n_ (the number of nodes) and a small constant value _c_>2.  

```text
in parallel:
    loop
      sync all known events to a random member
    end loop

    loop
      receive a sync
      create a new event
      call divideRounds
      call decideFame
      call findOrder
    end loop
    
    procedure divideRounds
      for each event x
        r ← max round of parents of x ( or 1 if none exist )
        if x can strongly see more than 2/3*n round r witnesses
          x.round ← r + 1
        else
          x.round ← r
        x.witness ← ( x has no self parent ) || ( x.round > x.selfParent.round )
        
    procedure decideFame
      for each event x in order from earlier rounds to later
        x.famous ← UNDECIDED
        for each event y in order from earlier rounds to later
          if x.witness and y.witness and y.round > x.round
            d ← y.round - x.round
            s ← the set of witness events in round y.round-1 that y can strongly see
            v ← majority vote in s ( is TRUE for a tie )
            t ← number of events in s with a vote of v
            if d = 1 // first round of the election
              y.vote ← can y see x ?
            else if d mod c > 0 // this is a normal round
                if t > 2* n /3 // if supermajority, then decide
                  x.famous ← v
                  y.vote ← v
                  break // y loop
                else // else, just vote
                  y.vote ← v
            else if t > 2* n /3 // this is a coin round
              y.vote ← v
            else // else flip a coin
              y.vote ← middle bit of y.signature
```
#### Criticisms

An attempt to address some of these criticisms has been presented. [[31]], 
- The HashGraph protocol is patented and is not open source.
- In addition, the HashGraph white paper assumes that _n_, the number of nodes in the network, is constant. In practice, _n_ can increase, but performance likely degrades badly as _n_ becomes large. [[32]]
- HashGraph is not as "fair" as claimed in their paper, with at least one attack being proposed. [[33]]

### SINTRA

SINTRA is a system implementation based on the **asynchronous** atomic broadcast protocol from Cachin et al. [[41]] This protocol consists of a reduction from Atomic Broadcast Channel (ABC) to Asynchronous Common Subset (ACS), as well as reduction from ACS to multi-value validated agreement (MVBA). According to Miller et al., the protocol suggested by Cahin et al., can be improved using a more effect ACS construction. [[6]]

### HoneyBadgerBFT

HoneyBadgerBFT was released in November 2016 and is seen as the first practical **asynchronous** BFT consensus algorithm. Designed with cryptocurrencies in mind, where bandwidth is considered scarce, but an abundance of CPU power is available. Thus, the protocol implements public-private key encryption to increase the efficiency of the establishment of consensus. The protocol works with a fixed set of servers to run the consensus; however, this leads to centralization and allows an attacker to specifically target these servers. [[9]]

In its threshold encryption scheme, any one party can encrypt a message using a master public key, and it requires f+1 correct nodes to compute and reveal decryption shares for a ciphertext before the plaintext can be recovered.

HoneyBadger offers a novel reductions from ABC to ACS that provides better efficiency (by O(N) factor) through batching, while using threshold encryption to preserve censorship resilience. Better efficiency is also obtained by cherry-picking improved instantiations of sub-components. For example, the expensive MVBA is circumvented by using an alternative ACS along with an effect RBC. [[6]]

### Stellar Consensus Protocol

Stellar Consensus Protocol (SCP) is an **asynchronous** protocol proposed by David Mazieres. It considered to be a global consensus protocol consisting of nomination protocol and ballot protocol, and is said to be BFT by bringing with it the concept of quorum slices and defeated byzantine fault tolerance. [[11]]

Each participant forms a quorum of other users, thus creating a trust hierarchy, which requires complex trust decisions. [[9]]

Initially the nomination proctor is run. During this, new values called candidate values are proposed for agreement. Each node receiving these values will vote for a single value among these. Eventually it results in unanimously selected values for that slot. 

After successful execution of nomination protocol, the nodes deploy the ballot protocol. This involves the federated voting to either commit or abort the values resulting from nomination protocol. This results in externalizing the ballot for the current slot. The aborted ballots are now declared irrelevant. However, there can be stuck states where nodes cannot reach a conclusion, whether to abort or commit a value. This situation is avoided by moving it to a higher valued ballot, considering it in a new ballot protocol execution. This aids in case a node believes that this stuck ballot was committed. Thus SCP assures avoidance and management of stuck states and thus provides liveliness. 

The concept of quorum slices in case of SCP provides asymptotic security and flexible trust, making it more acceptable than other earlier consensus algorithms utilizing Federated BFT, like the Ripple consensus protocol. [[14]] Here, the user is provided more independence in deciding whom to trust. [[15]]

SCP protocol claims to be free of blocked states, provides decentralized control, asymptotic security, flexible trust and low latency. But it does not guarantee safety all the time. If the user node chooses an inefficient quorum slice security is not guaranteed.

### LinBFT

LinBFT is a Byzantine fault tolerance protocol for block chain systems that allows for the amortized *O(n)* communication volume per block under reasonable conditions (where *n* is the number of participants) while satisfying deterministic guarantees on safety and liveness. It satisfies liveness in a **partially synchronous** network. 

LinBFT cuts down its *O(n*<sup>4</sup>*)* complexity by implementing changes each by  *O(n)*: linear view change, threshold signatures and verifiable random functions. 

This is clearly optimal, in the sense that disseminating a block already takes *O(n)* transmissions. 

LinBFT is designed to be implemented for permission-less, public block chain systems and takes into account anonymous participants without a public-key infrastructure, PoS, rotating leader and a dynamic participant set. [[16]]

For instance, participants can be anonymous, without a centralized public key infrastructure (PKI) public key among themselves, and participate in a distributed key generation (DKG) protocol required to create threshold signatures, both of which are communication-heavy processes. 

LinBFT is compatible with proof-of-state, which counters Sybil attacks and deters dishonest behavior through slashing. [[16]]

### Algorand 

The Algorand WhitePaper was released in May 2017, and is a **synchronous** BFT consensus mechanism; where the blocks get added at a minimum rate. [[25]] 

Algorand allows participants to privately check whether they are chosen for consensus participation and requires only one message per user, thus limiting possible attacks. [[9]]

Alogrand, scales up to 500 000 users by employing Verifiable Random Functions, which are pseudo-random functions able to provide verifiable proofs that the output of said function is correct.[[9]]

It introduces the concept of a *concrete coin*. Most of these BFT algorithms require some sort of randomness oracle, but all nodes need to see the same value if the oracle is consulted. This had previously been achieved through a _common coin_ idea; the *concrete coin* uses a much simpler approach; but only returns a binary value. [[25]]

### Thunderella

Thunderella, implements an asynchronous strategy, where a **synchronous** strategy is used as a fall back in the event of a malfunction [[26]], thus it achieves both robustness and speed. 

It can be applied in permissionless networks using proof-of-work. Network robustness and "instant confirmations" requires both 75% of the network to be honest, as well as the presence of a leader node.

### Snowflake to Avalanche

This consensus protocol was first seen in the WhitePaper entitled "Snowflake to Avalanche". Outlined in the paper are four protocols which are building blocks forming a protocol family. These leaderless Byzantine fault tolerance protocols are build on a metastable mechanism and are referred to as: Slush; Snowflake; Snowball and Avalanche.

The protocols published by Team Rocket differ from the traditional consensus protocols and the Nakamoto consensus protocols by not requiring an elected leader, but instead the protocol simply guides all the nodes to consensus. 

These four protocols are described as a new family of protocols due to this concept of metastability: a means to establish consensus by guiding all nodes towards an emerging consensus without requiring leaders, while still maintaining the same level of security and inducing a speed that exceeding current protocols. 

This is achieved through the formation of 'sub-quorums', which are small randomized samples from nodes on the network. This allows for greater throughputs and sees parallel consensuses running before they merge to form the overarching consensus: what can be seen as similar in nature to the gossip protocol. 

With regards to safety, throughput (the number of transactions per second) and scalability (the number of people supported by the network) Slush, Snowflake, Snowball and Avalanche seem to be able to achieve all three. They impart a probabilistic safety guarantee in the presence of Byzantine adversaries and achieve a high throughput and scalability due to their concurrent nature. A **synchronous** network is assumed. 

This is the current problem facing the design of BFT protocols, in that a system can be very fast when a small number of nodes are active, since there are less decisions to make, however, when there are many users and an increase in transactions, the system cannot be maintained. 

Unlike the PoW implementation, which requires constant active participation from the miners, Avalanche can function with the even when nodes are dormant. 

While traditional consensus protocols require *O*(n<sup>2</sup>) communication, their communication complexity ranges from *O(kn log n)* to *O(kn)* for some security parameter *k<<n*. In a sense, Team Rocket highlight that the communication complexity of their protocols is less intensive than that of *O*(n<sup>2</sup>) communications, thus making these protocols faster and more scalable. 

The reason for this is *O*(n<sup>2</sup>) suggests that the rate of growth of function is determined by n<sup>2</sup> where *n* is the number of people on the network. Thus, the addition of a person exponentially increases the time taken to disseminate the information on the network while traditional consensus protocols require everyone to communicate with one another- making it a laborious process. [[18]]

Despite assuming a synchronous network, which is susceptible to the DoS attacks, this new family of protocols "reaches a level of security that is simply good enough while surging forward with other advancements". [[18]] 

### PARSEC

PARSEC is a byzantine fault tolerant consensus algorithm possessing **weak synchrony** assumptions (highly asynchronous, assuming random delays with finite expected value)

Similar to HashGraph, it has no leaders, no round robin, no proof of work and reaches eventual consensus with probability one. It differs from HashGraph, in that it provides high speed in the absence and presence of faults.  Thus, it avoids the structures of delegated PoS (DPoS), which requires a trusted set of leaders, and does not have a round robin (where a permissioned set of miners sign each block)

It is fully open, unlike HashGraph, which is patented and closed sourced. The referenece implementation of PARSEC, written in Rust, was realeased a few weeks after the whitepaper [[1]][[37]]. 

The general problem of reaching Byzantine agreement on any value is reduced to the simple problem of reaching binary Byzantine agreement on the nodes participating in each dicision. This has allowed for PARSEC to reuse the binary Byzantine agreement protocol (Signature-Free Asynchronous Byzantine Consensus) after adapting it to the gossip protocol. [[5]] 

Similar to Honeybadger BFT, this protocol is composed through the additions of interesting ideas presented in literature. 

Like HashGraph and Avalanche, a gossip protocol is used to allow efficient communication between nodes. [[1]]

Finally, the need for a trusted leader or a trusted setup phase implied is removed by porting the key ideas to an asynchronous setting [[3]]

The network is set to *N of N* instances of the algorithm communicating via randomly synchronous connections. 

Due to random synchrony, all users can reach an agreement on what is going on, there is no guarantee for nodes on the timing that they should be receiving messages and a possibility of up to *t* Byzantine (arbitrary) failures are allowed, were *3t*<N. The instances where no failures have occurred are deemed correct or honest, while the failed instances are termed faulty or malicious. Since a Byzantine failure model allows for malicious behavior, any set of instances containing more than 2/3N of them are referred to as the supermajority. 

When a node receives a gossip request, it creates a new event and sends a response back (in HashGraph, the response was optional). Each gossip event contains [[35]]:

1. The data being transmitted
2. The self-parent (the hash of another gossip event created by the same node)
3. The other-parent (a hash of another gossip event created by a different node)
4. The _Cause_ for creation which can either be a Request for information, a Response to another node’s request, or an 
   _Observation_. An observation is when a node creates a gossip event to record an observation that the node made themselves.
5. Creator ID (public key)
6. Signature – signing the above information.

The self-parent and other-parent prevents tampering because they are signed and related to other gossip events [[35]].

As with HashGraph, it is difficult for adversaries to interfere with the consensus algorithm because all voting is *virtual* and done without sharing details of votes cast; each node figures out what other nodes would have voted based on their
copy of the gossip graph.

PARSEC also uses the concept of a *concrete coin*, from Algorand that is used to break ties; particularly in cases where an adversary is carefully managing communication between nodes in order to maintain a deadlock on votes.

First nodes try and converge on a 'true' result for a set of results. If this is not achieved, they move onto step 2, which is trying to converge on a 'false' result. If consensus still cannot be reached, a coin flip is made and we go back to step 1
in another voting round.

Similar to HashGraph, it is not clear that PARSEC will scale well as _n_ grows to millions of nodes. 

## Democratic BFT

This is a deterministic Byzantine consensus algorithm that relies on a new weak coordinator. This protocol is implemented in the Red Belly Block chain and is said to achieve 30 000 transactions/second on Amazon Cloud Trials [[36]], Through the coupling with an optimized variant of the reduction of multivalve to binary consensus from Ben-Or et al., the Democratic BFT (DBFT) consensus algorithm was generated which terminates in 4 message delays in the good case, when all non-faulty processes propose the same value. [[17]]

The term weak coordinator is used to describe the ability of the algorithm to terminate in the presence of a faulty or slow coordinator unlike previous algorithms that do not have the ability to terminate. The fundamental idea here is to allow processes to complete asynchronous rounds as soon as they receive a threshold of messages, instead of having to wait for a message from a coordinator that may be slow. 

The resulting algorithm assumes partial synchrony, is resilience optimal, time optimal and does not require signatures. 

Moving away from the impossibility of solving consensus in asynchronous message systems, where processes can be faulty or *Byzantine*, the technique of randomization or additional synchrony is adopted. 

Randomized algorithms can use per-process "local" coins or a shared *common coin* to solve consensus probabilistically among *n* processes despite $t<n/3$ Byzantine processes. When based on local coins, the existing algorithms converge *O(n*<sup>2.5</sup>*)* expected time. 

A recent randomized algorithm that does not contain a signature solves consensus in *O*(1) expected time under a fair scheduler. 

To solve the consensus problem deterministically and prevent the use of the common coin, researchers have assumed partial or eventual synchrony. Here, these solutions require a unique coordinator process, referred to as the leader, in order to remain non-faulty. There are both advantages and disadvantages to this technique: the advantage is if the coordinator is non-faulty and if the messages are delivered in a timely manner in an asynchronous round, then the coordinator broadcasts its proposal to all processes and this value is decided after a contest number of message delays; however a faulty coordinator can dramatically impact the algorithm performance by leveraging the power it has in a round and imposing its value to all. Non-faulty processes thus have no other choices but to decide nothing in this round. 

This protocol sees the use of a weak coordinator; a weak coordinator allows for the introduction of a new deterministic Byzantine consensus algorithm that is time optimal, resilience optimal and does not require the use of signatures. Unlike the classic, strong coordinator, the weak coordinator does not impose its value. It allows non-faulty processes to decide a value quickly, without the need of the coordinator, while helping the algorithm to terminate if non-faulty processes know that they proposed distinct values that might all be decided. In addition, the presence of a weak coordinator allows rounds to be executed optimistically without waiting for a specific message. This is unlike classic BFT algorithms that have to wait for a particular message from their coordinator and occasionally has to recover from a slow network or faulty coordinator. 

With regards to the problem of a slow of Byzantine coordinator, the weak coordinator helps agreement by contributing a value while still allowing termination in a constant number of message delays and thus is unlike the classic coordinator or the eventual leader which cannot be implemented in BAMP<sub>n,t</sub>[*t<n/3*].   

The validation of protocol was conducted similarly to that of the HoneyBadger block chain, where "Coin", the randomization algorithm from Moustefaoui et al. was used [[38]]. Using the 100 Amazon Virtual Machines located in 5 data centers on different continents, it was shown that the DBFT algorithm outperforms that of "Coin"; which is known to terminate in *O*(1) round in expectation. In addition, since Byzantine behaviors have been seen to severely affect the performance of strong coordinator-based consensus, 4 different Byzantine attacks have been tested in the validation. 

## Summary

Important characteristics of each protocol are summarized in the table below. 

| Protocol                   | Permissionless Blockchain |  Timing Assumptions   | Decentralized Control | Low Latency | Flexible Trust | Asymptotic Security |
| -------------------------- | :-----------------------: | :-------------------: | :-------------------: | :---------: | :------------: | :-----------------: |
| Hyperledger Fabric (HLF)   |                           | Partially synchronous |           ✓           |             |       ✓        |                     |
| Tendermint                 |                           | Partially synchronous |                       |      ✓      |       ✓        |          ✓          |
| Paxos                      |             ✓             | Partially synchronous |           ✓           |      ✓      |       ✓        |                     |
| Chandra-Toureg             |             ✓             | Partially synchronous |           ✓           |             |       ✓        |                     |
| Raft                       |             ✓             |  Weakly synchronous   |           ✓           |      ✓      |       ✓        |                     |
| HashGraph                  |             ✓             |     Asynchronous      |           ✓           |      ✓      |       ✓        |                     |
| SINTRA                     |             ✓             |     Asynchronous      |           ✓           |             |       ✓        |                     |
| HoneyBadgerBFT             |             ✓             |     Asynchronous      |           ✓           |      ✓      |       ✓        |          ✓          |
| Stellar Consensus Protocol |             ✓             |     Asynchronous      |           ✓           |      ✓      |       ✓        |          ✓          |
| LinBFT                     |             ✓             | Partially synchronous |           ✓           |             |       ✓        |                     |
| Algorand                   |             ✓             |      Synchronous      |           ✓           |      ✓      |       ✓        |                     |
| Thunderella                |             ✓             |      Synchronous      |           ✓           |      ✓      |       ✓        |                     |
| Avalanche                  |             ✓             |      Synchronous      |           ✓           |      ✓      |       ✓        |                     |
| PARSEC                     |             ✓             |  Weakly synchronous   |           ✓           |             |       ✓        |                     |
| Democratic BFT             |             ✓             | Partially synchronous |           ✓           |      ✓      |       ✓        |                     |



## References 

[[1]] Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC) WhitePaper, Chevalier et al., http://docs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf, Date accessed: 2018-08-30

[1]: http://docs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf
"Protocol for Asynchronous, Reliable, 
Secure and Efficient Consensus (PARSEC) 
WhitePaper, Chevalier et al." 

[[2]] Signature-Free Asynchronous Byzantine Consensus with t<n/3 and O(n2) Messages, Mostefaoui et al., https://hal.inria.fr/hal-00944019v2/document, Date accessed: 2018-08-30

[2]: https://hal.inria.fr/hal-00944019v2/document
"Signature-Free Asynchronous Byzantine Consensus 
with t<n/3 and O(n2) Messages,
Mostefaoui et al." 

[[3]] Byzantine Agreement Made Trivial, Micali.,https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Distributed%20Computation/BYZANTYNE%20AGREEMENT%20MADE%20TRIVIAL.pdf, Date accessed: 2018-08-30

[3]: https://people.csail.mit.edu/silvio/Selected%20Scientific%20Papers/Distributed%20Computation/BYZANTYNE%20AGREEMENT%20MADE%20TRIVIAL.pdf
"Byzantine Agreement Made Trivial
Micali" 

[[4]] Gossip Protocol. Wikipedia https://en.wikipedia.org/wiki/Gossip_protocol, Date accessed: 2018-09-07

[4]: https://en.wikipedia.org/wiki/Gossip_protocol
"Gossip Protocol, Wikipedia"

[[5]] Project Spotlight: Maidsafe and PARSEC Part 1, https://medium.com/@flatoutcrypto/project-spotlight-maidsafe-and-parsec-part-1-4830cec8d9e3, Date accessed: 2018-08-30

[5]: https://medium.com/@flatoutcrypto/project-spotlight-maidsafe-and-parsec-part-1-4830cec8d9e3
"Project Spotlight: Maidsafe and PARSEC Part 1" 

[[6]] The Honey Badger of BFT Protocols WhitePaper, Miller et al.  https://eprint.iacr.org/2016/199.pdf, Date accessed: 2018-08-30

[6]: https://eprint.iacr.org/2016/199.pdf
"The Honey Badger of BFT Protocols WhitePaper,
Miller  et al."

[[7]] Blockchain, cryptography and consensus, Cachin,  https://cachin.com/cc/talks/20170622-blockchain-ice.pdf, Date accessed: 2018-09-04

[7]: https://cachin.com/cc/talks/20170622-blockchain-ice.pdf
"Blockchain, cryptography and consensus 2017, Cachin"

[[8]] Comments from Medium: I don't see how it's plausible for parallel forks of the hash chain to be finalized concurrently, https://medium.com/@shelby_78386/i-dont-see-how-it-s-plausible-for-parallel-forks-of-the-hash-chain-to-be-finalized-concurrently-cb57afe9dd0a, Date accessed: 2018-09-14 

[8]: https://medium.com/@shelby_78386/i-dont-see-how-it-s-plausible-for-parallel-forks-of-the-hash-chain-to-be-finalized-concurrently-cb57afe9dd0a
"Comments from Medium: 
I don't see hope it's plausible
for parallel forks of the hash chain
to be finalized concurrently"

[[9]] High-Performance Consensus Mechanisms for Blockchains, Rusch,  http://conferences.inf.ed.ac.uk/EuroDW2018/papers/eurodw18-Rusch.pdf, Date accessed: 2018-08-30

[9]: http://conferences.inf.ed.ac.uk/EuroDW2018/papers/eurodw18-Rusch.pdf
"High-Performance Consensus Mechanisms for Blockchains,
Rusch"

[[10]] Untangling Blockchain: A Data Processing View of Blockchain Systems, Dinh et al., https://arxiv.org/pdf/1708.05665.pdf, Date accessed: 2018-08-30

[10]: https://arxiv.org/pdf/1708.05665.pdf
"Untangling Blockchain: A Data Processing View of Blockchain Systems"

[[11]] Survey of Consensus Protocols of Blockchain Applications, Sankar et al., https://ieeexplore.ieee.org/document/8014672/, Date accessed: 2018-08-30

[11]: https://ieeexplore.ieee.org/document/8014672/
"Survey of Consensus Protocols of Blockchain Applications,
4th International Conference on Advanced Computing 
and Communication Systems, Sankar et al."

[[12]] The Stellar Consensus Protocol: A Federated Model for Internet-level Consensus, Mazières, https://www.stellar.org/papers/stellar-consensus-protocol.pdf, Date accessed: 2018-08-30

[12]: https://www.stellar.org/papers/stellar-consensus-protocol.pdf
"The Stellar Consensus Protocol: 
A Federated Model for Internet- level
Consensus, Mazières"

[[13]] Architecture of the Hyperledger Blockchain Fabric, Cachin, https://www.zurich.ibm.com/dccl/papers/cachin_dccl.pdf, Date accessed: 2018-09-16

[13]: https://www.zurich.ibm.com/dccl/papers/cachin_dccl.pdf
"Architecture of the Hyperledger 
Blockchain Fabric, Cachin"

[[14]] The Ripple Protocol Consensus Algorithm, Schwartz et al.,  https://ripple.com/files/ripple_consensus_whitepaper.pdf, Date accessed: 2018-09-13

[14]: https://ripple.com/files/ripple_consensus_whitepaper.pdf
"The Ripple Protocol Consensus 
Algorithm, Schwartz et al."

[[15]] Tendermint: Consensus without Mining, Kwon,  https://tendermint.com/static/docs/tendermint.pdf, Date accessed: 2018-09-20

[15]: https://tendermint.com/static/docs/tendermint.pdf
"Tendermint: Consensus without Mining, Kwon"

[[16]] LinBFT: Linear-Communication Byzantine Fault Tolerance for Public Blockchains,  Yang, https://arxiv.org/pdf/1807.01829.pdf, Date accessed: 2018-09-20

[16]: https://arxiv.org/pdf/1807.01829.pdf
"LinBFT: Linear-Communication Byzantine 
Fault Tolerance for Public Blockchains, Yang" 

[[17]] DBFT: Efficient Byzantine Consensus with a Weak Coordinator and its Application to Consortium Blockchains, Crain et al., http://gramoli.redbellyblockchain.io/web/doc/pubs/DBFT-preprint.pdf, Date accessed: 2018-09-30

[17]: http://gramoli.redbellyblockchain.io/web/doc/pubs/DBFT-preprint.pdf
"DBFT: Efficient Byzantine Consensus 
with a Weak Coordinator and its Application 
to Consortium Blockchains, Crain et al."

[[18]] Protocol Spotlight: Avalanche Part 1, https://flatoutcrypto.com/home/avalancheprotocol, Date Accessed: 2018-09-09

[18]: https://flatoutcrypto.com/home/avalancheprotocol
"Protocol Spotlight: Avalanche Part 1"

[[19]] Breaking down the Blockchain Scalability Trilemma, Asolo, https://bitcoinist.com/breaking-down-the-scalability-trilemma/, Date accessed: 2018-10-01

[19]: https://bitcoinist.com/breaking-down-the-scalability-trilemma/
"Breaking down the Blockchain 
Scalability Trilemma, Asolo,"

[[20]] Byzantine Fault Tolerance, Demicoli, https://blog.cdemi.io/byzantine-fault-tolerance/, Date accessed: 2018-10-01

[20]: https://blog.cdemi.io/byzantine-fault-tolerance/
"Byzantine Fault Tolerance, Demicoli"

[[21]] Consensus Mechanisms, Wikipedia,  https://en.wikipedia.org/wiki/Consensus_(computer_science), Date accessed: 2018-10-01

[21]: https://en.wikipedia.org/wiki/Consensus_(computer_science)
"Consensus Mechanisms, Wikipedia"

[[22]]  Impossibility of Distributed Consensus with One Faulty Process, Fischer et al., https://groups.csail.mit.edu/tds/papers/Lynch/jacm85.pdf, Date accessed: 2018-09-30

[22]: https://groups.csail.mit.edu/tds/papers/Lynch/jacm85.pdf
"Impossibility of Distributed Consensus 
with One Faulty Process, Fischer et al."

[[23]] A brief Tour of FLP Impossibility,  https://www.the-paper-trail.org/post/2008-08-13-a-brief-tour-of-flp-impossibility/, Date accessed: 2018-09-30

[23]: https://www.the-paper-trail.org/post/2008-08-13-a-brief-tour-of-flp-impossibility/
"A brief Tour of FLP Impossibility"

[[24]] Demystifying HashGraph: Benefits and Challenges,  Jia, . https://hackernoon.com/demystifying-hashgraph-benefits-and-challenges-d605e5c0cee5, Date accessed: 2018-09-09

[24]: https://hackernoon.com/demystifying-hashgraph-benefits-and-challenges-d605e5c0cee5
"Demystifying HashGraph, Jia"

[[25]] Algorand WhitePaper, Chen and Micali, https://arxiv.org/pdf/1607.01341.pdf , Date accessed: 2018-09-13

[25]: https://arxiv.org/pdf/1607.01341.pdf
"Algorand WhitePaper 
Chen and Micali"

[[26]] Thunderella: Blockchains with Optimistic Instant Confirmation, Pass and Shi, https://eprint.iacr.org/2017/913.pdf, Date accessed: 2018-09-13

[26]: https://eprint.iacr.org/2017/913.pdf
"Thunderella WhitePaper, Pass and Shi"

[[27]] Chandra-Toueg Consensus Algorithm, Wikipedia, https://en.wikipedia.org/wiki/Chandra%E2%80%93Toueg_consensus_algorithm, Date accessed: 2018-09-13

[27]: https://en.wikipedia.org/wiki/Chandra%E2%80%93Toueg_consensus_algorithm
"Chandra-Toueg Consensus Algorithm, Wikipedia"

[[28]] Raft, Wikipedia, https://en.wikipedia.org/wiki/Raft_(computer_science), Date accessed: 2018-09-13

[28]: https://en.wikipedia.org/wiki/Raft_(computer_science)
"Raft, Wikipedia"

[[29]] Paxos, Wikipedia, https://en.wikipedia.org/wiki/Paxos_(computer_science), Date accessed: 2018-10-01

[29]: https://en.wikipedia.org/wiki/Paxos_(computer_science)
"Paxos, Wikipedia"

[[30]] The Swirlds Hashgraph consensus algorithm: Fair, fast, byzantine fault tolerance, Baird, https://www.swirlds.com/downloads/SWIRLDS-TR-2016-01.pdf, Date accessed: 2018-09-30

[30]: https://www.swirlds.com/downloads/SWIRLDS-TR-2016-01.pdf "Hashgraph WhitePaper, Baird"

[[31]] Swirlds and Sybil Attacks, Baird, http://www.swirlds.com/downloads/Swirlds-and-Sybil-Attacks.pdf, Date accessed: 2018-09-30 

[31]: http://www.swirlds.com/downloads/Swirlds-and-Sybil-Attacks.pdf "Swirlds and Sybil Attacks, Baird"

[[32]] Demystifying HashGraph: Benefits and Challenges, Jia, https://hackernoon.com/demystifying-hashgraph-benefits-and-challenges-d605e5c0cee5, Date accessed: 2018-09-30

[32]: https://hackernoon.com/demystifying-hashgraph-benefits-and-challenges-d605e5c0cee5
"Demystifying HashGraph"

[[33]] HashGraph: A WhitePaper Review, Graczyk, https://medium.com/opentoken/hashgraph-a-whitepaper-review-f7dfe2b24647, Date accessed: 2018-09-30

[33]: https://medium.com/opentoken/hashgraph-a-whitepaper-review-f7dfe2b24647
"HashGraph: A WhitePaper Review"

[[34]] Tendermint Explained- Bringing BFT-based PoS to the Public Blockchain Domain, https://blog.cosmos.network/tendermint-explained-bringing-bft-based-pos-to-the-public-blockchain-domain-f22e274a0fdb, Date accessed: 2018-09-30

[34]: https://blog.cosmos.network/tendermint-explained-bringing-bft-based-pos-to-the-public-blockchain-domain-f22e274a0fdb
"Tendermint Explained- Bringing BFT-based
PoS to the Public Blockchain Domain"

[[35]] Project Spotlight: Maidsafe and PARSEC Part 2, https://flatoutcrypto.com/home/maidsafeparsecexplanationpt2, Date accessed: 2018-09-18

[35]: https://flatoutcrypto.com/home/maidsafeparsecexplanationpt2
"Project Spotlight: Maidsafe and PARSEC Part 2"

[[36]] Red Belly Blockchain, https://www.ccn.com/tag/red-belly-blockchain/, Date accessed: 2018-10-10

[36]: https://www.ccn.com/tag/red-belly-blockchain/
"Red Belly Blockchain"

[[37]] Procotol for Asynchronous, Reliable, Secure and Efficent Consensus, https://github.com/maidsafe/parsec, Date accessed 2018-10-22

[37]: https://github.com/maidsafe/parsec
"GitHub repository: Protocol for Asynchronous,
Reliable, Secure and Efficient Consensus"

[[38]] Signature-Free Asynchronous Byzantine Consensus with $t<n/3$ and *O*(n<sup>2</sup>) Messages, https://hal.inria.fr/hal-00944019v2/document, Date accessed 2018-10-22

[38]: https://hal.inria.fr/hal-00944019v2/document
"Signature-Free Asynchronous Byzantine 
Consensus with $t<n/3$ and 
*O*(n<sup>2</sup>) Messages, 
Mostefaoui et al."

[[39]] Byzantine Fault Tolerance. Wikipedia https://en.wikipedia.org/wiki/Byzantine_fault_tolerance, Date accessed: 2018-10-22

[39]: https://en.wikipedia.org/wiki/Byzantine_fault_tolerance
"Byzantine Fault Tolerance, Wikipedia"

[[40]] Making Byzantine Fault Tolerant Systems Tolerate Byzantine Faults, Clement et al., https://www.usenix.org/legacy/event/nsdi09/tech/full_papers/clement/clement.pdf, Date accessed 2018-10-22

[40]: https://www.usenix.org/legacy/event/nsdi09/tech/full_papers/clement/clement.pdf
"Making Byzantine fault Tolerant Systems Tolerate Byzantine Faults , 
Clement et al."

[[41]] Secure and Efficent Asynchronous Broadcast Protocols, Cachin et al., https://www.shoup.net/papers/ckps.pdf, Date accessed 2018-10-22

[41]: https://www.shoup.net/papers/ckps.pdf
"Secure and Efficent Asynchronous Broadcast Protocols, 
Cachin et al."

[[42]] Asynchronous secure computations with optimal resilience, Ben-Or et al., https://dl.acm.org/citation.cfm?id=198088, Date accessed 2018-10-22

[42]: https://dl.acm.org/citation.cfm?id=198088
"Asynchronous secure computations with 
optimal resilience, Ben-Or et al." 

[[43]] Asynchronous secure computations with optimal resilience, Ben-Or et al., https://dl.acm.org/citation.cfm?id=198088, Date accessed 2018-10-22

[43]: https://dl.acm.org/citation.cfm?id=198088
"Asynchronous secure computations with 
optimal resilience, Ben-Or et al." 
