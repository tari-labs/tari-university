# The I2P Network - An introduction and Comparison to Tor and VPNs.

- [Introduction](#introduction)

- [An Introduction to the i2P network](#Introduction)

  - [What is I2P](#what-is-I2P)
  - [How it Works](#scalability-and-fault-tolerance)
  - [Understanding Routers](#distributed-data-storage)



## Introduction

I2P (Invisible Internet Project), Tor and VPNs (Virtual Private Networks) are well known anonymity networks used by millions of users across the world. Most people use them as ways to securely transfer data over the internet with their identities concealed. These networks have very similar characteristics but are also very big differences and hence work in very specific ways.

In this report we'll examine what the I2P network is, the paradigms of how it works, its security infrastructure and its usefulness in the blockchain domain.


## What is I2P
I2P (known as the Invisible Internet Project and founded in 2003) is a network layer that runs on a distributed network of computers on a global network infrastructure. This network layer provides a set of functions that runs on each computer and provides encrypted, one-way connections to and from other computers within the network. These functions are wrapped in a *"router"* that is installed during setup and configuration.


## How Does It Work
The first concept to understand about I2P is that it's primary an enclosed network that runs within the Internet infrastructure (reffered to as the Clearnet in this paradigm). Unlike VPN's and the Tor network, which are built to communicate with the Internet anonymously, I2P works as a decentralised network of that operates within the Internet - i.e. an Internet within the internet. Interaction is done on a peer to peer level and there is no centralised authority that handles the network or keeps track of the active peers. Tor and VPNs, on the other hand have centralised authorities where the message/data and network is managed. Since I2P works within it's own network, it is primarily made up of anonymous and hidden sites (called eepsites) that exist only within the network and are only accessible to people using I2P. These can be easily created using an **I2PTunnel** service that uses a standard web server. Another concept of note is I2P is not inherently an "outproxy" network i.e. it's not intended for accessing the internet. This is because the client you send a message to is the cryptographic identifier, not some IP address, so the message must be addressed to someone running I2P. Browsing the internet is however possible through opening an outproxy that allows creating an anonymous internet connection. (ref https://blokt.com/guides/what-is-i2p-vs-tor-browser#How_does_I2P_work) ref: https://geti2p.net/en/docs/api/i2ptunnel


#### The Infrastructure
1. **Routing Infrastructure & Anonymity:** To protect a users' anonymity within the network, I2P works by installing an I2P routing service within a user's device. This router creates temporary, encrypted, one way connections with other I2P routers on other computers. These one-way connections are comprised of an *Outbound Tunnel* and an *Inbound Tunnel* that is contained within each device. When communication is occurring, data leaves the user's devices via the outbound tunnels and is received on other devices through the device's inbound tunnels.
Therefore, a single round-trip request message and its response between two parties needs four tunnels. (reference https://censorbib.nymity.ch/pdf/Hoang2018a.pdf).
Messages that leave one device do not travel directly to the inbound tunnel of the router intended. Instead, the outbound gateway router queries a distributed network database  to get the address of the inbound router. The first time a client wants to contact another client, they make a query against the fully distributed "network database" - comprised of a custom structured distributed hash table (DHT) based off the Kademlia algorithm. This is done to find the other client's inbound tunnels efficiently, but subsequent messages between them usually includes that data so no further network database lookups are required. This distributed network database means no one knows the other's address. ref: https://geti2p.net/en/about/intro

*Diagram of Inbound and Outbound Tunnels.*

2. **Networking & Network Database:** Messages that travel through the network are protected using a technique known as *Onion Routing*.  A tunnel is a directed path through an explicitly selected list of routers. Layered encryption is used, so each of the routers can only decrypt a single layer. The decrypted information contains the IP of the next router, along with the encrypted information to be forwarded. Each tunnel has a starting point (the first router, also known as "gateway") and an end point. Messages can be sent only in one way. To send messages back, another tunnel is required. [[2]]

//  Image Of Network


3. **Encryption:**
The way it all works is the message creator (client in the network) explicitly defines the path that messages will be sent out (the outbound tunnel), and the message recipient explicitly defines the path that messages will be received on (the inbound tunnel).


## Understanding Routing - *Onion Routing*

Arbitrary data may be stored and replicated by a subset of nodes for later retrieval. Data is hashed using a
consistent hashing function (such as SHA256) to produce a key for the data. That data is propagated and
eventually stored on the node or nodes whose node IDs are "closer" to the key for that data for some distance
function.

Partitioned data storage has limited usefulness to a typical blockchain, as each full node is required to keep a copy
of all transactions and blocks for verification.


## Message Types

Arbitrary data may be stored and replicated by a subset of nodes for later retrieval. Data is hashed using a
consistent hashing function (such as SHA256) to produce a key for the data. That data is propagated and
eventually stored on the node or nodes whose node IDs are "closer" to the key for that data for some distance
function.

Partitioned data storage has limited usefulness to a typical blockchain, as each full node is required to keep a copy
of all transactions and blocks for verification.

## Threat Model, Security and Vulnerability Attacks



## Comparisons to Tor & VPNs




## Usage within the Blockchain Domain




## Conclusions

The following graph is replicated and simplified from [[8]]. Degree is the number of neighbors with which a node must maintain in contact.

| Parameter                              | CAN                                     | CHORD                   | Kademlia                                                  | Koord                                    | Pastry                             | Tapestry                  | Viceroy                         |
| -------------------------------------- | --------------------------------------- | ----------------------- | --------------------------------------------------------- | ---------------------------------------- | ---------------------------------- | ------------------------- | ------------------------------- |
| Foundation                             | d-dimensional torus                     | Circular space          | XOR metric                                                | de Bruijn graph                          | Plaxton-style mesh                 | Plaxton-style mesh        | Butterfly network               |
| Routing function                       | Map key-value pairs to coordinate space | Matching key to node ID | Matching key to node ID                                   | Matching key to node ID                  | Matching key and prefix in node ID | Suffix matching           | Levels of tree, vicinity search |
| Routing performance (network size $n$) | $O(dn^{(2/d)})$                         | $O(log(n))$             | $O(log(n)) + c$ $c$ is small                              | Between $O(log(log(n)))$ and $O(log(n))$ | $O(log(n))$                        | $O(log(n))$               | $O(log(n))$                     |
| Degree                                 | $2d$                                    | $O(log(n))$             | $O(log(n))$                                               | Between constant to $log(n)$             | $O(2log(n))$                       | $O(log(n))$               | Constant                        |
| Join/Leaves                            | $2d$                                    | $log(n)^2$              | $O(log(n)) + c$ $c$ is small                              | $O(log(n))$                              | $O(log(n))$                        | $O(log(n))$               | $O(log(n))$                     |
| Implementations                        | \-\-                                    | OpenChord, OverSIM      | Ethereum [3], Mainline DHT (BitTorrent), I2P, Kad Network | \-\-                                     | FreePastry                         | OceanStore, Mnemosyne [4] | \-\-                            |

The popularity of Kademlia over other DHTs is likely due to its relative simplicity and performance. The rest of this section dives deeper into Kademlia.

### Kademlia

Kademlia is designed to be an efficient means for storing and finding content in a distributed peer-to-peer (P2P) network.
It has a number of core features that are not simultaneously offered by other DHTs [[2]], such as:

- The number of messages necessary for nodes to learn about each other, is minimized.
- Nodes have enough information to route traffic through low-latency paths.
- Parallel and asynchronous queries are made to avoid timeout delays from failed nodes.
- The node existence algorithm resists certain basic distributed denial-of-service (DDoS) attacks.

#### NodeID

A node selects an $n$-bit ID, which is opaque to other nodes on the network. The network design
relies on node IDs being uniformly distributed by some random procedure. A node's position is
determined by the shortest unique prefix of its ID, which forms a tree structure with node IDs
as leaves [[2]]. This ID should be reused when the node rejoins the network. The following figure shows a binary tree structure in a three-bit key space:

<div class="mermaid">
graph TD
AR[ ] --- |0| A0( )
AR --- |1| A1[ ]
A0 --- |0| A00[ ]
A0 --- |1| A01[ ]
A00 --- |0| N000[Node 000]
A00 --- |1| N001[Node 001]
A01 --- |0| N010[Node 010]
A01 --- |1| N011[Node 011]
A1 --- |0| A10[ ]
A1 --- |1| A11[ ]
A10 --- |0| A100[Node 100]
A10 --- |1| A101[Node 101]
A11 --- |0| A110[Node 110]
A11 --- |1| A111[Node 111]
</div>
The bit length of the Node ID should be sufficiently large to make collisions extremely unlikely when using a uniformly distributed random number generator [[2]].

#### Bootstrapping a Node

A node wishing to join the network for the first time has no contacts in its $k$-buckets. In order for the node to establish
itself on the network, it must contact one, or more than one, bootstrap node. These nodes are not special in any way other than
being listed in some predefined list. They simply serve as a first point of contact for the requesting node to become
known to more of the network and to find their closest peers.

There are a number of ways that bootstrap nodes can be obtained, including adding addresses to a configuration
and using [DNS seeds](https://bitcoin.org/en/glossary/dns-seed).

The joining process is described as follows [[2]]:

1. A joining node generates a random ID.
2. It contacts a few nodes it knows about.
3. It sends a `FIND_NODE` lookup request of its newly generated node ID.
4. The contacted nodes return the closest nodes they know about. The newly discovered nodes are added to the joining node's routing table.
5. The joining node then contacts some of the new nodes it knows about. The process then continues iteratively until the joining node
   is unable to locate any closer nodes.

This _self-lookup_ has two effects: it allows the node to learn about nodes closer to itself; and it populates other nodes'
routing tables with the node's ID. [[1]]

#### XOR metric

The Kademlia paper published in 2002 [[2]] offered the novel idea of using the XOR ($\oplus​$) operator
to determine the distance and therefore the arrangement of peers within the network.
Defined as:

$$ distance(a, b) = a \oplus b$$

This works, because XOR exhibits the same mathematical properties as any distance function.

Specifically, [[1]]

- $a \oplus a = 0$
- $a \oplus b > 0$ for $a \neq b$
- $a \oplus b = b \oplus a$
- Triangle property: $a \oplus b + b \oplus c \geq a \oplus c$

The XOR metric implicitly captures a notion of distance in the preceding tree structure [[2]].

#### Protocol

Kademlia is a relatively simple protocol consisting of only four remote procedure call (RPC) messages that facilitate two independent concerns:
peer discovery and data storage/retrieval.

The following RPC messages are part of the Kademlia protocol:

- Peer discovery
  - `PING`/`PONG` - used to determine liveness of a peer.
- - `FIND_NODE` - returns at most $k$ nodes, which are closer to a given query value.


- Data storage and retrieval
  - `STORE` - request to store a $\langle key, value \rangle$ pair.
  - `FIND_VALUE` - behaves the same as `FIND_NODE` by returning the $k$ closest nodes. If a node has the requested $\langle key, value \rangle$ pair,
    it will instead return the stored value.

Notably, there is no `JOIN` message. This is because there is no explicit join in Kademlia. Each peer has a chance of being added to a
routing table of another node whenever an RPC message is sent/received between them [[2]]. In this way, the node becomes known to the network.

##### Lookup Procedure

The lookup procedure allows nodes to locate other nodes, given a node ID. The procedure begins by
the initiator concurrently querying the closest $\alpha$ (concurrency parameter) nodes to the target node ID
it knows about. The queried node returns the $k​$ closest nodes it knows about. The querying node then proceeds in rounds,
querying closer and closer nodes until it has found the node. In the process, both the querying node and the intermediate nodes have learnt about each other.

##### Data Storage and Retrieval Procedure

The storage and retrieval procedure ensures that $\langle key, value \rangle$ pairs are reliably stored and able to be

retrieved by participants in the network.

The storage procedure uses the [lookup procedure](#lookup-procedure) to locate the closest nodes to the key, at which

point it issues a `STORE` RPC message to those nodes. Each node republishes the $\langle key, value \rangle$ pairs to

increase the availability of t he data. Depending on the implementation, the data may eventually expire (say 24 hours).

Therefore, the original publisher may be required to republish the data before that period expires.

The retrieval procedure follows the same logic as storage, except a `FIND_VALUE` RPC is issued and the data received.

##### Routing Table

Each node organizes contacts into a list called a routing table. A routing table is a binary tree
where the leaves are buckets that contain a maximum of $k$ nodes, aptly named $k$-buckets.
These are nodes with some common node ID prefix, which is captured by the [XOR metric](#xor-metric).

For instance, given node $A(1100)$ with peers $B(1110)$, $C(1101)$, $D(0111)$ and $E(0101)$:

The distances from node $A​$ would be

$A \oplus B = 0010 (2)$

$A \oplus C = 0001 (1)​$

$A \oplus D = 1011 (11)$

$A \oplus E = 1001 (9)$

$A$, $B$ and $C$ share the same prefix up to the first two most significant bits (MSBs). However, $A$, $C$ and $D$ share

no prefixed bits and are therefore further apart. In this example, $A$, $B$ and $C$ would in the same bucket and

$D$, $E$ in their own bucket.

Initially, a node's routing table is not populated with $k​$-buckets, but may contain a single node in a
single $k​$-bucket. As more nodes become known, they are added to the $k​$-bucket until it is full.
At this point, the node splits the bucket in two: one for nodes that share the same prefix as itself
and one for all the others.

![Kad-Routing-table](./assets/Kad-Evolution-Routing-Table.png)

This guarantees that for bucket $j$, where $0 <= j < k$, there is at least one node $N$ in node $A$'s routing table for which

$$ 2^j <= distance(A, N) < 2^{(j+1)} $$

##### $k$-bucket ordering

Peers within $k​$-buckets are sorted from least to most recently seen.

Once a node receives a request or reply from a peer, it checks to see if the peer is contained in the
appropriate $k​$-bucket. Depending on whether or not the peer already exists, the entry is either moved or appended to the
tail of the list (most recently seen). If a particular bucket is already size $k​$, the node tries to `PING` the first
peer in the list (least recently seen). If the peer does not respond, it is evicted and the new peer is
appended to the bucket, otherwise the new peer is discarded. In this way, the algorithm is biased towards peers
that are long-lived and highly available.

#### Kademlia Attacks

Some notable attacks in the Kademlia scheme:

##### Node Insertion Attack

Since there is no verification of a node's ID, an attacker can select their ID to occupy a particular keyspace in the network.
Once an attacker has inserted themselves in this way, they may censor or manipulate content in that keyspace, or eclipse nodes [[9]].

##### Eclipse Attack

An attacker takes advantage of the fact that in practice, there are relatively few nodes in most parts of a 160-bit keyspace.
An attacker injects themselves closer to the target than other peers and eventually could achieve a dominating position.
This can be done cheaply if the network rules allow many peers to come from the same IP address.

## DHT Vulnerabilities and Attacks

### Eclipse Attack

An Eclipse attack is an attack that allows adversarial nodes to isolate the victim from the rest of its peers and filter its view of the rest of the
network. If the attacker is able to occupy all peer connections, the victim is eclipsed.

The cost of executing an eclipse attack is highly dependent on the architecture of the network and can range from a small number of machines
(e.g. with hundreds of node instances on a single machine) to requiring a full-fledged botnet. Reference [[6]] shows that an eclipse attack on Ethereum's Kademlia-based DHT can be
executed using as few as two nodes.

Mitigations include:

- Identities must be obtained independently from some random oracle.
- Nodes maintain contact with nodes outside of their current network placement.

### Sybil Attack

Sybil attacks are an attempt by colluding nodes to gain disproportionate control of a network. and are often used as a vector
for other attacks. Many, if not all, DHTs have been designed under the assumption that a low fraction of nodes are malicious.
A Sybil attack attempts to break this assumption by increasing the number of malicious nodes.

Mitigations include:

- Associating a cost with adding new identifiers to the network.
- Reliably joining real-world identifiers (IP address, MAC address, etc.) to the node identifier, and rejecting a threshold of duplicates.
- Having a trusted central authority that issues identities.
- Using social information and trust relationships.

### Adaptive Join-Leave Attack

An adversary wants to populate a particular keyspace interval $I$ with bad nodes in order to prevent a particular file
from being shared. Let's suppose that we have a network with node IDs chosen completely at random through some random oracle.
An adversary starts by executing join/leaves until it has nodes in that keyspace. After that they proceed in rounds,
keeping the nodes that are in $I$ and rejoining the nodes that aren't, until control is gained over the interval.

It should be noted that if there is a large enough cost for rejoining the network, there is a disincentive for this attack.
In the absence of this disincentive, the [cuckoo rule](#cuckoo-rule) is proposed as a defence.

## Cuckoo Rule

Given a network that is partitioned into groups or intervals, and in which nodes are positioned uniformly and randomly.
Adversaries may proceed in rounds, continuously rejoin nodes from the least faulty group until control is gained over
one or more groups as described in [Adaptive Join-Leave Attack](#adaptive-join-leave-attack).

The cuckoo rule is a join rule that moves (cuckoos) nodes in the same group as the joining node to random locations
outside of the group. It is shown that this can prevent adaptive join-leave attacks with high probability,
i.e. a probability $1 - 1/N$, where $N$ is the size of the network.

Given:

- $I$ - keyspace group in $[0,1)$;
- $n$ - number of honest nodes;
- $ \epsilon n$ - number adversarial nodes for constant $\epsilon < 1$;
- therefore, the network size $N$ is $n + \epsilon n$;
- $k$-region is a region in $[0,1)$ of size $k/n$;
- $R_k(x)$ is a unique $k$-region containing $x$.

And with the following two conditions:

- Balancing Condition - the interval $I$ contains at least $O(log(n))$ nodes.
- Majority Condition - honest nodes are in the majority in $I$.

The cuckoo rule states:

> If a new node $v​$ wants to join the system, pick a random $x \in [0, 1)​$. Place $v​$ into $x​$ and move
> all nodes in $R_k(x)​$ to points in $[0, 1)​$ chosen uniformly and independently at random (without replacing any
> further nodes) [[5]].

It is concluded that for a constant fraction of adversarial peers, where $\epsilon < 1 - 1/k$ for any constant, $k > 1$

is sufficient to prevent adaptive join-leave attacks with high probability.

Sen, Freedman [[7]] modelled and analysed the Cuckoo Rule and found that, in practice, it tolerates very few adversarial nodes.

|                                                              |      |                                                              |
| :----------------------------------------------------------: | :--: | :----------------------------------------------------------: |
| ![Commensal Cuckoo Figure1](./assets/Commensal-Cuckoo-Figure1.png) |      | ![Commensal Cuckoo Figure2](./assets/Commensal-Cuckoo-Figure2.png) |
| (Cuckoo rule) Minimum group size <br>needed to tolerate different $\epsilon$ for 100,000 rounds.<br>Groups must be large (i.e. 100s to 1,000s of nodes) to guarantee correctness [[7]] |      | (Cuckoo rule) Number of rounds the system maintained correctness with an average group<br> size of 64 nodes, varied. Simulation was halted<br> after 100,000 rounds. Failure rates drop dramatically past a certain threshold for different N [[7]] |

Notably, they show that rounds to failure (i.e. more than one-third of nodes in a given group are adversarial) decreases dramatically
with an increasing but small global fraction of adversarial nodes. An amendment rule is proposed, which allows smaller group sizes
while maintaining Byzantine correctness. Reference [[7]] warrants more
investigation, but is out of the scope of this report.

## Conclusion

DHTs are a proven solution to distributed storage and discovery. Kademlia, in particular, has been successfully implemented and
sustained in file-sharing and blockchain networks with participants in the millions. As with every network, it is not without its
flaws, and careful network design is required to mitigate attacks.

Novel research exists, which proposes schemes for protecting networks against control from adversaries. This research becomes
especially important when control of a network may mean monetary losses, loss of privacy or denial of service.

## References

[[1]] Wikipedia: "Distributed Hash Table" [online]. Available: https://en.wikipedia.org/wiki/Distributed_hash_table. Date accessed: 2019-03-08.

[1]: https://en.wikipedia.org/wiki/Distributed_hash_table. "Wikipedia: Distributed Hash Table"

[[2]] Kademlia: A Peer-to-Peer Information System" [online]. Available: https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf. Date accessed: 2019-03-08.

[2]: https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf "Original Kademlia paper"

[[3]] Ethereum Wiki [online]. Available: https://github.com/ethereum/wiki/wiki/Kademlia-Peer-Selection#lookup. Date accessed: 2019-03-12.

[3]: https://github.com/ethereum/wiki/wiki/Kademlia-Peer-Selection "Kademlia Peer Selection"

[[4]] Wikipedia: "Tapestry (DHT)" [online]. Available: https://www.wikiwand.com/en/Tapestry_(DHT). Date accessed: 2019-03-12.

[4]: https://www.wikiwand.com/en/Tapestry_(DHT) "Tapestry (DHT)"

[[5]] Towards a Scalable and Robust DHT [online]. Available: http://www.cs.jhu.edu/~baruch/RESEARCH/Research_areas/Peer-to-Peer/2006_SPAA/virtual5.pdf. Date accessed: 2019-03-12.

[5]: http://www.cs.jhu.edu/~baruch/RESEARCH/Research_areas/Peer-to-Peer/2006_SPAA/virtual5.pdf "Towards a Scalable and Robust DHT"

[[6]] Low-resource Eclipse Attacks on Ethereum’s Peer-to-Peer Network [online]. Available:  https://www.cs.bu.edu/~goldbe/projects/eclipseEth.pdf. Date accessed: 2019-03-15.

[6]: https://www.cs.bu.edu/~goldbe/projects/eclipseEth.pdf "Low-Resource Eclipse Attacks on Ethereum’s Peer-to-Peer Network"

[[7]]: Commensal Cuckoo: Secure Group Partitioning for Large-scale Services [online]. Available: http://sns.cs.princeton.edu/docs/ccuckoo-ladis11.pdf. Date accessed: 2019-03-15.

[7]: http://sns.cs.princeton.edu/docs/ccuckoo-ladis11.pdf "Commensal Cuckoo: Secure Group Partitioning for Large-Scale Services"

[[8]]: Overlay and P2P Networks [online]. Available: https://www.cs.Nhelsinki.fi/webfm_send/1339.
Date accessed: 2019-04-04.

[8]: https://www.cs.helsinki.fi/webfm_send/1339 "Overlay and P2P networks"

[[9]]: Poisoning the Kad Networ" [online]. Available: https://www.net.t-labs.tu-berlin.de/~stefan/icdcn10.pdf. Date accessed: 2019-04-04.

[9]: https://www.net.t-labs.tu-berlin.de/~stefan/icdcn10.pdf "Poisoning the Kad Network"

## Contributors

- https://github.com/sdbondi
- https://github.com/hansieodendaal
- <https://github.com/anselld>
