# Consensus Mechanisms 

## [Understanding of the Principle](https://medium.com/the-daily-bit/9-types-of-consensus-mechanisms-that-you-didnt-know-about-49ec365179da)

- Consensus mechanisms are ways to guarantee a mutual agreement on a data point and the state. 

Note: With blockchains, consensus mechanisms ensure that each player in the network has a copy of the same ledger. Different consensus mechanisms impact the security and the economic framework of the overarching cryptographic protocol (code of conduct) in varying ways. 

+++

Creating consensus mechanisms involves the study of mechanism design, which is a two-step process: 
1. Consider the desired outcome 
2. Work backward to create a game that incentivises players to fulfil that outcome

---

## [Focus on Bitcoin](https://thecontrol.co/cryptoeconomics-101-e5c883e9a8ff) 

### Brief history on decentralised systems

- 


Note: Decentralised P2P systems based on cryptography were not new in 2009 (examples include Kazaa and Bittorrent). What these earlier decentralised systems lacked was economic incentives, and the lack of baked in economic incentives is arguably what stifled these early P2P systems from persisting and thriving over time. 

Satoshi added economic incentives to P2P systems when he created Bitcoin in 2009. It was actually previously believed to be impossible to achieve consensus among nodes (the Byzantine General’s Problem)

+++

[Note](https://www.usenix.org/system/files/conference/nsdi16/nsdi16-paper-eyal.pdf): Bitcoin has emerged as the first widely-deployed, decentralised global currency, and sparked hundred of copycat currencies. Overall, cryptocurrencies have garnered much attention from the financial and tech sectors, as well as academics; achieved wide market penetration in underground economies; reached a $12B market cap; and attracted close to $1B in venture capital. The core technological innovation powering these systems is the Nakamoto consensus protocol for maintaining a distributed ledger known as the blockchain. The blockchain technology provides a decentralised, open, Byzantine fault-tolerant transaction mechanism, and promises to become the infrastructure for a new generation of Internet interaction, including anonymous online payments, remittance, and transaction of digital assets. 

---

## [The need for consensus mechanisms](https://cointelegraph.com/news/why-blockchain-needs-proof-of-authority-instead-of-proof-of-stake) 

Note: The cryptocurrency world is maturing and the debate over the right long-term consensus protocol is intensifying. 

The purpose of a consensus algorithm in a public blockchain network is to make sure that the network’s participants agree on the current state of the blockchain without the need to trust each other or to have a central authority. 

---

## Proof of Work 

Note: The use of Proof of Work mining was initially proposed to establish that a given block had required a certain amount of work to be mined. This allowed users to simply pick the longest valid chain with the highest amount of work as the correct chain. 

However, Proof of Work is extremely inefficient in terms of energy consumption. This makes it expensive and incentivizes miners to centralise the hashing power. So, instead of pushing us towards a truly distributed network, these concentrated mining farms have become de facto authorities. 

Another alternative was needed.

+++

## Proof of Stake 

Note: A Proof of Stake algorithm has nothing to do with mining. Instead, it is about validating. The specific actor responsible for the next block in the chain is determined by the Proof of Stake algorithm. In order to avoid overly concentrating this power, the algorithm must have some kind of randomness. At the least, voting shares must be distributed properly to avoid morphing into a centralized system. 

In a Proof of Stake system, each validator must own some stake in the network. These stakes are bonded, which means that network participants deposit some money into the network using it as a collateral to vouch for a block. 

In a Proof of Work network, everyone accepts the chain as valid because a significant amount of effort has been employed. Meanwhile, in a Proof of Stake network participants trusts the chain with the highest collateral. 

Within the cryptocurrency world Proof of Work remains the most widely adopted consensus algorithm. However, a few prominent projects including NXT, BitShares and Ethereum use or are migrating to Proof of Stake. 

Even within the Bitcoin community, some members are considering trying to change the digital currency’s Proof of Work consensus mechanism to address scaling issues and improve the network’s operation. 

But could there be a better alternative?

+++

## Proof of Authority 

Note: VIVA introduces the concept of Proof of Authority as an algorithm which delivers instant transactions and seamless consensus over a truly distributed network. 

According to William Banks, CTO at VIVA, “while Proof of Stake might have certain advantages, it is not a panacea. The problem is that there is no guarantee that the validator with the highest collateral deposited a for a block is going to operate the network in its best interests.”

“In fact, Proof of Stake coins are plagued with issues because rational people tend to act in their own self-interest. PoS works only because the best interests of the largest stakeholders usually do align with those of the network. In the case of a disagreement, however, the largest stakeholder might assume the role of the supreme commander.”

The distributed Proof of Stake algorithm was created to solve problems with the earlier Proof of Work algorithms. To make it work the decisions are weighted based on multiple factors. 

First and foremost, the size of the stake and the interests of a validator are taken into account. Secondly, it is important to check when their decision last became the primary decision agreed upon by the network’s participants. Finally, it needs to be considered whether the outcome of this decision met with approval but he majority of the network participants.  



