# zk-SNARKs


- [Introduction](#introduction) 
- [What is ZKP? A Complete Guide to Zero Knowledge Proof](#what-is-zkp-a-complete-guide-to-zero-knowledge-proof) 
  - [Overview](#overview)
  - [Summary](#summary)
- [Introduction to zk-SNARKs](#introduction-to-zksnarks)  
  - [Overview](#overview-1)
  - [Summary](#summary-1)
  - [Video](#video)
- [Comparing General Purpose zk-SNARKs](#comparing-general-purpose-zk-snarks) 
  - [Overview](#overview-2)
  - [Summary](#summary-2)
- [Quadratic Arithmetic Programs - from Zero to Hero](#quadratic-arithmetic-programs---from-zero-to-hero) 
  - [Overview](#overview-3)
  - [Summary](#summary-3)
- [Explaining SNARKs Series: Part I to Part VII](#explaining-snarks-series-part-i-to-part-vii) 
  - [Overview](#overview-4)
  - [Part I: Homomorphi-c Hidings](#part-i-homomorphic-hidings) 
  - [Part II: Blind Evaluation of Polynomials](#part-ii-blind-evaluation-of-polynomials) 
  - [Part III: The Knowledge of Coefficient Test and Assumption](#part-iii-the-knowledge-of-coefficient-test-and-assumption) 
  - [Part IV: How to make Blind Evaluation of Polynomials Verifiable](#part-iv-how-to-make-blind-evaluation-of-polynomials-verifiable) 
  - [Part V: From Computations to Polynomials](#part-v-from-computations-to-polynomials) 
  - [Part VI: The Pinnochio Protocol](#part-vi-the-pinocchio-protocol) 
  - [Part VII: Pairings of Elliptic Curves](#part-vii-pairings-of-elliptic-curves)
- [zk-SHARKSs: Combining Succinct Verification and Public Coin Setup](#zk-sharkss-combining-succinct-verification-and-public-coin-setup) 
  - [Background](#background)
  - [Summary](#summary-4)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations) 
- [References](#references)


## Introduction 

Zero-knowledge proof protocols have gained much attention in the past decade due to the popularity of cryptocurrencies. 
A Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (zk-SNARK) though referred to here as an _argument of 
knowledge_ is a special kind of a zero-knowledge proof. The difference between a _proof of knowledge_ and an _argument 
of knowledge_ is rather technical for the intended audience of this report, the distinction lies in the difference 
between what is called _statistical soundness_ and _computational soundness_. The technical reader is referred to 
[[1]] or [[2]]. 


## What is ZKP? A Complete Guide to Zero Knowledge Proof

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

A zero-knowledge proof is a technique one uses to prove to a verifier that one has knowledge of some secret information 
without disclosing the information. This is a powerful tool in the blockchain world, particularly in cryptocurrencies, 
as the aim is to achieve a trustless network, that is, anyone in the network should be able to verify information 
recorded in a block.  

### Summary 

In [this post](https://101blockchains.com/zero-knowledge-proof/) Hasib Anwar gives an excellent zero-knowledge 
infographic which summarises what a zero-knowledge proof is, its main properties (completeness, soundness and 
zero-knowledge), as well as its _use cases_ such as _authentication_, _secure data sharing_ and _file-system control_. 
Find what Hasib Anwar calls a _complete guide to zero-knowledge proofs_ 
[here](https://101blockchains.com/zero-knowledge-proof/#prettyPhoto).  


## Introduction to zk-SNARKs

<div>
  <p style="float: left;">
    <img src="sources/chris_reitweissner.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Dr. Christian Rietwiessner</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Ethereum Foundation
    <br>
    <br>
  </p>
</div>


### Overview 

A typical zero-knowledge proof protocol involves at least two participants, called the _Verifier_ and the _Prover_. 
The _Verifier_ sends a challenge to the _Prover_ in the form of a computational problem. The _Prover_ has to solve the 
computational problem and, without revealing his solution, send _proof_ of his correct solution to the Verifier. 

### Summary  

zk-SNARKs are important in blockchains for at least two reasons. Firstly, blockchains are by nature not scalable, and 
thus benefit in that zk-SNARKs allow a verifier to verify a given _proof_ of a computation without having to actually 
carry out the computation. Secondly, blockchains are public and need to be _trustless_ (as explained above), and it is 
the _zero-knowledge_ property of zk-SNARKs as well as the possibility to put in place a so called _trusted setup_ that 
makes this _almost_ possible. 

He uses an example of a mini 4x4 Sudoku challenge as an example of a interactive zero-knowledge proof. And explains how 
it would fall short of the _zero-knowledge_ property without the use of homomorphic encryption as well as putting in 
place a _trusted setup_. He proceeds to explain how computations involving _polynomials_ are better suited as challenges 
posed by the _Verifier_ to the _Prover_.   

### Video 

The slides to the talk can be found [here](https://chriseth.github.io/notes/talks/intro_to_zksnarks/#/27).  

<iframe width="640" height="360" src="https://www.youtube.com/embed/jr95o_k_SwI" frameborder="0" allow="accelerometer; 
autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


## Comparing General Purpose zk-SNARKs

<div>
  <p style="float: left;">
    <img src="sources/ronald_mannak.png" width="90" />
  </p>
  <p>
    <br>
    &nbsp;&nbsp;&nbsp;&nbsp;<strong>Ronald Mannak</strong><br>
    &nbsp;&nbsp;&nbsp;&nbsp;Open source blockchain developer
    <br>
    <br>
  </p>
</div>


### Overview

Recently, zkSNARK constructs such as Supersonic [[8]] and Halo [[9]] 
were created mainly for efficiency of proofs. The following article by Ronal Mannak gives a quick survey of the most 
recent developments, comparing general-purposed zk-SNARKs. It is an easy to read article, and gives relevant reference 
to scholarly research papers for the technical reader.  

### Summary

The main drawback with zk-SNARKs is their reliance on a _common reference string_ that is created using a _trusted setup_. 
In [this post](https://medium.com/coinmonks/comparing-general-purpose-zk-snarks-51ce124c60bd?) Ronald Mannak mentions three issues with reference strings or having a trusted setup. First, a leaked reference string 
can be used to create undetectable fake proofs. Second, one setup is only applicable to one computation thus making 
smart contracts impossible. Third, reference strings are not upgradable which means a whole new _ceremony_ is require 
even for minor _bug fixes_ in crypto coins.   

After classifying zk-SNARKs according to the type of trusted setup they use, Ronald Mannak compares their 
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
checks its veracity [[4]]. In a ZK-SNARK proof, a 
computation is verified step by step [[5]]. To do so, the computation is 
first turned into an arithmetic circuit, then each of its wires is assigned a value that results from feeding specific 
inputs to the circuit. Each computing node of the arithmetic circuit (called a “gate” in analogy to the nomenclature of 
electronic circuits) is transformed into a constraint, that verifies the output wire has the value it should have for 
the values assigned to the input wires. This process involves transforming statements or computational problems into 
various formats on which a zkSNARK proof can be performed. Eran Tomer in [[3]] 
gives the following 5-step process of achieving a zkSNARK,   

`Computational Problem  —>  Arithmetic Circuit  —>  R1CS  —>  QAP  —>  Linear PCP  —>  zk-SNARK`

### Summary 

"_Quadratic Arithmetic Programs_" by Vitalik Buterin, 2016-12-12  

In [this post](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649?source=post_page-----f6d558cea649----------------------) Vitalik Buterin explains how zk-SNARKs work by use of an example, focusing on first three steps of Eran Tomer's 5-step 
process given above. He explains how a computational problem can be written as an arithmetic circuit, converted into an 
rank-1 constraint system or R1CS, and ultimately transforming the R1CS into a quadratic arithmetic program. Vitalik's 
blogpost can be found [here](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649). 

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

The explanation of zk-SNARKs given by Vitalik Buterin above, and similar explanations by _Alex Pinto_ in [[4]] and [[5]] though excellent in elucidating the R1CS and the QAP concepts, do not explain how zero-knowledge is achieved in zk-SNARKs. For a step-by-step and mathematical explanation of how zero-knowledge is achieved in zk-SNARKs, as used in Zcash, see the _Explaining SNARKs 7-part series_ by Ariel Gabizon [[7]]. 

### [Part I: Homomorphic Hidings]

This [post](https://z.cash/blog/snark-explain) explains how zk-SNARKs use _homomrphic hiding_ or _homomorphic encryption_ in order to achieve zero-knowledge proofs. 
Gabizon dives into the _mathematics_ that underpins the cryptographic security of homomorphic encryption afforded by the 
difficulty of solving _discrete log problems_ in a finite group of a large _prime_ order. 

### [Part II: Blind Evaluation of Polynomials]

This [post](https://z.cash/blog/snark-explain2) explains how tThe power of the _homomorphic property_ of these type of hidings is seen in how it easily extends to linear combinations. 
And since any polynomial evaluated at a specific value  $x = \bf{s} $  is a _weighted linear combination_ of powers of  
$\bf{s}​$ , this property allows sophisticated zero-knowledge proofs to be set up. 

For example, two parties can set up a _zero-knowledge proof_ where the Verifier can request the Prover to prove 
knowledge of the 'right' polynomial  $P(x)$ , without revealing  $P(x)$  to the Verifier. All that the Verifier requests 
is for the Prover to evaluate  $P(x)$  at a secret point  $\bf{s}$ , without learning what  $\bf{s}$  is. So, instead of 
sending  $\bf{s}$  in the open, the Verifier sends homomorphic hidings of the necessary power of $\bf{s}$. The Prover 
therefore simply evaluates the right linear combination of the hidings as dictated to by the polynomial  $P(x)$. That's 
how the Prover performs what is called a _blind evaluation of the polynomial_  $P(x)$  at a secret point  $\bf{s}$  
only known by the Verifier.  

### [Part III: The Knowledge of Coefficient Test and Assumption]

In this [post](https://z.cash/blog/snark-explain3)  Gabizon notes that it is necessary to force the Prover to comply with the rules of the protocol. Although this is 
covered in the next part of the series, he herein considers _the knowledge of coefficient test_ or _KC Test_, as well as its _KC assumption_. 

The KC Test is in fact a _proof_ in the form of a challenge that a Verifier poses to a Prover. The Verifier sends a pair  
$( a, b )$  of elements of a prime field, where  $a$  is such that  $b = \alpha a$ , to the Prover. The Verifier challenges the Prover to produce a similar pair  $( a', b' )$  where  $b' = \alpha a' $ for the same scalar  $\alpha$ . The KC assumption is that if the Prover succeeds with a non-negligible probability then he knows the ratio between  $a$  and  $a'$. Gabizon explains how this two concept can be formalized by using something called an _extractor_ of the Prover. 

### [Part IV: How to make Blind Evaluation of Polynomials Verifiable]

In [this part](https://z.cash/blog/snark-explain4) of the series Ariel Gabizon explains how to make the _blind evaluation of polynomials_ of Part II above, 
verifiable. This requires an extension of the _Knowledge of Coefficient Assumption_ considered in Part III. Due to the 
homomorphic property of the used homomorphic hiding function, the Prover is able to receive several hidings of 
$\alpha$-pairs from the Verifier, evaluates the polynomial  $P(x)$  on a particular linear combination of these hidings 
of  $\alpha$-pairs and send the resulting pair to the Verifier. Now, according to the extended _Knowledge of Coefficient 
Assumption_ of degree  $d$ , the Verifier can know with a high probability that the Prover knows the 'right' polynomial. 
$P(x)​$  without disclosing it.   

### [Part V: From Computations to Polynomials]

The aim in [this post](https://z.cash/blog/snark-explain5) is to translate statements that to be proved and verified into the language of polynomials. Ariel Gabizon 
explains the same process discussed by Vitalik Buterin above, of how a computational problem is transformed into an 
_arithmetic circuit_ and ultimately into a _quadratic arithmetic program_ or QAP. But, unlike Vitalik, he makes no 
mention of constraint systems. 

### [Part VI: The Pinocchio Protocol]

The [Pinnochio protocol](https://z.cash/blog/snark-explain6) is used as an example of how the QAP computed in the previous parts of this series can be used 
between both the Prover and the Verifier to achieve a zero-knowledge proof with negligible probability that the Verifier 
would accept a wrong polynomial as correct. The low probability is guaranteed by a well-known theorem that "two different 
polynomials of degree at most   $2d$  can agree on at most  $2d$ points in the given prime field. Gabizon further 
discusses, how to restrict the Prover to choose her polynomials according to the assignment  $\bf{s}$  given by the 
Verifier, and how the Prover can use randomly chosen field elements to _blind_ all the information she sends to Bob. 

### [Part VII: Pairings of Elliptic Curves]

The aim in [this part](https://z.cash/blog/snark-explain7) of the series is to set up a common reference string (CRS) model which can be used to convert the 
_verifiable blind evaluation_ of the polynomial of Part IV into a _non-interactive proof system_. For this purpose a 
homomorphic hiding that supports both addition and multiplication is needed. Such a homomorphic hiding is created from 
what is known as _Tate Pairings_. Since such _Tate pairings_ emanate from Elliptic Curve Groups, Gabizon starts by 
defining these type of groups. 

The Pinnochio SNARK however uses a relaxed notion of a _non-interactive proof_ where the use of a CRS is allowed. The 
CRS is created, before any proofs are constructed, according to a certain randomised process and broadcasts it to all 
parties. The assumption here is that the randomness used in creating the CRS is not known to any of the parties. 

The intended non-interactive evaluation protocol has three parts; Setup, Proof, and Verification. In the Setup, the CRS 
together with a random field element  $\bf{s}$ are used to calculate the Verifier's challenge (that is, the set of  
$\alpha$-pairs a in Part IV.)   



## zk-SHARKSs: Combining Succinct Verification and Public Coin Setup

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

Most of the research done on zero-knowledge proofs has been about efficiency of these type of proofs, making them more 
practical, especially in cryptocurrencies. One of the most recent innovations is that of the so called _zkSHARKs_ 
(which is short for _zero-knowledge Succinct Hybrid ARguments of Knowledge_) designed by Mariana Raykova, Eran Tromer 
and Madars Virza.  


### Summary 

Madars Virza starts with a concise survey of the best zkSNARK protocols and their applications while giving an assessment 
of the efficiency of zero-knowledge proof implementations in the context of blockchains. He mentions that although zero-knowledge proofs have found practical applications in _privacy preserving cryptocurrencies_, _privacy preserving smart contracts_, _proof of regulatory compliance_ and _blockchain-based sovereign identity_, they still have a few shortcomings. While QAP-based ZK-proofs can execute fast verifications, they still require a _trusted setup_. Also, in PCP-based zk-SNARKs, the speed of verification decays with the increasing statement size. 

He mentions that the danger of slow verification can tempt miners to skip validation of transactions, something can cause 
forks such as the July 2015 Bitcoin fork. He uses the Bitcoin fork example and slow verification to motivate for a 
zero-knowledge protocol that allows multiple verification modes. This will  allow miners to carry out an 
"_optimistic verification_" without losing much time and later check the validity of transactions by using 
"_prudent verification_." The _zkSHARK_ is introduced as one such zero-knowledge protocol that implements these two 
types of verification modes. It is a _hybrid_ as it incorporates a _NIZK proof_ inside a _SNARK_. The internal design of 
the NIZK verifier is algebraic in nature, using a new compilation technique for linear PCPs. The special-purpose SNARK 
which constitutes the main part of the zkSHARK is dedicated to verifying an "_encoded polynomial delegation_" problem.  

The design of zkSHARKs is ingenious, and a brilliant idea, moreso that it aims at avoiding unnecessary coin forkings.    

### Video 

The slides to the [talk](https://dci.mit.edu/zksharks) can be found [here](https://madars.org/sharks/2019-04-10-SHARK-zkproof-workshop.pdf).

<iframe width="640" height="360" src="https://www.youtube.com/embed/OP8ydUxAVt4" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>


## Conclusions, Observations and Recommendations

zk-SNARKs have found many appllications in zero-knowledge proving systems, libraries of proving systems such as 
_libsnark_ and _bellman_, general-purpose compilers from high-level languages like _Pinnochio_, and some examples of 
circuits such as _Zcash Sprout_ and _Spacesuit_ [[11]](https://zkp.science/) The content reflected in this curated 
content report, though not all inclusive, covers all the necessary basics one needs to understand zk-SNARKs and their 
implementations. There are many coins that use zk-SNARKs, such as Zerocoin, Zcash, and recently, Bitcoin and Monero. 
The specific details of how exactly they are implemented can be found in the respective websites of these 
cryptocurrencies. A paper like [[10]]("DIZK: A Distributed Zero Knowledge Proof System," UC Berkeley. 
https://www.usenix.org/conference/usenixsecurity18/presentation/wu) is one of the best papers on zk-SNARKs, at least from 
a cryptographer's point of view. 


## References 

[[1]] G. Brassard, D. Chaum and C Crepeau, "Minimum Disclosure Proofs of Knowledge" [online]. Available:
http://crypto.cs.mcgill.ca/~crepeau/PDF/BCC88-jcss.pdf. Date accessed: 2019&#8209;12&#8209;07.



[1]: http://crypto.cs.mcgill.ca/~crepeau/PDF/BCC88-jcss.pdf
"Minimum Disclosure
Proofs of Knowledge"



[[2]] M. Nguyen, S. J Ong and S. Vadhan, "Statistical Zero-knowledge Arguments for NP from Any One-way Function (Extended 
Abstract)" [online]. Available: http://people.seas.harvard.edu/~salil/research/SZKargs-focs.pdf. Date accessed: 
2019&#8209;12&#8209;07.



[2]: http://people.seas.harvard.edu/~salil/research/SZKargs-focs.pdf
"Statistical Zero-knowledge
Arguments for NP from
Any One-way Function
(Extended 
Abstract)"



[[3]] E. Tromer, "Lecture 12: Verified Computation and its Applications, Course Conclusion", 2016 [online]. 
Available: http://www.cs.tau.ac.il/~tromer/istvr1516-files/lecture12-verified-computation.pdf. Date accessed: 
2019&#8209;12&#8209;07.  



[3]: http://www.cs.tau.ac.il/~tromer/istvr1516-files/lecture12-verified-computation.pdf
"Lecture 12: Verified 
Computation and its 
Applications, Course Conclusion"



[[4]] A. Pinto, "Constraint Systems for ZK SNARKs" [online]. Available: 
http://coders-errand.com/constraint-systems-for-zk-snarks/. Date accessed: 2019&#8209;03&#8209;06. 

 

[4]: http://coders-errand.com/constraint-systems-for-zk-snarks/
"Constraint Systems for ZK SNARKs"



[[5]] A. Pinto, "The Vanishing Polynomial for QAPs", 23 March 2019 [online]. Available: 
http://coders-errand.com/the-vanishing-polynomial-for-qaps/. Date accessed: 2019&#8209;10&#8209;10.



[5]: http://coders-errand.com/the-vanishing-polynomial-for-qaps/
"The Vanishing Polynomial
for QAPs"



[[6]] A. Gabizon, "Explaining SNARKs Part I: Homomorphic Hidings"[online]. Available: 
https://electriccoin.co/blog/snark-explain/. Date accessed: 2019&#8209;10&#8209;25.



[6]: https://electriccoin.co/blog/snark-explain/
"Explaining SNARKs Part I:
Homomorphic Hidings"



[[7]] A. Gabizon, "Explaining SNARKs - 7-Part-Series - Ariel Gabizon.pdf" [online]. Available: 
https://electriccoin.co/blog/snark-explain5/. Date accessed: 2019&#8209;10&#8209;30.



[7]: https://electriccoin.co/blog/snark-explain5/
"Explaining SNARKs - 
7-Part-Series - 
Ariel Gabizon.pdf"



[[8]] B. Bünz, B. Fisch and A. Szepieniec, "Transparent SNARKs from DARK Compilers" [online]. Available: 
https://eprint.iacr.org/2019/1229. Date accessed: 2019&#8209;12&#8209;07.



[8]: https://eprint.iacr.org/2019/1229
"Transparent SNARKs
from DARK Compilers"



[[9]] S. Bowe, J. Grigg and D. Hopwood, "Halo: Recursive Proof Composition without a Trusted Setup" [online]. Available: 
https://eprint.iacr.org/2019/1021. Date accessed: 2019&#8209;12&#8209;07. 



[9]: https://eprint.iacr.org/2019/1021
"Halo: Recursive Proof
Composition without
a Trusted Setup"



[[10]] H. Wu, W. Zheng, A. Chiesa, R. A. Popa and I. Stoica, "DIZK: A Distributed Zero Knowledge Proof System", 
UC Berkeley [online]. Available: https://www.usenix.org/conference/usenixsecurity18/presentation/wu. Date accessed: 
2019&#8209;12&#8209;07.



[10]: https://www.usenix.org/conference/usenixsecurity18/presentation/wu
"DIZK: A Distributed
Zero Knowledge
Proof System"



[[11]] "Zero-Knowledge Proofs: What are they, how do they work, and are they fast yet?" [online]. Available: 
https://zkp.science/. Date accessed: 2019&#8209;12&#8209;07.



[11]: https://zkp.science/
"Zero-Knowledge Proofs:
What are they,
how do they work,
and are they fast yet?" 