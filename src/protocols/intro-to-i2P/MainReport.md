# The I2P Network - An introduction and Comparison to Tor and VPNs.

- [Introduction](#introduction)

- [An Introduction to the i2P network](#Introduction)

  - [What is I2P](#what-is-I2P)
  - [How it Works](#scalability-and-fault-tolerance)
  - [Understanding Routers](#distributed-data-storage)



## Introduction

I2P (Invisible Internet Project), Tor and VPNs (Virtual Private Networks) are well known anonymity networks used by millions of people across the world. Most people use them as a way to securely and privately browse the internet. These networks have very similar characteristics but also very big differences and hence work in very specific ways.

In this report we'll examine what the I2P network is, the paradigms of how it works, its security infrastructure and its usefulness in the blockchain domain.


## What is I2P
I2P (known as the Invisible Internet Project and founded in 2003) is a network layer that runs on a distributed network of computers on a global network infrastructure. This network layer provides a set of functions that runs on each computer and provides encrypted, one-way connections to and from other computers within the network. These functions are wrapped in a *"router"* that is installed during setup and configuration of the network.


## How Does It Work
The first concept to understand about I2P is that its primarily an enclosed network that runs within the Internet infrastructure (reffered to as the Clearnet in this paradigm). Unlike VPN's and the Tor network, which are built to communicate with the Internet anonymously, I2P works as a decentralised network of that operates within the Internet - i.e. an Internet within the internet. Interaction is done on a peer to peer level and there is no centralised authority that handles the network or keeps track of the active peers. Tor and VPNs, on the other hand have centralised authorities where the messages/data and network is managed. Since I2P works within it's own network, it is primarily made up of anonymous and hidden sites (called eepsites) that exist only within this network and are only accessible to people using I2P. These sites can be easily created using an **I2PTunnel** service that uses a standard web server. Another concept of note is I2P, by design, is not inherently an "outproxy" network i.e. it's not intended for accessing the internet. This is because the client you send a message to is the cryptographic identifier, not some IP address, so the message must be addressed to someone running I2P. Browsing the internet is however possible through opening an outproxy that allows creating an anonymous internet connection. (ref https://blokt.com/guides/what-is-i2p-vs-tor-browser#How_does_I2P_work) ref: https://geti2p.net/en/docs/api/i2ptunnel


#### The Infrastructure
1. **Routing Infrastructure & Anonymity:** I2P works by installing an I2P routing service within a user's device. This router creates temporary, encrypted, one way connections with other I2P routers on other devices. Connections are refered to as one way because they are made up of an *Outbound Tunnel* and an *Inbound Tunnel*. When communication is occurring, data leaves the user's devices via the outbound tunnels and is received on other devices through their inbound tunnels. Messages do not travel two ways within the same tunnel. Therefore, a single round-trip request message and its response between two parties needs four tunnels. (reference https://censorbib.nymity.ch/pdf/Hoang2018a.pdf).
Messages that leave one device do not travel directly to the inbound tunnel of the destination devices's intended. Instead, the outbound router queries a distributed network database by travelling through exploratory channels to get the address of the inbound router. This database is comprised of a custom Kademlia style distributed hash table (DHT), and it contains the router information and destination information.
For each application/client, the I2P router keeps a pool of tunnel pairs. Exploratory tunnels for interactions with the netDB are shared among all users of
a router. If a tunnel in the pool is about to expire or the tunnel is no longer useable the router creates a new tunnel and adds it to the pool. It is important to recall later that tunnels periodically expire every ten minutes, and hence, need to be refreshed frequently. This is a security measure, done to prevent long-lived tunnels from becoming a threat to anonymity. ref: https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf
This intermediate database is there to find the other client's inbound tunnels efficiently, and to anonymise the addresses of devices communicating with each other. ref: https://geti2p.net/en/about/intro

*Diagram of Inbound and Outbound Tunnels.*

2. **Networking & Network Database:** The Distributed Database in I2P (called netDb) contains two types of data - the router information and destination information. When a message is leaving one router, it needs to know some key pieces of data (known as *RouterInfo*) about the the other router.
This RouterInfo is stored in the Network Database with the router's identity as the Key. These keys indexing the routers and hidden services are calculated by a SHA256 hash function of a 32-byte binary search key which is concatenated with a UTC date string. The date string is added because these hash values change every day at UTC 00:00.

To request a resource (or router info), a client requests the desired key from the server node considered closest to the key. If the piece of data is located at the server node, it is returned to the client. Otherwise, the server uses its local knowledge of participating nodes and returns the server it
considers nearest to the key. If the returned server is closer to the key than the one currently tried, the client continues the search at this server ref: https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf

The Router structure comprises of: (ref: https://censorbib.nymity.ch/pdf/Hoang2018a.pdf)
- The router's identity (an encryption key, a signing key, and a certificate)
- The contact addresses at which it can be reached
- When this was created/published
- A set of arbitrary text options (related to the state/properties of the router)
- The signature of the above, generated by the identity's signing key

The Arbitrary text options are used by other routers for basic decisions. i.e. Should we connect to this router? Should we attempt to route a tunnel through this router? Does the router meet a minimum threshold for routing tunnels.


## Garlic Routing
Garlic Routing is a way of building paths/tunnels through which messages/data in the I2P network travels. When a message leaves the application/client it encrypted to the recipient's public key, then that encrypted message is encrypted with instructions specifying the next hop. This message travels this way through the each hop until it reaches the recipient.
During the transportation of the message, it is furthermore bundled with other messages. This means that any messsage travelling in the network could contain a number of other messages bundled with it. In essense, garlic routing does two things:
1. Layered Encryption
2. Bundles multiple messages together

The following image represents the end to end message bundling.

/// IMAGE



## Threat Model, Security and Vulnerability Attacks
One of the disadvantages and limitations of the of the Tor network is it's in ability to scale and vulnerability to attacks. By design, it works by routing information through a number of intermediate nodes that eventually connect to exit nodes that work as trusted authority servers. Each of these servers keeps track of all the nodes in the network and their performance. These exit nodes also act as proxies, allowing Tor users to access the clearnet without revealing their identity.
As there are only few trusted authority servers, the integrity of these nodes is essential for the entire network, making them a valuable target for attacks. reference: https://sites.cs.ucsb.edu/~chris/research/doc/raid13_i2p.pdf

Instead of storing the network's metadata in a group of trusted authority servers, I2P keeps this data in the Distributed Hash Table. This approach makes it harder to attack the network since it runs on normal I2P nodes and provides a small group of authority servers (about 3% of the network)

In general, the I2P network has no explicit threat model specified but there are common attacks and existing defences against it.



### Sybil Attacks









## Comparisons to Tor & VPNs
Heavily decentralized. Tor has a user:relay ratio of 165:1 (excluding non-public bridge relays; see metrics) while I2P has a user:relay ratio of 0.99:1 (a very limited amount of users don't route traffic for others because they are, for example, in a hostile country with a limited number of I2P users). This means that you would need a a lot more resources to have a chance of deanonymizing users by observing network traffic over malicious nodes (meaning a set of relays that are all observed by a hostile entity) for I2P than for Tor.

No central point of failure for building tunnels. Tor has directory servers that form a catalog of (public) Tor relays. A user asks these directory servers for (a copy of the entire list of) Tor relays (or just part of them?) including their properties (such as Exit Node, Guard Node, Fast Node, etc.) If (a number of?) these directory servers are compromised, they could manipulate the information that they are supplying to the users that use those compromised directory servers. The Tor directory servers can also be attacked, making it impossible for users to form tunnels because they lack the required information. I2P uses DHT which allows all I2P relays to inform other I2P relays of relays that they known. There is no central (set of) point(s) that can be attacked to make building of tunnels impossible (except attacking all I2P relays).

Asymmetric tunnels. I'll use an analogy to explain this. This analogy is wrong and inaccurate in some regards because the contents of the traffic that is sent through Tor and through I2P is encrypted and cannot be read. The amount of intermediary countries used also doesn't match. The purpose of the analogy is to make you understand the difference. With Tor, you send a letter from US to Canada through France, Germany and Brazil (in that order). The letter reads "Please send me the combination of our granddad's bank vault now that he has deceased.". The letter that is sent in reply from your friend in Canada (reading "19502118") is sent to your address in the US through Brazil, Germany and France (in that order). With I2P, the first letter (from US to Canada) is sent through France, Germany and Brazil (in the order), but the second letter is sent through Paraguay, Norway and Ukraine (in that order). Suppose the postal services in France, Germany, Brazil and Paraguay are compromised. In that case, those postal services can figure out that 19502118 is the combination for your granddad's bank vault, if you were using Tor to send the both of those letters. If you used I2P, they would not be able to figure out what the combination for the vault is, although they do know that you have requested the combination for the vault. A version of the above scenario that is more true to the nature of Tor and I2P would include letters sent in an unbreakable envelopes (the encrypted data). If that was the case, the compromised postal services would be able to confirm that a letter was sent from a person in the US to a person in Canada in both the case of Tor and I2P, but only in the case of Tor would they be able to also confirm that a letter was sent from that person in Canada to that person in the US. (They would also be able to guess that it was probably a letter in reply to the US -> Canada letter because of the rapid response time).

Short-lived tunnels. Adapting the analogy above, this means that communications between the US resident and the Canadian resident are only shortly passed through Brazil, Germany and France + Paraguay, Norway and Ukraine. Much sooner than is the case with Tor will I2P change the intermediary nodes that the communications are using (to, for example, Peru, Mexico and Australia + Greece, Nicaragua and Russia). This is useful because if a tunnel is compromised, you will only send data using that tunnel for a short amount of time, thus limiting the amount of data that is compromised (though the data is encrypted, so unless the server you are connecting to is also compromised, the adversary cannot inspect the unencrypted data).

Some protection against human errors. Tor simply relays TCP/IP packets while I2P is able to modify or trim those packets for some tunnels (such as the default IRC tunnel) to prevent human errors. Once again, an analogy is useful, though not accurate. Suppose you want to anonymously leak a document to a newspaper. You decide to use the (analog) Tor network to prevent your identity from being compromised. You send the letter through Bolivia, Colombia and Japan and then finally to the US HQ of a newspaper. Unfortunately though, you have forgotten to remove some identifying remarks from your letter (your data). Let's for the sake of clarity say that you have left fingerprints on your letter (a digital equivalent would be HTTP headers that indicate the local server time). You can then be deanonymized even though the delivery of the letter was securely anonymous.

BitTorrent functionality. Unlike Tor, I2P has been designed with BitTorrent support in mind (can someone verify this?). Tor isn't supportive of the Tor network being used for clearnet BitTorrent activity and, unlike I2P, it doesn't have its own internal BitTorrent functionality.

Weaknesses of I2P compared to Tor

Technical

No family flag for relays. This means that if one entity controls a bunch of relays, he can add this information to his relays so that the anonymization software will never choose more than 1 relay from the same family to build a tunnel. I'm not sure if I2P is actually missing this feature!

Non-technical / social

Lower amount of users (though more relays).

No extensive documentation and noob-friendly start-up tutorials (though there has been some progress as of late).

No extensive academic peer reviewing.

No noob-friendly user interface.

No noob-proof out-of-the-box solutions like the Tor Browser Bundle.

No (charismatic) public representative like Jacob Appelbaum is for the Tor Project.


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

- https://github.com/mhlangagc
- https://github.com/hansieodendaal
- <https://github.com/anselld>
