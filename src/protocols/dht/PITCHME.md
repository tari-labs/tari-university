@snap[center span-100]
## Distributed Hash Tables
@snapend

---

@snap[center span-100]
## Existing P2P networks

The distributed networks we know and loved have some major flaws
@snapend

+++

@snap[center span-100]
![flooding](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/presentation/flooding.jpeg)

### Flooding

#### @color[#fff](Gnutella relied on flooding the network to propagate messages)
@snapend

+++

@snap[center span-100]
![unstructured](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/presentation/unstructured.jpg)

### Unstructured

#### @color[#fff](Freenet did not guarantee that data would be found)
@snapend

+++

@snap[center span-100]
![single point of failure](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/presentation/single-point-of-failure.jpg)

### Single Point of Failure

#### @color[#fff](Napster relied on centralized components such as trackers and index servers)

@snapend

+++

![Something had to be done](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/presentation/something-had-to-be-done.jpg)

---

# Enter DHTs

In 2001/2002, four DHT projects were introduced: CAN, Chord, Pastry and Tapestry.
They aimed to have a lookup efficiency of $O(log(n))$ similar to that of a centralized index,
while having the benefits of a decentralized network.

- Fault-tolerance
- Censorship-resistance
- Scalability
- Not having to trust a central authority

+++

## What is a DHT?

- Overlay network
- Key-value database
- Data storage and peer discovery tasks are distributed across many nodes to form a cohesive service
- DHT designs allow the network to tolerate nodes coming and going with a low probability of failure
- Network size can increase indefinitely

Bittorrent is the largest decentralized network, containing in tens of millions of concurrent users
and hundreds of millions of active users.

+++ 

## What is a DHT?

The various DHT schemes achieve this in different ways. However, they have a number of aspects in common:

- Each participant has some unique network identifier
- They perform peer lookup, data storage and retrieval services
- There is some implicit or explicit joining procedure
- Communication need only occur between neighbors rather than by the entire network or centralized server

+++

## Put it on the blockchain!

![best blockchain dev](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/presentation/best-blockchain-dev.jpg)

+++

## Maybe not

<em>"A blockchain needs a distributed hash table like a fish needs a bicycle." - G. Maxwell</em>

- A blockchain network is one big broadcast 
- Every participant needs to know the entire blockchain state

The Ethereum blockchain makes use of DHTs for efficient peer discovery.

+++

## Are DHTs secure?

- DHT networks are inherently more difficult to take down than a typical centralized system

- No DHT scheme or implementation is perfect
- Kinds of attacks
    - Denial of service attacks
    - Sybil attacks
    - Eclipse attacks
    - Join-leave attacks

---

## DHT Schemes

- Chord
    - Circular space
    - Used by: Cassandra DB
- Kademlia
    - XOR Metric
    - Used by: BitTorrent, Ethereum, eMule and more.
- Koorde
    - de Bruijn graph
- Pastry & Tapestry
    - Plaxton-style mesh
- Viceroy
    - Butterfly network
    
+++

## DHT Schemes cont.

- diverse algorithms
- most schemes are able to perform lookups in $O(log(n))$
- Kademlia is popular due to it's relative simplicity/performance and it's proven track record on large networks
  
---

## Kademlia

Kademlia is designed to be an efficient means for storing and finding content in a distributed peer-to-peer (P2P) network.
It has a number of core features that are not simultaneously offered by other DHTs, such as:

- Minimizing the number of messages necessary for nodes to learn about each other
- Nodes are able route traffic through low-latency paths
- Queries can be executed in parallel to avoid timeout delays from failed nodes
- The node existence algorithm resists certain basic distributed denial-of-service (DDoS) attacks

+++

## Kademlia cont.

- Node ID
- XOR Metric
- Routing table (grouped in $k$-buckets)
- Protocol

+++

## Node ID

- Each node has a node ID 
    - chosen at random (presumably)
    - consisting of `n` bits (keyspace)
    - `n` should be sufficiently large to make collisions extremely unlikely (e.g. 160-bits)
    - typically a hash of a public key
- Each participant need only maintain connections to their "closest" peers
    - Closeness is given by the non-euclidean XOR metric
- On first join, a node contacts predefined bootstrap nodes
    - Config, DNS seeds etc.
    - The joining node will do a lookup of itself to find neighbors

+++

## XOR Metric

$$distance(a, b) = a \oplus b$$

XOR has all the properties of a distance function (metric space)

- Identity: $a \oplus a = 0$
- Non-negativity: $a \oplus b > 0$ for $a \neq b$
- Symmetry: $a \oplus b = b \oplus a$ 
- Triangle inequality: $a \oplus b + b \oplus c \geq a \oplus c$

+++
 
## XOR Metric cont.

- The XOR metric has the property of grouping nodes with the same binary prefix together.

- For example, 
  - $A \oplus B = 0010 (2)$
  - $A \oplus C = 0001 (1)​$
  - $A \oplus D = 1011 (11)$
  - $A \oplus E = 1001 (9)$
- (B, C) and (D, E) can be grouped simply by comparing the XOR value falls within a particular range

+++

## $k$-buckets 

- Peers are stored in buckets of size $k$, named $k$-buckets
- $k$ is a network wide parameter

+++

## $k$-buckets 

- Initially, a node's routing table has a single $k​$-bucket
- As more nodes become known, they are added to the $k​$-bucket until it is full.
- At this point, the node splits the bucket in two:
    - one for nodes that share the same prefix as itself
    - one for all the others
    
+++

## $k$-buckets cont.

![routing table tree](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/Kad-Evolution-Routing-Table.png)   

+++

## $k$-buckets cont.

It is guaranteed that for bucket $j$, where $0 <= j < k$,
there is at least one node $N$ in node $A$'s routing table for which

$$ 2^j <= distance(A, N) < 2^{(j+1)} $$

+++

## $k$-buckets cont.

- Peers within $k​$-buckets are sorted from least recently seen (LRS) to most recently seen (MRS)
- New nodes might be added to a $k$-bucket if:
    - The $k$-bucket has less than $k$ peers
    - otherwise, the least recently seen node is `PING`ed 
        - if that node responds, the new node is discarded
        - otherwise, the LRS node is discarded and new node is added to the end of the $k$-bucket (MRS)
- This algorithm is biased towards peers that are long-lived and highly available.
  
+++


## Protocol

Kademlia is a simple protocol 

It consists of four RPC messages that facilitate two concerns:
- peer discovery
- data storage/retrieval

+++

## Protocol cont.

The following RPC messages are part of the Kademlia protocol:

- Peer discovery
  - `PING`/`PONG`
      - used to determine liveness of a peer
  - `FIND_NODE`
      - returns a number of nodes which are closer to a given key

- Data storage/retrieval
  - `STORE`
      - request to store a $\langle key, value \rangle$ pair
  - `FIND_VALUE`
      - behaves the same as `FIND_NODE` by returning closer nodes to the given key
      - A node will return the requested $\langle key, value \rangle$ pair if it has it
      - The key is a hash of the data
        
+++

## Protocol cont.

- There is no explicit `JOIN` message
- Each peer has a chance of being added to a routing table of another node whenever an
  message is sent/received between them

---

## Attacks 

- Denial of service
- Eclipse
- Sybil
- (Adaptive) join-leave attacks

+++

## Eclipse Attacks

- Adversarial nodes to isolate the victim from the rest of its peers and filter its view of the rest of the network. 
- An attacker is able to occupy all peer connections, the victim is eclipsed.

+++

## Eclipse Attacks cont.

#### How?

- An attacker takes advantage of the fact that in practice, there are relatively few nodes in most parts a 160-bit keyspace
- Choose a targeted node identifier
- They inject themselves closer to the target than other peers to achieve a dominating position
- This can be done very cheaply if, for instance, the network rules allow many peers to come from the same IP address
- Most networks have redundancies and basic protections
- In practise, an attacker would likely need control of a botnet to isolate specific keys and peers

+++

## Eclipse Attacks cont.

#### Why?

- File-sharing
    - prevent a particular item from being located and shared
    - Torrent poisoning

- Blockchain
    - perform a zero confirmation double-spend attack,
    - hijack enough mining hash power to create the longest chain,
    - make it easier to perform a 51% attack, or
    - perform an $N$ confirmation double spend attack.

+++

## Eclipse Attacks Mitigation

- Require an unique IP per node
- Identities must be obtained independently from some random oracle. e.g. 
    - Distributed RNGs
    - De/centralized registration
- Maintain contact with nodes outside of their current network placement

+++

## Sybil Attacks

- an attempt by colluding nodes to gain disproportionate control of a network
- are often used as a vector for other attacks

Many, if not all, DHTs have been designed under the assumption that a low fraction of nodes are malicious.
A Sybil attack attempts to break this assumption.

+++

# Sybil Attack Mitigations

- Associating a cost with adding new nodes to the network
- Reliably joining real-world identifiers (IP address, MAC address, etc.) to the node identifier, and rejecting a threshold of duplicates
- Having a trusted central authority or secure decentralized scheme that issues identities
- Using social information and trust relationships

+++

## Adaptive Join-Leave Attacks

- Suppose, node IDs are chosen by some uniformly random procedure

- An attacker is able to determine if their nodes have entered a particular network location 
    - therefore,
        - can adapt their attack, proceeding in rounds
        - rejoin nodes which fall outside of a particular area until control is achieved
        
+++

## Adaptive Join-Leave Attack Mitigations

- Make it expensive to continuously rejoin the network
- A simple solution: When a new node joins, randomize all node IDs on the network.
    - this obviously doesn't scale!
    - Another solution: Cuckoo Rule

---

## Cuckoo Rule

![Cuckoo rule](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/presentation/cuckoo-rule.png)

<p>Proposed by Awerbuch and Schneider</p>

+++

## Cuckoo Rule cont.

Given:
- $I$ - keyspace in $[0,1)$
- $n$ - number of honest nodes
- $ \epsilon n$ - number adversarial nodes for constant $\epsilon < 1$
- $N$ - the network size where $N = n + \epsilon n$ 
- so, the network contains less than $1/2$ adversarial nodes
- $k$-region is a region in $[0,1)$ of size $k/n$
- $R_k(x)$ is a unique $k$-region containing $x$

+++

## Cuckoo Rule cont.

The cuckoo rule states:

> If a new node $v​$ wants to join the system, pick a random $x \in [0, 1)​$. Place $v​$ into $x​$ and move
> all nodes in $R_k(x)​$ to points in $[0, 1)​$ chosen uniformly and independently at random (without replacing any
> further nodes) 

- In order for the network to be in a "good" state, each $k$-region must
    - be balanced (nodes are well-distributed across all regions)
    - be correct (contain less than $1/3$ bad nodes)

They conclude, this is sufficient to prevent join-leave attacks with high probability.

+++

## Cuckoo Rule Analysis

- Sen and Freedman modelled the cuckoo rule 
  - nodes may rejoin in the same $k$-region with non-negligible probability
  - they show that the number of rounds it takes to gain control of a $k$-region decreases dramatically
    with a small global increase in adversarial nodes
    
+++

## Cuckoo Rule Analysis

![Commensal Cuckoo rule](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/Commensal-Cuckoo-Figure2.png)

- Number of rounds the system maintained correctness with an average group size of 64 nodes, varied.
- Failure rates drop dramatically past a certain threshold for different $N$

+++

## Cuckoo Rule Analysis

![Commensal Cuckoo rule](https://raw.githubusercontent.com/tari-labs/tari-university/master/src/protocols/dht/assets/Commensal-Cuckoo-Figure1.png)

- Minimum group size needed to tolerate different $\epsilon$ for 100,000 rounds.
- Groups must be large (i.e. 100s to 1,000s of nodes) to guarantee correctness

+++

## Cuckoo Rule Analysis 

### Commensal Cuckoo Rule

Some amendments to the Cuckoo Rule:

- If an insufficient number of nodes have been cuckoo-ed from the interval, the join request is rejected
- a subset of nodes are cuckoo-ed out of the interval instead of every node

---

## Questions?

