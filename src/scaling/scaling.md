# Scaling

## Purpose

The blockchain scalability problem refers to the discussion concerning the limits on the transaction throughput a 
blockchain network can process. It is related to the fact that records (known as blocks) in the bitcoin blockchain are 
limited in size and frequency [[1]].

## Definitions

- From **Bitcoin StackExchange**: The term "on-chain scaling" is frequently used to exclusively refer to 
increasing the blockchain capacity by means of bigger blocks. However, in the literal sense of the term, it should refer 
to any sort of protocol change that improves the network's capacity at the blockchain layer. These approaches tend to 
provide at most a linear capacity increase, although some are also scalability improvements [[2]].

  Examples:

  - blocksize/blockweight increase;
  - witness discount of segregated witness;
  - smaller size of Schnorr signatures;
  - Bellare-Neven signature aggregation;
  - key aggregation.

- From **Tari Labs**: 
Analogous to the OSI layers for communication, in blockchain technology decentralized Layer 2 protocols, also commonly 
referred to as Layer 2 scaling, refers to transaction throughput scaling solutions. Decentralized Layer 2 protocols run 
  on top of the main blockchain (off-chain), while preserving the attributes of the main blockchain (e.g. crypto-economic 
  consensus). Instead of each transaction, only the resultant of a number of transactions is embedded on-chain [[3]].

## References

[[1]] Wikipedia: "Bitcoin Scalability Problem" [online]. Available: <https://en.wikipedia.org/wiki/Bitcoin_scalability_problem>. 
Date accessed: 2019&#8209;06&#8209;11.

[1]: https://en.wikipedia.org/wiki/Bitcoin_scalability_problem
"Bitcoin Scalability Problem"

[[2]] Bitcoin StackExchange: "What is the Difference between On-chain Scaling and Off-chain Scaling?" [online]. 
Available: <https://bitcoin.stackexchange.com/questions/63375/what-is-the-difference-between-on-chain-scaling-and-off-chain-scaling>. 
Date accessed: 2019&#8209;06&#8209;10.

[2]: https://bitcoin.stackexchange.com/questions/63375/what-is-the-difference-between-on-chain-scaling-and-off-chain-scaling
"What is the Difference between On-chain 
Scaling and Off-chain Scaling?"

[[3]] Tari Labs: "Layer 2 Scaling Survey - What is Layer 2 Scaling?" [online]. 
Available: [layer2scaling-landscape/layer2scaling-survey.html#what-is-layer-2-scaling](layer2scaling-landscape/layer2scaling-survey.html#what-is-layer-2-scaling). 
Date accessed: 2019&#8209;06&#8209;10.

[3]: layer2scaling-landscape/layer2scaling-survey.html#what-is-layer-2-scaling
"Layer 2 Scaling Survey - 
What is Layer 2 Scaling?"