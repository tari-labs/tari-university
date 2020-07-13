
# Trustless Recursive Zero-Knowledge Proofs

- [Introduction](#introduction) 
- [Notation and Preliminaries](#notation-and-preliminaries)
  - [Fields and Elliptic Curves](#fields-and-elliptic-curves)
     - [Example 1](#example-1)
  - [Arithmetic Circuits and R1CS](#arithmetic-circuits-and-r1cs) 
- [Recursive Proof Composition Overview](#recursive-proof-composition-overview) 
  - [Verification Amortization Strategies](#verification-amortization-strategies)
- [Proof-Carrying Data](#proof-carrying-data) 
  - [What is a PCD?](#what-is-a-pcd) 
  - [Trustlessness via PCDs](#trustlessness-via-pcds)
- [Cycles of Elliptic Curves](#cycles-of-elliptic-curves) 
  - [Why Cycle of Elliptic Curves?](#why-cycle-of-elliptic-curves)
    - [The Native Field Arithmetic](#the-native-field-arithmetic)
    - [Example 2](#example-2)
    - [The Order of the Scalar Field](#the-order-of-the-scalar-field) 
    - [No Prover-Verifier Dichotomy](#no-prover-verifier-dichotomy)
  - [Pairing-friendly Elliptic Curves](#pairing-friendly-elliptic-curves) 
  - [Amicable Pairs of Elliptic Curves](#amicable-pairs-of-elliptic-curves) 
- [Why the interest in Halo Protocol?](#why-the-interest-in-halo-protocol) 
  - [Brief Survey: Recursive Proofs Protocols](#brief-survey-recursive-proofs-protocols) 
  - [Halo Protocol Design and Bulletproofs](# ) 
- [Implications to Tari Blockchain](#implications-to-tari-blockchain) 
- [References](#references)
- [Appendices](#appendices) 
  - [Pairing Cryptography](#pairing-cryptography) 

 



## Introduction 

In pursuit of blockchain scalability investigation in this report focuses on 
the second leg of recursive proof composition: 
"proofs that verify other proofs". 

Recursive proof composition is like a machinery with many moving parts. 
For the sake of completion, the aim here is to present only the most crucial 
cryptographic tools and techniques required to achieve recursive proof composition. 

Amortization strategies that recursive proof composition uses to accomplish reduced 
verification costs are enabled by a powerful tool called a 
[Proof-Carrying Data](#proof-carrying-data), first introduced by Alessandro Chiesa in his PhD thesis [[4]]. 
The resulting proof system is made efficient and more practical by the use of a technique 
that utilises [cycles of elliptic curves](#cycles-of-elliptic-curves), first realised by Eli Ben-Sasson et al [[2]]. 



## Notation and Preliminaries 

Notation and terminology in this report is standard. But in order to achieve a clear understanding of
the concept of [cycle of elliptic curves](#cycle-of-elliptic-curves), a few basic facts about elliptic curves are herein ironed 
out. 

### Fields and Elliptic Curves

A **field** is any set 
$\mathbb{F}$ 
of objects upon which addition '$+$' and multiplication '$*$' 
are defined in a specific way, and such that 
$( \mathbb{F}, + )$ 
forms an abelian group and 
$( \mathbb{F} \backslash \\{ 0 \\} , * )$ 
also forms 
an abelian group of non-zero elements with an identity $1_{\mathbb{F}}$.  The most common notation for 
$ \mathbb{F} \backslash \\{ 0 \\} $ is $ \mathbb{F}^{\times}.$ 
A more elaborate definition of a field can be found [here](http://localhost:3000/cryptography/r1cs-bulletproofs/mainreport.html#appendix-a-definition-of-terms). 

Note that in blockchain research papers an elliptic curve refers to what is actually an elliptic curve group. 
This can cause confusion especially when talking about the **scalar field** as opposed to the **base field**. 

- In Mathematics an **elliptic curve** $E$ over a field 
$\mathbb{F}$ 
generally refers to a locus, 
that is 
the curve formed by all the points satisfying a given equation. 
For example, an equation of the form 
$y^2 = x^3 - ax + b$ 
where $a$ and $b$ 
are elements of some field $\mathbb{F}$, 
say the rationals $\mathbb{Q}$, the reals $\mathbb{R}$ or 
the complex numbers $\mathbb{C}.$ 
Such a field $\mathbb{F}$ is referred to as the **base field** for $E$. 

- The fields; $\mathbb{Q}$, $\mathbb{R}$ and $\mathbb{C}$; are infinite sets and are thus not 
useful for cryptographic purposes. 
In cryptography, base fields that are of practical purposes are preferably finite and 
of a large prime order. 
This ensures that the discrete log problem is sufficiently difficult, making the cryptosystem 
secure against common cryptanalytic attacks. 

- Note that all finite fields are either of prime order or power of a prime. 
So then any finite field $\mathbb{F}$ is either $\mathbb{F}\_p$ 
or $\mathbb{F}\_{p^n}$ for some prime number $p$. 
See [[6], Lemma 3.19] for the proof of this fact. 
Actually, it can be shown that the orders of 
their respective multiplicative groups $\mathbb{F}\_p^{\times}$ and $\mathbb{F}\_{p^n}^{\times}$ are 
$p-1$ and $p^n-1$, [[6], Proposition 6.1]. 

- An **elliptic curve group** $E(\mathbb{F})$ is formed by first defining 'addition' of elliptic curve points, 
 picking a point $(x,y)$ on the curve $E$ and using it to generate a cyclic group by 
 doubling it, $2 \cdot (x,y)$, and forming all possible scalar multiples $\alpha \cdot (x,y)$. 
 The group 'addition' of points and doubling are illustrated in [Figure 1](#fig_gpa) below.
 All these points generated by $(x,y)$ together with the *point at infinity*, $ \mathcal{O}$, form an algebraic group 
 under the defined 'addition' of points. 
 
 <p align="center"><a name="fig_pad"> </a><img src="sources/points-addition-and-doubling.png" width="600" /></p>
<div align="center"><b>Figure 1: Points Addition and Doubling [[17]] </b></div>
 
 
- A **scalar field** of an elliptic curve $E$ over a base field $\mathbb{F}$ comes in when 
an elliptic curve group $E(\mathbb{F})$ is defined. And typically, the aim is for $E(\mathbb{F})$
to be of a prime order $p$. 
So the scalar field of $E(\mathbb{F})$ is actually the finite field that is isomorphic to 
(i.e., has the same order as) the largest cyclic subgroup of the elliptic curve group. 
Thus if $\\# E(\mathbb{F}) = p$, then the scalar field is $\mathbb{F}_p$. 

So then, unlike an elliptic curve $E$ over a field $\mathbb{F}$, an elliptic curve group $E(\mathbb{F})$ is discrete, 
consisting of only a finite number of points. The sizes of these groups are bounded by what is known as 
the Hasse bound [[12]]. Algorithms used to compute these sizes are also known, see [[13]].
 
 #### Example 1 
 
 Consider the curve $E$ of points satisfying this equation 
 $$y^2 = x^3 - 43x + 166$$ 
 Pick the point $(x,y) = (3,8)$ 
 which is on the curve $E$ because 
 $\ \ x^2 = 8^2 = 64\ \ $ 
 and 
 $\ \ 3^3 - 43(3) + 166 = 64$.  
 Doubling yields 
$\ \ 2 \cdot (3,8) = (-5, -16)$, 
 and the rest of the scalar multiples are 
$\ \ 3\cdot (3,8) =(11, -32)$, 
$\ \ 4\cdot (3,8) = (11, 32)$, 
$\ \ 5\cdot (3,8) = (-5, 16)$, 
$\ \ 6\cdot (3,8) = (3, -8)\ \ $ 
and 
$\ \ 7\cdot (3,8) = \mathcal{O}$.  
See [[10]] for full details on this example.  
Note that the elliptic curve group $E(\mathbb{Q})$ generated by the point $(3,8)$ is cyclic of order 
$7$, denoted by $ \\# E(\mathbb{Q}) = 7$. Since $7$ is a prime number, $E(\mathbb{Q})$ is 
isomorphic to the field $\mathbb{F}_7 = \mathbb{Z}_7$. 
Therefore $\mathbb{F}_7$ is the so called scalar field for the elliptic curve group $E(\mathbb{Q})$, while 
$\mathbb{Q}$ is the base field. 

In order to be in line with the literature, an elliptic curve will henceforth be 
referring to an elliptic curve group $E(\mathbb{F})$. And unless otherwise stated, 
the base field will be a field of a large prime order $p$, denoted by $\mathbb{F}_p$.


### Arithmetic Circuits and R1CS 

A zero-knowledge proof typically involves two parties, the prover $\mathcal{P}$ and the verifier $\mathcal{V}$; 
the prover $\mathcal{P}$ has to convince the verifier $\mathcal{V}$ that he knows the correct solution to a 
set computational problem without disclosing the exact solution. 
So the prover has to produce a proof $\pi$ that attests to the his knowledge of the correct solution, and 
the verifier must be able to check its veracity without accepting false 
proofs. 

Arithmetic circuits are computational models commonly used when solving NP statements.
The general process for a zero-knowledge proof system is to convert the set computation or statement being proved into an arithmetic circuit $\mathcal{C}$ and further encode the circuit into an equivalent constraint system. 
These three are all equivalent in the sense that the proof $\pi$ satisfies the constraint system if and only if it satisfies 
$\mathcal{C}$, and if and only if the prover has the correct solution to the original computational problem. 



## Recursive Proof Composition Overview 

In their recent paper, Benedikt Buenz et al. report that,
"Recursive proof composition has been shown to lead to powerful primitives 
such as incrementally-verifiable computation (IVC) and proof-carrying data (PCD)," 
in [[1]]. 
Thus recognising the two main components of recursive proof composition, IVC and PCD. 
The former was adequately investigated in [[9]], and the latter is now the focus of this report. 
 
 
### Verification Amortization Strategies

These strategies are briefly mentioned here but their detailed descriptions can be found in [[9]]. 

**Verifiable Computation** 
allows a verifier to delegate expensive computations to untrusted third parties and be able 
check correctness of the proofs these third parties submit.   

**Inductive proofs** 
take advantage of whatever recursion that may be found in a computational problem. 
Thus a verifier need only check correctness of the "base step" and the 
"inductive step" according to the Principle of Mathematical Induction. 
Making this a powerful tool when it comes to the scalability problem 
particularly for blockchains.


**Incrementally Verifiable Computation** 
in addition to delegating computations to several untrusted parties, the verifier does not execute 
verification as often as he receives proofs from third parties but rather collects these proofs and 
only executes a single proof at the end. 

**Nested Amortization**
the strategy here is to reduce the cost of an expensive computation to a sub-linear 
cost (logarithmic relative to the cost of the original) by collapsing the 
cost of two computations to a cost of one.  




## Proof-Carrying Data 

Recursive proof composition uses an abstraction called *proof-carrying data* (PCD) when dealing with 
distributed computations. These PCDs are powerful tools that enable practical implementation 
of the above verification amortization strategies. 

### What is a PCD? 

**Proof-Carrying Data**, or PCD, is a cryptographic mechanism that allows 
proof strings $\pi_i$ to be carried along with messages $m_i$ in a distributed computation [[15]]. 


<p align="center"><a name="fig_dco"> </a><img src="sources/distributed-computation.png" width="450" /></p>
<div align="center"><b>Figure 2: Distributed Computation [[3]] </b></div> 

Such proofs $\pi_i$ attest to the fact that their corresponding messages $m_i$ as well as the 
history leading to the messages comply with a specified predicate. 
The assumption here is that there are specific invariant properties that all propagated
messages need to maintain. 

An obvious example of a distributed computation is PoW blockchains is mining. And of course, 
the integrity of any PoW blockchains relies on the possibility for proper validation of the PoW. 

### Trustlessness via PCDs

A PCD achieves trustlessness in the sense that untrusted parties carry out 
distributed computations, and a protocol compiler $\mathbf{\Pi}$ is used to enforce 
compliance to a predicate specified by the proof system designer. 
Such a compiler is typically defined as a function 
$$\mathbf{\Pi}(m\_i, m\_{loc,i}, \mathbf{m}\_{inc} ) \in \\{ 0, 1 \\}$$
taking as inputs a newly formed message $m_i$ at node $i$, a local data $m\_{loc,i}$ only known to 
node $i$, and a vector $\mathbf{m}\_{inc}$ of all incoming messages received by node $i$.  

Since a PCD is equipped with a protocol compiler $\Pi$ that ensures that every 
message is predicate-compliant, then it enables trustless cryptographic proof systems 
for two reasons: 
- mutually distrustful parties can perform distributed computations 
that run indefinitely, and
- due to proof strings attached to all previous messages, any node $j$ can verify any intermediate state of the computation and propagate a 
new message $m_j$ with its proof string $\pi_j$ attached to it.  

It now becomes clear how recursive proof composition accomplishes blockchain scalability. 
Any new node can take advantage of IVC and a PCD to succinctly verify the current state of 
the blockchain without concern about the chain's historical integrity.  

The PCD abstraction is no doubt the very secret to achieving blockchain scaling 
especially via recursive proof composition. 


## Cycles of Elliptic Curves 

Given the above discussion on PCDs, note that the cryptographic technique of using 
a *cycle of elliptic curves* is not much about scalability but rather about efficiency of 
arithmetic circuits. The main trick is to exploit the proof system's field structure. 

### Why Cycle of Elliptic Curves?

Why then the need for a second elliptic curve that warrants the use of a cycle of elliptic curves? 

The reason a second elliptic curve is needed is due to the inherent field structure of 
elliptic curves. There are two aspects of the field structure to carefully 
consider; 
- firstly, the best field arithmetic for arithmetic circuits that are instantiated with $E(\mathbb{F}_q)$, and 
- secondly, the possible or mathematically permissible orders of the scalar field of $E(\mathbb{F}_q)$. 

#### The Native Field Arithmetic 

When instantiating a proof system's arithmetic circuit using an elliptic curve 
$E(\mathbb{F}\_q)$, it is 
important to take cognizance of the two types of field arithmetic involved: 
the *base field* arithmetic and the *scalar field* arithmetic. 

For an elliptic curve $E(\mathbb{F}_p)$ where $p$ is a prime for simplicity, 
- the base field is $\mathbb{F}\_p$ and so its arithmetic consists of addition modulo $p$ 
and multiplication modulo $p$, 
- the scalar field is $\mathbb{F}\_r$, where $r$ is the prime order of the largest 
subgroup of $E(\mathbb{F}\_p)$, so in this case the arithmetic consists of addition modulo $r$ 
and multiplication modulo $r$. 

The question now is which arithmetic is better to use in a circuit instantiated with $E(\mathbb{F}_p)$. 

The next example makes the choice obvious. 

#### Example 2 

Consider the elliptic curve $E(\mathbb{Q})$ from [Example 1](#example-1). 
Since $E(\mathbb{Q})$ is isomorphic to $\mathbb{F}_7$, adding two elements 
in $E(\mathbb{Q})$ amounts to 
adding their scalars modulo $7$. 
That is, 
$$\ \ \alpha \cdot (3,8) + \beta \cdot (3,8) = ((\alpha + \beta) \text{ mod } 7) \cdot (3,8)$$ 
So, for example, if $\alpha = 5$ and $\beta = 6$, the sum of the points becomes
$$\ \ 5 \cdot (3,8) + 6 \cdot (3,8) = 4 \cdot (3,8)$$ 
It follows then that when instantiating any circuit using $E(\mathbb{Q})$, the arithmetic of the scalar field
$ \mathbb{F}_7 $ is more natural to use than the base field's. 

**Remark:** For practical purposes the order of the base field is always a large prime, and 
thus the infinite field of rationals $\mathbb{Q}$ is never used. 
Yet, even in cases where the base field is large and of prime order, 
a more native arithmetic is that of the scalar field. 

#### The Order of the Scalar Field 

For any finite field $\mathbb{F}\_q$, 
as noted earlier [here](#fields-and-elliptic-curves), 
either $q = p$ or $q = p^n$ for some $p$ a prime. 
Also, either $\\# (\mathbb{F}\_q^{\times}) = p - 1$ or $\\# (\mathbb{F}_q^{\times}) = p^n -1$.  

According to a well-known result, called Lagrange's Theorem, the order of any subgroup of a 
finite group $\mathbb{G}$ must divide  $\\# (\mathbb{G})$. 

It follows then that the order of any scalar field $\mathbb{F}\_q$ divides either 
$p - 1$ or $p^n -1$. 

It is therefore mathematically impossible for the scalar field of the elliptic curve $E(\mathbb{F}_q)$ 
to have the same order as the base field $\mathbb{F}_q$. 


#### No Prover-Verifier Dichotomy

In the envisaged proof-of-proofs system using recursive proof composition, the tremendous 
accomplishment is to allow every participant to simultaneously be a prover and a verifier. 
However, this presents a serious practical problem. 

Note the following common practices when implementing proof systems (gleaning information from [[15]]); 
- The proof system is instantiated with an elliptic curve $E$ over a base field $\mathbb{F}_q$ 
- The most natural field arithmetic for the arithmetic circuit is the scalar field's, 
$\mathbb{F}_r$-arithmetic 
- When computing operations on elliptic curve points inside the proof system verifier, the 
base field arithmetic is used. i.e., verifier uses $\mathbb{F}_q$-arithmetic 

See [Figure A1, BLS signature verification](#fig_bls) below for an illustration of how verification arithmetic can differ from 'prover' arithmetic. 

Normally, when there is a clear dichotomy between a prover and a verifier, the use of two 
distinct field arithmetics would not be an issue. 

"But here we are encoding our verifier inside of our arithmetic circuit; thus, 
we will need to simulate $\mathbb{F}_q$ arithmetic inside of $\mathbb{F}_r$ arithmetic," as Straka clarifies in [[15]].

Basically, the problem is that there is no efficient way to use both field arithmetics when 
there is no prover-verifier dichotomy. 

The ideal solution would be choosing the elliptic curve $E(\mathbb{F}_q)$ in such a way 
that $q = r$. 
However, it is mathematically impossible for 
$\\# (\mathbb{F}_r)$ to equal $\\# (\mathbb{F}_q)$, 
as observed above [here](#the-order-of-the-scalar-field). 

The adopted solution is to find a second elliptic curve, say $E(\mathbb{F}_r)$, with a 
scalar field $\mathbb{F}_q$ if possible. 
This is the reason why pairing-friendly curves, and recently Amicable pairs of elliptic curves, have 
come to be deployed in zero-knowledge proof systems such as zkSNARKs. 







### Pairing-friendly Elliptic Curves 

It was Groth in [[18]] who first constructed "a pairing-based 
(preprocessing) SNARK for arithmetic circuit satisfiability, which is an NP-complete language". 
When it comes to a scalable zero-knowledge proof system, it was Ben-Sasson et al in [[2]] 
who first presented a practical recursive proof composition that uses a cycle of elliptic curves. 

**Definition 1:** 
Given an elliptic curve $E$ over a field $\mathbb{F}$, the **embedding degree** of an elliptic curve 
$E(\mathbb{F}_q)$ is the smallest positive integer $k$ such that $r\$  divides $p^k - 1$, where $r$ is the order 
of the largest cyclic subgroup of the elliptic curve $E(\mathbb{F}_q)$. 
 

**Definition 2:** 
For implementing pairing-based cryptographic systems, elliptic curves with small embedding degree $k$ 
and large prime-order subgroups are called **pairing-friendly**.  

According to Freeman et al [[14]], "pairing-friendly curves are rare and thus require specific constructions". 
They furnish what they call a "single coherent framework" of constructions of pairing-friendly 
elliptic curves known to the authors at the time of publication. 

The Coda block chain [[19]] is an example of a deployed protocol using the Ben-Sasson approach in [[2]] 
using pairing-friendly MNT curves of embedding degrees 4 and 6. 



### Amicable Pairs of Elliptic Curves 

According to Bowe et al in [[8]], pairing-based curves of small embedding degree like 
MNT4/MNT6 constructions used in Coda require curves of size 
approaching 800 bits for the 128-bit security level. 

Constructions of pairing-friendly curves were mostly restricted to embedding degrees 
less than $k \leq 6$, until Barreto and Naehrig constructed curves of prime order and 
embedding degree $k = 12$ in [[20]].  

Some researchers, such as Chiesa et al [[21]], do not make any distinction between 
a cycle of pairing-friendly elliptic curves and Aliquot cycle of elliptic curves defined below. 
Except that Aliquot cycles are more concerned with primality. 


Minimum required properties of the second elliptic curve that forms an Amicable Pair with the 
elliptic curve group $E(\mathbb{F}_q)$ are, 
- the second curve must also be of a large prime order, 
- it must be compatible with the first in the sense that the verifier operations can be 
efficiently carried out in the second, 
- its DLP must be comparably as difficult as in the first $E(\mathbb{F}_q)$.  

An elliptic curve $E$ over a field $\mathbb{F}$ has a **good reduction** at a prime $p$ if 
the elliptic curve group $E(\mathbb{F}_p)$ has all the above mentioned properties. 


**Definition 3:** [[16]] 
An **Aliquot Cycle** of an elliptic curve $E$ over $\mathbb{Q}$ refers to a sequence of 
distinct primes $(p_1, p_2, \dots , p_l)$ such that $E$ has good reduction at every 
$p_i$ and such that 
$$  \\# E(\mathbb{F}\_{p\_{1}}) = p_2 ,\ \ \\# E(\mathbb{F}\_{p\_{2}}) = p_3 ,\ \ \dots\ \ , 
\\# E(\mathbb{F}\_{p\_{l-1}}) = p\_l ,\ \ \\# E(\mathbb{F}\_{p\_{l}}) = p\_1 $$

An Aliquot cycle is a generalized concept of an Amicable pair. So an Amicable pair is in fact 
an Aliquot cycle of length 2. That is, a cycle of only two elliptic curves. 

**Definition 4:** [[16]] 
An **Amicable Pair** of an elliptic curve $E$ over $\mathbb{Q}$ is any pair of primes $(p, q)$ such that $E$ has good reduction 
at $p$ and $q$ such that 
 $$\\# E(\mathbb{F}\_p) = q \text{  }  \text{  and  }  \text{  } \\# E(\mathbb{F}\_q) = p $$


Depending on the curve in case, and unlike pairing-friendly curves, some curves have a large number of amicable pairs. 
For instance, Silverman and Stange report in [[16]] that the curve of $y^2 = x^3 + 2$ has 
more than 800 amicable pairs using prime numbers that are less than $10^6$. 

See below a simplified depiction of a recursive proof system using 
an Amicable pair of elliptic curves. 
 
<p align="center"><a name="fig_apps"> </a><img src="sources/amicable-pair-1way-circuit.png" width="600" /></p>
<div align="center"><b>Figure 4: Amicable Pair Proof System </b></div> 








## Why the interest in Halo Protocol? 

   

### Brief Survey: Recursive Proofs protocols 

- table - comparison () [[7]]  
Coda (2018) - Common Reference String  
Sonic (2019) - updatable Structured Reference String  
Halo (2019) - no Trusted Setup   
- Coda & Halo (cf. my presentation on Amortized IPP)


<p align="center"><a name="fig_tth"> </a><img src="sources/tweedledum-tweedledee-halo.png" width="500" /></p>
<div align="center"><b>Figure 5: Tweedledee and Tweedledum: Halo Protocol's Pair of Elliptic Curves [[22]] </b></div> 


### Halo Protocol Design and Bulletproofs

First of all, Halo uses no trusted setup. 

The designers have taken some Bulletproofs techniques and modified for use in Halo 
- Pedersen Commitment Scheme generalized ~ Polynomial Commitment Schemes
- Modified Inner-product  
A generalized Schnorr Protocol ... giving it a Mimblewimble touch 






## Implications to Tari Blockchain 

We have focused on recursive proof composition, 
 ... Buenz et al say, 
"PCD supports computations defined on (possibly infinite) directed acyclic graphs, 
with messages passed along directed edges" [[1]] 
but what comes for free with this report is that PCDs can be used in Layer 2 for verification 
of digital assets. 
A kind-of-a PCD-powered hashgraph could be considered for the DAN.   



## References 

[[1]] B. Buenz, P. Mishra, A. Chiesa and N. Spooner, "Proof-Carrying Data from Accumulation Schemes", May 25, 2020
[online]. Available: <https://eprint.iacr.org/2020/499.pdf>. 
Date accessed: 2020&#8209;07&#8209;01. 

[1]: https://eprint.iacr.org/2020/499.pdf "Proof-Carrying Data from Accumulation Schemes" 



[[2]] E. Ben-Sasson, A. Chiesa, E. Tromer, and M. Virza, "Scalable Zero Knowledge via Cycles of Elliptic Curves", September 18, 2016 [online]. Available: <https://eprint.iacr.org/2014/595.pdf>. 
Date accessed: 2020&#8209;07&#8209;01.    

[2]: https://eprint.iacr.org/2014/595.pdf "Scalable Zero Knowledge via Cycles of Elliptic Curves" 



[[3]] A. Chiesa and E. Tromer, “Proof-Carrying Data and Hearsay Arguments from Signature Cards”, 
ICS ’10. 2010 [online]. Available: <https://people.eecs.berkeley.edu/~alexch/docs/CT10.pdf>. 
Date accessed: 2020&#8209;07&#8209;01.

[3]: https://people.eecs.berkeley.edu/~alexch/docs/CT10.pdf “Proof-Carrying Data and Hearsay Arguments from Signature Cards”



[[4]] A. Chiesa, "Proof-Carrying Data," PhD Thesis, MIT, June 2010. [online]. Available: 
<https://pdfs.semanticscholar.org/6c6b/bf89c608c74be501a6c6406c976b1cf1e3b4.pdf>.  Date accessed: 2020&#8209;07&#8209;01.  
 
[4]: https://pdfs.semanticscholar.org/6c6b/bf89c608c74be501a6c6406c976b1cf1e3b4.pdf "Proof-Carrying Data"




[[5]] E. Ben-Sasson, A. Chiesa, P. Mishra and N. Spooner, "Proof-Carrying Data from Accumulation Schemes",  May 25, 2020 [online]. Available: <https://eprint.iacr.org/2020/499.pdf>. Date accessed: 2020&#8209;07&#8209;06.
 
[5]: https://eprint.iacr.org/2020/499.pdf "Proof-Carrying Data from Accumulation Schemes" 



[[6]] A. Landesman, "NOTES ON FINITE FIELDS", [online]. Available: <https://web.stanford.edu/~aaronlan/assets/finite-fields.pdf>. Date accessed: 2020&#8209;07&#8209;06. 
 
[6]: https://web.stanford.edu/~aaronlan/assets/finite-fields.pdf "NOTES ON FINITE FIELDS" 



[[7]] E. Ben-Sasson, "The ZKP Cambrian Explosion and STARKs" November 2019 
[online]. Available: <https://starkware.co/decks/cambrian_explosion_nov19.pdf>. 
Date accessed: 2020&#8209;07&#8209;05. 

[7]: https://starkware.co/decks/cambrian_explosion_nov19.pdf "The ZKP Cambrian Explosion and STARKs"



[[8]] S. Bowe, J. Grigg and D. Hopwood, "Halo: Recursive Proof Composition without a
Trusted Setup" [online]. Available: <https://pdfs.semanticscholar.org/83ac/e8f26e6c57c6c2b4e66e5e81aafaadd7ca38.pdf>. Date accessed: 2020&#8209;07&#8209;05.

[8]: https://pdfs.semanticscholar.org/83ac/e8f26e6c57c6c2b4e66e5e81aafaadd7ca38.pdf "Halo: Recursive Proof Composition without a Trusted Setup"



[[9]] Tari Labs University, "Amortization of Bulletproofs Inner-product Proof", May 2020 [online]. Available: 
<https://tlu.tarilabs.com/cryptography/amortization-bp-ipp/mainreport.html#amortization-of-bulletproofs-inner-product-proof>. Date accessed: 2020&#8209;07&#8209;06. 

[9]: https://tlu.tarilabs.com/cryptography/amortization-bp-ipp/mainreport.html#amortization-of-bulletproofs-inner-product-proof "Amortization of Bulletproofs Inner-product Proof" 



[[10]] P. Bartlett, "Lecture 9: Elliptic Curves, CCS Discrete Math I", 2014 [online]. Available: 
<http://web.math.ucsb.edu/~padraic/ucsb_2014_15/ccs_discrete_f2014/ccs_discrete_f2014_lecture9.pdf>. Date accessed: 2020&#8209;07&#8209;06. 

[10]: http://web.math.ucsb.edu/~padraic/ucsb_2014_15/ccs_discrete_f2014/ccs_discrete_f2014_lecture9.pdf "Lecture 9: Elliptic Curves, CCS Discrete Math I"



[[11]]  S. Bowe, J. Grigg and D. Hopwood, "Halo: Recursive Proof Composition without a
Trusted Setup", Electric Coin Company, 2019 [online]. Available: <https://pdfs.semanticscholar.org/83ac/e8f26e6c57c6c2b4e66e5e81aafaadd7ca38.pdf>. Date accessed: 2020&#8209;07&#8209;06.

[11]: https://pdfs.semanticscholar.org/83ac/e8f26e6c57c6c2b4e66e5e81aafaadd7ca38.pdf "Halo: Recursive Proof Composition without a Trusted Setup"



[[12]] M.C. Welsh, "ELLIPTIC CURVE CRYPTOGRAPHY" REUpapers, 2017, [online]. Available: 
<http://math.uchicago.edu/~may/REU2017/REUPapers/CoatesWelsh.pdf>. Date accessed: 2020&#8209;07&#8209;07.

[12]: http://math.uchicago.edu/~may/REU2017/REUPapers/CoatesWelsh.pdf "ELLIPTIC CURVE CRYPTOGRAPHY" 



[[13]]  A.V. Sutherland, "Elliptic Curves, Lecture 9 Schoof's Algorithm", Spring 2015, [online]. Available: 
<https://math.mit.edu/classes/18.783/2015/LectureNotes9.pdf>. Date accessed: 2020&#8209;07&#8209;07.    

[13]: https://math.mit.edu/classes/18.783/2015/LectureNotes9.pdf "Elliptic Curves, Lecture 9 Schoof's Algorithm" 



[[14]] D. Freeman, M. Scott and E. Teske, "A TAXONOMY OF PAIRING-FRIENDLY ELLIPTIC CURVES", [online]. 
Available: <https://eprint.iacr.org/2006/372.pdf>. Date accessed: 2020&#8209;07&#8209;07. 

[14]: https://eprint.iacr.org/2006/372.pdf "A TAXONOMY OF PAIRING-FRIENDLY ELLIPTIC CURVES"



[[15]] M. Straka, "Recursive Zero-knowledge Proofs: A Comprehensive Primer" [online], 2019‑12‑08. 
Available: <https://www.michaelstraka.com/posts/recursivesnarks/>. Date accessed: 2020&#8209;07&#8209;07. 

[15]: https://www.michaelstraka.com/posts/recursivesnarks "Recursive Zero-knowledge Proofs: A Comprehensive Primer" 



[[16]] J.H. Silverman and K.E. Stange, "Amicable pairs and aliquot cycles for elliptic curves" [online], 2019‑12‑08. 
Available: <https://arxiv.org/pdf/0912.1831.pdf>. Date accessed: 2020&#8209;07&#8209;08. 

[16]: https://arxiv.org/pdf/0912.1831.pdf "Amicable pairs and aliquot cycles for elliptic curves" 



[[17]] A. Menezes, "An Introduction to Pairing-Based Cryptography" [online]. 
Available: <https://www.math.uwaterloo.ca/~ajmeneze/publications/pairings.pdf>. Date accessed: 2020&#8209;07&#8209;11.

[17]: https://www.math.uwaterloo.ca/~ajmeneze/publications/pairings.pdf "An Introduction to Pairing-Based Cryptography" 


 
[[18]] E. Groth, "On the Size of Pairing-based Non-interactive Arguments" [online]. Available: 
<https://eprint.iacr.org/2016/260.pdf>. Date accessed: 2020&#8209;07&#8209;12.

[18]: https://eprint.iacr.org/2016/260.pdf "On the Size of Pairing-based Non-interactive Arguments"



[[19]] I. Meckler and E. Shapiro, "Coda: Decentralized cryptocurrency at scale" O(1) Labs whitepaper. May 10, 2018. [online]. Available: <https://cdn.codaprotocol.com/v2/static/coda-whitepaper-05-10-2018-0.pdf>. Date accessed: 2020&#8209;07&#8209;12.

[19]: https://cdn.codaprotocol.com/v2/static/coda-whitepaper-05-10-2018-0.pdf "Coda: Decentralized cryptocurrency at scale"



[[20]] P.L.M.N. Barreto, M. Naehrig, "Pairing-Friendly Elliptic Curves of Prime Order" [online]. Available: <https://eprint.iacr.org/2005/133.pdf>. Date accessed: 2020&#8209;07&#8209;12.

[20]: https://eprint.iacr.org/2005/133.pdf "Pairing-Friendly Elliptic Curves of Prime Order"



[[21]] A. Chiesa, L. Chua and M. Weidner, "On cycles of pairing-friendly elliptic curves" [online]. Available: <https://arxiv.org/pdf/1803.02067.pdf>. Date accessed: 2020&#8209;07&#8209;12.

[21]: https://arxiv.org/pdf/1803.02067.pdf "On cycles of pairing-friendly elliptic curves" 


[[22]] S. Bowe, "Halo: Recursive Proofs without Trusted Setups (video)", Zero Knowledge Presentations Nov 15, 2019 [online]. 
Available: <https://www.youtube.com/watch?v=OhkHDw54C04>. Date accessed: 2020&#8209;07&#8209;13.

[22]: https://www.youtube.com/watch?v=OhkHDw54C04 "Halo: Recursive Proofs without Trusted Setups (video)"






## Appendices 

### Pairing Cryptography 

Pairing-based cryptographic systems are defined on pairings like the Weil pairing and the Tate pairing all 
characterised by a bilinear mapping defined on a pair of groups; an additively group including 
a special element $\mathcal{O}$, and a multiplicative group. 

Although applications of pairing-based cryptography were known for a long time in the form of 
one-round three-party key agreements, they became more popular with the emergence of 
identity-based encryption. 


Let $n$ be a prime number, $P$ a generator of an additively-written group of $G_1 = ⟨P⟩$ with identity $\mathcal{O}$, and $G_T$ a multiplicatively-written group of order $n$ with identity $1$.

**Definition A1:** [[17]] 
A **bilinear pairing** on $(G_1, G_T)$  is a map 
$$ \hat{e} : G_1 \times G_1  \to G_T$$ 
(a) (*bilinear*) For all $R$, $S$, $T \in G_1$, $\ \ \hat{e} (R + S,T) = \hat{e} (R,T) \hat{e}(S,T)\ \ $ and 
$\ \ \hat{e} ( R , S + T ) = \hat{e} ( R , S ) \hat{e} ( R , T )$   
(b) (*non-degeneracy*) $\hat{e} (P, P ) \not= 1$  
(c) (*computability*)  The map $ \hat{e}$ can be efficiently computed. 

One of the most important properties of the bilinear map $\hat{e}$ is that, for all $a$,$b$ $\in \mathbb{Z}$, 
$$ \hat{e}(aS,bT) = \hat{e}(S,T)^{ab}\ \$$

Take the Boneh-Lynn-Shacham short signature, or BLS-signature, as an example of a 
pairing-based cryptographic primitive. 
  
**Example A1** 
The BLS-signature uses a bilinear map $\hat{e}$ on $(G_1, G_T)$ for which the Diffie-Hellman Problem 
is intractable. 
Say, Alice wants to send a message to Bob with an attached signature.  
- Alice randomly selects an integer $a \in \[ 1, n-1 \]$ and creates a public key $A = aP$ where $P$ 
is generator of the group $G_1$. 
- Alice's BLS-signature on a message $m \in \\{ 0, 1 \\}^n$ is $S = aM$ where $M = H(m)$ and $H$ a 
hash function $H : \\{ 0, 1 \\}^n \to G_1 \backslash \mathcal{O}$.  

How can Bob or any party verify Alice's signature?
- By first computing $M = H(m)$ and then check if $\hat{e}(P,S) = \hat{e}(A,M)$ 


This why the verification works, 
$$\hat{e}(P,S) = \hat{e}(P, aM) = \hat{e}(P,M)^a$$ 
$$\hat{e}(A,M) = \hat{e}(aP,M) = \hat{e}(P,M)^a $$ 

See the diagram below that illustrates the verification. 

<p align="center"><a name="fig_bls"> </a><img src="sources/bls-signature.png" width="600" /></p>
<div align="center"><b>Figure A1: BLS Signature Verification </b></div>





