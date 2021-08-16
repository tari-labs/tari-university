# Understanding Byzantine Fault-tolerant Consensus

- [Background](#background)
- [Randomized Gossip Methods](#randomized-gossip-methods)
  - [Summary](#summary)
  - [Video](#video)
  - [Slides](#slides)
- [Byzantine Fault Tolerance, Blockchain and Beyond](#byzantine-fault-tolerance-blockchain-and-beyond)
  - [Summary](#summary-1)
  - [Video](#video-1)

## Background

When considering the concept of consensus in cryptocurrency and cryptographic protocols, the Byzantine Generals Problem
is often referenced, where a protocol is described as being Byzantine Fault Tolerant (BFT). This stems from an analogy,
as a means to understand the problem of distributed consensus.

**To classify Byzantine failure:**
If a node in a system is exhibiting Byzantine failure, it is called a traitor node. The traitor (which is a flaky or
malicious node) sends conflicting messages, leading to an incorrect result of the calculation that the distributed
system is trying to perform.

## Randomized Gossip Methods

<div>
  <p style="float: left;">
    <img src="sources/dahlia_malkhi.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Dr. Dahlia Malkhi</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Ph.D. Computer Science
    <br>
    <br>
  </p>
</div>

### Summary

*"Randomized Gossip Methods"* by Dahlia Malkhi, PWL Conference, September 2016.

As an introduction, gossip-based protocols are simple, robust, efficient and fault tolerant. This talk provides insight
into gossip protocols and how they function, as well as the reasoning behind the instances in which they do not function. It
touches on three protocols from randomized gossip methods: Rumor Mongering, which spreads gossip in each communication;
Name Dropper, which pushes new neighbors in each communication; and Scalable Weakly-consistent Infection-style Process
Group Membership (SWIM), which pulls a heartbeat in each communication.

### Video

**Note:** Transcripts are available [here](https://github.com/papers-we-love/pwlconf-info/blob/master/2016/dahlia-malkhi/dahlia_pwlconf_captions.srt).

<iframe width="1000" height="800" src="https://www.youtube.com/embed/Gxf5glthqrk" frameborder="0"
allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

### Slides

<embed src="https://speakerd.s3.amazonaws.com/presentations/c1c33de400a44e90911569ce999bde95/PWL-Sep-2016hotel.pdf"
type="application/pdf" width="750" height="600" />

<br>

## Byzantine Fault Tolerance, Blockchain and Beyond

<div>
  <p style="float: left;">
    <img src="sources/ittai_abraham.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Dr. Ittai Abraham</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Ph.D. Computer Science
    <br>
    <br>
  </p>
</div>

### Summary

*"BFT, Blockchain and Beyond"* by Ittai Abraham, Israel Institute for Advanced Studies, 2018.

This talk provides an overview of blockchain and decentralized trust, with the focus on Byzantine Fault Tolerance (BFT).
Traditional BFT protocols are assessed and compared with the modern Nakamoto Consensus.

The presentation looks at a hybrid solution of combining traditional and modern consensus mechanisms. The talk delves
into the types of consensus; asynchrony, synchrony and partial synchrony, and provides a brief history of all three, as well as
their recent implementation and responsiveness.

In addition, the fundamental trade-off of decentralized trust is assessed, and performance,
decentralization and privacy are compared.

### Video

**Part 1**
<iframe width="1000" height="800" src="https://www.youtube.com/embed/N_3r-NkBUTk" frameborder="0" allow="accelerometer;
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

**Part 2**
<iframe width="1000" height="800" src="https://www.youtube.com/embed/c72itS8BfC0" frameborder="0" allow="accelerometer;
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
