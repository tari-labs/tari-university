Scalable, Decentralised, Secure. Choose Two.

# Consensus mechanism comparison

Scope: We're looking at Byzantine Fault Tolerant consensus mechanisms.

Strictly speaking, Bitcoin is *NOT* a BFT-CM because there is _never_ finality in nitcoin ledgers; there is _always_ a 
chance (however small) that someone can 51% attack the network and rewrite the entire history. This is why Bitcoin
only needs 51% honest nodes to reach "consensus". It's a _probabilistic_ consensus, rather than deterministic.

So in this paper, we're going to look at some proposals for conventional BFT-CM that requires 67% honest nodes to
guarantee consensus. We will examine 

* the assumptions that each proposal makes (and how strong those assumptions are in
real-world adversarial environments), 
* the theoretical efficiency of each proposal, 
* whether there are any reference implementations,
* the degree of permissionless that is tolerated (e.g. fully permissioned nodes vs. digital signing of messages vs. totally open), and 
* whether these proposals could be used in Tari for a maintaining distributed digital asset state.

## Terminology

There are many confusing terms in the art.

### Consensus

Distributed agents (these could be computers, generals co-ordinating an attack, or sensors in a nuclear plant) 
that communicate via a network (be it digital, courier or mechanical) need to agree on facts in order to act
as a coordinated whole.

When *all* the (non-faulty) agents _agree on a given fact_, then we say that the network is in consensus.

Formal requirements for a consensus protocol may include [[WPC]]:

* **Agreement**: All correct processes **must** agree on the same value.
* **Weak validity**: For each correct process, its output **must** be the input of _some_ correct process.
* **Strong validity**: If all correct processes receive the same input value, then they **must** all output that value.
* **Termination**: All processes **must** _eventually_ decide on an output value.

#### Some interesting proofs

For systems with _n_ processors, of which _f_ are Byzantine, it has been shown that _no algorithm exists_ 
that solves the consensus problem for _f > n/3_.[[ATT04]]



### Binary consensus

A special case of the consensus problem called binary consensus restricts the input and hence the output domain to a 
single binary digit {0,1}. When the input domain is large relative to the number of processes, for instance an input 
set of all the natural numbers, it can be shown that consensus is impossible in a synchronous message passing model [[WPC]].

### Synchrony

Many proposals make one or other assumption about _synchrony_.

_Synchronous_ protocols make the assumption that a message will be delivered within a certain time, ΔT.
                                                                                                   
In synchronous systems it is assumed that all communications proceed in rounds. In one round a process may send all 
the messages it requires while receiving all messages from other processes. In this manner no message from one round 
may influence any messages sent within the same round [[WPC]].

In a  _partially synchronous_ system, the system can alternate between good and bad periods of synchrony [[WPC]].

Asynchronous systems make no assumptions about when or whether messages arrive at their destination. Interestingly,
it has been proven that in a fully asynchronous, deterministic system _there is no consensus solution that can guarantee consensus
if even one node fails_ (the FLP theorem). This result does not state that consensus can never be reached: 
merely that under the model's assumptions, no algorithm can _always_ reach consensus in bounded time. 
In practice it is highly unlikely to occur. [[WPC]]

### Determistic protocols

For deterministic protocols, all honest nodes reach consensus by round _r_ for some a priori known constant _r_. 
For non-deterministic or probabilistic protocols, the probability that an honest node is undecided after _r_ rounds 
approaches zero as r approaches infinity. [[HN1]]

For synchronous protocols, roughly speaking, messages are guaranteed to be delivered after a certain bound ∆, but the asynchronous protocols don’t have such a bound.

## A brief survey of BFT-CMs

Many peer-to-peer online Real-time strategy games use a modified Lockstep protocol as a consensus protocol in order 
to manage game state between players in a game. Each game action results in a game state delta broadcast to all other 
players in the game along with a hash of the total game state. Each player validates the change by applying the delta 
to their own game state and comparing the game state hashes. If the hashes do not agree then a vote is cast, and those 
players whose game state is in the minority are disconnected and removed from the game (known as a desync.)[[WPC]]

## Paxos

The Paxos family of protocols includes a spectrum of trade-offs between the number of processors, number of message 
delays before learning the agreed value, the activity level of individual participants, number of messages sent, 
and types of failures. Although the FLP theorem says that there's no deterministic fault-tolerant consensus protocol 
that can guarantee progress in an asynchronous network, Paxos guarantees safety (consistency), and the conditions that 
could prevent it from making progress are difficult to provoke [[WPP]].

Paxos achieves consensus as long as there are f failures, where _f < (n-1)/2_. These failures cannot be Byzantine (otherwise
the BFT proof would be violated). Thus it is assumed that messages are never corrupted, and that nodes do not collude to
subvert the system.

Paxos proceeds through a set of negotiation rounds, with one node having 'Leadership' status. Progress will stall if the
leader becomes unreliable, until a new leader is elected, or if suddenly an old leader comes back online and a dispute
between two leader nodes arises.

### Raft

Raft is a consensus algorithm designed as an alternative to Paxos. It was meant to be more understandable than Paxos
 by means of separation of logic, but it is also formally proven safe and offers some additional features [[WPR]]. 

Raft achieves consensus via an elected leader. 

Each follower has a timeout in which it expects the heartbeat from the leader. It is thus a synchronous protocol.

If the leader fails, an election is held to find a new leader. This entails nodes nominating themselves on a first-come, 
first-served basis. Hung votes require the election to be scrapped and restarted.

This suggests that a high degree of cooperation is required by nodes and that malicious nodes could easily collude to 
disrupt a leader and then prevent a new leader from being elected.

Raft is a simple algorithm but is clearly unsuitable for consensus in cryptocurrency applications.

## Chandra-Toueg

The Chandra–Toueg consensus algorithm, published by Tushar Deepak Chandra and Sam Toueg in 1996, relies on a special
 node that acts as a _failure detector_. In essence, it pings other nodes to make sure they're still responsive.
 
 This implies that
 * The detector stays online.
 * The detector must somehow be made aware when new nodes join the network.
  
The algorithm itself is similar to the Paxos algorithm, which also relies on failure detectors and as such requires
_f < n/2_, where n is the total number of processes.


## HashGraph

The Hashgraph consensus algorithm [[SHG]], was released in 2016.  It claims Byzantine fault tolerance under complete 
asynchrony assumptions, no leaders, no round robin, no proof-of-work, and eventual consensus with probability one. 
And high speed in the absence of faults. 

It is based on the gossip protocol, which is a fairly efficient distribution strategy that entails nodes randomly 
sharing information with each other, similar to how human beings gossip with each other.
 
Nodes jointly build a hash graph reflecting all of the gossip events. This allows Byzantine agreement to be achieved 
through virtual voting. Alice does not send Bob a vote over the Internet. Instead, Bob calculates what vote Alice would 
have sent, based on his knowledge of what Alice knows. 

HashGraph uses digital signatures to prevent undetectable changes to transmitted messages.

HashGraph does not violate the FLP theorem, since it is _non-deterministic_.

The Hash graph has some similarities to a blockchain. To quote the white paper: "The hashgraph consensus algorithm is
equivalent to a block chain in which the 'chain' is constantly branching, without any pruning, where no blocks are ever 
stale, and where each miner is allowed to mine many new blocks per second, without proof-of-work" [[SHG]].

Because each node keeps track of the hash graph, there's no need to have voting rounds in HashGraph; each node already 
knows what all of its peers will vote for and thus consensus is reached purely by analyzing the graph. 


### The HashGraph Algorithm: The Gossip protocol

The gossip protocol works like this:

Alice selects a random peer node, say Bob, and sends him _everything she knows_. She then selects another random node
and repeats the process indefinitely.

Bob, on receiving Alice's information, marks this as a gossip event and fills in any gaps in his knowledge from Alice's
information. Once done, he continues gossiping with his updated informaiton.

In this way, information spreads throughout the network in an exponential fashion.

![Figure 1 - HashGraph](../assets/gossip.png 'The history of any gossip protocol can be represented by a directed graph, 
where each member is a column of vertices. Each transfer event is shown as a new vertex with two edges linking the 
immediately-preceding gossip events.')

The gossip history can be represented as a directed graph, as in Figure 1. 

HashGraph introduces a few important concepts that are used repeatedly in later BFT consensus algorithms: famous 
witnesses, and strongly seeing.

### Ancestors

If an event (_x1_) comes before another event (_x2_), and they are connected by a line; the older event is an _ancestor_ of that event.

If both events were created by the _same node_, then _x1_ is a _self-ancestor_ of _x2_. 

**Note**: The gossip protocol defines an event as being a (self-)ancestor of itself!

### Seeing

If an event _x1_ is an ancestor of _x2_, then we say that _x1_ **sees** _x2_ as long as the node is not aware of any
forks from _x2_.

So in the absence of forks, all events will _see_ all of their ancestors.


```text
     +-----> y
     |
x +--+
     |
     |
     +-----> z
```

In the example above, x is an ancestor to both y and z. However, because there is no ancestor relationship between 
y and z, the _seeing_ condition fails, and so y cannot see x, and z cannot see x.

It may be the case that it takes time before nodes in the protocol detect the fork. For instance Bob may create z and y;
 but share z with Alice and y with Charlie. Both Alice and Charlie will eventually learn about the deception, but until
 that point, Alice will believe that y sees x, and Charlie will believe that z sees x.
 
 This is where the concept of _strongly seeing_ comes in. 


### Strongly seeing

If a node examines its hash graph and notices that an event z _sees_ an event x, and not only that, but it can draw
an ancestor relationship (usually via multiple routes) through a super-majority of peer nodes, and that a different
event from each node also sees x; then we say that according to this node, that z _strongly sees_ x.

The following example comes from [[HGP]]:

![Strongly seeing example](../assets/strongly-seeing.png)

## Consensus 

### Gossiping

The main consensus algorithm loop consists of every node (Alice), selecting a random peer node (Bob) and sharing their 
graph history. Now Alice and Bob have the same graph history.

Alice and Bob both create a new event with the new knowledge they've just learnt from their peer.

Alice repeats this process continuously.

### Internal consensus

After a sync, a node will determine the order for as many events as possible, using three procedures.
The algorithm uses constant _n_ (the number of nodes) and a small constant value _c_ > 2.  

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

## Drawbacks

* HashGraph is patented [[HGP]].
* The HashGraph white paper assumes that _n_, the number of nodes in the network, is constant. In practice, _n_ can increase,
  but performance likely degrades badly as _n_ becomes large [[HN1]].
* HashGraph is probably not "fair" as claimed in their paper, with at least one attack being proposed [[GRA18]]

Baird tries to address some of these drawbacks in a follow-up paper [[BAI16]]

# Algorand

Released in May 2017 [[CHE17]], Algorand is a synchronous BFT consensus mechanism, meaning that blocks
 get added at a minimum rate. It claims to work in a permissionless environment, but the synchrony aspect of the design
 makes Algorand unsuitable for a high-throughput network like Tari.
 
 Algorand does however, introduce the concept of a _concrete coin_. Most of these BFT algorithms need some sort of 
 randomness oracle, but all nodes need to see the same value if the oracle is consulted. This had previously been
 achieved through a _common coin_ idea, which is a complicated piece of mathematical 'magic'.
 
 Concrete coin uses a much simpler approach; but only returns a binary value.

# Thunderella

Thunderella takes an interesting approach to reaching consensus. It uses an asynchronous strategy, with
a synchronous strategy as a fall back  'in case something goes wrong' [[TWP]]. The idea is that you get the best of both
worlds: robustness and speed.

It can be applied in permissionless networks using proof-of-work. Netowrk robustness and "instant confirmations" requires
 both 75% of the network to be honest, and the presence of a leader node.
 
# HoneyBadgerBFT

Honeybadger BFT styles itself as the first practical asynchronous BFT consensus algorithm. It was released in November 2016.
It was designed with cryptocurrencies in mind, where bandwidth is considered scarce, but CPU power is abundant. Therefore
HoneyBadgerBFT introduces the use of public-private key encryption to improve the efficiency of the consensus-reaching
communication. 

In its threshold encryption scheme, any one party can encrypt a message using a master public key, and it requires f+1 
correct nodes to compute and reveal decryption shares for a ciphertext before the plaintext can be recovered.
 
# PARSEC

Parsec was developed by MaidSafe and the whitepaper was released on 20 June 2018 [[MSP]]. Parsec builds on the ideas
of Hashgraph, HoneybadgerBFT and others and claims to have produced a consensus algorithm that

* is asynchronous
* has no leaders or round-robin mechanisms
* reaches consensus with probability one.

The reference implementation of PARSEC, written in Rust, was released a few weeks after the whitepaper [[PSC]].

Similarly to HashGraph, Parsec reduces the problem to a _binary_ Byzantine problem and solves that through a set of
voting rounds on a gossip graph.

PARSEC is essentially Hashgraph with the added capability for higher churn of nodes and not patented. 

When a node receives a gossip request, it 
creates a new event and sends a response back (in HashGraph, the response was optional). Each gossip event contains [[FOC18]]:

1. The data being transmitted
1. The self-parent (the hash of another gossip event created by the same node)
1. The other-parent (a hash of another gossip event created by a different node)
1. The _Cause_ for creation which can either be a Request for information, a Response to another node’s request, or an 
   _Observation_. An observation is when a node creates a gossip event to record an observation that the node made themselves.
1. Creator ID (public key)
1. Signature – signing the above information.

The self-parent and other-parent prevents tampering because they are signed and related to other gossip events [[FOC18]].
As with HashGraph, it's difficult for adversaries to interfere with the consensus algorithm because all voting is _virtual_
and done without sharing details of votes cast; each node figures out what other nodes would have voted based on their
copy of the gossip graph.

PARSEC is relatively efficient, requiring O(n log n) messages and O(log N) stages to reach consensus [[MSP]].

PARSEC also uses the concept of a "concrete coin", from Algorand that is used to break ties; particularly in cases where an adversary
is carefully managing communication between nodes in order to maintain a deadlock on votes.

## Voting Algorithm

First nodes, try and converge on a 'true' result for a set of results. If they can't, they move onto step 2, which is
trying to converge on a 'false' result. If consensus still cannot be reached, a coin flip is made and we go back to step 1
in another voting round.

## Drawbacks

Similar to HashGraph. It's not clear that PARSEC will scale well as _n_ grows to millions of nodes, but at least PARSEC is
open source and a reference implementation has already been released in Rust.

# Avalanche

Avalanche was released by an anonymous group called 'Team Rocket' in May 2018. [[FOC18b]]. IT is a synchronous protocol, but
Team Rocket claim that the synchrony requirements are "good enough" for real-world applications.

It's so early in this space that no-one has really critically compared Avalanche to say, PARSEC or any of the other new
protocols presented here. A request for comment on the SAFE netowrk forum yielded the opinion (unsurprisingly) that [PARSEC
was marginally better than Avalanche](https://safenetforum.org/t/let-s-compare-parsec-against-avalanche-the-self-claimed-revolutionary-consensus-breakthrough-since-bitcoin/23677/2).



# References
1. Consensus mechanisms. Wikipedia. [[WPC]]
1. Attiya, Hagit (2004). Distributed Computing 2nd Ed. Wiley. pp. 101–103. [[ATT04]]
1. Paxos (Computer Science). Wikipedia. [[WPP]]
1. Raft (Computer Science). Wikipedia. [[WPR]]
1. Protocol for Asynchronous, Reliable, Secure and Efficient Consensus (PARSEC). Chevalier _et. al._, June 20, 2018 [[MSP]]
1. Project Spotlight: Maidsafe and PARSEC Part 1. Flatout Crypto. Jun 24, 2018. [[FCP1]]
1. The Swirlds Hashgraph consensus algorithm: Fair, fast, byzantine fault tolerance, Leemon Baird, May 31, 2016. SWIRLDS-TR-2016-01 [[SHG]]
1. Swirlds Intellectual Property. https://www.swirlds.com/ip/ [[HGP]]
1. Hashgraph: A Whitepaper Review, Michael Graczyk, https://medium.com/opentoken/hashgraph-a-whitepaper-review-f7dfe2b24647 [[GRA18]]
1. Swirlds and Sybil Attacks, L. Baird, Jun 6, 2016, http://www.swirlds.com/downloads/Swirlds-and-Sybil-Attacks.pdf [[BAI16]]
1. Demystifying Hashgraph: Benefits and Challenges, Yaoqi Jia, Nov 8, 2017. https://hackernoon.com/demystifying-hashgraph-benefits-and-challenges-d605e5c0cee5
1. The Honey Badger of BFT Protocols, A. Miller _et. al._, https://eprint.iacr.org/2016/199.pdf [[HBP]]
1. ALGORAND, J. Chen and S. Micali, https://arxiv.org/pdf/1607.01341.pdf [[CHE17]]
1. Thunderella: Blockchains with Optimistic Instant Confirmation, R. Pass and E. Shi, 31 MArch 2018. https://eprint.iacr.org/2017/913.pdf
1. PARSEC Source Code Repository. https://github.com/maidsafe/parsec [[PSC]]
1. Project Spotlight: Maidsafe and PARSEC Part 2, FlatOutCrypto, 25 Jun 2018. https://flatoutcrypto.com/home/maidsafeparsecexplanationpt2 [[FOC18]]
1. Protocol Spotlight: Avalanche Part 1, 29 Jun 2018. https://flatoutcrypto.com/home/avalancheprotocol [[FOC18b]]

[WPC]: https://en.wikipedia.org/wiki/Consensus_(computer_science) 'Wikipedia - Consensus mechanisms'
[WPP]: https://en.wikipedia.org/wiki/Paxos_(computer_science) 'Wikipedia - Paxos'
[WPR]: https://en.wikipedia.org/wiki/Raft_(computer_science) 'Wikipedia - Raft'
[ATT04]: https://www.amazon.com/Distributed-Computing-Fundamentals-Simulations-Advanced/dp/0471453242 'Attiya, Hagit (2004). Distributed Computing 2nd Ed. Wiley. pp. 101–103. ISBN 978-0-471-45324-6.'
[FCP1]: https://flatoutcrypto.com/home/maidsafeparsecexplanation 'Project Spotlight: Maidsafe and PARSEC Part 1'
[MSP]: ocs.maidsafe.net/Whitepapers/pdf/PARSEC.pdf 'PARSEC WhitePaper'
[SHG]: https://www.swirlds.com/downloads/SWIRLDS-TR-2016-01.pdf 'Hashgraph Whitepaper'
[HGP]: https://www.swirlds.com/ip/ 'Swirlds Intellectual Property'
[GRA18]: https://medium.com/opentoken/hashgraph-a-whitepaper-review-f7dfe2b24647 'Hashgraph: A Whitepaper Review'
[BAI16]: http://www.swirlds.com/downloads/Swirlds-and-Sybil-Attacks.pdf 'Swirlds and Sybil Attacks'
[HN1]: https://hackernoon.com/demystifying-hashgraph-benefits-and-challenges-d605e5c0cee5 'Demystifying HashGraph'
[HBP]: https://eprint.iacr.org/2016/199.pdf 'HoneyBadgerBFT Whitepper'
[CHE17]: https://arxiv.org/pdf/1607.01341.pdf 'Algorand whitepaper'
[TWP]: https://eprint.iacr.org/2017/913.pdf ' Thunderella Whitepaper'
[PSC]: https://github.com/maidsafe/parsec 'PARSEC Github repository'
[FOC18]: https://flatoutcrypto.com/home/maidsafeparsecexplanationpt2 'Project Spotlight: Maidsafe and PARSEC Part 2 - FlatOutCrypto'
[FOC18b]: https://flatoutcrypto.com/home/avalancheprotocol 'Project Spotlight: Avalanche Part 1 - FlatOutCrypto'