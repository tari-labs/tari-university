# Byzantine Fault Tolerance and Consensus Mechanisms 

- Will look at concensus in general as a concept
- How it is implemented in different cypotcurrency protocols?

Note: The Byzantine Generals Problem is referenced when discussing cryptocurrency and cryptographic protocol- when a protocol is described as being byzantine fault tolerant (or BFT)- This stems from a simple analogy, as a means to understand the problem of distrubuted consensus 

---

## Byzantine Fualt Tolerance 

---

## The Premise

- A Byzantine army is tryong to attack a city 
- there are several generals, who have this city encircled 
- Some generals want to attack and others want to retreat
- If the generals do not agree on a plan of action, the army will loss the battle 
- So... the majority has to agree to attack or the majority has to agree to retreat
- If the majority comes to a consensus on the strategy - otherewise there may be defeated 
- The generals cannot just talk to one another 
- they have to use messengers, becuase there camps are so far away 
- In addition to be far away from one another, there are generals that are not honest (and may vote the wrong way, just to confuse the situation)

==> therein lies the problem-- when you have peeople distributed in that way, how do those poeple come to consensus, and agree on something 

In Really it revoles around getting 51% of the people to agree on something and all move forward with that strategy. (in the case of the blockchain, all move forward with a cerain set of rules, and a certain view on history i.e. what is in the blockchain)

---

### What is a blockchain?

It is a distributed ledger- a source of truth for history 

Because we have thousands of poeple using thsi ledger, how do we get them all to agree on something-- even worse, how to we make sure that someone with a bad agenda (a bad actor) doesn't come in and try to rewrite history  

---

## [Understanding of the Principle](https://medium.com/the-daily-bit/9-types-of-consensus-mechanisms-that-you-didnt-know-about-49ec365179da)

- Consensus mechanisms are ways to guarantee a mutual agreement on a data point and the state. 

Note: With blockchains, consensus mechanisms ensure that each player in the network has a copy of the same ledger. Different consensus mechanisms impact the security and the economic framework of the overarching cryptographic protocol (code of conduct) in varying ways. 

+++

Creating consensus mechanisms involves the study of mechanism design, which is a two-step process: 
1. Consider the desired outcome 
2. Work backward to create a game that incentivises players to fulfil that outcome

(QUick divertion into the question of the need of incentivising- Algorand and other examples of blockchains without incentives)

---

## [Focus on Bitcoin](https://thecontrol.co/cryptoeconomics-101-e5c883e9a8ff) 

### Brief history on decentralised systems


Note: Decentralised P2P systems based on cryptography were not new in 2009 (examples include Kazaa and Bittorrent). What these earlier decentralised systems lacked was economic incentives, and the lack of baked in economic incentives is arguably what stifled these early P2P systems from persisting and thriving over time. 

Satoshi added economic incentives to P2P systems when he created Bitcoin in 2009. It was actually previously believed to be impossible to achieve consensus among nodes (the Byzantine General’s Problem)

+++

[Note](https://www.usenix.org/system/files/conference/nsdi16/nsdi16-paper-eyal.pdf): Bitcoin has emerged as the first widely-deployed, decentralised global currency, and sparked hundred of copycat currencies. Overall, cryptocurrencies have garnered much attention from the financial and tech sectors, as well as academics; achieved wide market penetration in underground economies; reached a $12B market cap; and attracted close to $1B in venture capital. The core technological innovation powering these systems is the Nakamoto consensus protocol for maintaining a distributed ledger known as the blockchain. The blockchain technology provides a decentralised, open, Byzantine fault-tolerant transaction mechanism, and promises to become the infrastructure for a new generation of Internet interaction, including anonymous online payments, remittance, and transaction of digital assets. 

---

## [The need for consensus mechanisms](https://cointelegraph.com/news/why-blockchain-needs-proof-of-authority-instead-of-proof-of-stake) 

Note: The cryptocurrency world is maturing and the debate over the right long-term consensus protocol is intensifying. 

The purpose of a consensus algorithm in a public blockchain network is to make sure that the network’s participants agree on the current state of the blockchain without the need to trust each other or to have a central authority. 

---

This is where Proof of Work comes in 


## Proof of Work 

- Concept developed in teh early 1990s as a proposal for how of to get aroudn something called the denial of service attack

Note:  DoS attack is simply a network connection or a system being flooded with requests that it has to serve back, and the system cannot detect whether these requests are legitmit or not 

One of the first implementations of this and where Bitcoin is routed in is called Hash Cash. It was developed by Adam Back (he is now a Bitocoin Core developer) in 1997. 

Proof of Work is essentailly a piece of data that very time consuming and computationally expensive to produce- but at the same time it has to be very simple for someone on the other side to verify that a person did that proof of work 

(Give Saduku example)

How does this help with the Byzantine Generals Problem?

- it makes it pretty expensive to become a bad actor to try an attack the network

Now in the case of Hash Cash- what it was actually being used for was a way to deter spam email. You would have to perform a amoutn of proof of work- which would take a while on a regular computer, and attach that in the header of the email you were sending. When the user on the other end would receive that email, they would check for that header, and be able to verify whether you put some work into that before you sent it.

And the theory here is that a spammer would not go out of their way and spend thousands or hundreds of thousands of dollars creating all of this proof of work to send emails-- it became more expensive to send spam.

So in bitcoin's case, it makes the proof of work the miners responsibility 

### The miners 

Tne miners are the people we are entrusting to be able to write history- they are the generals in this case. So bitcoin automatically adjusts the difficulty of generating this proof of work, so it works out at roughly ten minutes for somebody to find a solution, amougst all the miners out there- (that's how we get to the ten minute block time- you have thousands of miners out there all trying to generate this proof of work, and in abouh ten minutes someone comes up with the solution, and is hence trusted to write to history (because they have put so much time and effort and energy into generating that thing- essentailly making it very costly for anyone to attack that network and do something dishonest. In Bitcoin's current state, it also acts as a way to mint the coins- so every mine, when they solve a block is actually rewarded with new coins, that have been minted in to the network- so that adds extra incentive - 'i should be have because i will be rewarded for all this work I'm doing to secure the network and write that immutable history'

So proof of work mining and the way it solves this distributed consensus problem is really one of the chief innovations in bitcoin and what made it successful- because it is very hard problem to solve 

But this is not to say that Proof of Work is the be all and end all- it does have some drawbacks:

1. It is inefficent and does not really solve anything (those millions and trillions of hashs that are being generated in trying to find a solution- they do not contribute back to society)
2. It is wasteful and costly from an electricity perspective (some estimates say that it costs half a billion dollars every year just to secure the network through mining (all the people running server farms with mining equipment- just for bitcoin- there is a lot more electricity going into all the other cryptocurrencies 
3. Because the chief resource that you a putting into securing this network is expensiev hardware, it actually creates an arms race amoungst the miners to try and buy up the most expensive, powerful and advanced mining hardware

Due to that arms race we have seen a lot of proprietory technology being developed- ASICs (applications specific integrated circuit) and realy it is just a special computer solely designed to mine bitcoin, or whatever hashing algorithm it is designed for). They are super efficient- but often the technology is close-sourced and expensive to procure from China. 

All this adds to the problem of ecntralisation 

So you have all the research and developemnt into these ASICs being done over in China-those people are not going to sell their technology which they can use to make lots of money mining bitcoin- they are going to keep it to themself, and they are going to mine and profit of bitcoin-- as a result we have around 70% of the mining power on bitcoin in the hands of a few miners over in China. 

Now no single miner holds over 51% of the hashing power over in China- however if a couple of those parties came together and formed a cartel- then it blows away a solution the byzantine generals problem- because it means that just two generals could colude and be able to write history 

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



