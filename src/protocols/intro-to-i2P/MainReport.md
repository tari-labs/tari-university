# The I2P Network - An Introduction

- [Background](#background)
- [Introduction to I2P Network](#introduction-to-i2p-network)
    - [What is I2P?](#what-is-i2p)
    - [How does it Work?](#how-does-it-work)
    - [Infrastructure](#infrastructure)
      - [Routing Infrastructure and Anonymity](#routing-infrastructure-and-anonymity)
      - [Networking and Network Database](#networking-and-network-database)
      - [Floodfill Routers](#floodfill-routers)
      - [Garlic Routing](#garlic-routing)
- [Threat Model, Security and Vulnerability Attacks](#threat-model-security-and-vulnerability-attacks)
    - [Sybil Attacks](#sybil-attacks)
    * [Eclipse Attacks](#eclipse-attacks)
    * [Brute Force Attacks](#brute-force-attacks)
    * [Intersection Attacks](#intersection-attacks)
    * [Denial of Service Attacks](#denial-of-service-attacks)
      + [Greedy User Attack](#greedy-user-attack)
      + [Starvation Attack](#starvation-attack)
      + [Flooding Attack](#flooding-attack)
- [How Tor works and compares to I2P](#comparison-with-tor)
- [Conclusion](#conclusion)
- [References](#references)
- [Contributors](#contributors)

## Background
Invisible Internet Project (I2P), Tor and Virtual Private Networks (VPNs) are well-known anonymity networks. They are all designed in different ways and for specific uses though most people use them with the intent of privately browsing the internet. In their functions, these networks have very similar characteristics but also have important differentiators in the how they work to anonymise and secure the internet traffic of users.

In this report, we'll examine what the I2P network is, the paradigms of how it works, its security infrastructure and its
potential or known use-cases in the blockchain domain.

## Introduction to I2P Network

### What is I2P?
I2P (known as the Invisible Internet Project - founded in 2003) is a low-latency network layer that runs on a distributed 
network of computers on a global network infrastructure. It is primarily built into applications such as email, Internet Relay Chat (IRC) and file 
sharing [[6]]. It works by automatically making each client in the network a node through which data and traffic is routed. The nodes that run support the network layer are responsible for providing encrypted, one-way connections to and from other computers within the network. 

### How does it Work?
I2P is an enclosed network that runs within the Internet
infrastructure (referred to as the clearnet in this paradigm). Unlike VPNs and Tor, which are inherently "outproxy" networks designed to
anonymous and privately communication with the Internet, I2P is designed as a peer to peer network. This means it has very little to no communication with the internet. This also means identifying each node in I2P is not done with an IP address but a cryptographic identifier. ([[1]], [[2]]). A node in the I2P network can either be a 
server that hosts a darknet service (similar to a website in the internet), or a client who accesses the servers and services hosted by other nodes [[6]]. Tor, on the other works by using a group of volunteer-operated relay servers/nodes that allow people to privately and securely access the internet. This means people can choose to volunteer as a relay node in the network and essentially donate bandwidth. [[13]]. Compared to Tor, each client/server in I2P is automatically a relay node. Whether data is routed through a specific node is normally bandwidth dependent. 

**Eepsites** Since there is no *internet* in I2P, the network is made up of its own anonymous and hidden sites (called *eepsites*). These exist only within the network and are only accessible to people using I2P. Services such as **I2PTunnel**, that use 
a standard web server, can be used to create sites like these.

## Infrastructure
#### Routing Infrastructure and Anonymity

I2P works by installing an I2P routing service within a client's device. This router creates temporary, encrypted, one-way 
connections with I2P routers on other devices. Connections are referred to as one way because they are made up of 
an *Outbound Tunnel* and an *Inbound Tunnel*. During any communication, data leaves the client's devices via the 
outbound tunnels and is received on other devices through their inbound tunnels. This hence means messages/data does not travel in two directions within 
the same tunnel. Therefore, a single round-trip request message and its response between two parties needs four tunnels [[4]], as shown in Figure&nbsp;1.
Messages sent from one device do not travel directly to the inbound tunnel of the destination device.
Instead, the outbound router queries a distributed network database for address of the inbound router. This database is comprised of a custom Kademlia style Distributed Hash Table (DHT)
that contains the router information and destination information.
For each application/client, the I2P router keeps a pool of tunnel pairs. Exploratory tunnels for interactions with the
network database are shared among all users of
a router. If a tunnel in the pool is about to expire or if the tunnel is no longer usable, the router creates a new tunnel
and adds it to the pool. It is important to recall later that tunnels periodically expire, every ten minutes, and hence,
need to be refreshed frequently. This is one of I2P's security measures that's done to prevent long-lived tunnels from becoming a threat
to anonymity [[3]].

<p align="center"><a name="fig_eca"> </a><img src="assets/network-topology.png" width="950" /></p>
<p align="center"><b>Figure&nbsp;1: Network Topology [<a href="https://www.delaat.net/rp/2017-2018/p97/presentation.pdf" title="Network Topology">6</a>]</b></p>

#### The Distributed Network Database

The NetDB, discussed earlier, is implemented as a Distributed Hash Table (DHT) and is propagated via nodes known as floodfill routers using the Kademlia algorithm. The netDB is one of the characteristics that makes I2P decentralised. To start participating in the network, a router instals a part of the NetDB. Obtaining the partial NetDB is called bootstrapping and happens by ’reseeding’ the router. By default, a router will reseed the first time by querying some hard-coded domain names. When a router successfully establishes a connection to one of these domains, a Transport Layer Security (TLS) connection is set up through which the router downloads a signed partial copy of the NetDB. Once the router can reach at least one other participant in the network, the router will query for other parts of the NetDB it does not have itself. [[12]]

The NetDB stores two types of data: 
1. **Router Info and how this works:**
When a message is leaving one router, it needs to know some key pieces of data (known as *RouterInfo*) about the other router.
The destination router info is stored in the netDb with the router's identity as the key. To request a resource (or RouterInfo), a client requests the desired key from the node considered to be closest to the key. If the piece of data is located at the node, it is returned to the client. Otherwise, the node uses its local knowledge of participating nodes and returns the node it considers to be nearest to the key. [[3]]. The router info in the netDB is made up of: ([[4]], [[6]]):

    - The router's identity - an encryption key, a signing key and a certificate.
    - The contact addresses at which it can be reached - protocol, Internet Protocol (IP), port.
    - When this was created or published.
    - Options - a set of arbitrary text options, e.g. bandwidth of router.
    - The signature of the above, generated by the identity's signing key.

2. **LeaseSets:** 
The LeaseSet specifies tunnel entry point to reach an endpoint. This specifies the routers that can directly contact the desired destination. It contains the following data:

    - Tunnel gateway router - given by specifying its identity.
    - Tunnel ID - tunnel used to send messages.
    - Tunnel expiration - when the tunnel will expire.
    - Destination itself - similar to router identity.
    - Signature - used to verify the LeaseSet.

#### Floodfill Routers
Special routers, referred to as *floodfill routers*, are responsible for storing the netDb. Participation in the floodfill 
pool can either be automatic or manual. Automatic participation occurs whenever the number of floodfill routers drops 
below a certain threshold, which is currently 6% of all nodes in the network ([[6]], [[7]]). When this happens, a node is 
selected to participate as a floodfill router based on criteria such as uptime and bandwidth. It should be noted that 
approximately 95% of floodfill routers are automatic [[8]]. The netDb is stored in a DHT format within 
the floodfill routers. A resource is requested from the floodfill router considered to be closest to that key. To have a 
higher success rate on a lookup, the client is able to iteratively look up the key. This means that the lookup continues 
with the next-closest peer should the initial lookup request fail.

#### Garlic Routing
Garlic routing is a way of building paths/tunnels through which messages/data in the I2P network travels. When a message
leaves the application/client, it is encrypted to the recipient's public key. That encrypted message is then encrypted with
instructions specifying the next hop. This message travels in this way through each hop until it reaches the recipient.
During the transportation of the message, it is bundled with other messages. This means that any message
travelling in the network could contain a number of other messages bundled with it. In essence, garlic routing does two
things:

- provides layered encryption; and
- bundles multiple messages together.

Figure&nbsp;2 illustrates the end-to-end message bundling:
<p align="center"><a name="fig_eca"> </a><img src="assets/garliccloves.png" width="850" /></p>
<p align="center"><b>Figure&nbsp;2: Garlic Routing <a href="https://github.com/ElementsProject/confidential-assets-demo" title="ElementsProject/confidential-assets-demo"></a></b></p>

## Threat Model, Security and Vulnerability Attacks
The I2P project has no explicit threat model specified but rather talks about common attacks and existing defenses against them. Overall, the design of I2P is motivated by threats similar to those addressed by Tor: The attacker can
observe traffic locally but not all traffic flowing through the network and integrity of all cryptographic primitives is assumed. Furthermore, an attacker is only allowed to control a limited amount of peers in the network (the website talks about not more than 20 % of nodes participating in the netDB and a similar fraction of total amount of nodes controlled by the malicious entity). In this section, we'll look at different threat models affecting the network. [[3]]

### Sybil Attacks
The Sybil attack, illustrated in Figure&nbsp;3, is a well-known anonymity system attack in which the malicious user creates multiple identities in an 
effort to increase control over the network. Running this over the I2P network is rather difficult. This is because 
participants/clients in the network evaluate the performance of peers when selecting peers to interact with, instead of 
using a random sample. Because running multiple identities on the same host affects the performance of each of those 
instances, the number of additional identities running in parallel is effectively limited by the need to provide each of 
them with enough resources to be considered as peers. This means that the malicious user will need a substantial 
amount of resources to create the multiple identities.

<p align="center"><a name="fig_eca"> </a><img src="assets/Sybil Attack.png" width="750" /></p>
<p align="center"><b>Figure&nbsp;3: Sybil Attack Illustration [<a href="https://www.delaat.net/rp/2017-2018/p97/presentation.pdf" title="Network Topology">5</a>]</b></p>

### Eclipse Attacks
In eclipse attacks, a set of malicious and colluding nodes arranges that a good node can 
peer only with malicious nodes. The union of malicious nodes therefore fools the good node into writing its
addresses into neighboring lists of good nodes. In a Sybil attack, a single malicious node possesses a large number of 
identities in the network to control some part of the network. If an attacker wants to continue a Sybil attack into an eclipse 
attack, the attacker will try to place malicious nodes in the strategic routing path in such a way that all traffic 
will pass through the attacker's node. However, the eclipse attack is possible even if there is a defence, such as certified node identities, against the Sybil 
attack [[8]].

### Brute Force Attacks
Brute force attacks on the I2P network can be mounted by actively watching the network's messages as they pass between all of 
the nodes and attempt to correlate messages and their route. Since all peers in the network are frequently 
sending messages, this attack is trivial. The attacker can send out large amounts of data (more than 2GB), observe all the nodes and narrow 
down those that routed the message. The large chunk of data is necessary because inter-router communication is encrypted 
and streamed, i.e. 1,024&nbsp;byte data is indistinguishable from 2,048&nbsp;byte data. Mounting this attack is, however, very difficult and 
one would need to be an Internet Service Provider (ISP) or government entity in order to observe a large chunk of the network.

### Intersection Attacks
Intersection attacks involve observing the network and node churns over time, and intersecting the peers that are online when a message 
goes through, in order to narrow down to a target. It is theoretically possible to mount this attack if the network is small, but 
impractical with a larger network.

### Denial of Service Attacks
Denial of service attacks include the following:

#### Greedy User Attack
A greedy user attack occurs when a user is consuming significantly more resources than they are willing to contribute. I2P has strong 
defences against these attacks, as users within the network are routers by default and hence contribute to the network by design.

#### Starvation Attack
A user/node may try to launch a starvation attack by creating a number of bad nodes that do not provide any resources or services to the network, causing existing peers to search through a larger network database or request more tunnels than 
should be necessary. An attempt to find useful nodes can be difficult, as there are no differences between them and failing or loaded nodes. 
However, I2P, by design, maintains a profile of all peers and attempts to identify and ignore poorly performing nodes, making 
this attack difficult.

#### Flooding Attack
In a flooding attack, the malicious user sends a large number of messages to the target's inbound tunnels or to the network at 
large. The targeted user can, however, detect this by the contents of the message and because the tunnel's tests will 
fail. The user can hence identify the unresponsive tunnels, ignore them and build new ones. They can also choose to 
throttle the number of messages a tunnel can receive. Although I2P has no defences against a network flooding attack, it is incredibly difficult to flood the network.

## How Tor works and Comparison with I2P
As previously mentioned, Tor works through volunteer relay nodes. These relay nodes, like I2P's nodes, are responsible for creating hops through which data is routed before reaching its intended destination on the Internet. They work by incrementally building a circuit of encrypted connections through relays on the network. The circuit is extended one hop at a time, and each relay along the way knows only which relay gave it data and which relay it is giving data to. No individual relay ever knows the complete path that a data packet has taken. Also no request uses the same path. Later requests are given a new circuit, to keep people from linking your earlier actions to the new ones. This process is also known as Onion Routing [[14]]. See the image below: 

<p align="center"><a name="fig_eca"> </a><img src="assets/htw3.png" width="750" /></p>
<p align="center"><b>Figure&nbsp;4: How Tor Works - Illustration [<a href="https://2019.www.torproject.org/about/overview.html.en" title="How Tor Works">13</a>]</b></p>

### Onion Routing - How is works
Onion Routing is essentially a distributed overlay network designed to anonymize TCP-based applications like web browsing, secure shell, and instant messaging. Clients choose a path through the network and build a circuit, in which each node, in the path knows its predecessor and successor, but no other nodes in the circuit. Traffic flows down the circuit in fixed-size cells, which are unwrapped by a symmetric key at each node (like the layers of an onion) and relayed downstream. [[14]]


The designated use of relay nodes in the Tor network gives the network a couple of important characeristics: 
- The stability of the network is proportional to the number of relay nodes in the network. The less the relay nodes the less stable the network becomes. 
- The security of the network is also proportinal to the number of relay nodes. The more the relay nodes the less vulnerable it is agaist attacks. 
- Finally, the speed of the network is proportinal to the number of relay nodes. The more nodes there are the faster the network becomes. [[13]]

### Types of Tor Relays/Nodes Routing
Tor's relay nodes do not all function in the same way. There are four types of relay nodes - a Guard/Entry, Middle, Exit and Bridge node. 

<p align="center"><a name="fig_eca"> </a><img src="assets/torCircuit.png" width="750" /></p>
<p align="center"><b>Figure&nbsp;5: The Tor Circuit [<a href="https://medium.com/coinmonks/tor-nodes-explained-580808c29e2d" title="The Tor Circuit">14</a>]</b></p>

#### 1. Guard or Entry Relay (non-exit relay)

A guard relay is the first relay in the Tor circuit. Each client that wants to connect to the Tor network will first connect to a Guard relay. This means guard nodes can see the IP Address of the client attempting to connect. It is worth noting that Tor publishes it's guard nodes and hence anyone can see them on websites such as this one: [[15]] Because seeing the IP address of a client is possible, there have been cases where attackers have filtered out traffic on the network using Circuit Fingerprinting techniques such as is documented in this paper. [[16]].

#### 2. Middle Relay
These cover most parts of the Tor netowrk and act as hops. They consist of relays through which data is passed in encrypted format. No node knows more than its predecessor and descendant. All the available middle relay nodes show themselves to the guard and exit nodes so that any may connect to them for transmission. Middle relays can never be exit relays within the network. [[13]]

#### 3. Exit Relay
These are nodes that send data to the desired destinations on the internet. The services Tor clients are connecting to (website, chat service, email provider, etc) will see the IP address of the exit relay instead of their real IP address of the Tor user. Because of this, exit relay owners are often are subject to numerous legal complaints and shut down threats. [[13]]

#### 3. Bridge Relay
The design of the Tor network means that the IP address of Tor relays is public as previously mentioned and shown here. [[15]] Because of this, Tor can be blocked by governments or ISPs by blacklisting the IP addresses of these public Tor nodes. Tor Bridges are nodes in the network that are not listed in the public Tor directory, making it harder for ISPs and governments to block them. They are meant for people who want to run Tor from their homes, have a static IP or don't have much bandwidth to donate. [[13]]


## Differences between I2P and Tor
<!-- <div align="center"><b>Table 1: Differences between I2P and Tor</b></div> -->


| I2P                                     | Tor                       |
| ---------------------------------       | --------------------------|
| Fully peer to peer: Self-organizing Nodes | Fully Peer to Peer: Volunteer Relay Nodes                         |
| Query netDb to find destination’s inbound tunnel gateway | Relays Data to the closest relay     |
| Limited to no exit nodes. Internal communication only | Designed and optimized for exit traffic, with a large number of exit nodes          |
| Designed primarily for file sharing    | Designed for anonymous Internet access |
| Unidirectional tunnels                 | Rendezvous point                        |
| Significantly smaller user base      | Generally bigger user base        |

**Source:** ([[9]], [[10]], [[11]]).

## Conclusion

In summary, Tor and I2P are two types of networks that anonymise and encrypt data moved within them. Each network is uniquely designed for a respective function. The I2P network is a designed for moving data in a Peer to Peer peer format, whereas Tor is designed for accessing the internet privately. 

Extensive research exists and continues to find ways to improve the security of these networks in their respective operational designs. This research becomes
especially important when control of a network may mean monetary losses, loss of privacy or denial of service.

## References

[[1]] B. Mann, "What Is I2P & How Does It Compare vs. Tor Browser?" [Online.]
Available: <https://blokt.com/guides/what-is-i2p-vs-tor-browser#How_does_I2P_work>. Date accessed: 2019&#8209;06&#8209;18.

[1]: https://https://blokt.com/guides/what-is-i2p-vs-tor-browser#How_does_I2P_work
"What Is I2P & 
How Does It Compare vs. 
Tor Browser?"

[[2]]: I2P: "I2PTunnel" [online]. Available: <https://geti2p.net/en/docs/api/i2ptunnel>. Date accessed: 2019&#8209;06&#8209;18.

[2]: https://geti2p.net/en/docs/api/i2ptunnel
"I2PTunnel"

[[3]]: C. Egger, J. Schlumberger, C. Kruegel and G. Vigna, "Practical Attacks Against the I2P Network" - Paper [online]. 
Available: <https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf>. Date accessed: 2019&#8209;06&#8209;18.

[3]: https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf
"Practical Attacks Against
the I2P Network"

[[4]] N. P. Hoang, P. Kintis, M. Antonakakis and M. Polychronakis, "An Empirical Study of the I2P Anonymity Network and 
its Censorship Resistance" [online]. Available: <https://censorbib.nymity.ch/pdf/Hoang2018a.pdf>. Date accessed: 
2019&#8209;06&#8209;18.

[4]: https://censorbib.nymity.ch/pdf/Hoang2018a.pdf
"An Empirical Study 
of the I2P Anonymity 
Network and its
Censorship Resistance"

[[5]] K. Alachkar and D. Gaastra, "Mitigating Sybil Attacks on the I2P Network Using Blockchain" - Presentation [online]. 
Available: <https://www.delaat.net/rp/2017-2018/p97/presentation.pdf>. Date accessed: 2019&#8209;06&#8209;20.

[5]: https://www.delaat.net/rp/2017-2018/p97/presentation.pdf
"Mitigating Sybil Attacks 
on the I2P Network 
Using Blockchain"

[[6]] K. Alachkar and D. Gaastra, "Blockchain-based Sybil Attack Mitigation: A Case Study of the I2P Network" - Report 
[online]. Available: <https://delaat.net/rp/2017-2018/p97/report.pdf>. Date accessed: 2019&#8209;06&#8209;20.

[6]: https://delaat.net/rp/2017-2018/p97/report.pdf
"Blockchain-based Sybil 
Attack Mitigation: 
A Case Study of the 
I2P Network"

[[7]] I2P: "The Network Database" [online]. Available: <https://geti2p.net/en/docs/how/network-database>. Date accessed: 
2019&#8209;06&#8209;20.

[7]: https://geti2p.net/en/docs/how/network-database
"The Network Database"

[[8]] H. Vhora and G. Khilari, "Defending Eclipse Attack in I2P using Structured
Overlay Network" [online]. Available: <http://ijsetr.org/wp-content/uploads/2015/05/IJSETR-VOL-4-ISSUE-5-1515-1518.pdf>. 
Date accessed: 2019&#8209;06&#8209;20.

[8]: http://ijsetr.org/wp-content/uploads/2015/05/IJSETR-VOL-4-ISSUE-5-1515-1518.pdf
"Defending Eclipse Attack 
in I2P using Structured
Overlay Network"

[[9]] M. Ehlert, "I2P Usability vs. Tor Usability - A Bandwidth and Latency Comparison" [online]. 
Available: <https://pdfs.semanticscholar.org/aa81/79d3da24b643a4d004c44ebe543000295d51.pdf>. Date accessed: 2019&#8209;06&#8209;20.

[9]: https://pdfs.semanticscholar.org/aa81/79d3da24b643a4d004c44ebe543000295d51.pdf
"I2P Usability vs. 
Tor Usability - 
A Bandwidth and Latency 
Comparison"

[[10]] I2P: "I2P Compared to Tor" [online]. Available: <https://geti2p.net/en/comparison/tor>. Date accessed: 2019&#8209;06&#8209;20.

[10]: https://geti2p.net/en/comparison/tor
"I2P Compared to Tor"

[[11]] I2P: "I2P Compared to Tor and Freenet" [online]. Available: <http://www.geti2p.org/how_networkcomparisons.html>. 
Date accessed: 2019&#8209;06&#8209;20.

[11]: http://www.geti2p.org/how_networkcomparisons.html
"I2P Compared to Tor 
and Freenet"


[[12]] Tim de Boer & Vincent Breider: "Invisible Internet Project - MSc Security and Network Engineering Research Project." [online].
Available: <https://www.delaat.net/rp/2018-2019/p63/report.pdf> Date accessed: 2019&#8209;07&#8209;10.

[12]: https://www.delaat.net/rp/2018-2019/p63/report.pdf
"Invisible Internet Project - MSc Security and Network Engineering Research Project"

[[13]] "Tor Project: How it works" [online].
Available: <https://2019.www.torproject.org/about/overview.html.en> Date accessed: 2019&#8209;08&#8209;05.

[13]:https://2019.www.torproject.org/about/overview.html.en
"Tor Project: How it works"

[[14]] "Tor: The Second-Generation Onion Router" [online]
Available: <https://svn.torproject.org/svn/projects/design-paper/tor-design.html>  Date accessed: 2019&#8209;08&#8209;05.

[14]:https://svn.torproject.org/svn/projects/design-paper/tor-design.html
"Tor: The Second-Generation Onion Router"

[[15]] "Tor Network Status" [online]  
Available: <https://torstatus.blutmagie.de/>  Date accessed: 2019&#8209;08&#8209;05.

[15]: https://torstatus.blutmagie.de/
"Tor Network Status"


[[6]] "Circuit Fingerprinting Attacks: Passive Deanonymization of Tor Hidden Service" [online]
Available: <https://people.csail.mit.edu/devadas/pubs/circuit_finger.pdf> Date accessed: 2019&#8209;08&#8209;05.

[16]: https://people.csail.mit.edu/devadas/pubs/circuit_finger.pdf
"TCircuit Fingerprinting Attacks: Passive Deanonymization of Tor Hidden Service"

## Contributors

- <https://github.com/mhlangagc>
- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
