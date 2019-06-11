# Understanding BFT Consensus

- [Background](#background)
- [Gossip Protocols](#gossip-protocols)
  - [Summary](#summary)
  - [Video](#video)
  - [Slides](#slides)
- [BFT, Blockchain and Beyond](#bft-blockchain-and-beyond)
  - [Summary](#summary-1)
  - [Video](#video-1)
  

## Background

When considering the concept of consensus in cryptocurrency and cryptographic protocols, the Byzantine Generals Problem is often referenced - where a protocol is described as being byzantine fault tolerant (or BFT). This stems from an analogy, as a means to understand the problem of distributed consensus.

**To classify Byzantine failure:**
It is said that some node in the system, which is exhibiting Byzantine failure is a traitor node. The traitor can send conflicting messages, leading to an incorrect result of the calculation that the distributed system is trying to perform where the cause is a flaky or malicious node.

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

"*Randomized Gossip Methods*" by Dahlia Malkhi, PWL Conf, September, 2016.

The talk touches on three protocols from randomized gossip methods; Rumor Mongering which spreads gossip in each communication, Name Dropper which pushes new neighbors in each communication and SWIM which pulls a heartbeat in each communication.   

### Video

_Note:_
- Transcripts available [here](https://github.com/papers-we-love/pwlconf-info/blob/master/2016/dahlia-malkhi/dahlia_pwlconf_captions.srt).

<iframe width="750" height="600" src="https://www.youtube.com/watch?v=Gxf5glthqrk" frameborder="0" 
allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


### Slides

<embed src="https://speakerdeck.com/paperswelove/randomized-gossip-methods-by-dahlia-malkhi" 
type="application/pdf" width="750" height="600" />

<br>


## BFT, Blockchain and Beyond

<div>
  <p style="float: left;">
    <img src="sources/ittai_abraham.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Dr. Ittai Abraham</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp; Ph.D. Computer Science
    <br>
    <br>
  </p>
</div>


### Summary

"*BFT, Blockchain and Beyond*" by Ittai Abraham, Israel Institie for Advaced Studies, 2018 

This talk provides an overview of blockchain, decentralised trust, with focus on Byzantine Fault Tolerance (BFT). Traditional BFT protocols are assessed and compared with the modern Makamoto Consensus. 

The presenation moves on to looking at a hybrid solution of combining traditional and modern consenus. 

### Video

<iframe width="750" height="600" src="https://www.youtube.com/watch?v=N_3r-NkBUTk" frameborder="0" allow="accelerometer; 
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>



