# zk-SNARKs


- [Introduction](#introduction)
- [What is ZKP? A Complete Guide to Zero-knowledge Proof](#what-is-zkp-a-complete-guide-to-zero-knowledge-proof)
  - [Overview](#overview)
  - [Summary](#summary)
- [Introduction to zk-SNARKs](#introduction-to-zk-snarks)
  - [Overview](#overview-1)
  - [Summary](#summary-1)
  - [Video](#video)
- [Comparing General-purpose zk-SNARKs](#comparing-general-purpose-zk-snarks)
  - [Overview](#overview-2)
  - [Summary](#summary-2)
- [Quadratic Arithmetic Programs - from Zero to Hero](#quadratic-arithmetic-programs---from-zero-to-hero)
  - [Overview](#overview-3)
  - [Summary](#summary-3)
- [Explaining SNARKs Series: Part I to Part VII](#explaining-snarks-series-part-i-to-part-vii)
  - [Overview](#overview-4)
  - [Part I: Homomorphic Hidings](#part-i-homomorphic-hidings)
  - [Part II: Blind Evaluation of Polynomials](#part-ii-blind-evaluation-of-polynomials)
  - [Part III: The Knowledge of Coefficient Test and Assumption](#part-iii-the-knowledge-of-coefficient-test-and-assumption)
  - [Part IV: How to make Blind Evaluation of Polynomials Verifiable](#part-iv-how-to-make-blind-evaluation-of-polynomials-verifiable)
  - [Part V: From Computations to Polynomials](#part-v-from-computations-to-polynomials)
  - [Part VI: The Pinocchio Protocol](#part-vi-the-pinocchio-protocol)
  - [Part VII: Pairings of Elliptic Curves](#part-vii-pairings-of-elliptic-curves)
- [zk-SHARKs: Combining Succinct Verification and Public Coin Setup](#zk-sharks-combining-succinct-verification-and-public-coin-setup)
  - [Background](#background)
  - [Summary](#summary-4)
- [References](#references)


## Introduction

Zero-knowledge proof protocols have gained much attention in the past decade, due to the popularity of cryptocurrencies.
A zero-knowledge Succinct Non-interactive Argument of Knowledge (zk-SNARK), referred to here as an _argument of
knowledge,_ is a special kind of a zero-knowledge proof. The difference between a _proof of knowledge_ and an _argument
of knowledge_ is rather technical for the intended audience of this report. The distinction lies in the difference
between _statistical soundness_ and _computational soundness_. The technical reader is referred to
[[1]] or [[2]].


zk-SNARKs have found many applications in zero-knowledge proving systems, libraries of proving systems such as
_libsnark_ and _bellman_ and general-purpose compilers from high-level languages such as _Pinocchio_. "DIZK: A
Distributed Zero Knowledge Proof System", by Wu et al. [[3]], is one of the best papers on zk-SNARKs, at least from a
cryptographer's point of view. Coins such as Zerocoin and Zcash use zk-SNARKs. The content reflected in this curated
content report, although not all inclusive, covers all the necessary basics one needs to understand zk-SNARKs and their
implementations.



## What is ZKP? A Complete Guide to Zero-knowledge Proof

<div>
  <p style="float: left;">
    <img src="sources/hasib_anwar.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Hasib Anwar</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Just is a born geek
    <br>
    <br>
  </p>
</div>

### Overview

A zero-knowledge proof is a technique one uses to prove to a verifier that one has knowledge of some secret information,
without disclosing the information. This is a powerful tool in the blockchain world, particularly in cryptocurrencies.
The aim is to achieve a trustless network, i.e. anyone in the network should be able to verify information recorded in a
block.

### Summary

In [this post](https://101blockchains.com/zero-knowledge-proof/), Anwar provides an excellent zero-knowledge
infographic that summarizes what a zero-knowledge proof is, its main properties (completeness, soundness and
zero-knowledge), as well as its _use cases_ such as _authentication_, _secure data sharing_ and _file-system control_.
Find what Anwar calls a _complete guide to zero-knowledge proofs_,
[here](https://101blockchains.com/zero-knowledge-proof/#prettyPhoto).


## Introduction to zk-SNARKs

<div>
  <p style="float: left;">
    <img src="sources/chris_reitweissner.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Dr Christian Reitwiessner</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Ethereum Foundation
    <br>
    <br>
  </p>
</div>



### Overview

A typical zero-knowledge proof protocol involves at least two participants: the _Verifier_ and the _Prover_.
The _Verifier_ sends a challenge to the _Prover_ in the form of a computational problem. The _Prover_ has to solve the
computational problem and, without revealing the solution, send proof of the correct solution to the *Verifier*.

### Summary

zk-SNARKs are important in blockchains for at least two reasons:

- Blockchains are by nature not scalable. They
  thus benefit in that zk-SNARKs allow a verifier to verify a given _proof_ of a computation without having to actually
  carry out the computation.
- Blockchains are public and need to be _trustless,_ as explained earlier. The _zero-knowledge_ property of
  zk&#8209;SNARKs as well as the possibility to put in place a so-called _trusted setup_ make this _almost_ possible.

Reitwiessner uses an example of a mini 4 x 4 Sudoku challenge as an example of an interactive zero-knowledge proof. He
explains how it would fall short of the _zero-knowledge_ property without the use of homomorphic encryption as well as
putting in place a _trusted setup_. Reitwiessner proceeds to explain how computations involving _polynomials_ are better
suited as challenges posed by the _Verifier_ to the _Prover_.

### Video

The slides from the talk can be found [here](https://chriseth.github.io/notes/talks/intro_to_zksnarks/#/27).

<iframe width="1000" height="800" src="https://www.youtube.com/embed/jr95o_k_SwI" frameborder="0" allow="accelerometer;
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


## Comparing General-purpose zk-SNARKs

<div>
  <p style="float: left;">
    <img src="sources/ronald_mannak.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Ronald Mannak</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Open-source Blockchain Developer
    <br>
    <br>
  </p>
</div>



### Overview

Recently, zk-SNARK constructs such as Supersonic [[4]] and Halo [[5]] were created mainly for efficiency of proofs. The
following article by Mannak gives a quick survey of the most recent developments, comparing general-purpose zk-SNARKs.
The article is easy to read. It provides the technical reader with relevant references to scholarly research papers.

### Summary

The main drawback of zk-SNARKs is their reliance on a _common reference string_ that is created using a _trusted setup_.
In [this post](https://medium.com/coinmonks/comparing-general-purpose-zk-snarks-51ce124c60bd?), Mannak mentions three
issues with reference strings or having a trusted setup:

- A leaked reference string can be used to create
  undetectable fake proofs.
- One setup is only applicable to one computation, thus making smart contracts impossible.
- Reference strings are not upgradeable. This means that a whole new _ceremony_ is required,
  even for minor _bug fixes_ in crypto coins.

After classifying zk-SNARKs according to the type of trusted setup they use, Mannak compares their
_proof_ and _verification sizes_ as well as _performance_.


## Quadratic Arithmetic Programs - from Zero to Hero

<div>
  <p style="float: left;">
    <img src="sources/vitalik_buterin.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Vitalik Buterin</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Co-founder of Ethereum
    <br>
    <br>
  </p>
</div>


### Overview

The zk-SNARK end-to-end journey is to create a function or a protocol that takes the proof, given by the Prover, and
checks its veracity. In a zk-SNARK proof, a computation is verified step by step. To do so, the computation is first
turned into an arithmetic circuit. Each of its wires is then assigned a value that results from feeding specific inputs
to the circuit. Next, each computing node of the arithmetic circuit (called a “gate” - an analogy of the nomenclature of
electronic circuits) is transformed into a constraint that verifies the output wire has the value it should have for the
values assigned to the input wires. This process involves transforming statements or computational problems into various
formats on which a zk-SNARK proof can be performed. The following seven steps depicts the process for achieving a zk-SNARK:

`Computational Problem  —>  Arithmetic Circuit  —>  R1CS  —>  QAP  —>  Linear PCP  —>  Linear Interactive Proof  ->  zk-SNARK`

### Summary

In [this post](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649?source=post_page-----f6d558cea649----------------------),
Buterin explains how zk-SNARKs work, using an example that focuses on the first three steps of the process given above.
Buterin explains how a computational problem can be written as an arithmetic circuit, converted into a rank-1 constraint
system or R1CS, and ultimately transform the R1CS into a quadratic arithmetic program.

## Explaining SNARKs Series: Part I to Part VII

<div>
  <p style="float: left;">
    <img src="sources/ariel_gabizon.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Ariel Gabizon</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Engineer at Zcash
    <br>
    <br>
  </p>
</div>

### Overview

The explanation of zk-SNARKs given by [Buterin above](#quadratic-arithmetic-programs---from-zero-to-hero), and similar
explanations by Pinto ([6]], [[7]]), although excellent in clarifying the R1CS and the Quadratic Arithmatic Program (QAP)
concepts, do not explain how zero-knowledge is achieved in zk-SNARKs. For a step-by-step and mathematical explanation of
how this is achieved, as used in Zcash, refer to the seven-part series listed here.

### [Part I: Homomorphic Hidings]

This [post](https://z.cash/blog/snark-explain) explains how zk-SNARKs use _homomorphic hiding_ or
_homomorphic encryption_ in order to achieve zero-knowledge proofs. Gabizon dives into the _mathematics_ that underpins
the cryptographic security of homomorphic encryption afforded by the difficulty of solving _discrete log problems_ in a
finite group of a large _prime_ order.

### [Part II: Blind Evaluation of Polynomials]

This [post](https://z.cash/blog/snark-explain2) explains how the power of the _homomorphic property_ of these types of
hidings is seen in how it easily extends to linear combinations. Since any polynomial evaluated at a specific value
$x = \bf{s} $ is a _weighted linear combination_ of powers of $\bf{s}​$, this property allows sophisticated
zero-knowledge proofs to be set up.

For example, two parties can set up a _zero-knowledge proof_ where the Verifier can request the Prover to prove
knowledge of the "right" polynomial $P(x)$, without revealing $P(x)$ to the Verifier. All that the Verifier requests
is for the Prover to evaluate $P(x)$ at a secret point $\bf{s}$, without learning what $\bf{s}$ is. Thus, instead of
sending $\bf{s}$ in the open, the Verifier sends homomorphic hidings of the necessary power of $\bf{s}$. The Prover
therefore simply evaluates the right linear combination of the hidings as dictated to by the polynomial $P(x)$. This is
how the Prover performs what is called a _blind evaluation of the polynomial_ $P(x)$ at a secret point $\bf{s}​$
only known by the Verifier.

### [Part III: The Knowledge of Coefficient Test and Assumption]

In this [post](https://z.cash/blog/snark-explain3), Gabizon notes that it is necessary to force the Prover to comply
with the rules of the protocol. Although this is covered in the next part of the series, in this post he considers _the
Knowledge of Coefficient (KC) Test_, as well as its _KC assumption_.

The KC Test is in fact a request for a _proof_ in the form of a challenge that a Verifier poses to a Prover. The
Verifier sends a pair  $( a, b )$  of elements of a prime field, where $a$ is such that $b = \alpha \cdot a$, to the
Prover. The Verifier challenges the Prover to produce a similar pair $( a', b' )$, where  $b' = \alpha \cdot a' $ for
the same scalar $\alpha$. The KC assumption is that if the Prover succeeds with a non-negligible probability, then the
Prover knows the ratio between $a$ and $a'$. Gabizon explains how this two concept can be formalized using an
_extractor_ of the Prover.

### [Part IV: How to make Blind Evaluation of Polynomials Verifiable]

In [this part](https://z.cash/blog/snark-explain4) of the series, Gabizon explains how to make the _blind
evaluation of polynomials_ of Part II above, verifiable. This requires an extension of the _Knowledge of Coefficient
Assumption_ considered in Part III. Due to the homomorphic property of the used homomorphic hiding function, the Prover
is able to receive several hidings of $\alpha​$-pairs from the Verifier, evaluate the polynomial $P(x)​$ on a
particular linear combination of these hidings of $\alpha​$-pairs and send the resulting pair to the Verifier. Now,
according to the extended _Knowledge of Coefficient Assumption_ of degree $d​$, the Verifier can know, with a high
probability, that the Prover knows the "right" polynomial,
$P(x)​$, without disclosing it.

### [Part V: From Computations to Polynomials]

[This post](https://z.cash/blog/snark-explain5) aims to translate statements that are to be proved and verified
into the language of polynomials. Gabizon explains the same process discussed above by Buterin, for transforming a
computational problem into an _arithmetic circuit_ and ultimately into a QAP. However, unlike Buterin, Gabizon does
not mention constraint systems.

### [Part VI: The Pinocchio Protocol]

The [Pinocchio protocol](https://z.cash/blog/snark-explain6) is used as an example of how the QAP computed in the
previous parts of this series can be used between both the Prover and the Verifier to achieve a zero-knowledge proof
with negligible probability that the Verifier would accept a wrong polynomial as correct. The low probability is
guaranteed by a well-known theorem that "two different polynomials of degree at most $2d$ can agree on at most $2d$
points in the given prime field". Gabizon further discusses how to restrict the Prover to choose their polynomials
according to the assignment $\bf{s}$ given by the Verifier, and how the Prover can use randomly chosen field elements
to _blind_ all the information they send to the Verifier.

### [Part VII: Pairings of Elliptic Curves]

[This part](https://z.cash/blog/snark-explain7) of the series aims to set up a Common Reference String (CRS) model that
can be used to convert the _verifiable blind evaluation_ of the polynomial of Part IV into a _non-interactive proof
system_. A homomorphic hiding that supports both addition and multiplication is needed for this purpose. Such a
homomorphic hiding is created from what is known as _Tate pairings_. Since _Tate pairings_ emanate from Elliptic Curve
Groups, Gabizon starts by defining these groups.

The Pinnochio SNARK, however, uses a relaxed notion of a _non-interactive proof_, where the use of a CRS is allowed. The
CRS is created before any proofs are constructed, according to a certain randomized process, and is broadcast to all
parties. The assumption here is that the randomness used in creating the CRS is not known to any of the parties.

The intended non-interactive evaluation protocol has three parts; Setup, Proof, and Verification. In the Setup, the CRS
and a random field element $\bf{s}$ are used to calculate the Verifier's challenge (i.e. the set of $\alpha$-pairs a in
Part IV).



## zk-SHARKs: Combining Succinct Verification and Public Coin Setup

<div>
  <p style="float: left;">
    <img src="sources/madars_virza.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Madars Virza</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Scientist, MIT
    <br>
    <br>
  </p>
</div>


### Background

Most of the research done on zero-knowledge proofs has been about the efficiency of these types of proofs and making
them more practical, especially in cryptocurrencies. One of the most recent innovations is that of the so-called
_zk-SHARKs_ (short for _zero-knowledge Succinct Hybrid ARguments of Knowledge_) designed by Mariana Raykova, Eran Tromer
and Madars Virza.


### Summary

Virza starts with a concise survey of the best zk-SNARK protocols and their applications while giving an assessment of
the efficiency of zero-knowledge proof implementations in the context of blockchains. He mentions that although
zero-knowledge proofs have found practical applications in _privacy preserving cryptocurrencies_, _privacy preserving
smart contracts_, _proof of regulatory compliance_ and _blockchain-based sovereign identity_, they still have a few
shortcomings. While QAP-based zero-knowledge proofs can execute fast verifications, they still require a _trusted
setup_. Also, in Probabilistically Checkable Proof (PCP)-based zk-SNARKs, the speed of verification decays with the
increasing statement size.

Virza mentions that the danger of slow verification can tempt miners to skip validation of transactions. This can cause
forks such as the July 2015 Bitcoin fork. He uses the Bitcoin fork example and slow verification as motivation for a
zero-knowledge protocol that allows multiple verification modes. This will allow miners to carry out an _optimistic
verification_ without losing much time and later check the validity of transactions by using _prudent verification_. The
_zk-SHARK_ is introduced as one such zero-knowledge protocol that implements these two types of verification modes. It
is a _hybrid,_ as it incorporates a Non-interactive Zero-knowledge (NIZK) proof inside a  SNARK. The internal design of
the NIZK verifier is algebraic in nature, using a new compilation technique for linear PCPs. The special-purpose SNARK,
which constitutes the main part of the zk-SHARK, is dedicated to verifying an _encoded polynomial delegation_ problem.

The design of zk-SHARKs is ingenious and aims at avoiding unnecessary coin forkings.

### Video

The slides from the [talk](https://dci.mit.edu/zksharks) can be found
[here](https://madars.org/sharks/2019-04-10-SHARK-zkproof-workshop.pdf).

<iframe width="1000" height="800" src="https://www.youtube.com/embed/OP8ydUxAVt4" frameborder="0" allow="accelerometer;
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


## References

[[1]] G. Brassard, D. Chaum and C. Crepeau, "Minimum Disclosure Proofs of Knowledge" [online]. Available:
<http://crypto.cs.mcgill.ca/~crepeau/PDF/BCC88-jcss.pdf>. Date accessed: 2019&#8209;12&#8209;07.

[1]: http://crypto.cs.mcgill.ca/~crepeau/PDF/BCC88-jcss.pdf
"Minimum Disclosure
Proofs of Knowledge"

[[2]] M. Nguyen, S. J. Ong and S. Vadhan, "Statistical Zero-knowledge Arguments for NP from any One-way Function
(Extended Abstract)" [online]. Available: <http://people.seas.harvard.edu/~salil/research/SZKargs-focs.pdf>.
Date accessed: 2019&#8209;12&#8209;07.

[2]: http://people.seas.harvard.edu/~salil/research/SZKargs-focs.pdf
"Statistical Zero-knowledge
Arguments for NP from
Any One-way Function
(Extended
Abstract)"

[[3]] H. Wu, W. Zheng, A. Chiesa, R. A. Popa and I. Stoica, "DIZK: A Distributed Zero Knowledge Proof System",
UC Berkeley [online]. Available: <https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf>.
Date accessed: 2019&#8209;12&#8209;07.

[3]: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf
"DIZK: A Distributed
Zero Knowledge
Proof System"

[[4]] B. Bünz, B. Fisch and A. Szepieniec, "Transparent SNARKs from DARK Compilers" [online]. Available:
<https://eprint.iacr.org/2019/1229.pdf>. Date accessed: 2019&#8209;12&#8209;07.

[4]: https://eprint.iacr.org/2019/1229.pdf
"Transparent SNARKs
from DARK Compilers"

[[5]] S. Bowe, J. Grigg and D. Hopwood, "Halo: Recursive Proof Composition without a Trusted Setup" [online]. Available:
<https://eprint.iacr.org/2019/1021.pdf>. Date accessed: 2019&#8209;12&#8209;07.

[5]: https://eprint.iacr.org/2019/1021.pdf
"Halo: Recursive Proof
Composition without
a Trusted Setup"

[[6]] A. Pinto, "Constraint Systems for ZK SNARKs" [online]. Available:
<http://coders-errand.com/constraint-systems-for-zk-snarks/>. Date accessed: 2019&#8209;03&#8209;06.


[6]: http://coders-errand.com/constraint-systems-for-zk-snarks/
"Constraint Systems for ZK SNARKs"

[[7]] A. Pinto, "The Vanishing Polynomial for QAPs", 23 March 2019 [online]. Available:
<http://coders-errand.com/the-vanishing-polynomial-for-qaps/>. Date accessed: 2019&#8209;10&#8209;10.

[7]: http://coders-errand.com/the-vanishing-polynomial-for-qaps/
"The Vanishing Polynomial
for QAPs"
