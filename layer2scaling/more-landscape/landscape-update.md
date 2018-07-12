# Layer 2 Scaling Survey

This report provides a survey of TumbleBit , Counterparty, Rootstock, Drivechains, Scriptless scripts, Braiding and Directed Acyclic Graph (DAG) as layer 2 scaling alternatives.

## What is Layer 2 scaling?

See [Layer 2 Scaling Survey](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md)

## How will this be applicable to Tari?

See [Layer 2 Scaling Survey](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md)

## Layer 2 scaling current initiatives (updated)

### #1 TumbleBit

#### What is it?

The TumbleBit protocol was invented at the Boston University. TumbleBit is a unidirectional, unlinkable payment hub that is fully compatible with the Bitcoin protocol. TumbleBit allows parties to make fast, anonymous, off-chain payments through an untrusted intermediary called the Tumbler. No-one, not even the Tumbler, can tell which payer paid which payee during a TumbleBit epoch. TumbleBit consists of two interleaved fair-exchange protocols that prevent bad acting from users or Tumblers. It combines off-chain cryptographic computations with standard on-chain Bitcoin scripting functionalities to realize smart contracts. ([[1]](http://cs-people.bu.edu/heilman/tumblebit/), [[2]](https://eprint.iacr.org/2016/575.pdf), [[8]](https://eprint.iacr.org/2016/056.pdf))

![TumbleBitOverview](C:\Users\pluto\Documents\Code\tari-university\layer2scaling\more-landscape\sources\TumbleBitOverview.PNG)

#### Who does it?

The Boston University provided a proof-of-concept and reference implementation alongside the white paper [[4]](https://github.com/BUSEC/TumbleBit). NTumbleBit [[5]](https://github.com/nTumbleBit/nTumbleBit) is being developed as a C# production implementation of the TumbleBit protocol that is currently being deployed by Stratis with their Breeze implementation ([[6]](https://stratisplatform.com/2017/07/17/breeze-tumblebit-server-experimental-release), [[7]](https://stratisplatform.com/2017/09/20/breeze-wallet-with-breeze-privacy-protocol-dev-update/)), currently at alpha/experimental release level in testnet.

NTumbleBit will be a cross-platform framework, server and client for the TumbleBit payment scheme. TumbleBit is separated into two modes, tumbler mode and payment hub mode. The tumbler mode improves transaction fungibility and offers risk free unlinkable transactions. Payment hub mode is a way of making off-chain payments possible without requiring implementations like Segwit or the lightning network. [[3]](https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e)

#### Strengths

Privacy on payment hubs - payment hub does not know where payments are being sent to.

#### Weaknesses

Tumblebit is implemented in Stratis but hasn't yet had wide adoptation.

#### Opportunities for Tari

Has alignment with Tari's base requirements.

#### Threats for Tari

The scalable, privacy-focused payment hubs could be a threat for Tari if TumbleBit and it's implementation were to focus on ticketing. But, Tari's privacy slider gives it the edge as the user is in control of which transactions are private, semi-private or public.

### #2 Counterparty

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #3 Rootstock

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #4 Drivechains

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #5 Scriptless scripts

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #6 Braiding

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

### #7 Directed Acyclic Graph (DAG)

#### What is it?

???

#### Who does it?

???

#### Strengths

- ???

#### Weaknesses

???

#### Opportunities for Tari

???

#### Threats for Tari

???

## Observations

- ???

## References

[1] TumbleBit: An Untrusted Bitcoin-Compatible Anonymous Payment Hub, http://cs-people.bu.edu/heilman/tumblebit, Date accessed: 2018-07-12.

[2] TumbleBit: An Untrusted Bitcoin-Compatible Anonymous Payment Hub, Heilman E. et. el., https://eprint.iacr.org/2016/575.pdf, Date accessed: 2018-07-08.

[3] Anonymous Transactions Coming to Stratis, [https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e](https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e), Date accessed: 2018-07-08.

[4] TumbleBit Proof of Concept GitHub Repository, https://github.com/BUSEC/TumbleBit, Date accessed: 2018-07-08.

[5] NTumbleBit GitHub Repository, https://github.com/nTumbleBit/nTumbleBit, Date accessed: 2018-07-12.

[6] Breeze Tumblebit Server Experimental Release, https://stratisplatform.com/2017/07/17/breeze-tumblebit-server-experimental-release, Date accessed: 2018-07-12.

[7] Breeze Wallet with Breeze Privacy Protocol, https://stratisplatform.com/2017/09/20/breeze-wallet-with-breeze-privacy-protocol-dev-update, Date accessed: 2018-07-12.

[8] Blindly Signed Contracts - Anonymous On-chain and Off-chain Bitcoin Transactions, Heilman E. et. el., https://eprint.iacr.org/2016/056.pdf, Date accessed: 2018-07-12.



## Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- [https://github.com/ksloven](https://github.com/ksloven)
- ???