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

A network is Byzantine Fault Tolerant when it can provide service and reach a consensus despite faults or failures of the system. The processes use a protocol for consensus or atomic broadcast (a broadcast where all correct processes in a system of multiple processes receive the same set of messages in the same order; that is, the same sequence of messages [[46]]) to agree on a common sequence of operations to execute. [[20]]

The literature on distributed consensus is vast, and there are many variants of previously proposed protocols being developed for block chains. They can be largely classified along a spectrum. One extreme consists of purely computation based protocols which use proof of computation to randomly select a node which single-handedly decides the next operation. The other extreme is purely communication based protocols in which nodes have equal votes and go through multiple rounds of communication to reach consensus, Practical Byzantine Fault Tolerance (PBFT) being the prime example, which is a replication algorithm designed to be BFT.  [[10]]

For systems with _n_ nodes, of which _f_ are Byzantine, it has been shown that _no algorithm exists_ that solves the consensus problem for _f > n/3_.[[21]]

So how then does the Bitcoin protocol get away with only needing 51% honest nodes to reach consensus?

Well, strictly speaking, Bitcoin is NOT a BFT-CM because there is never absolute finality in bitcoin ledgers; there is always a
chance (however small) that someone can 51% attack the network and rewrite the entire history. Bitcoin is a probabilistic consensus, rather than deterministic.

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

A △T-synchronous network guarantees that every message sent is delivered after at most a delay of △T (where △T is a measure of real time) [[6]] Synchronous protocols come to a consensus after △T. [[5]]

#### Partial Synchrony 

Here, the network retains some form of a predefined timing structure, however it can operate without knowing said assumption of how fast nodes can exchange messages over the network. Instead of pushing out a block every x seconds, in a partially synchronous block chain would gauge the limit, with messages always being sent and received within the unknown deadline. 

Partially synchronous protocols come to a consensus in an unknown, but finite period. [[5]]

##### Unknown-△T Model 

The protocol is unable to use the delay bound as a parameter. [[6]] 

##### Eventually Synchronous  

The message delay bound △ is only guaranteed to hold after some (unknown instant, called the "Global Stabilization Time". [[6]]

#### Weak Synchrony 

Most existing Byzantine fault tolerant systems, even those called 'robust' assume some variation of *weak synchrony*, where messages are guaranteed to be delivered after a certain bound △T, but △T may be time-varying or unknown to the protocol designer. 

However, the liveness properties of weakly synchronous protocols can fail completely when the expected timing assumptions are violated (e.g., due to a malicious network adversary). In general, liveness refers to a set of properties of concurrent systems, that require a system to make progress despite the fact that its concurrently executing components may have to "take turns" in critical sections, parts of the program that cannot be simultaneously run by multiple components.[[47]] 

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


