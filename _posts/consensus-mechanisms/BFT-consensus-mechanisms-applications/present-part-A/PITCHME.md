---
marp: true
theme: default
paginate: true
footer: © Tari Labs, 2018-2021. (License : CC BY-NC-SA 4.0)
---

<style>
section {
  font-size: 1.5em;
}
</style>

# Applications of Byzantine Consensus Mechanisms

---

## Consensus

- Distributed agents (these could be computers, generals co-ordinating an attack, or sensors in a nuclear plant)
  that communicate via a network (be it digital, courier or mechanical) need to agree on facts in order to act
  as a coordinated whole.
- When all non-faulty agents agree on a given fact, then we say that the network is in consensus.
- Consensus is achieved when all non-faulty agents, agree on a prescribed fact.

---

There are a host of formal requirements which a consensus protocol may adhere to; these include:

- **Agreement:** Where all correct processes agree on the same fact
- **Weak Validity:** Where for all correct processes, the output must be the input for some correct process
- **Strong Validity:** Where if all correct processes receive the same input value, they must all output that value
- **Termination:** All processes must eventually decide on an output value

---

## Byzantine Fault Tolerance

- Several papers in the literature contextualize the problem using generals at different camps, situated outside the enemy castle, needing to decide whether or not to attack. A consensus algorithm that would fail, would perhaps see one general attack while all the others stay back, resulting in the vulnerability of first general
- One key property of a block chain system is that the nodes do not trust each other, meaning that some may behave in Byzantine manners. The consensus protocol must therefore tolerate Byzantine failures.

---

- A network is Byzantine Fault Tolerant when it can provide service and reach a consensus despite faults or failures of the system. The processes use a protocol for consensus or atomic broadcast (a broadcast where all correct processes in a system of multiple processes receive the same set of messages in the same order; that is, the same sequence of messages) to agree on a common sequence of operations to execute.
- For systems with _n_ nodes, of which _f_ are Byzantine, it has been shown that _no algorithm exists_ that solves the consensus problem for _f > n/3_.
- So how then does the Bitcoin protocol get away with only needing 51% honest nodes to reach consensus?
  Well, strictly speaking, Bitcoin is NOT a BFT-CM because there is never absolute finality in bitcoin ledgers; there is always a
  chance (however small) that someone can 51% attack the network and rewrite the entire history. Bitcoin is a probabilistic consensus, rather than deterministic.

---

## Deterministic and Non-Deterministic Protocols

- Deterministic, bounded Byzantine agreement relies on consensus being finalized for each epoch before moving to the next one ensuring that there is some safety about a consensus reference point prior to continuing. If instead you allow an unbounded number of consensus agreements within the same epoch, then there is no overall consensus reference point with which to declare finality and thus safety is compromised.
- For non-deterministic or probabilistic protocols, the probability that an honest node is undecided after _r_ rounds approaches zero as _r_ approaches infinity.
- Non-deterministic protocols which solve consensus under the purely asynchronous case potentially rely on random oracles and generally incur high message complexity overhead, as they depend on reliable broadcasting for all communication.

---

## The 'Scalability' Trilemma

**Decentralization** : a core principle on which majority of the systems are build, taking into account censorship-resistance and ensuring that everyone, without prejudice, is permitted to partake in the decentralized system.

**Scalability** : encompasses the ability of the network to process transactions. Thus, if a public block chain is deemed to be efficient, effective and usable, it should be designed to handle millions of users on the network.

**Security** : refers to the immutability of the ledger and takes into account threats of 51% attacks, Sybil attacks and DDoS attacks etc.

---

The scalability of BFT protocols considering the number of participants is highly limited and the performance of most protocols deteriorates as the number of involved replicas increases. This effect is especially problematic for BFT deployment in permissionless block chains.
The problem of BFT scalability is twofold: a high throughput as well as a large consensus group with good reconfigurability that can tolerate a high number of failures are both desirable properties in BFT protocols, but are often in direct conflict.
Several approaches have been employed to remedy these problems, e.g. threshold cryptography, creating new consensus groups for every round, or limiting the number of necessary messages to reach consensus.

---

## Permissionless Byzantine Fault Tolerant Protocols

_BFT protocols face several limitations when utilized in permissionless block chains. They do not scale well with the number of participants, resulting in performance deterioration for the targeted network sizes. In addition, they are not well established in this setting, thus they are prone to security issues, e.g. Sybil attacks. Currently, there are approaches that attempt to circumvent or solve this problem._

---

### Paxos

- The Paxos family of protocols includes a spectrum of trade-offs between the number of processors, number of message delays before learning the agreed value, the activity level of individual participants, number of messages sent, and types of failures.
- Although the FLP theorem states that there is no deterministic fault-tolerant consensus protocol that can guarantee progress in an asynchronous network, Paxos guarantees safety (consistency), and the conditions that could prevent it from making progress are difficult to provoke.
- Paxos achieves consensus as long as there are f failures, where f < (n-1)/2. These failures cannot be Byzantine (otherwise the BFT proof would be violated). Thus it is assumed that messages are never corrupted, and that nodes do not collude to subvert the system.
- Paxos proceeds through a set of negotiation rounds, with one node having 'Leadership' status. Progress will stall if the leader becomes unreliable, until a new leader is elected, or if suddenly an old leader comes back online and a dispute between two leader nodes arises.

---

### Chandra-Toueg

- The Chandra–Toueg consensus algorithm was published by Tushar Deepak Chandra and Sam Toueg in 1996.
- It relies on a special node that acts as a failure detector. In essence, it pings other nodes to make sure they're still responsive.
- This implies that the detector stays online and that the detector must continuously be made aware when new nodes join the network.
- The algorithm itself is similar to the Paxos algorithm, which also relies on failure detectors and as such requires f<n/2, where n is the total number of processes.

---

### Raft

- Raft is a consensus algorithm designed as an alternative to Paxos
- It was meant to be more understandable than Paxos by means of separation of logic, but it is also formally proven safe and offers some additional features.
- Raft achieves consensus via an elected leader. Each follower has a timeout in which it expects the heartbeat from the leader. It is thus a synchronous protocol. If the leader fails, an election is held to find a new leader. This entails nodes nominating themselves on a first-come, first-served basis. Hung votes require the election to be scrapped and restarted. This suggests that a high degree of cooperation is required by nodes and that malicious nodes could easily collude to disrupt a leader and then prevent a new leader from being elected. Raft is a simple algorithm but is clearly unsuitable for consensus in cryptocurrency applications.

---

- While Paxos and Raft and many other well-known protocols tolerate crash faults, Byzantine fault tolerant protocols beginning with PBFT, tolerate even arbitrary corrupted nodes. Many subsequent protocols offer improved performance, often through optimistic execution that provides excellent performance when there are no faults, clients do not contend much, and the network is well behaved, and at least some progress otherwise.
- In general, BFT systems are evaluated in deployment scenarios where latency and CPU are the bottleneck, thus the most effective protocols reduce the number of rounds and minimize expensive cryptographic operations.

---

### HashGraph

- The Hashgraph consensus algorithm, was released in 2016.
- It claims Byzantine fault tolerance under complete asynchrony assumptions, no leaders, no round robin, no proof-of-work, eventual consensus with probability one, and high speed in the absence of faults.
- It is based on the gossip protocol, which is a fairly efficient distribution strategy that entails nodes randomly sharing information with each other, similar to how human beings gossip with each other.
- Nodes jointly build a hash graph reflecting all of the gossip events.
- This allows Byzantine agreement to be achieved through virtual voting.
- Alice does not send Bob a vote over the Internet. Instead, Bob calculates what vote Alice would have sent, based on his knowledge of what Alice knows.
- HashGraph uses digital signatures to prevent undetectable changes to transmitted messages.
- HashGraph does not violate the FLP theorem, since it is non-deterministic.

---

The Hash graph has some similarities to a block chain.
_"The HashGraph consensus algorithm is equivalent to a block chain in which the 'chain' is constantly branching, without any pruning, where no blocks are ever stale, and where each miner is allowed to mine many new blocks per second, without proof-of-work"_

Because each node keeps track of the hash graph, there is no need to have voting rounds in HashGraph; each node already knows what all of its peers will vote for and thus consensus is reached purely by analyzing the graph.

---

#### The Gossip Protocol

The gossip protocol works like this:

Alice selects a random peer node, say Bob, and sends him everything she knows. She then selects another random node and repeats the process indefinitely.

Bob, on receiving Alice's information, marks this as a gossip event and fills in any gaps in his knowledge from Alice's information. Once done, he continues gossiping with his updated information.

---

The basic idea behind the Gossip Protocol is the following:

- A node wants to share some information to the other nodes in the network.
- Then periodically it randomly selects a node from the set of nodes and exchanges the information.
- The node that receives the information and then randomly selects a node from the set of nodes and exchanges the information, and so on.
- The information is periodically sent to N targets, where N is the fanout.
- The cycle is the number of rounds to spread the information. The fanout is the number of nodes a node gossips with in each cycle.
- With a fanout=1, O(LogN) cycles are necessary for the update to reach all the nodes. In this way, information spreads throughout the network in an exponential fashion.

HashGraph introduces a few important concepts that are used repeatedly in later BFT consensus algorithms: famous witnesses, and strongly seeing.

---

#### Ancestors

If an event (_x1_) comes before another event (_x2_), and they are connected by a line; the older event is an ancestor of that event.

If both events were created by the _same node_, then _x1_ is a _self-ancestor_ of _x2_.

**Note**: The gossip protocol defines an event as being a (self-)ancestor of itself!

---

#### Seeing

If an event _x1_ is an ancestor of _x2_, then we say that _x1_ **sees** _x2_ as long as the node is not aware of any forks from _x2_.

So in the absence of forks, all events will _see_ all of their ancestors.

```text
    +-----> y
    |
x +--+
    |
    +-----> z
```

---

In the example above, _x_ is an ancestor to both _y_ and _z_. However, because there is no ancestor relationship between _y_ and _z_, the seeing condition fails, and so _y_ cannot see _x_, and _z_ cannot see _x_.

It may be the case that it takes time before nodes in the protocol detect the fork. For instance Bob may create _z_ and _y_; but share _z_ with Alice and _y_ with Charlie. Both Alice and Charlie will eventually learn about the deception, but until that point, Alice will believe that _y_ sees _x_, and Charlie will believe that _z_ sees _x_.

This is where the concept of _strongly seeing_ comes in.

---

#### Strongly seeing

If a node examines its hash graph and notices that an event _z_ _sees_ an event _x_, and not only that, but it can draw an ancestor relationship (usually via multiple routes) through a super-majority of peer nodes, and that a different event from each node also sees _x_; then it is said that according to this node, that _z_ _strongly sees_ _x_.

---

#### The Construct of Gossiping

The main consensus algorithm loop consists of every node (Alice), selecting a random peer node (Bob) and sharing their graph history. Now Alice and Bob have the same graph history.

Alice and Bob both create a new event with the new knowledge they have just learnt from their peer.

Alice repeats this process continuously.

---

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
```

---

- Here we have the Swirlds HashGraph consensus algorithm.
- Each member runs this in parallel.
- Each sync brings in new events, which are then added to the hash graph. All known events are then divided into rounds.
- Then the first events in each round are decided as being famous or not (through purely local Byzantine agreement with virtual voting).
- Then the total order is found on those events for which enough information is available. If two members independently assign a position in history to an event, they are guaranteed to assign the same position, and guaranteed to never change it, even as more information comes in. Furthermore, each event is eventually assigned such a position, with probability one.

---

```text
procedure divideRounds
   for each event x
     r ← max round of parents of x ( or 1 if none exist )
     if x can strongly see more than 2/3*n round r witnesses
       x.round ← r + 1
     else
       x.round ← r
     x.witness ← ( x has no self parent ) || ( x.round > x.selfParent.round )
```

---

What was just shown is deemed the divideRounds procedure. As soon as an event x is known, it is assigned a round number x round, and the boolean value x witness is calculated, indicating whether it is the first event that a member created in that round.
​

```text
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

---

This is the decideFame procedure. For each witness event (i.e., an event x where x.witness is true), decide whether it is famous (i.e., assign a boolean to x.famous). This decision is done by a Byzantine agreement protocol based on virtual voting. Each member runs it locally, on their own copy of the hashgraph, with no additional communication. It treats the events in the hashgraph as if they were sending votes to each other, though the calculation is purely local to a member’s computer. The member assigns votes to the witnesses of each round, for several rounds, until more than 2/3 of the population agrees.

---

#### Criticisms

- The HashGraph protocol is patented and is not open source.
- In addition, the HashGraph white paper assumes that _n_, the number of nodes in the network, is constant. In practice, _n_ can increase, but performance likely degrades badly as _n_ becomes large.
- HashGraph is not as "fair" as claimed in their paper, with at least one attack being proposed.
