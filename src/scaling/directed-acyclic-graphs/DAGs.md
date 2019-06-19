# Directed Acyclic Graphs

- [Background](#background)
- [Braiding the Blockchain](#braiding-the-blockchain)
  - [Summary](#summary)
  - [Video](#video)
  - [Slides](#slides)
- [GHOST-DAG](#ghost-dag)
  - [Summary](#summary-1)
  - [Video](#video-1)
  - [Slides](#slides-1)
- [SPECTRE](#spectre)
  - [Summary](#summary-2)
  - [Video](#video-2)
- [PHANTOM](#phantom)
  - [Summary](#summary-3)
  - [Video](#video-3)


## Background

The principle of 
[Directed Acyclic Graphs](../layer2scaling-landscape/layer2scaling-survey.md#directed-acyclic-graph-dag-derivative-protocols) 
(DAGs) in blockchain is to present a way to include traditional off-chain blocks into the ledger, which is governed by 
mathematical rules. The main problems to be solved by the DAG derivative protocols are:

1. inclusion of orphaned blocks (decrease the negative effect of slow propagation); and
2. mitigation against selfish mining attacks.


## Braiding the Blockchain

<div>
  <p style="float: left;">
    <img src="sources/bob_mcelrath.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Dr. Bob McElrath</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Ph.D. Theoretical Physics
    <br>
    <br>
  </p>
</div>


### Summary

*"Braiding the Blockchain"* by Dr. Bob McElrath, Scaling Bitcoin, Hong Kong, December 2015.

This talk discusses the motivation for using Directed Acyclic Graphs (DAGs), which are orphaned blocks, throughput and more 
inclusive mining. New terms are defined to make DAGs applicable to blockchain, as it needs to be more specific: Braid vs. 
DAG, Bead vs. block, Sibling and Incest.

The Braid approach:

- incentivizes miners to quickly transmit beads;
- prohibits parents from containing conflicting transactions (unlike GHOST or SPECTRE);
- constructs beads to be valid Bitcoin blocks if they meet the difficulty target.


### Video

**Note:** Transcripts are available [here](https://diyhpl.us/wiki/transcripts/scalingbitcoin/hong-kong/braiding-the-blockchain/).

<iframe width="750" height="600" src="https://www.youtube.com/embed/8IlZ80mPWfE" frameborder="0" 
allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


### Slides

<embed src="https://hongkong2015.scalingbitcoin.org/files/presentations/DAY2/2_breaking_the_chain_1_mcelrath.pdf" 
type="application/pdf" width="750" height="600" />

<br>


## GHOST-DAG

<div>
  <p style="float: left;">
    <img src="sources/aviv_zohar.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Aviv Zohar</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Prof. at The Hebrew University and Chief Scientist @ QED-it
    <br>
    <br>
  </p>
</div>



### Summary

*"The GHOST-DAG Protocol"* by Yonatan Sompolinsky, Scaling Bitcoin, Tokyo, October 2018.

This talk discusses the goal going from chain to DAG being to get an ordering of the blocks
that does not change when time between blocks approaches block propagation time; and security that does not break at 
higher throughputs. Terms introduced here are *Anticone* (the view of the DAG a block sees in the past and the future) 
and $k​$-cluster (a set of blocks with an *Anticone* at most $k​$). The protocol also makes use of a greedy algorithm in 
order to find the optimal $k​$-cluster at each step as it attempts to find the overall optimal $k​$-cluster.


### Video

**Notes:**

- Start watching at 30min or open the video in a separate tab [here](https://youtu.be/3Hksieg5GdM?t=1791).
- Transcripts are available [here](http://diyhpl.us/wiki/transcripts/scalingbitcoin/tokyo-2018/ghostdag/).

<iframe width="750" height="600" src="https://www.youtube.com/embed/3Hksieg5GdM" frameborder="0" allow="accelerometer; 
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


### Slides

<embed src="https://tokyo2018.scalingbitcoin.org/files/Day2/the-ghost-dag-protocol.pdf" 
type="application/pdf" width="750" height="600" />

<br>


## SPECTRE

<div>
  <p style="float: left;">
    <img src="sources/aviv_zohar.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Aviv Zohar</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Prof. at The Hebrew University and Chief Scientist @ QED-it
    <br>
    <br>
  </p>
</div>



### Summary

*"Scalability II - GHOST/SPECTRE"* by Dr. Aviv Zohar, Technion Summer School on Decentralized Cryptocurrencies and 
Blockchains, 2017.

This talk discusses the application of DAGs in the SPECTRE protocol. Three insights into the Bitcoin protocol are 
shared: DAGs are more powerful; Bitcoin is related to voting; and amplification. These are discussed in relation to 
SPECTRE, while properties sought are consistency, safety and weak liveness. Voting outcomes are strongly rejected, 
strongly accepted or pending.


### Video

<iframe width="750" height="600" src="https://www.youtube.com/embed/5mEaBXl3BMM" frameborder="0" allow="accelerometer; 
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

<br>


## PHANTOM

<div>
  <p style="float: left;">
    <img src="sources/yonatan_ompolinsky.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Yonatan Sompolinsky</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Founding Scientist, Daglabs
    <br>
    <br>
  </p>
</div>



### Summary

""BlockDAG Protocols - SPECTRE, PHANTOM"* by Yonatan Sompolinsky,  Blockchain Protocol Analysis and Security 
Engineering, Stanford, 2018.

This talk introduces the BlockDAG protocol PHANTOM, and motivates it by virtue of blockchains not being able to scale 
and BlockDAGs being a generalization of blockchains. The mining protocol references all tips in the DAG (as opposed to 
the tip of the longest chain) and also publishes all mined blocks as soon as possible (similar to Bitcoin). Blocks 
honestly created (i.e. honest blocks) will only be unconnected if they were created at approximately the same time. 
PHANTOM's goal is to recognize honest $k$-clusters, order the blocks within and disregard the rest.


### Video

<iframe width="750" height="600" src="https://www.youtube.com/embed/57DCYtk0lWI" frameborder="0" allow="accelerometer; 
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

