---
layout: post
title:  Pool Mining
date:   2020-05-04 15:00:00 +0300
image:  '/images/banner-05.jpg'
category: mining
tags:   [mining]
usemathjax: true
featured:
excerpttext: Pool software is software which allows many cryptocurrency miners to be able to be connected to it so that resources are pooled over the network...
---

## Table of Contents

- [Background](#background)
- [Shares](#shares)
- [Pool Types](#pool-types)
- [Calculating Rewards](#calculating-rewards)
- [Port Difficulty](#port-difficulty)
- [Stratum Mining](#calculating-rewards)


## Background

Pool software is software which allows many cryptocurrency miners to be able to be connected to it so that
resources are pooled over the network and the block reward split between them based on some predetermined method
implemented by the pool software based on the work that the miner contributed to the probability of finding a block.

In most pool implementations the more hashpower a miner can contribute to the pool the higher the reward will be for the miner.
Miners are typically rewarded at the end of each mining round for the contribution of hashpower they made to the pool,
although that is not always the case.

A mining round is usually calculated from the time the pool could add a block to
the chain to the time that it adds a block to the chain. This means that a mining round could be anywhere between a
few minutes to many hours.

Many pools implement a system that only sends the rewards to a miners' wallet address once rewards have accumulated
above a (usually settable per wallet address) minimum payout threshold. This both prevents numerous dust transactions on
the blockchain for every miner in the pool and ensures the minimum reasonable transaction fee payable for the reward to
be transferred to the miner from the pool. Transaction fees for transferring rewards to the miners are due to the reward
being issued to the pools' wallet initially, thereafter it is normal transaction. The absolute minimum payout is
usually fixed so that there is at least enough of the reward to cover the fee a pool charges to pay out the reward.

## Shares

Shares are a concept central to how pools function, each potential solution a miner contributes towards finding a block
is called a share.

There are also different kinds of shares, for example:
1) Accepted - A valid potential block solution which is used in reward calculation for the mining round.
2) Rejected - An invalid solution to a block, this usually indicates a problem with how a miner is configured.
3) Expired - An accepted share which was not used in calculating rewards for the miner for the mining round.
4) Stale - Equivalent to an expired share.

## Pool Types

Like there are many different kinds of miners, there are also a few types of mining pools.

The Solo Mining Pool is a pool which allows miners to compete for the reward, since the reward is only paid out to
the individual miner who finds the block. This is the least popular of the pools but may still be useful to
a mining operator operating a large warehouse of mining hardware where they would prefer a large reward in a single
transaction instead of many small individual rewards, which would help them save on fees.

The Proportional Mining pool is the most prominent kind of mining pool currently available. Miners are paid out
proportionally to the shares they contribute to finding a block. Various methods of reward payout calculations
exist and are covered in the next section.

The Peer-to-Peer Mining Pool (P2Pool) decentralizes the pool server. This removes the possibility of cheating by the
pool operator and also removes the server being a single point of failure. Miners work on a side blockchain called a
share chain, mining at a lower difficulty at a rate of one share block at a smaller interval of the main chain.
Once a share block reaches the network target it is transmitted to the main chain. Miners are then rewarded proportionally
to the shares submitted prior to the target block. This comes at a cost however as each miner needs to run a full node,
bearing the weight of both hardware expenses and network bandwidth.

The Multipool is a special category of pool which switches between different altcoins and constantly calculates which
coin is most profitable to mine at that present time. The two key factors in that calculation is block time and
exchange price. Multipools usually automatically exchange all altcoins mined to a singular mainstream coin to reduce the
need of keeping various wallets for each coin. Multipools help increase or at least stabilize the value of the coin they
exchange the altcoins for. [[1]]


## Calculating Rewards

The reward a miner receives for a mining round is dependent on what payout method a pool implements, this can
vary from pool to pool. Some could be considered more fair than others but ultimately it is up to the miners'
operator to decide which pool is the best fit for them on what is presently available for the cryptocurrencies
they intend to mine.

Common payout methods include, but are not limited to:
1) Pay-Per-Share (PPS): Miners earn a guaranteed income based on the probability that the pool mines a block, but
 not the actual performance of the pool. [[1]][[2]]
2) Full Pay-Per-Share (FPPS): Same as PPS except the pools also include transaction fees and the block subsidy in the
payout. Usually leading to larger rewards for participant miners. [[2]]
3) Pay-Per-Last N Shares (PPLNS): This pays out rewards proportionally to the last number (N) of shares contributed at
the time of block discovery. Only the most recent N shares are considered, the rest are excluded. [[1]][[2]]
4) Shared Maximum Pay-Per-Share (SMPPS): The same as PPS except that the rewards are based on the rewards earned by the
pool, so the pool can never pay out more than it earns. [[2]]
5) Equalized Shared Maximum Pay-Per-Share (ESMPPS): Same as SMPPS but rewards are distributed equally to all miners
in the pool. [[3]]
6) Recent Shared Maximum Pay-Per-Share (RSMPPS): The same as SMPPS except that more recent shares are weighted more
heavily than older shares in the mining round. This means that shares contributed early are worth less than
those contributed later in the round. [[2]]
7) Score Based System: Very similar to RSMPPS except that the scoring average is a rolling average of your mining hash
rate for the round. If your shares contributed are steady and constant, the reward will be roughly constant as well. If
this is not the case you will be rewarded based on an adjusted hash rate for the round. [[2]]
8) Double Geometric Method: This method is a cross between PPLNS and a geometrically calculated reward that equalizes
payouts against the duration of a mining round. Longer rounds yield higher rewards than shorter rounds. [[1]][[2]]

## Port Difficulty

Some mining pools expose different ports for different difficulties.

Mining at lower difficulties is possible to do since the target difficulties and the actual achieved difficulty of a hash
are different. A lower difficulty means more hashes will be accepted as valid, however the achieved difficulty of a random hash
can be orders of magnitude higher than the target difficulties.

Ports usually exposed are as follows:
1) High: Intended for Ant Miners and ASICs.[[4]]
2) Medium: Inteded for GPU Miners. [[4]]
3) Low: Intended for CPU and older mining hardware. [[4]]

Some pools also allow miners to select their difficulty or automatically suggest a difficulty to a miner based on the
capabilities of the miner itself. The lower the difficulty being mined by a miner, the lower that share is worth,
even if it is by chance the share that results in a new block.

This is not to be confused with cryptocurrencies which expose different ports for different networks (e.g testnet and stagenet).

## Stratum Mining

Stratum mining and its' counterpart GBT (getblocktemplate) are distributed pooled mining protocols that allow the client
to generate work instead of having to rely on network-based pooling servers.

Both protocols above are replacements for the obsolete getwork mining protocol. [[5]] Additionally stratum was
originally designed for the lightweight Bitcoin client Electrum.

Stratum is simply a line based protocol which uses plain tcp sockets. The payload is encoded as json-rpc messages. A
client simply opens a socket and writes a request to the server in the form of JSON messages finished with a newline
character. Every line received by the client is again a valid JSON-RPC fragment containing the response.

A stratum proxy can be used to connect miners which only support the old getwork protocol to stratum.

The protocol is vulnerable in its' present form to eavesdropping attacks as network traffic is not encrypted and has
a number of other shortcomings due to it being developed without the input of the wider community. [[5]][[6]]

Development efforts on Stratum V2 have addressed many issues present in Stratum V1. [[6]]

## References

[[1]] Wikipedia: "Mining Pool" [online]. Available: <https://en.wikipedia.org/wiki/Mining_pool>. Date accessed:
2020&#8209;10&#8209;21.

[1]: https://en.wikipedia.org/wiki/Mining_pool
"Mining Pool"

[[2]] Dummies: "What Is Pool Mining?" [online]. Available: <https://www.dummies.com/personal-finance/investing/what-is-pool-mining/>. Date accessed:
2020&#8209;10&#8209;21.

[2]: https://www.dummies.com/personal-finance/investing/what-is-pool-mining/
"Mining Pool"

[[3]] Bitcoin: "Comparison of Mining Pools" [online]. Available: <https://en.bitcoin.it/wiki/Comparison_of_mining_pools> Date accessed:
2020&#8209;10&#8209;21.

[3]: https://en.bitcoin.it/wiki/Comparison_of_mining_pools
"Comparison of Mining Pools"

[[4]] Reddit: "Difficulty Ports" [online]. Available: <https://www.reddit.com/r/gpumining/comments/7qiqyz/difficulty_ports/> Date accessed:
2020&#8209;10&#8209;21.

[4]: https://www.reddit.com/r/gpumining/comments/7qiqyz/difficulty_ports/
"Difficulty Ports"

[[5]] Bitcoin: "Stratum Mining Protocol" [online]. Available: <https://en.bitcoin.it/wiki/Stratum_mining_protocol> Date accessed:
2020&#8209;10&#8209;21.

[5]: https://en.bitcoin.it/wiki/Stratum_mining_protocol
"Stratum Mining Protocol"


[[6]] Stratum Protocol: "Stratum V2" [online]. Available: <https://www.stratumprotocol.org/> Date accessed:
2020&#8209;10&#8209;21.

[6]: https://www.stratumprotocol.org/
"Stratum Protocol"
