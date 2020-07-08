
# Trustless Recursive Zero-Knowledge Proofs

- [Introduction](#introduction) 
- [Notation and Preliminaries](#notation-and-preliminaries)
  - [Fields and Elliptic Curves](#fields-and-elliptic-curves)
     - [Example 1](#example-1)
  - [Arithmetic Circuits and R1CS](#arithmetic-circuits-and-r1cs) 
- [Recursive Proof Composition Overview](#recursive-proof-composition-overview) 
  - [Verification Amortization Strategies](#verification-amortization-strategies)
  - [Complexity of Delegated Computations](#complexity-of-delegated-computations)
- [Proof-Carrying Data](#proof-carrying-data) 
  - [What is a PCD?](#what-is-a-pcd)
  - [Proof-Carrying Data Systems](#proof-carrying-data-systems)
- [Cycles of Elliptic Curves](#cycles-of-elliptic-curves) 
  - [Aliquot Cycle of Elliptic Curves](#aliquot-cycle-of-elliptic-curves) 
    - [Pairing-friendly Elliptic Curves](#pairing-friendly-elliptic-curves)
    - [Amicable Pairs of Elliptic Curves](#amicable-pairs-of-elliptic-curves)
  - [Why a Cycle of Elliptic Curves?](#why-a-cycle-of-elliptic-curves)
    - [The Native Field Arithmetic](#the-native-field-arithmetic)
    - [Example 2](#example-2)
    - [Order of the Scalar Field](#order-of-the-scalar-field)  
- [Why the interest in Halo Protocol?](#why-the-interest-in-halo-protocol) 
  - [Brief Survey: Recursive Proofs Protocols](#brief-survey-recursive-proofs-protocols) 
  - [Halo Protocol Design and Bulletproofs](# ) 
- [Implications to Tari Blockchain](#implications-to-tari-blockchain) 
- [References](#references)
- [ ](# ) 
- [ ](# ) 




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
that utilises [cycles of elliptic curves](#cycles-of-elliptic-curves), first realised by Eli Ben-Sasson et al [[5]]. 



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
This can cause confusion especially when talking about the base field as opposed to the scalar field.  

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
- Note that all finite fields are either of prime order or powers of a prime. 
See [[6], Lemma 3.19] for the proof of this fact.  

- A **scalar field** of an elliptic curve $E$ comes in when 
an elliptic curve group $E(\mathbb{F})$ is defined, and this is 
typically when the base field is of prime order 
(i.e., when the base field $\mathbb{F} = \mathbb{F}_p$). 
So the scalar field of $E(\mathbb{F}_p)$ is actually the finite field that is isomorphic to 
(i.e., has the same order as) the largest cyclic subgroup of the elliptic curve group. 

- An **elliptic curve group** $E(\mathbb{F})$ is formed by first defining 'addition' of elliptic curve points, 
 picking a point $(x,y)$ on the curve $E$ and using it to generate a cyclic group by 
 doubling it and forming all possible scalar multiples. 
 All these points generated by $(x,y)$ together with the point at infinity, $ \mathcal{O}$, form an algebraic group 
 under the defined 'addition' of points.  

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
$7$, denoted by $ \\# E(\mathbb{Q}) = 7$. Therefore isomorphic to the field $\mathbb{F}_7 = \mathbb{Z}_7$. 
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
$\mathcal{C}$, and if and only it satisfies the prover the correct solution to the original computational problem. 



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


### Complexity of Delegated Computations 

Delegated computations differ in complexity. 
The more complex the computation the bigger the number of proof elements, 
and the higher the verification cost. 

For instance, 
if the delegated computation is inversion of a field element, then 
only one proof element is sent to the verifier, and the verification cost is one multiplication. 

A delegated inner-product proof, on the other hand, involves at least $2log_2(n)$ proof 
elements instead of $2n$. And the verification costs at least $2log_2(n)$ multiplications. 

Fortunately, most computations that can be delegated possess strong algebraic properties, 
and these properties allow optimization of the computations (cf. [[9]]) 




## Proof-Carrying Data 

Recursive proofs use an abstraction called *proof-carrying data* (PCD) when dealing with 
distributed computations. These PCDs are powerful tools that enable practical implementation 
of the above amortization strategies. 


### What is a PCD? 

**Proof-Carrying Data**, or PCD, is a cryptographic mechanism that allows 
proof strings $\pi_i$ to be carried along with messages $m_i$ in a distributed computation [[15]]. 


<p align="center"><a name="fig_dco"> </a><img src="sources/distributed-computation.png" width="450" /></p>
<div align="center"><b>Figure 1: Distributed Computation [[3]] </b></div> 

Such proofs $\pi_i$ attest to the fact that their corresponding messages $m_i$ as well as the 
history leading to the messages comply with a specified predicate. 
The assumption here is that there are specific invariant properties that all propagated
messages need to maintain. 

Thus a PCD achieves a trustlessness in the sense that untrusted parties carry out 
distributed computation, and a protocol compiler $\mathbf{\Pi}$ is used to enforce 
compliance to the specified predicate. Such a compiler is typically defined as 
$$\mathbf{\Pi}(m_i, m_{loc,i}, \mathbf{m}_{inc} ) = \\{ 0, 1 \\}$$
taking as inputs a newly formed message $m_i$ at node $i$, local data only known to 
node $i$, and a vector $\mathbf{m}\_{inc}$ of all messages received by node $i$.  



## Cycles of Elliptic Curves 

The cryptographic technique of using a cycle of elliptic curves seeks to improve efficiency of 
arithmetic circuits by exploiting the proof system's field structure. 

### Aliquot Cycle of Elliptic Curves 

**Definition 1 :** [[16]] 
An Aliquot Cycle of an elliptic curve $E$ over $\mathbb{Q}$ refers to a sequence of 
distinct primes $(p_1, p_2, \dots , p_l)$ such that $E$ has good reduction at every 
$p_i$ and such that 
$$  \\# E(\mathbb{F}\_{p\_{1}}) = p_2 ,\ \ \\# E(\mathbb{F}\_{p\_{2}}) = p_3 ,\ \ \dots\ \ , 
\\# E(\mathbb{F}\_{p\_{l-1}}) = p\_l ,\ \ \\# E(\mathbb{F}\_{p\_{l}}) = p\_1 $$




#### Pairing-friendly Elliptic Curves 

**Definition 2 :** [[14]] 



#### Amicable Pairs of Elliptic Curves 

**Definition 3 :** [[16]] 
Amicable pair of an elliptic curve E  is any pair of primes $(p, q)$ such that has good reduction 
at $p$ and $q$ and such that 
 $$ \\# E(\mathbb{F}\_p) = q , \text{ and } \\# E(\mathbb{F}\_q) = p $$



- a diagram of the "Amicable Pair-based Zero-Knowledge Proof" 


### Why a Cycle of Elliptic Curves?

Why then the need for a second elliptic curve that warrants the use of cycles of elliptic curves? 

#### The Native Field Arithmetic 

When instantiating a proof system's arithmetic circuit using an elliptic curve 
$E(\mathbb{F}_p)$, it is 
important to take cognizance of the two types of field arithmetic involved. 
Between the base field arithmetic and the scalar field arithmetic 
of $E(\mathbb{F}_p)$, one is more natural than the other.

The next example makes the choice obvious. 

#### Example 2 

Consider the elliptic curve $E(\mathbb{Q})$ from [Example 1](#example-1). 
Since $E(\mathbb{Q})$ is isomorphic to $\mathbb{F}_7$, adding two elements 
in $E(\mathbb{Q})$ amounts to 
adding their scalars modulo $7$. 
That is, 
$\ \ \alpha \cdot (3,8) + \beta \cdot (3,8) = ((\alpha + \beta) \text{ mod } 7) \cdot (3,8)$  
So, for example, if $\alpha = 5$ and $\beta = 6$, the sum of the points becomes
$\ \ 5 \cdot (3,8) + 6 \cdot (3,8) = 4 \cdot (3,8)$  
It follows then that when instantiating any circuit using $E(\mathbb{Q})$, the arithmetic of the scalar field
$ \mathbb{F}_7 $ is more natural than the base field's. 

Note that for practical purposes the order of the scalar field is always a large prime. Yet, even in cases 
where the base field is also large and of prime order, a more native arithmetic is that of the scalar field. 

#### Order of the Scalar Field 

It is mathematically impossible for the scalar field of the elliptic curve $E(\mathbb{F}_p)$ 
to have the same order as the base field $\mathbb{F}_p$. 




## Why the interest in Halo Protocol? 

The first without a trusted setup. 

### Brief Survey: Recursive Proofs protocols 

- table - comparison () [[7]] 
- Coda & Halo (cf. my presentation on Amortized IPP)

### Halo Protocol Design and Bulletproofs

Polynomial Commitment Schemes
Pedersen Commitment Scheme generalized 
Modified Inner-product 





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



[[3]] A. Chiesa and E. Tromer, “Proof-Carrying Data and Hearsay Arguments from Signature Cards”, ICS ’10. 2010 [online]. Available: <https://people.eecs.berkeley.edu/~alexch/docs/CT10.pdf>. Date accessed: 2020&#8209;07&#8209;01.

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



