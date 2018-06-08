# Lightning Network for Dummies 

![Lightning](lightning.png)

---
## Setting the scene 

In order for Bitcoin, or any other cryptocurrency to hold its value- **It needs value proposition**

However, currently Bitcoin  is too slow and too expensive to be a viable method of payment for everyday use. 

* Visa- The payment network Visa achieved 47 000 peak transactions per second (tps) on its network during the 2013 holidays- currently averaging hundreds of millions per day. 
* Bitcoin- Bitcoin supports less than 7 transactions per second with a 1 megabyte lock limit 

Note: It needs value proposition. In saying that, as a cryptocurrency, it still possess the most important feature: decentralisation. For bitcoin to succeed, the benefits of using it to buy a cup of coffee needs to outweigh those fusing status-quo payments, like cash or credit. At this point in time, bitcoin’s benefits do not outweigh cash or credit. It’s slow and more importantly, too expensive. Although decentralisation is imperative- nobody wants to wait a minimum of 10 minutes to receive 2 confirmation and pay multi dollar fees just to buy a cup of coffee. That’s why solutions to this problem have been debated for the las several years. 

---

Solutions can be boiled down to two primary components: 
1. bigger blocks 
2. Off-chain scaling 

Note: On its face- increasing the block size seems like the logical solution. It’s been done before, so why arbitrarily limit it at 1MB. Satoshi Nakamoto- Bitcoin’s creator- stated that blocks should grow as big as they need to be; and implied that blocks should increase as they approach max capacity 

---

However, the argument is: 
* Big blocks increase the cost of running full-node 
* Big blocks will lead to a centralisation in mining (only a few parties would be able to do block validation)

Note: If scaling takes place now, Bitcoin risks technical issues and centralization

---

## Introducing Lightning
![Lightningnetworknodes](lightningnetworknodes.png)

---

* Lightning is a decentralised network of bilateral bitcoin payment channels off the bitcoin blockchain 
* Lightning transactions are typically small, and most are not broadcast to the blockchain
* With LN you need to pay onchain transaction fee only 2 times: to open and to close the channel. Users can also create many channels and route their payments via other people’s channels
* It is expected to be a game changer in the cryptocurrency’s evolution
* The Aim: The Network will speed up transaction processing and decrease associated costs on bitcoin’s blockchain

---

**Assuming enough people open Lightning payment channels, there will eventually be a sizeable bitcoin liquidity pool distributed across the network. This question is, how to enable it to be shared.**

Note: Lightning developers are designing a routing facility that identifies which network nodes have sufficient funds to make a payment, calculates the shortest viable route to the payment destination across those nodes, and sends the payment. If this works, it would resolve the bitcoin dilemma. 

---

* The Lightning Network (LN) mainnet reached over 1 000 active nodes in its short history
* There are currently just over 11 000 active nodes on the main Bitcoin blockchain 

Note: It remarkable is how quickly users have flocked to the LN mainnet, a clear indication that enthusiasm for this scaling technology remains high. 

---

From the whitepaper, in order to use the LN, a common-user needs to : 
1. Create the parent (funding transaction)
2. Create the children (commitment transitions and all spends from the commitment transactions)
3. Sign the children 
4. Exchange the signatures for the children 
5. Sign the parent 
6. Exchange the signatures for the parent 
7. Broadcast the parent on the blockchain 

Note: Essentially, you fund the network with a transaction on the Bitcoin Mainnet and commitment transactions re-shift the original balances. To sign the funding transaction, they need to exchange their parent signatures and broadcast them back on the mainnet. 

---

![Lightningnetworktransactions](LNTXS.png)

---

* Economics leads to centralisation 

In order to use the lightning network, and then use your Bitcoin again onchain, it requires a funding transaction, and a broadcast (or closing) transaction. Each transaction requires a fee. Unless fees are greatly reduced, users will not be blindly opening up channels with one another.

Note: If I wanted to purchase a coffee from my local coffee shop, I might open up a payment channel with them, as I visit them often. Opening a channel with a funding transaction could potentially make economic sense if I frequently visit the coffee shop and make multiple purchases within a given period of time. However, I would still have to consider that the amount funded in the Funding Transaction will not be available to me on the Bitcoin mainnet. 

---

YOU DON’T NECESSARILY NEED TO HAVE AN OPEN CHANNEL WITH EVERYONE, IF THERE’S A ROUTE ON THE NETWORK TO THAT MERCHANT

![Distributednetwork](distributednetwork.png)

A well-distributed Lightning Network will require substantial cooperation from the network users. 

---

Important to note:
* Lightning Network will be an open market
* A third party will be able to set their fee structure to connect end-users who wish to transact with one another
* The third party needs to possess the necessary capital to process the transaction. 

---

If Alice and Bob do not have an open channels and Alice wants to send Bob 0.5 BTC, they’ll both need to be connected to a third party (or a series of 3rd parties). Say if Charles (the third party) only possesses 0.4 BTC in his respective channels with other users, the transaction will not be able to go through that route.

---

What if a well-funded third party exists...


![Centraliseddecentraliseddistribution](Centraliseddecentraliseddistributed.png)

Note: What if Alice, Bob, Charles, Danny, Eddy, Francis and Gina are all connected to that third party? Not only will each route always possess the necessary capital, it will also be the shorted route. There will be no economic incentive for each user to open up new channels with each other. So, in practice, Lightning is more likely to look like this: 

---

* Such hits are likely to always be the shortest route- they can raise fees because longer routes will results in incrementally larger fees
* The lightning network will be a hub and spoke network of bi-directional payment channels


Note: If a route has three intermediary nodes, each intermediary requires a fee. If each fee is 0.5 satoshis (totalling 1.5 satoshis), the large hop can just set their fees at 1.4999 satoshis to compete. Also, they can provide further incentive to create the scenario described, by covering mainnet funding transaction fees for all users. 

---

## Who benefits?

**Whoever runs and operates the centralised hubs**

*Thus, it seems that the Lightning Network is nothing more than shifting fees away from miners and the mainnet and- in effect centralising the network, by creating Visa and MasterCard like companies which operate Lightning hubs. Further, it is still not fully understood how ‘trustless’ these companies will be.*  

---

## Problematic Implications of the Lightning Network 

There are issues with the implentation of this type of network:

* It may not be particularly user friendly 

Note: As innovative as this idea is, questions have been raised as to how intuitive it will be for a common consumer. An overly complex user-engagement will lead users to remaining on the mainnet, bogging down the network, keeping fees high and transactions backlogged.

---

* Fees 

Bitcoin’s congestion is one among several factors the influence its transaction fees. The fee itself is a large component of LN’s overall costs. 

There are two parts to LN's overall cost
1. The fee equivalent to bitcoin’s transaction charges in order to open and close channels between parties
2. Separate routing fee for transfer payments between channels. 

Note: Lightning network is often touted as a solution to the problem of bitcoin’s rising transaction fees. Its proponents claim that transaction fees, which is one of the direct consequences of bitcoin’s clogged network, will come down after the technology takes transactions off the main blockchain

---

* High onchain fees will lead to LN centralisation 

*Why should user open 10 channels for $500, when he can just open one channel with a big hit for $50 or even for ‘free’ and route all payments with 1-2 hops?*

The solution may be Channel Factories - but the final implementation is not clear and system will probably stop being ‘trustless’

Note: Towards the end of 2017, we saw a $50 fee for 1-input-transaction due to the cryptocurrency mania spike. 

+++

## What are Channel Factories?

From the paper: Scalable Funding of Bitcoin Micropayment Channel Networks
* The concept proposes to create a ‘Masterchannel’ which in turn allows LN users to open subchannels in between
* Users sub-channels can be opened and closed while the ‘masterchannel or factory’ remains open. 

In short, channel factories are payment channels that can be used to create more payment channels

Note: Lightning Network supporters like the idea of channel factories, and LN technology has been researched and developed heavily over the past few months. Those who dislike the purpose of this particular second layer concept believe that this latest idea pushes the LN technology even further behind because it’s essentially pushing the protocol to a third layer solution. 

---

* Low onchain fees will decrease LN adoption 

On the other hand, if BTC onchain fees will somehow stay low or suddenly decrease, then it will also decrease LN adoption, because users will be less incentivised to open channels with each other, which will put the LN system in danger. 

Note: So LN might become a complex system with a few beneficiaries, which will require high onchain fees to exist. That will financially incentivise big hits, watchtowers and other centralised LN services to lobby high fees by riding community influentials and using sock puppets, thus achieving even more centralisation and higher revenues. 

---

The concept of so-called ‘free’ wi-fi/trial will be used in LN as well. After implementing dual funded channels, big merchants  may offer to pay ‘opening’ TX fees for a user if they buy something worth of $N. This will lead to more centralisation and less anonymity. 

Note: Why bother creating a channel with a ‘small’ node, pay high unchain fees and later pay more off chain fees for routing, when you can just open one channel with a big hit for ‘free’ and route most payments with 1 hop?- most people will always choose the most convent option. 

---

* Pre-funded 

Lightning’s pre-funded channels tie up funds that could be used for other purposes. Because of this, people may choose to keep very low balances in their Lightning channels, topping them up frequently rather than making infrequent balance adjustments.

---

However, since onchain fees will be high, it makes sense to deposit lots of money to a new LN channel, and spend funds over the next few weeks with lower off chain fees. 

Recharging a channel won’t help, because user still needs to pay onchain fees and thus he should ‘deposit’ a big sum upfront again, to save on fees in the future. 

Note: Why bother depositing lots of funds upfront to save on future fees, if user can keep using Visa or other alt coins with low transaction fees? Solution: this won’t be a problem if everybody uses LN, because all payments would be done off chain, but this is unlikely to happen soon 

---

* Closing channels uncooperatively 

In order to prevent cheating by broadcasting an older state of LN channel, there is CSV (CheckSequenceVerify) time-lock

The most common delay will probably be around 144 blocks (~1 day), but it might vary a lot- If we decrease a time-lock value, so user can get money onchain much faster, but that has its tradeoffs.  

Note: The CSV  will literally lock the funds for certain amount of time, giving a counterparty a chance to punish a cheater by broadcasting a transaction that contains a ‘secret key’ from that old state.-- There is no perfect solution with current logic, because time-locks will always balance between safety and liquidity, but Submarine Swaps might help to get money onchain faster, if they will be widely adopted. 3rd party LN Watchtowers might also decrease a common delay time without a big safety trade-off 

+++

## What are Submarine Swaps 

Submarine Swaps essentially let users send Lightning payments to a middleman on the Lightning Network; that middleman will send a corresponding amount of bitcoin to a regular (on-chain) Bitcoin address. It also works the other way around: users can send regular on-chain payments to the middleman; that middleman will then send a corresponding amount of bitcoin to a receiving Lightning node on the Lightning Network.

---

* Exhausted Channels  

This can occur when all or most money is on one side. It will be very dangerous if a counterparty has an older state, where all or most money on his side, because it becomes very lucrative to cheat (risk almost nothing for a chance to get all channel’s funds) 

+++

For example:

Alice opens a channel with Bob and funded it for 1 BTC. After a few transactions, all 1 BTC moved to Bob’s side. 

Alice  (1BTC)---(0BTC) Bob <--Old state

Alice  (0BTC)---(1BTC) Bob <--Latest state

Now Alice can try to cheat Bob by broadcasting on the blockchain an old state, where she had all the funds. If Bob punished her, Alice will lose nothing, but onchain fees for a cheating transaction. If Alice succeeds, she will get 1 BTC. 

+++

## Minimum collateral 

* Setting a minimum collateral value to prevent exhaustion might create problems when other nodes will check for an available channel capacity for routing
* Setting fixed minimum collateral value during channel creation, but it will again introduce several issues, because a system will be less flexible to changes

---

* Remaining online at all times

Nodes on bitcoin’s lightning network are required to be online at all times in order to send and receive payments. 
Going offline creates its own set of problems on the Lightning Network.
It is possible for one of the two parties from a payment channel to close the channel and pocket funds while the other is away.  This is known as Fraudulent Channel Close.  

An offline stance could also bring down the network. 

---

## Alt-coin solutions

### Dash 

* DASH employs a hybrid-system of Proof-of-Work (whereby transactions are mined by miners) and Proof-of-Service (similar to Proof-of-Stake, where Masternodes stake 1000 DASH but also provide network services) 
* Masternodes can instantly verify transactions on a master block
* Not every use-case requires an instant verification, but this optional service makes DASH a viable option today to buy that coffee
* DASH also has an optional privacy feature, albeit, rather limited to other privacy cryptocurrencies

+++

* DASH has agreed to incrementally test and increase the block she from 2MB blocks through 400MB blocks
* It is also funding research for dedicated hardware for miners and nodes to process increased volume. 

## Concluding Notes

1. On-boarding fees will be too expensive. Further, people will not be able to redeem their money by closing the channel if someone tries to steal it due to high fees.  
2. If fees are low, there is no reason to use LN in the first place 

+++

4. It is assumed that LN will be decentralized. It's highly likely that hub operators will be required to keep track of everyone using their hub and report to governments. This defeats the point of Bitcoin and disqualifies this approach as a legitimate scaling option.  
5. Nobody wants to lock up all of their Bitcoins inside of a channel for a prolonged period

+++

6. The LN hub is a middleman
7. Complexity: Bitcoin is complex enough for new users. Adding another layer isn't something they will understand and be willing to use.  
8. Off line payments are not supported: clients have to be talking to each other in realtime (money is stuck at the current state of the channel, if partner has not signed- you have to constantly monitor the status of the bitcoin blockchain, to ensure private transactions are not being broadcast back to bitcoin network)

+++

9. Not good for large transactions-  the amount that you can send, is limited by how large the payment transactions are. So people needing to transact big amounts might as well open another payment channel 
10. There are limits to the amount transferred using lightning (microtransactions)
11. It is going to be difficult to be constantly monitoring the blockchain and keeping a real time connection with all of their peers- centralised hubs offering a lightning node as a service with essentially acts a hub to interact with others on the lightning network 


