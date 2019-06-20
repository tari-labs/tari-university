# Introduction to I2P Network and Comparison with Tor

- [Background](#background)
- [Introduction to I2P Network](#introduction-to-i2p-network)
    - [What is I2P?](#what-is-i2p)
    - [How does it Work?](#how-does-it-work)
    - [Infrastructure](#infrastructure)
      - [Routing Infrastructure and Anonymity](#routing-infrastructure-and-anonymity)
      - [Networking and Network Database](#networking-and-network-database)
    - [Garlic Routing](#garlic-routing)
- [Threat Model and Security](#threat-model-and-security)
- [Vulnerability Attacks](#vulnerability-attacks)
  * [Sybil Attacks](#sybil-attacks)
  * [Eclipse Attacks](#eclipse-attacks)
  * [Brute Force Attacks](#brute-force-attacks)
  * [Intersection Attacks](#intersection-attacks)
  * [Denial of Service Attacks](#denial-of-service-attacks)
    + [Greedy User Attack](#greedy-user-attack)
    + [Starvation Attack](#starvation-attack)
    + [Flooding Attack](#flooding-attack)
- [Comparison with Tor](#comparison-with-tor)
- [Conclusion](#conclusion)
- [References](#references)
- [Contributors](#contributors)

## Background
I2P (Invisible Internet Project), Tor and VPNs (Virtual Private Networks) are well known anonymity networks used by
millions of people across the world. Most people use them as a way to securely and privately browse the internet. These
networks have very similar characteristics but also very big differences and hence work in very specific ways.

In this report we'll examine what the I2P network is, the paradigms of how it works, its security infrastructure and its
usefulness in the blockchain domain.

## Introduction to I2P Network

### What is I2P?

I2P (known as the Invisible Internet Project and founded in 2003) is a low latency network layer that runs on a distributed network
of computers on a global network infrastructure. It's primarily built in applications such as email, IRC and file sharing. [[7]] This network layer provides a set of functions that runs on each
computer and provides encrypted, one-way connections to and from other computers within the network. These functions are
wrapped in a *"router"* that is installed during setup and configuration of the network.

### How does it Work?
The first concept to understand about I2P is that its primarily an enclosed network that runs within the Internet
infrastructure (referred to as the Clearnet in this paradigm). Unlike VPN's and the Tor network, which are built to
communicate with the Internet anonymously, I2P works as a decentralised network of that operates within the Internet -
i.e. an Internet within the internet. Interaction is done on a peer to peer (node to node) level and there is no centralised authority
that handles the network or keeps track of the active peers. A node in the network can either be a server that
hosts a darknet service, or a client who accesses said servers to use their services . [[7]] Tor and VPNs, on the other hand have centralised
authorities where the messages/data and network is managed. Since I2P works within its own network, it is primarily
made up of anonymous and hidden sites (called eepsites) that exist only within this network and are only accessible to
people using I2P. These sites can be easily created using an **I2PTunnel** service that uses a standard web server.
Another concept of note is I2P, by design, is not inherently an "outproxy" network i.e. it's not intended for accessing
the internet. This is because the client you send a message to is the cryptographic identifier, not some IP address, so
the message must be addressed to someone running I2P. Browsing the internet is however possible through opening an
outproxy that allows creating an anonymous internet connection. [[1]] [[2]]


### Infrastructure
#### Routing Infrastructure and Anonymity

I2P works by installing an I2P routing service within a user's device. This router creates temporary, encrypted, one way connections with other I2P routers on other devices. Connections are
referred to as one way because they are made up of an *Outbound Tunnel* and an *Inbound Tunnel*. When communication is
occurring, data leaves the user's devices via the outbound tunnels and is received on other devices through their
inbound tunnels. Messages do not travel two ways within the same tunnel. Therefore, a single round-trip request message
and its response between two parties needs four tunnels. [[4]].
Messages that leave one device do not travel directly to the inbound tunnel of the destination devices's intended.
Instead, the outbound router queries a distributed network database by travelling through exploratory channels to get
the address of the inbound router. This database is comprised of a custom Kademlia style distributed hash table (DHT),
and it contains the router information and destination information.
For each application/client, the I2P router keeps a pool of tunnel pairs. Exploratory tunnels for interactions with the
netDB are shared among all users of
a router. If a tunnel in the pool is about to expire or the tunnel is no longer useable the router creates a new tunnel
and adds it to the pool. It is important to recall later that tunnels periodically expire every ten minutes, and hence,
need to be refreshed frequently. This is a security measure, done to prevent long-lived tunnels from becoming a threat
to anonymity. [[3]]
<p align="center"><a name="fig_eca"> </a><img src="assets/network-topology.png" width="950" /></p>
<p align="center"><b>Figure&nbsp;1: Network Topology [<a href="https://www.delaat.net/rp/2017-2018/p97/presentation.pdf" title="Network Topology">6</a>]</b></p>

#### Networking and Network Database

The Distributed Database in I2P (called netDb) contains two types of data - the
Router information and LeaseSets. When a message is leaving one router, it needs to know some key pieces
of data (known as *RouterInfo*) about the other router.
This RouterInfo is stored in the Network Database with the router's identity as the Key. These keys indexing the routers
and hidden services are calculated by a SHA256 hash function of a 32-byte binary search key which is concatenated with a
UTC date string. The date string is added because these hash values change every day at UTC 00:00 - a key security measure.

To request a resource (or router info), a client requests the desired key from the server node considered closest to the
key. If the piece of data is located at the server node, it is returned to the client. Otherwise, the server uses its
local knowledge of participating nodes and returns the server it considers nearest to the key. If the returned server is
closer to the key than the one currently tried, the client continues the search at this server [[3]]

The Router structure comprises of: [[7]][[4]]

- The router's identity - an encryption key, a signing key, and a certificate
- The contact addresses at which it can be reached - Protocol, IP, Port
- When this was created/published
- Options - A set of arbitrary text options e.g. Bandwidth of Router
- The signature of the above, generated by the identity's signing key

The LeaseSet specifies tunnel entry point to reach an endpoint. This specifies the routers that can directly
contact the desired destination. It contains the data described below:

- Tunnel gateway router: Given by specifying its identity.
- Tunnel ID - Tunnel used to send messages
- Tunnel expiration - When the tunnel will expire
- Destination itself - Similar to router identity
- Signature - Used to verify the LeaseSet


#### Floodfill Routers
Special routers, refered to as Floodfill routers are responsible for storing the netDB. Participation in the Floodfill pool can either be automatic or manual. Automatic participation occurs whenever the number of Floodfill routers drops below a certain threshold, which is
currently 6% of all nodes in the network. [[8]] [[7]] When this happens, a node is selected to participate as a Floodfill router based on criteria such as uptime and bandwidth. It should be noted that about 95% of the Floodfill routers are automatic [8]. The netDb is stored in a Distributed Hash Table (DHT) format within the Floodfill routers. When resource is requested from the Floodfill router considered closest to that key. To have a higher success rate on a lookup, the client is able to iteratively look up the key. This means that the lookup continues with the next-closest peer should the initial lookup request fail.

### Garlic Routing
Garlic Routing is a way of building paths/tunnels through which messages/data in the I2P network travels. When a message
leaves the application/client it encrypted to the recipient's public key, then that encrypted message is encrypted with
instructions specifying the next hop. This message travels this way through the each hop until it reaches the recipient.
During the transportation of the message, it is furthermore bundled with other messages. This means that any messsage
travelling in the network could contain a number of other messages bundled with it. In essence, garlic routing does two
things:
1. Layered Encryption
2. Bundles multiple messages together

The following image represents the end to end message bundling.
<p align="center"><a name="fig_eca"> </a><img src="assets/garliccloves.png" width="650" /></p>
<p align="center"><b>Figure&nbsp;2: Garlic Routing <a href="https://github.com/ElementsProject/confidential-assets-demo" title="ElementsProject/confidential-assets-demo"></a></b></p>


## Threat Model, Security and Vulnerability Attacks
One of the disadvantages and limitations of the of the Tor network is its inability to scale and vulnerability to attacks. By design, it works by routing information through a number of intermediate nodes that eventually connect to exit nodes that work as trusted authority servers. Each of these servers keeps track of all the nodes in the network and their performance. These exit nodes also act as proxies, allowing Tor users to access the clearnet without revealing their identity. As there are only few trusted authority servers, the integrity of these nodes is essential for the entire network, making them a valuable target for attacks. [[3]]

Instead of storing the network's metadata in a group of trusted authority servers, I2P keeps this data in the Distributed Hash Table. This approach makes it harder to attack the network since it runs on normal I2P nodes and provides a small group of authority servers (about 3% of the network).

In general, the I2P network has no explicit threat model specified but there are common attacks and existing defences against it.



### Sybil Attacks
The Sybil attack is a well known anonymity system attack in which the malicious user creates multiple identities as an effort to increase control over the network. Running this over the I2P network is rather difficult. This is because participants/clients in the network evaluate the performance of peers when selecting peers to interact with instead of using a random sample. Because running multiple identities on the same host impacts the performance of each of those instances, the number of additional identities running in parallel is effectively limited by the need to provide each of them with enough resources for being considered as peers. The hence means the malicious user will need a substantial amount of resources to create the multiple identities.
<p align="center"><a name="fig_eca"> </a><img src="assets/Sybil Attack.png" width="950" /></p>
<p align="center"><b>Figure&nbsp;3: Sybil Attack Illustration [<a href="https://www.delaat.net/rp/2017-2018/p97/presentation.pdf" title="Network Topology">6</a>]</b></p>


### Eclipse Attacks
In Eclipse attack, a set of malicious and colluding nodes arranges for a good node in such a way that the good node can peer only with malicious nodes. So the union of malicious nodes together makes a good node fool by writing their addresses into neighbour list of good node. In Sybil attack, a single malicious node possesses large number of identities in the network to control some part of the network. If attacker wants to continue sybil attack into Eclipse attack, the attacker will try to place the malicious nodes in the
strategic routing path in such a way that all traffic will pass through the attacker node. However the Eclipse attack is possible even if there is a defence against the sybil attack such as certified node identities [[9]]


### Brute force attacks
Brute force attacks on the i2P network can be mounted by actively watching the network's messages pass between all of the nodes and attempting to correlate which message follows which path. Since all peers in the network are frequently sending messages, this attack is trivial. The attacker can send out large data (+2GB), observe all the nodes and narrow down those that routed the message. The large chunk of data is necessary because inter-router communication is encrypted and streamed i.e. 1024byte data is indistinguishable from 2048bytes. Mounting this attack is however very difficult and would need one to be an ISP in order to observe a large chunk of the network.


### Intersection attacks
These involve observing the network and node churns over time, and intersecting which peers are online when a message goes through to narrow down to a target. It is theoretically possible to mount this attack if the network is small, but impractical with a larger network.

### Denial of Service Attacks
There are a couple of these types of attacks available:

#### Greedy User Attack
This is when a user is consuming significantly more resources than they are willing to contribute. I2P has strong defences against these as users within the network are routers by default and hence contribute to the network by design.

#### Starvation Attack
A user/node may try to create a starvation attack by creating a number of bad nodes that do not provide any resources/services to the network, causing existing peers to search through a larger network database or request more tunnels than should be necessary. An attempt to find nodes can be difficult as they are not different from failing or loaded nodes. However, I2P, by design maintains a profile on all peers and attempts to identify and ignore bad performing nodes, making this attack difficult.

#### Flooding Attack
In this attack the malicious user sends a large number of messages to the target's inbound tunnels or to the network at large. The targeted user can however detect this both by the contents of the message and because the tunnel's tests will fail. The user can hence identify the unresponsive tunnels, ignore them and build new ones. They can also choose to throttle the number of messages a tunnel can receive. However, I2P has no defences for a network flooding attack but flooding the network is an incredibly difficult attack.

## Comparison with Tor
The primary differences between Tor and I2P lie in the design/intent of the service and consequentially the threat model. The primary difference is Tor takes a directory/ central authority approach in its design i.e. clients in its network route their messages via central servers. These authority servers act as monitors of the network as well and traffic
routers. I2P on the other had uses a decentralised server approach (netDB to store information about each router in the network. There are more finer differences between the two. The following table draws a comparison between the two networks.

TABLE OF DIFFERENCES

| I2P                                     | Tor                       |
| ---------------------------------       | --------------------------|
| Fully Peer to Peer and self organising  | Server                         |
| Query NetDB to find destination’s inbound tunnel gateway | Centralised Authority/ Directory to relay data        |
| Limited to no exit nodes. Internal Communication Only | Designed and optimised for exit traffic, with a large number of exit nodes           |
| Designed Primarily for file sharing     | Designed for Anonymous Internet Access |
| Garlic routing                          | Onion Routing                        |
| Uni-directional tunnels                 | Rendezvous Point                         |
| Significantly Smaller User Base         | Bigger User Base          |
| Floodfill peers ("directory servers") are varying and untrusted | Hard Coded Central Authority |
| Tunnels in I2P are short lived | Tor tunnels have a long lived capacity.



[[10]] [[11]] [[12]]

## Conclusion

The I2P Network is a proven network for the movement of messages/data in an anonymous and secure approach. As much as
it's possible to browse the internet with it, it is primarily and limited to communication within its networks Tor is,
however, perfect for anonymous internet browsing and provides the tools and service for this.

Extensive research exists and continues to find ways to improve the security of these networks. This research becomes
especially important when control of a network may mean monetary losses, loss of privacy or denial of service.


## References

[[1]] B. Mann, "What Is I2P & How Does It Compare vs. Tor Browser?" [Online.]
Available: <https://blokt.com/guides/what-is-i2p-vs-tor-browser#How_does_I2P_work>. Date accessed: 2019-06-18

[[2]]: "An overview of the I2P network". <https://geti2p.net/en/docs/api/i2ptunnel>  Date accessed: 2019-06-18.

[[3]]: "Practical Attacks Against The I2P Network" [Paper] <https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf>. Date accessed: 2019-06-18

[[4]] An Empirical Study of the I2P Anonymity Network and its
Censorship Resistance" [online]. Available: <https://censorbib.nymity.ch/pdf/Hoang2018a.pdf>. Date accessed: 2019-06-18.

[[5]] <https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf> Date accessed: 2019-06-18.

[[6]] <https://www.delaat.net/rp/2017-2018/p97/presentation.pdf> Date accessed: 2019-06-20.

[[7]] Blockchain-based Sybil Attack Mitigation: A Case Study of the I2P Network <https://delaat.net/rp/2017-2018/p97/report.pdf> Date accessed: 2019-06-20.

[[8]] <https://geti2p.net/en/docs/how/network-database> Date accessed: 2019-06-20.

[[9]] Defending Eclipse Attack in I2P using Structured
Overlay Network <http://ijsetr.org/wp-content/uploads/2015/05/IJSETR-VOL-4-ISSUE-5-1515-1518.pdf> Date accessed: 2019-06-20.

[[10]] I2P Usability vs. Tor Usability
A Bandwidth and Latency Comparison <https://pdfs.semanticscholar.org/aa81/79d3da24b643a4d004c44ebe543000295d51.pdf> Date accessed: 2019-06-20

[[11]] Comparison of Tor and I2P Terminology <https://geti2p.net/en/comparison/tor> Date accessed: 2019-06-20
## Contributors

[[12]] <http://www.geti2p.org/how_networkcomparisons.html> Date accessed: 2019-06-20

- <https://github.com/mhlangagc>
- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
