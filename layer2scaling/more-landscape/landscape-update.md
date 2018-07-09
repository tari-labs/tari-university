# Layer 2 Scaling Survey

This report provides a survey of TumbleBit , Counterparty, Rootstock, Drivechains, Scriptless scripts, Braiding and Directed Acyclic Graph (DAG) as layer 2 scaling alternatives.

## What is Layer 2 scaling?

See [Layer 2 Scaling Survey](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md)

## How will this be applicable to Tari?

See [Layer 2 Scaling Survey](https://github.com/tari-labs/tari-university/blob/master/layer2scaling/layer2scaling-landscape/layer2scaling-survey.md)

## Layer 2 scaling current initiatives (updated)

### #1 TumbleBit

#### What is it?

TumbleBit is a unidirectional, unlinkable payment hub that is fully compatible with the Bitcoin protocol. TumbleBit allows parties to make fast, anonymous, off-block chain payments through an untrusted intermediary called the Tumbler [[1]](http://wp.internetsociety.org/ndss/wp-content/uploads/sites/25/2017/09/ndss201701-3HeilmanPaper.pdf). It allows parties to make payments through an untrusted Tumbler. No-one, not even the Tumbler, can tell which payer paid which payee during a TumbleBit epoch. TumbleBit consists of two interleaved fair-exchange protocols that prevent theft of bitcoins by cheating users or a malicious Tumbler [[3]](https://github.com/BUSEC/TumbleBit).

#### Who does it?

Stratis uses a C# implementation of TumbleBit called NTumblebit. NTumbleBit will be a cross-platform framework, server and client for the TumbleBit payment scheme. TumbleBit is separated into two modes, tumbler mode and payment hub mode. The tumbler mode improves transaction fungibility and offers risk free unlinkable transactions. Payment hub mode is a way of making off-chain payments possible without requiring implementations like Segwit or the lightning network. [[2]](https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e)

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

[1] TumbleBit: An Untrusted Bitcoin-Compatible Anonymous Payment Hub, Heilman E. et. el., http://wp.internetsociety.org/ndss/wp-content/uploads/sites/25/2017/09/ndss201701-3HeilmanPaper.pdf, Date accessed: 2018-07-08.

[2] Anonymous Transactions Coming to Stratis, [https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e](https://medium.com/@Stratisplatform/anonymous-transactions-coming-to-stratis-fced3f5abc2e), Date accessed: 2018-07-08.

[3] Tumblebit, https://github.com/BUSEC/TumbleBit, Date accessed: 2018-07-08.

## Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- [https://github.com/ksloven](https://github.com/ksloven)
- ???