# The Background

The USD equivalent of average transaction fees on the Bitcoin blockchain exceeded a dollar for most of 2017, and hit an all-time high of over $50 in December 2017. 

It is difficult to think of a cryptocurrency-integrated economy of money, information, and services because the possibility of using the Bitcoin network to send transactions worth pennies or even fractions of pennies (which would be necessary for a such an economy to exist) is not realistic in the current technological paradigm. Furthermore, the Bitcoin blockchain has long been overloaded with pending transactions, and the advent of the cryptoKitties game on Ethereum recently demonstrated that the younger platform has scaling problems of its own. 

The number  of people and other entities interacting with these blockchains today is small compared to the number that would need to access them if blockchain-integrated services were to go mainstream at the global level- thus, scaling has become a central concern in the world of blockchain development.[[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)     

# Introduction  

The increasing popularity of Bitcoin and other blockchain based payment systems lead to new challenges, in particular regarding scalability and transaction speed. During peaks of incoming transactions, the blockchain cannot process them fast enough and a backlog is created. A second major problem is transaction speed, the time from initiating a transaction until one can assume that the transaction has concluded, and is thus irreversible. 

With inter block times typically in the range of minutes and multiple blocks needed to reasonably prevent double spending, transactions take minutes to hours until the payment is confirmed. This may be acceptable for long-term Bitcoin investors, but not for everyday shopping or interacting with a vending machine. 

To solve both, scalability and speed, micropayment channel networks have been proposed. A micropayment channel provides a way to trustlessly track money transfers between two entities off-blockchain with smart contracts. [[2]](https://www.tik.ee.ethz.ch/file/a20a865ce40d40c8f942cf206a7cba96/Scalable_Funding_Of_Blockchain_Micropayment_Networks)

Second-layer scaling is the increase of capacity of blockchain networks through secondary solutions that lay on top of main protocols. Blockchain-based cryptocurrencies such as bitcoin, Etheruem, and Litecoin rely on second-layer scaling to process payments, transactions, and transfer data at faster speeds with lower costs. 

Public blockchain networks have consensus algorithms and protocols that require proof-of-work (PoW) and mining to verify and confirm transactions. On-chain transactions are directly picked up and verified by miners within the ecosystem. Off-chain or second-layer transactions are stored in channels that are first processed or settled outside of the main blockchain network and later confirmed by miners. 

As noted by Ethereum co-founder Vitalik Buterin, leading public blockchain networks and cryptocurrencies such as bitcoin, Ethereum, Bitcoin Cash, and Litecoin are currently limited to processing less than 10 transactions per second. An increase in the on-chain capacity of transactions on any of the public blockchain may lead to centralisation, as individual and independent node operators are no longer able to handle massive amounts of data. 

In September 2017, Buterin stand, “Bitcoin is currently processing a bit less than three transactions per second and if it goes close to four, it is already at peak capacity. Ethereum has been doing five per song and if it goes to above six, then it is also at peak capacity. On the other hand uber on average does 12 riders per second, Paypal several hundred, Visa several thousand, major stock exchanges tens of thousands, and in loT, you’re talking hundreds of thousands per second.”

There exists a tradeoff between flexibility and decentralisation in increasing the on-chain capacity of any public blockchain network without the implementation of second-layer solutions. [[3]](https://lunyr.com/article/Second-Layer_Scaling)

“Second layer” is a term used to denote all the platforms and protocols that host or process data so that the base layer blockchain in a network doesn’t have to. 

Base layer blockchains, like all blockchains, can only fit a certain amount of data into watch block. This feature, while necessary, has the effect of limiting how many transactions the network can process in a given period of time. By offloading activity from the main chain onto second layer platforms, the blockchain can handle significantly more transactions.[[4]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-2)   

Currently, the leading blockchain networks (in terms of number of users) process information too slowly to service a mainstream user base in the billions, or even in the tens of millions. While cryptocurrency has captured the public’s attention, many of the entities interacting directly with blockchain networks are still hobbyists, ideologies, and technologists. As Ethereum founder Vitalik Buterin himself recently noted, the Ethereum network is currently too ‘unscalable’. 

Blockchains may eventually become an integral part of our infrastructure, but they will only het the chance to play that role after scaling up to become many times more powerful than they are today.[[5]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-3)   

# Scaling Issues of Blockchain 

Since 2015, the popularity and usage of cryptocurrencies have increased dramatically. With a two-year span, the daily transaction volume of bitcoin and Ethereum blockchain networks have increased to 130 000 transactions per day, all of which are settled on-chain. 

According to Bitcoin Fees, a bitcoin fee predictor developed by earn.com founded by venture capitalist and A16Z partner Balaji Srinivasan, the fastest and cheapest bitcoin transaction fee as of December 16, 2017 is 420 satoshi per byte. Given that a median-size bitcoin transaction is 226 bytes, the fee bitcoin wallets recommend is estimated to be 94 920 satoshi or $16.7. 

The transaction fee of bitcoin varies depending on the size of the mempool, the holding area of unconfirmed transactions awaiting to be picked up and confirmed by miners. If the size of the bitcoin mempool is larger than 100 million bytes, high transaction fees are required for miners to prioritise the transactions ahead of others. The bitcoin community refers to the competition of transition fees as fee market, because users compete against each other to have transactions confirmed by miners on the network. 

High transactions  fees and scalability issues of the Bitcoin network restricts its ability to operate as a cryptocurrency and a medium of exchange, especially for small payments, On Ethereum, a decentralised computing network designed specifically for decentralised applications, expensive fees or gas costs create an inefficient and challenging environment for both developers and users, as it drives the cost of running decentralised applications significantly higher. 

Underling scalability issues of bitcoin, Ethereum, and every other cryptocurrency in the market are considered to the most urgent technical problems blockchain developers have to address moving forward. Buterin explained that scaling of Ethereum could take 3 to 5 years, while Litecoin creator Charlie Lee stated that scaling of bitcoin and Litecoin to an extent in which both networks can be used as effect currencies could take at least 5 years.

On CNBC’s Squawk Box, Lee stated, “I think we’re still maybe five years away before people actually start using bitcoin and lite coin in real world use as a currency.”

Scalability of public blockchains remain as urged issues for developers to address as several studies including a research paper released by Cambridge disclosed that only 5.8 million active users utilise cryptocurrencies on a regular basis, as of early 2017 and the number of active cryptocurrency users is increasing at an exponential rate.[[4]](https://lunyr.com/article/Second-Layer_Scaling)

# Second-Layer Scaling of Bitcoin 

On August 23 at block height 481 824 at 1:57 UTC by BTCC, the Bitcoin Core development team’s scaling solution and transaction malleability fix Segregated Witness (SegWit) was activated. The official integration of SegWit into the codebase of the Bitcoin protocol officially enabled second-layer solutions such as Lightning and MimbleWimble, by providing necessary infrastructures for secondary solutions to operate on top of. 

As of December 2017, three main bitcoin-based second-layer scaling solutions are being developed Rootstock (RSK), Lightning Network, and MimbleWimble. [[4]](https://lunyr.com/article/Second-Layer_Scaling)

# What is the Second Layer  

The second layer refers to all auxiliary platforms and protocols that act as layer where activities can be offloaded from main chains to save storage space- in some instances these digital spaces are blockchains, but they don’t function as standalone blockchains; the transactions that they support must eventually be verified on the base layer. [[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)

# Survey 

## Payment Channels 

A payment channel is an instrument that two users mutually create. It takes the form of a wallet in the Bitcoin context and of an EDCC(executable distributed code contract) also known as a smart contract, in an Ethereum-based environment. Its basic purpose is to allow two parties to execute a theoretically unlimited number of transactions off chain, settling accounts with the main chain only when the parties created the challe, deposit more funds into it, or cash out. 

As more transactions take place on the second layer, the amount of code written to the base layer decreases. This increases the main chain’s processing speed because it allows for more transactions to be processed in a single block. Cryptographic mechanisms encoded into the architecture of payment channels prevent participants from withdrawing funds that they are not entitled to. [[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)

## Scalable Funding of Bitcoin Micropayment Channel Networks 

In November 2017, three developers and engineers Conrad Burchert, Christian Decker, and Roger Wattenhofer from ETH Zurich and Blockstream co-wrote and released a study entitled “Scalable Funding of Bitcoin Micropayment Channel Networks,” introducing a solution to an underlying issue in Lightning and micro payment channel networks in general. 

One of the major issues with second-layer scaling solutions and micropayment channels is storing funds off-chain, outside of the actual blockchain networks. With bitcoin for instance, to run micropayment channels on top of Bitcoin protocol, funds are required to be stored or locked-in elsewhere for easy and simple access to the micropayment channels, allowing instantaneous and near-zero fee transactions. 

As the paper of Blockstream and ETH Zurich notes, “Two parties cooperating in a channel must lock funds into a shared account. The locked-in funds should be sufficient to provide enough capacity for peaks of transactions. There is a conflict of the two aims to have a low amount of funds locked up in a channel, while at the same time being flexible for these peaks.”

The solution introduced by Blockstream and ETH Zurich solved the issue of locked-in funds and increase in blockchain capacity by only broadcasting payment channels in case of disputes. With it, users are able to enter the micropayment channel system with one blockchain transactions and open many channels without stressing the original blockchain network. This method saves up to 96 percent fo the blockchain space, which can support more than 2 billion users with micropayments. [[3]](https://lunyr.com/article/Second-Layer_Scaling) 

## Second-Layer Scaling of Ethereum 

Ethereum is fundamentally and structurally different from bitcoin and as a result, second-layer scaling solutions of bitcoin are not applicable on the Ethereum blockchain network. Since its launch in 2015, Ethereum has operated as a protocol for decentralised applications. By nature, it is required to deal with more transactions and settlement of larger volumes of data than bitcoin. 

As demonstrated by Etherscan, the Ethereum blockchain network process nearly 1 million transaction on a daily basis, around 2.5 times more than the bitcoin blockchain. Unlike the bitcoin network, which primarily settled payments and transactions, the Ethereum network processes information for decentralised applications. 

In the first week of December 2017, a popular Ethereum-based decentralised collectibles game CryptoKitties triggered an Ethereuem network congestion, due to a rapid surge in its transaction volume and sale of digital cartoon kittens on the Ethereum blockchain. At one point, CryptoKitties accounted for 14 percent of Ethereum network’s daily transactions. As of December 16 2017, CryptoKitties account for 4 percent of the total Ethereum daily transaction volume, while decentralised cryptocurrency exchange EtherDelta accounts for 8.69 percent of the daily Ethereum transaction volume. 

In an interview with South Korean mainstream media outlet JoongAng, Ethereum co-founder Vitalik emphasised the necessity of second-layer scaling solution implementation, specifically software like Plasma and Riden that reduce congestion on the Ethereum network by creating interoperable and interconnected blockchain networks on top of Ethereum. [[3]](https://lunyr.com/article/Second-Layer_Scaling)

## Cross-Chain Atomic Swaps 

Atomic swaps offer the potentially vital service of easily trading one type of digital asset for another, at little or no cost to either party, and without relying on a cryptocurrency exchange as an intermediary. In a hypothetical future of many blockchains, this would be an important enabler of interoperability between blockchain ecosystems. 

To perform a cross-chain atomic swap, parties must set up tow multi-signature wallets (or multi-signature contracts, in the Ethereum-based networks) that are similar in structure to payment channels. These instruments are governed by cryptographic mechanisms which, among other things, prevent a single party from withdrawing the contents of both wallets. Additionally , any time one party withdraws received funds, a cryptographic ‘secret’ is automatically revealed to the counterparty, with enables it to withdraw as well. [[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)  

## The Lightning Network

The Lightning Network is essentially a web of many payment channels on the Bitcoin blockchain. It allows parties that have not opened a payment channel together to exchnage funds through intermediary channels. For instance, if Alice and Bob; Bob and Charlie; and Charlie and Diana have payment channels open, Alice can pay Diana by way of Bob and Charlie. [[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)

Bitcoin’s Lightning Network is being tested by many startups in the bitcoin, Litecoin and cryptocurrency industries. Three of the most active startups working on implementing Lightnign are ACINQ, Blockstream, and Lightning Labs. On December 6, ACINQ, Blockstream, and Lightning Labs achieved a major milestone in regards to Lightning implementation by entering versions 1 of the Lightning specifications. 

ACINQ founder Pierre-Marie Padiou explained, “This is the Lightning standard we’ve been working on for more than a year. There’s been a lot of work from us and from all participants. It’s a big milestone.”

During a similar timeframe, Blockstream engineer Christian Decker completed more than 70 successful tests of all of the active Lightning implementations. Successful tests of Decker enabled ACINQ, Blockstream, and Lightning Labs to send Lightning payments on the bitcoin mainnet, involving different compatible implemation. Essentially, the tests of Decker and a series of successful transactions processed by ACINQ proved the applicability and potential of Lightning as bitcoin micropayment and second-layer scaling technology. [[3]](https://lunyr.com/article/Second-Layer_Scaling)

## The Raiden Network

Raiden is a second-layer operating natively on Ethereum that focus on settling micropayment and small transactions. Raiden is attempting to build Ethereum as a potential peer-to-peer (P2P) cash system and retail payments platform by processing transactions with substantially lower fees. 

The official whitepaper of Raiden describes the solution as ‘Ethereum’s version of Bitcoin’s Lightning Network, enabling newer-instant, low-fee, scalable, and privacy-preserving payments.” Raiden bypasses global consensus by relying on hash-locked transfers called balance proofs. It enables users to create an unlimited number of micropayment channels, allowing users to initiate many P2P payments before broadcasting the final transactions to the original Ehtereum blockchain network. 

Raiden’s paper notes that transaction fees on the Ethereum network are independent on the actual amount of value transferred, unlike bitcoin. Therefore, on-chain translations on Ethereum are particularly beneficial and useful in processing large-scale payments with a small few for less than $1. 

With relatively small payments less than $1, it is not possible or incredibly impractical to broadcast transitions on-chain. Raiden is capable of processing small payments with almost no fees instantaneously. Such efficiency opens the Ethereum network to various applications, including online video streaming, donations, small payments for goos and services and cross-blockchain swaps. 

The Raiden Network also scales linearly, providing a solution to an internet problem fall existing blockchains in the market. The Raiden Network paper further read, “in stark contrast, capacity of the Reaiden Network scales linearly with the number of users, leading to an efficient and future-proof, decentralised transfer network.” [[3]](https://lunyr.com/article/Second-Layer_Scaling) 

## RSK

RSK, formeraly known as Rootstock, is a Bitcoin sidechain onto which the Ethereum Virtual Machine has been cloned, meaning that the platform suports EDCCs. While RSK is not a second layer chain, members of the team behind the project have plans for some rather unusual second layer apparatus that they intend to build on top of it. [[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)

Rootstock (RSK), which is structurally similar to Ethereum is an open-source project and smart contracts platform that operates with a 2-way peg to the main Bitcoin blockchain. RSK itself exists outside of the Bitcoin protocol but is capable of sharing, sending, and receiving information with, to, and from the original Bitcoin blockchain network. The 2-way peg between RSK and bitcoin allows near instant payments and higher scalability, as it relieves the Bitcoin network of network congestion, stress, high transactions volumes, and maximised mempool. [[3]](https://lunyr.com/article/Second-Layer_Scaling)

## Mimblewimble

Mimblewimble is a privacy-focused solution that focuses on providing privacy and fungibility to bitcoin users. In bitcoin and cryptocurrencies in general, privacy is deeply rooted into scalability because private transactions are significantly smaller in size in contrast to normal transactions. As such, solutions like Mimblewimble are capable of decreasing the size of transactions on public blockchain networks and decreasing transaction fees as a result. [[3]](https://lunyr.com/article/Second-Layer_Scaling)   

## Lumino and the LTCP

Lumino is essentially a Lightning-type network built atop the RSK sidechain. One of its most distinguishing features is its proposed integration with the Lumino Transaction Compression Protocol (LTCP), a system of computing logic that significantly reduces the quantity of code that must be written to the RSK chain in order to create, top up, or settle a payment channel. It uses a series of references to other transactions, to abbreviate the code that must be written to the base layer in order to open, deposit funds to, or close Lumino’s constituent payment channels. LTCP’s white paper, written by Sergio Demian Lerner, projects that it could enable the RSK platform to accommodate up to a billion users. [[1]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-1)    

## Plasma 

Among the second layer solutions currently being pioneered, Plasma has been one of the most talked-about. In this configuration, there are many blockchains that branch out from one another in what Joseph Poon, who co-wrote the foundational paper on Plasma with Ethereum creator Vitalik Buterin, describes as a tree-like formation. 

To establish a Plasma network, one must publish a set of smart contracts, also known as EDCCs, to main chain, which lay out the rule that will govern the tree of Plasma blockchains. A Plasma blockchain that branches off the base layer is called a ‘child chain’.  

In a Plasma blockchain, the validators (whose role is akin to that of miners on the main chain) report the activity taking place on a child chain since its last block was mined, they deliver to the base layer a ‘blockheader hash’ (a string of characters cryptographically derived from information related to the latest block’s contents) saving precious space there.

If other users believe that a validator has misrepresented what happened on a Plasma chain, they can submit fraud proofs contacting contradictory data. If a fraud proof successfully disproves a validator’s representations, that Plasma chain will be rolled back to the last block that has not been successfully disputed, meaning that the transitions which occurred after the last validated block will effectively be undone. 

Because a Proof of Stake (PoS) consensus mechanism would be used to verify transactions on Plasma blockchains, one would have to deposit funds into a Plasma EDCC in order to become eligible to be a validator. If a validator submits an invalid block, they are stripped of these funds. 

If users suspect untoward activity on a child chain where they have some funds, they can simply take their tokens and jump from that child chain onto the ‘parent chain’ from which it branches. If necessary, they can repeat this process ad infinitum until they’re all the way back on the main chain. 

To borrow a metaphor from Poon, a tree of Plasma chains is like a court system in which higher counts can overturn the rulings of lower courts, and the base layer functions as the supreme court. It would be inefficient for the supreme court to hear every case, but avenues exist through which any case can reach the supreme court. [[4]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-2)   

Introduced by bitcoin’s Lightning co-author Joseph Poon and Buterin, Plasma is a second-layer scaling solution which creates an incentivised ecosystem in which smart contracts can continue operations autonomously via network transaction fees. Through successful implementation, Plasma could increase or scale the amount of scale updates per second to billions, allowing the Ethereum network to run more active and larger decentralised applications. 

The working pair of Plasma entitled “Scalable Autonomous Smart Contracts” released on August 11 2017, read “we compose blockchains into a tree hiearchy, and treat each as an individual branch blockchain with enforced blockchain history and MapReducible computation committed into merle proofs. By framing one’s ledger entry into a child blockchain which is enforced by the parent chain, one can enable incredible scale with minimised trust.”

In essence, Plasma represents a system of interconnected blockchains arranged in a tree structure. Each blockchain network on top of the main Ethereum network is incentivised to carry out certain operations. Consequently, the main Ethereum blockchain network is not burdened to settle every single operation and process. Plasma undertakes the vision of preventing a user from verifying every single transaction that is sent to the system and the blockchain network. 

Given that only merkelised commitment are sent to the main Ethereum blockchain periodically, Plasma s a second-layer solution also enables scalable, low cost transaction and computation, as the paper of Poon and Buterin emphasised. [[3]](https://lunyr.com/article/Second-Layer_Scaling)  

## Sharding 

In a sharing system, users are siloed in ‘shards’, blockchains that act as ‘galaxies’ unto themselves, which communicate with the main chain but are distinct from it. 

This way, rather than millions of users conducting all their transactions on the base layer, as is currently the case, there are many shards, each of which supports thousands of users and processes their transactions internally. These walled-off galaxies are maintained by a single ‘validator manger contract’ (VMC) on the main chain. 

In early implementations, communication across shards is expected to be impossible. 

Validators are tased with creating ‘collations’ essentially ‘blocks’, for a particular shard over a specified period of time. Any validator can be sleeted to create collations for any shard, and they are to be selected shortly before the period begins, giving them minimal opportunity to hatch nefarious schemes to cheat that shard’s users. 

Validators need only create collations relating to the shard for which they’ve been selected, saving main chain miners and shard validators alike significant computational power and disk space compared to what is required by the system currently in place on the Ethereum network. [[4]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-2)   

## State Channels 

Payment channels falls into the category of state channels. While a payment channel only allows for the transfer of funds, users of a state channel can also make non-monetary changes to the star of a ledger without needing to individually record each change to the base layer. These channels, can support both finical and non-financial applications of blockchain technology. 

Ledger Labs founder Jeff Coleman, who is credited as the first to theorise the mechanism, envisions a ‘judge’ contract the would adjudicate on the actual state of a channel before that state is recorded to the main chain in the event of a disagreement. He appears to anticipate that conflicts over a channel’s state will not arise often, however, meaning that the adjudicating contract would only be called in exceptional cases. [[5]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-3)   

## Atomic Multi-Path Payments (On the Lightning Network)

In a February email, Lightning Labs’ chief technology officer and co-founder, Olaoluwa Osuntokun, and head of Cryptographic Engineering, Conner Fromknecht, suggested a novel use for Lighting networks: the atomic multi path payment. 

This scheme allows a payment to be automatically broken into pieces and prevents delivered payment fragments from being withdrawn until the entire amount is available to the recipient. By doing away with the need to send funds in a lump sum, it eliminates the requirement that a payment path be made up entirely of high-capacity channels. 

Large transfers tend to make payment channels less useful because they leave the funds in those channels skewed towards one party. When this happens to too many payment channels, more channels must be opened and/or funds must be deposited into existing channels to keep the network functional. Atomic multi-oath payments couple reduce the frequency with which channels need to be opened or topped up to compensate for channel imbalance. This would mean less funding being spent on main chain transaction fees in the name of maintaining network utility, and therefore lead to a reduction in costs associated with using the Lightning Network.  

The scheme would also enhance user privacy by keeping intermediary senders (those simply forwarding along a payment) from identifying the ultimate sender, the ultimate receiver, and the total amount of funds transferred between those two parties. [[5]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-3)   

## Plasma Cash 

Plasma Cash builds on the basic tree-like structure of Plasma but incorporates several other features as well. The most notable of theses is a mechanism in the main chain EDCCs (aka smart contracts) that establish the rules that will govern the set of Plasma blockchains. Whereas the Plasma systems proposed in the past would create an ‘arbitrary unit of Plasma Ether’ when users deposit Ether tokens into a Plasma contract, Plasma Cash contracts would create tokens within the Plasma network that are linked to unique strings of characters, which act as IDs. 

It is important to note that in Ethereum network, some data relating to transactions are compiled into a data structure known as a Merkle tree. In a Plasma Cash system, a transition that spends a coin with a certain ID cannot be arbitrarily included ‘anywhere in the Merkle tree’, instead, it must be specifically recorded in the position corresponding to that coin’s ID. 

The primary benefit of this feature is that it dramatically reduces the amount of data that clients are required to store. Earlier Plasma proposals required that every user of a given Plasma blockchain download everyone of that chain’s Plasma blocks. Plasma Cash users, on the other hand, would need to verify the ‘availability and correctness of the Plasma chain’ only at the positions in the Merkle tree that correspond to ‘any coins that they own and any coins that they care about’.

To send funds using such a blockchain, one must also send “proof data’ of those tokens’ histories to prove ownership over them. Periodically ‘checkpointing’ state of a Plasma chain on the main chain would allow the reports of these histories to be abbreviated, so that only proof data that has been generated since the last checkpointing exercise needs to be included for a transaction to be processed. 

Buterin named several benefits of adopting such a system, including the fact that malicious actors in earlier Plasma contracts. Hackers in a Plasma Cash world would have to steal specific tokens from specific entities, which is much harder to do, considering that each coin’s history is recorded individually. Also, a cryptocurrency exchange built atop Plasma Cash could provide order matching services without taking custody of users’ tokens, which could drastically reduce the risk of large-scale-thefts by leaving customers in constant possession of their own funds. [[5]](https://www.ethnews.com/a-survey-of-second-layer-solutions-for-blockchain-scaling-part-3)   

## Proof of Stake 

Several cryptocurrencies, Ethereum in particular, are exploring the possibility of utilising a hybrid proof of work (PoW) and Proof-of-Stake (PoS) to maximise the potential of senior-layer solutions. 

Plasma is an example of a second-layer solution that uses PoS to relieve the Ethereuem blockchain network of its load and congestion. Because Plasma is a second-layer solution in which various interconnected and interoperable blockchain networks exist in a tree-like structure and each blockchain has users with stake or stakeholders, it does not require PoW to challenge invalid state transitions. 

Stakeholders of each blockchain within the Plasma system will automatically challenge any invalid information or transition, and escalate it to the main Ethereum blockchain to ensure that the Plasma network remains valid. 

The primary purpose of utilising PoS with second-ayer solutions on Ethereum, as explained by Ethereum team leader and Solidity creator Christian Reitwiessner is to prevent users from being overloaded with transaction verification and this slowing down the network. Reitweissner noted, “scalability is only achieved once a user does not have to verify every single transaction that is sent to the system.” [[3]](https://lunyr.com/article/Second-Layer_Scaling)  

## Understanding Blockchain Fundamentals: Byzantine Fault Tolerance 

Blockchains are inherently decentralised system which consists of different actors who act depending on their incentives and on the information that is available to them. 

Whenever a new transaction gets broadcasted to the network, nodes have the option to include that transaction to their copy of they ledger or to ignore it. When a new transaction gets broadcasted to the network, nodes have the option to include that transaction to their copy of their ledger or to ignore it. When the majority of the actors which comprise the network decide n a single state, consensus is achieved. [[6]](https://medium.com/loom-network/understanding-blockchain-fundamentals-part-1-byzantine-fault-tolerance-245f46fe8419)

## Oracles 

Blockchain oracles provide the necessary data to trigger smart contracts to execute when the original terms of the contract are met. These conditions could be anything associated with the smart contract- temperature, payment completion, price changes, etc. Oracles are the only way for smart contracts to interact with data outside of the Blockchain environment. 

Oracles are radically important. Smart contracts cannot function without some data source. Without access to these sources of information, use cases for smart contracts drop to use a tiny fraction of their potential. 

However, with these systems, smart contracts have real world applications in virtually every field available. Once data hits the Blockchain, the information can be used to execute the contracts and provide use cases, which can disrupt industries across the board. 

Why can’t decentralised applications communicate with the real world without oracles?

There is a fundamental difference of formats. Blockchain is deterministic, meaning that is a reflection of a specific series of events which take place one after another in sequential order- series of transactions. Accessing information outside of the chain would require data points that are not sequential, and would therefore be impossible for Blockchain to use or make sense of. This aspect of Blackchin gives it immutability, but reduces flexibility. 

The off-chain world, however, is non-deterministic, meaning that there is no recording of the events in the specific sequence that they have taken place, which creates problems with transparency. Data points can be generated from and understood at any point, providing incased flexibility, but difficult in communicating with the blockchain. 

This foundation distinction makes the two worlds incompatible with each other by default, and only the presence of an oracle can make two-way communication between them possible. 

Because oracles are, themselves, smart contracts, designed to interact with the blockchain by providing necessary data, they require developers with expertise in both off-chain and decentralised fields. 

The recent and profound need for external data on Blockchain has given rise to new and interesting developments in the space. For example, oracles would allow blockchain connections to any existing API, allow payments using traditional payment networks from blockchain, and would allow interchain connections between smart contracts and other blockchains. 

The marketplace for these highly specialised middleware software models is growing rapidly , and, as new ways to utilise blockchain technology are being conceived for everyday, the demand will only increase. 

Currently, the marketplace for these types of contracts have continued to expand, and is being led by several companies that are active in developing oracles. Oraclise has been an industry leader in oracle technology. Other startups like ChainLink and Blocksense are also seeking to take market share in this area. Finally, large scale corporations (IBM and Microsoft) are seeing the potential for huge market presence and are developing these platforms now. 

As the increasing number of use cases for smart contracts continues to rise, the need for new oracle structures will also rise as the structural framework that makes smart contracts possible. This will drive increasing investment  and design into the market space, and new innovations will make blockchain-to-web communication more simple and elegant. 

One of the more likely future trends is the development of a unified, integrated platform for communication between blockchain and the outside world. Standardised tools and interfaces make it easier for both the developers and the users of blockchain-enabled services. [[7]](https://cointelegraph.com/explained/blockchain-oracles-explained)



## Contributors

- [https://github.com/Kevoulee](https://github.com/Kevoulee)