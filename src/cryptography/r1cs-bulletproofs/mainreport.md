##### 


# **Rank-1 Constraint System with Application to Bulletproofs**



- [Introduction](#introduction) 
- [Arithmetic Circuits](#arithmetic-circuits)
  - [Overview](#overview)
  - [Definition of Arithmetic Circuit](#definition-of-arithmetic-circuit)
  - [Example of Arithmetic Circuit](#example-of-arithmetic-circuit) 
- [Rank-1 Constraint Systems](#rank-1-constraint-systems)
  - [Overview](#overview-2)
  - [Definition of Constraint System](#definition-of-constraint-system)
  - [Definition of Rank-1 Constraint System](#definition-of-rank-1-constraint-system)
  - [Example of Rank-1 Constraint System](#example-of-rank-1-constraint-system) 
- [From Arithmetic Circuits to Programmable Constraint Systems for Bulletproofs ](#from-arithmetic-circuits-to-programmable-constraint-systems-for-bulletproofs)
- [zk-SNARK Constraint Systems](#zk-snark-constraint-systems) 
- [Interstellar's Bulletproof Constraint System](#interstellars-bulletproof-constraint-system)
  - [Overview](#overview-3) 
  - [Easy to Build Constraint Systems](#easy-to-build-constraint-systems) 
  - [About Gadgets](#about-gadgets) 
  - [Previous Work on Verifiable Shuffles](#previous-work-on-verifiable-shuffles)
  - [Interstellar's Concluding Remarks](#interstellars-concluding-remarks) 
  - [R1CS Factorization Example for Bulletproofs ](#r1cs-factorization-example-for-bulletproofs)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)
- [References](#references) 
- [Contributors](#contributors) 





## Introduction 

This report explains the technical underpinnings of Rank-1 Constraint Systems (R1CSs) as applied 
to zero-knowledge Succinct Non-interactive Argument of Knowledge (zk-SNARKs) and Bulletproofs, with a special focus on Bulletproofs. 

The literature on the use of R1CSs in zk-SNARKs shows that this mathematical tool is used simply as one part of many in 
a complex process towards achieving a zk-SNARK proof. Not much attention is given to it, not even in explaining 
what "rank-1" actually means. Although the terminology is similar to the traditional _rank of a matrix_ in 
linear algebra, examples on the Internet do not yield a _reduced matrix with only one non-zero row or column_. 

R1CSs became more prominent for Bulletproofs due to research work done by Cathie Yun and her colleagues at Interstellar. The constraint 
systems, in particular R1CSs, are used as an add-on to Bulletproof protocols. The title of 
Yun's article, "Building on Bulletproofs" 
[[1]] suggests this is true. One of the Interstellar team's goals is to use the constraint system in their 
Confidential Asset Protocol called the Cloak and in their envisaged Spacesuit. 
Despite their work on using R1CS being research in progress, their detailed notes on constraint systems and their 
implementation in RUST are available in [[13]]. 

Most recent constructions of zk-SNARKs and Bulletproofs involve _arithmetic circuits_ and _R1CSs_, 
either explicitly or implicitly. 

The aim of this report is to: 

- highlight the connection between arithmetic circuits and R1CSs; 
- clarify the difference R1CSs make in Bulletproofs and in range proofs; 
- compare R1CSs in zk-SNARKs and Bulletproofs; 
- explain what is meant by the following in [[12]] - "We note that a range proof 
  using the protocol of [[BCC+16]](Bootle et al's paper [[4]]) would have required implementing the commitment opening 
  algorithm as part of the verification circuit, which we are able to eliminate." 



## Arithmetic Circuits

### Overview 

Arithmetic circuits are said to be the most natural and standard model for computing polynomials. Every 
function $ \mathcal{H} : \lbrace 0 , 1 \rbrace^n → \lbrace 0 , 1 \rbrace^m $ of fixed input and output _length_ 
can be represented as an arithmetic circuit over any finite field $ \mathcal{F}_p$ [[2]]. 

According to [[3]], arithmetic circuits are a more highly structured model of computation 
than Boolean circuits. For instance, when studying arithmetic circuits, one is interested in _syntactic_ 
computation of polynomials, whereas in studying Boolean circuits, one is interested in the _semantics_ of the computation. 
In the Boolean case, one is not interested in any specific polynomial representation of the function, 
but only wants to compute some representation of the function. In the arithmetic case, one focuses on a 
_specific representation_ of the function. 

For our purposes, arithmetic circuits have as inputs variables $ x_1, \dots , x_n $. Computations are 
performed using the arithmetic operations $ + $ and $ × $, and may involve constants from a field $\mathcal{F}_p$. 
Complexity measures associated with such circuits are _size_ and _depth_, which capture _the number of gates_ in a 
circuit and _the maximum distance between an input and an output_, respectively [[3]].



### Definition of Arithmetic Circuit 

An ***arithmetic circuit*** $\mathcal{A}$ over the field $\mathcal{F}$ and the set of variables 
$X = \lbrace  {x_1,\dots,x_n} \rbrace$ is a _directed acyclic graph_ such that the vertices of $\mathcal{A}$ are called 
_gates_, while the edges are called _wires_ [[3]]: 

- A gate is said to be of _in-degree_ $l$ if it has $l$ incoming wires, and similarly, of _out-degree_ $k$ if 
  it has $k$ outgoing wires. 
- Every gate in $\mathcal{A}$ of _in-degree_ 0 is labeled by either a variable ${x_i}$ or some field 
  element from $\mathcal{F}$. Such a gate is called an _input gate_. Every gate of _out-degree_ 0 is called an 
  _output gate_. 
- Every other gate in $\mathcal{A}$ is labeled by either $\otimes$ or $\oplus$, and called a 
  _multiplication gate_ or _addition gate_, respectively. We are interested in _binary_ circuits, i.e. circuits where 
  each gate has two _input wires_ and one _output wire_. 
- An _arithmetic circuit_ is called a _formula_ if it is a _directed tree_ whose edges are _directed_ from the leaves 
  to the root. 
- The size of $\mathcal{A}$, denoted $|\mathcal{A}|$, is the number of _wires_, i.e. _edges_, in $\mathcal{A}$. 

A typical multiplication gate has a _left input_ $a_L$, a _right input_ $a_R$ and 
an _output_ $a_O$. Also, we note that $ a_L \cdot a_R - a_O = 0 $.

<div align="center"><b>
<img src="sources/basic-multiplication-gate.png" alt="basic-multiplication-gate" style="zoom:67%;" />
</b></div> 
<div align="center"><b>Figure 1: Typical Multiplication Gate</b></div> 
We note that in cases where the inputs and outputs are all _vectors_ of 
$n$ components, i.e. $\mathbf{a_L} = ( a_{L, 1}, a_{L, 2} , \dots , a_{L, n})$, 
$\mathbf{a_R} = ( a_{R, 1}, a_{R, 2} , \dots , a_{R, n})$ 
and $\mathbf{a_O} = ( a_{O, 1}, a_{O, 2} , \dots , a_{O, n})$, 
then _multiplication_ of $\mathbf{a_L}$ and $\mathbf{a_R}$ is defined as an _entry-wise_ 
product called the _**Hadamard product**_; 

$$
\mathbf{a_L}\circ \mathbf{a_R} = \big(( a_{L, 1} \cdot a_{R, 1} ) , ( a_{L, 2} \cdot a_{R, 2} ) , \dots , ( a_{L, n} \cdot 
a_{R, n} ) \big) =  \mathbf{a_O}
$$

The equation ${ \mathbf{a_L}\circ \mathbf{a_R} = \mathbf{a_O} }$ is referred to as a _multiplicative constraint_, 
but is also known as the _Hadamard relation_ [[4]]. 


### Example of Arithmetic Circuit 

An arithmetic circuit computes a polynomial in a natural way, as shown in this example. Consider the following 
arithmetic circuit $\mathcal{A}$ with inputs $\lbrace x_1 , x_2 , 1 \rbrace$ over some field $\mathcal{F}$: 

<div align="center"><b>
<img src="sources/polynomial-eg-ac.png" alt="polynomial-eg-ac" style="zoom:67%;" />
</b></div> 
<div align="center"><b>Figure 2: Arithmetic Circuit</b></div> 
The output of $\mathcal{A}$ above is the polynomial $x^2_1 \cdot x_2 + x_1 + 1 $ of _total degree_ three. 

A typical computational problem would involve finding the solution to, let's say, $x^2_1 \cdot x_2 + x_1 + 1 = 22$. 
Or, in a proof of knowledge scenario, the prover has to prove to the verifier that they have the correct solution to such 
an equation. 

Following the wires in the example shows that an arithmetic circuit actually _breaks down_ the given computation into 
_smaller equations_ corresponding to each _gate_: 

$$
u = x_1 \cdot x_1 \quad \text{,} \quad v = u \cdot x_2 \quad \text{,} \quad y = x_1 + 1 \quad \text{and} \quad z = v + y
$$

The variables $u, v$ and $ y $ are called _auxiliary variables_ or _low-level variables_, while 
${ z }$ is the _output_ of $ \mathcal{A} $. Thus, in addition to computing polynomials naturally, an 
arithmetic circuit helps in reducing a computation to a _low-level_ language involving only _two_ variables, 
_one operation_ and an _output_. 

Although [[3]] describes arithmetic circuits as being a more highly structured model of computation 
than Boolean circuits, it also points out that "we do not know how to efficiently reconstruct a circuit using only queries to 
the polynomial it computes". 

Zero-knowledge (ZK) proofs, such as zk-SNARKs and Bulletproofs, require that _statements to be proved_ are expressed in their 
simplest terms for efficiency. According to [[5]], "the ZK SNARK end-to-end journey is to create a _function_ 
to write proofs about", but they "must work with specific constructs". That is why arithmetic circuits are so important 
in making ZK more efficient: "these _functions_ have to be specified as sequences of very simple terms, 
namely, additions and multiplications of only two terms in a particular field" [[5]].

In verifying a zk-SNARK proof, the verifier needs to carry out a step-by-step check of the computations, i.e. for 
each gate, the verifier has to check if the _output_ $ a_O $ is correct with respect to the given _inputs_ 
$ a_L $ and $ a_R $, and test if $ a_L \cdot a_R - a_O = 0 $ for each multiplication 
gate. This requires that an _addition gate_ be treated as some form of a _multiplication gate_. This is explained later in 
this report. 





## Rank-1 Constraint Systems 

### Overview

Although a computation problem is typically expressed in terms of a high-level programming language, a zk-SNARK requires 
it to be expressed in terms of a set of _quadratic constraints_, which is closely related to circuits of logical gates. 

R1CS is short for Rank-1 Constraint System. These types of constraint systems have been featured in several 
constructions of zk-SNARKs. At times they were simply referred to as _quadratic constraints_ or _quadratic equations_; refer to 
[[6]], [[7]] and [[8]]. 



### Definition of Constraint System 

A constraint system was originally defined by Bootle 
et al in [[4]]. The Dalek team give a more general definition of a constraint system in [[10]]: 

"A **constraint system** is a collection of arithmetic constraints over a set of variables. There are two kinds of 
variables in the constraint system:

1. ${m}$ **high-level** variables, the input secrets ${ \mathbf{v}}$, 
2. $ n$ **low-level** variables, the internal input vectors ${ \mathbf{a}_L}$ and 
   ${ \mathbf{a}_R},$ and output vectors ${ \mathbf{a}_O } $ of the multiplication gates." 

Specifically, a **Rank-1 Constraint System** **(R1CS)** is a system that consists of two sets of constraints: 

- ${ n}$ _multiplicative constraints_, $ \mathbf{ a_L \circ a_R = a_O } $, where "${ \circ }$" is the Hadamard product; and
- ${ q}$ _linear constraints_, $\mathbf{W_L\cdot { a_L} + W_R\cdot { a_R} + W_O\cdot { a_O } = W_V\cdot { v + c} } $, where $\mathbf{W_L, W_R, W_O}$ and $\mathbf{W_V}$ are weights applied to respective input vectors and output vectors [[10]]. 

Note that it was Bootle et al. who first expressed _arithmetic circuit satisfiability_ in terms of the _Hadamard relation_ 
and linear constraints [[10]]. In their definition, the above linear constraints are written as:

$$
\mathbf{W_L\cdot { a_L} + W_R\cdot { a_R} + W_O\cdot { a_O } = c }
$$

That is, without the vector $\mathbf{v}$ and its weight $\mathbf{W_V} $. This is explained later. 



### Definition of Rank-1 Constraint System 

This paragraph provides a simplified definition of an R1CS as it applies to zk-SNARKs [[9]]. 

An R1CS is a sequence of groups of three vectors ${ \bf{a_L}}, { \bf{a_R}}, { \bf{a_O}} ,$ and the 
solution to an R1CS is a vector ${ \bf{s}}$ that satisfies the equation:

$$
{ \langle {\bf{a_L , s}} \rangle \cdot \langle {\bf{a_R , s }} \rangle - \langle {\bf{a_O , s }} \rangle = 0 }
$$




### Example of Rank-1 Constraint System 

One solution to the equation ${x^2_1 x_2 + x_1 + 1 = 22}$, from the preceding example of an arithmetic circuit, is ${ x_1 = 3}$ 
and ${ { x_2 = 2 }}$ belonging to the appropriate field ${ \mathcal{F}}$. Thus the solution vector 
${ { s = ( const , x_1 , x_2 , z , u , v , y )}}$ becomes ${ { s = ( 1 , 3 , 2 , 22 , 9 , 18 , 4 )}}$. 

It is easy to check that the R1CS for the computation problem in the preceding example is as follows (one need only test 
if ${ \langle {\bf{a_L , s}} \rangle  \cdot  \langle {\bf{a_R , s }} \rangle - \langle {\bf{a_O , s }} \rangle = 0}$ 
for each equation). 



<div align="center"><b>Table 1: Equations and Rank-1 Constraint System Vectors</b></div>  


| Equation                        | Rank-1 Constraint System Vectors                             |
| ------------------------------- | ------------------------------------------------------------ |
| ${ u = x_1\cdot x_1}$          | $ {\bf{a_L}} = ( 0 , 1 , 0 , 0 , 0 , 0 , 0 ) , \ \ {\bf{a_R}} = ( 0 , 1 , 0 , 0 , 0 , 0 , 0  ) ,\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 0 , 1 , 0 , 0  ) $ |
| $ { v = u\cdot x_2 }$          | $ {\bf{a_L}} = ( 0 , 0 , 0 , 0 , 1 , 0 , 0 ) ,\ \ {\bf{a_R}} = ( 0 , 1 , 0 , 0 , 0 , 0 , 0  ),\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 0 , 0 , 1 , 0 )  $ |
| $ { y = 1\cdot( x_1 + 1 ) } $ | ${\bf{a_L}} = ( 1 , 1 , 0 , 0 , 0 , 0 , 0 ),\ \ {\bf{a_R}} = ( 1 , 0 , 0 , 0 , 0 , 0 , 0 ),\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 0 , 0 , 0 , 1 ) $ |
| $ { z = 1\cdot( v + y )} $    | ${\bf{a_L}} = ( 0 , 0 , 0 , 0 , 0 , 1 , 1 ),\ \ {\bf{a_R}} = ( 1 , 0 , 0 , 0 , 0 , 0 , 0 ),\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 1 , 0 , 0 , 0 )$ |





In a more formal definition, an **R1CS** is a set of three matrices ${\bf{ A_L , A_R }}$ and ${\bf A_O}$, 
where the rows of each matrix are formed by the corresponding vectors $ {\bf{a_L }}$, ${ \bf{a_R }}$ and 
${ \bf{a_O}} $, respectively, as shown in Table 1: 



$$
\bf{A_L} = \bf{\begin{bmatrix} 0&1&0&0&0&0&0 \\\\ 0&0&0&0&1&0&0 \\\\ 1&1&0&0&0&0&0 \\\\ 0&0&0&0&0&1&1 \end{bmatrix}} \text{,} \quad 
\bf{A_R} = \bf{\begin{bmatrix} 0&1&0&0&0&0&0 \\\\ 0&1&0&0&0&0&0 \\\\ 1&0&0&0&0&0&0 \\\\ 1&0&0&0&0&0&0 \end{bmatrix}} \text{,} \quad 
\bf{A_O} = \bf{\begin{bmatrix} 0&0&0&0&1&0&0 \\\\ 0&0&0&0&0&1&0 \\\\ 0&0&0&0&0&0&1 \\\\ 0&0&0&1&0&0&0 \end{bmatrix}}
$$



We observe that ${ \bf{ (A_L\cdot s^T) * (A_R\cdot s^T ) - (A_O\cdot s^T)} = 0 }$, where "${\bf{ \cdot }}$" is 
_matrix multiplication_ and ${ \bf s^T}$ is the transpose of the solution vector ${ \bf{s}}$. 

  



## From Arithmetic Circuits to Programmable Constraint Systems for Bulletproofs 

[_Interstellar's Programmable Constraint Systems for Bulletproofs_](#interstellars-bulletproof-constraint-system) 
is an extension of _Zero-knowledge Proofs for Arithmetic Circuits_ by Bootle et al. [[4]], enabling protocols that 
support _proving of arbitrary statements_ in ZK using constraint systems. Although our focus here is on the 
two works of research [[4]] and [[11]], we recognize the _Bulletproofs paper_ by Bunz et al. [[12]] as a bridge 
between the two. We thus split the comparison to be among **three** works of research as shown in Table 2.

All these are ZK proofs are based on the difficulty of the discrete logarithm problem. 

<div align="center"><b>Table 2: Comparison of three Research Works on ZK Proofs</b></div> 


| No.  | Efficient Zero-knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting [[4]] (2016) | Bulletproofs: Short Proofs for Confidential Transactions and More [[12]] (2017) | Programmable Constraint Systems  [[11]] (2018)               |
| ---- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| 1.   | Introduces the Hadamard relation and linear constraints.     | Turns the Hadamard relation and linear constraints into a single linear constraint, and these are in fact the R1CS. | Generalizes constraint systems and uses what is called gadgets as building blocks for constraint systems. |
| 2.   | Improves on Groth's work [[15]] on ZK proofs. Reducing a $\sqrt{N}$  complexity to  $6log_2(N) + 13$, where $N$  is the circuit size. | Improves on Bootle et al.'s work [[4]]. Reducing a $2log_2(N) + 13$ complexity to  $6log_2(N) + 13​$, where $N​$ is the circuit size. | Adds constraint systems to Bunz et al.'s work on Bulletproofs, which are short proofs, and the complexity advantage is seen in proving several statements at once. |
| 3.   | Introduces logarithm-sized inner-product ZK proofs.          | Introduces Bulletproofs, extending proofs to proofs of arbitrary statements. The halving method is used on the inner-products, resulting in the above reduction in complexity. | Introduces gadgets that are actually add-ons to an ordinary ZK proof. A range proof is an example of a gadget. |
| 4.   | Uses Fiat-Shamir heuristics in order to achieve non-interactive ZK proofs. | Bulletproofs have no trusted setup, as they use  the Fiat Shamir heuristics to achieve non-interactive ZK proofs. | Merlin transcripts are specifically used for a Fiat-Shamir transformation to achieve non-interactive ZK proofs. |
| 5.   | The Pedersen commitments are used in order to achieve ZK property. | Eliminates the need for a commitment algorithm by including Pedersen commitments among the inputs to the verification proof. | Low-level variables, representing inputs and outputs to multiplication gates, are computed per proof and committed using a single vector Pedersen commitment. |
| 6.   | The ZK proof involves conversion of the arithmetic circuit into an R1CS. | The mathematical expression of a Hadamard relation is closely related to an arithmetic circuit. The use of this relation plus linear constraints as a single constraint amounts to using a constraint system. | Although arithmetic circuits are not explicitly used here, the Hadamard relation remains the same as first seen in Bulletproofs, more so in the inner-product proof. |
| 7.   | The ZK proof here is for NP statements based on the difficulty of the discrete logarithm problem. | As mentioned above, Bulletproofs extend ZK proofs (such as range proofs) to proofs on arbitrary statements. They are also described as short non-interactive proofs for arithmetic circuits without a trusted setup. | Interstellar is building an Application Programming Interface (API) that allows developers to choose their own collection of gadgets suitable for the protocol they wish to develop. |






## zk-SNARK Constraint Systems 

### Overview 

How exactly does one go _**from an arithmetic circuit to an R1CS**_ ? 

Eli Ben-Sasson et al. [[8]] mention that the difference in computational power among the parties makes _succinct verification_ 
a requirement. Hence, the pursuit of short and fast proofs systems. These make it possible for a verifier to check 
a non-deterministic polynomial-time computation in a much shorter time than the time required to run the computation 
when given a valid NP witness. They also state that the difficulty in studying the efficiency of proofs for NP 
statements is the _problem of representation_. Arithmetic circuits and the R1CS are some of such _representations_ of 
statements to be proved. 

### Summary 

In his blogpost entitled "_Constraint Systems for ZK SNARKs_" [[5]], 
Alex Pinto wrote with the intention to elucidate the difficulties he came across while making "DIZK, a library written 
in JAVA, work with the same zkSNARK scheme as used by ZoKrates and libsnark, which are written in C++". He says the 
biggest obstacle was "proper encoding of the function that we want to prove statements for". So he focuses specifically 
on how to express a computational problem in terms of an _arithmetic circuit_, as well as on how to _convert the arithmetic 
circuit into an R1CS_. Although Pinto covers the same concepts covered by Buterin in [[9]] (there is a summary of 
Buterin's article in [zk-SNARKs](https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html#quadratic-arithmetic-programs---from-zero-to-hero)), 
Pinto gives some finer details. 

- The _first step_ is to express the given computational problem in terms of _addition_ and _multiplication_ of only two 
  terms. This involves introducing _auxiliary variables_ in order to express the given computational problem in terms of 
  _equations_ with a single operation between two variables. The result is an arithmetic circuit.
- The _second step_ is to encode the arithmetic circuit as an R1CS.

Pinto explains how this is achieved. He makes it easy to understand how the _solution vector_ as shown in Table 1 is 
constructed, as well as how addition gates need to be expressed as some _multiplication gate_ in order to understand the 
resulting R1CS. Pinto uses an example inspired by RSA's well-known _difficulty of factorization problem_: "Proving that 
one knows prime factors $ p $ and $ q $  such that $ n + 1 = (p + 3)(q + 2) $." 


When comparing zk-SNARKs and Bulletproofs, Lovesh Harchandani in [[14]] mentions the two main 
disadvantages of zkSNARKs: 

- Firstly, there is a _trusted setup_ used for a one-time generation of protocol parameters (e.g. a so-called 
_Common Reference String_ (CRS)). The problem with such a setup is that the generated parameters (or the CRS) could be 
used to leak some secrets. One way of avoiding such possible leakage of secrets is to use a Multiparty 
Computation (MPC) as utilized in cryptocurrencies such as Zcash. 
- Secondly, a new _trusted setup_ (i.e. a fresh CRS) is needed for each circuit, 
simply because each new computational challenge will have different constraints. 

The Bulletproofs constraint system, on the other hand, has no trusted setup. 




## Interstellar's Bulletproof Constraint System

### Overview 

The Interstellar team paved the way towards the implementation of several cryptographic primitives in the RUST language, 
including [_Ristretto_](https://docs.rs/curve25519-dalek/0.15.1/curve25519_dalek/ristretto/index.html), a construction 
of a prime-order group using a cofactor-8 curve known as Curve25519. They reported on how they implemented Bulletproofs 
in Henry de Valence's article entitled "[Bulletproofs pre-release](https://medium.com/interstellar/bulletproofs-pre-release-fcb1feb36d4b)". 
An update on their progress in extending the [Bulletproofs implementation](http://github.com/dalek-cryptography/bulletproofs/) 
to a constraint system API, which enables ZK proofs of arbitrary statements, was given in Cathie Yun's 
article, [Programmable Constraint Systems for Bulletproofs](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7) 
[[11]]. However, it was Bootle et al. [[4]] who first used the Hadamard relation and linear constraints, which together 
form the constraint system as formalized by the Interstellar team. Most of the mathematical background of these 
constraints and bulletproofs is contained Bunz et al.'s paper [[12]]. For an extensive study on Bulletproof 
protocols, refer to the previous report [here](https://tlu.tarilabs.com/preface/learning/bulletproofs.html). 

Dalek's constraint system, as defined earlier in [Definition of Constraint System](#definition-of-constraint-system), is a collection of arithmetic constraints of two types, 
_multiplicative constraints_ and _linear constraints_, over a set of high-level and low-level variables. 

As to why there is a vector $\mathbf{v}$ and its weight $\mathbf{W_V}$ in the definition, Bunz et al. 
explain that "we include additional commitments $V_i$ as part of our statement, and give a protocol for a more general 
relation, where the _linear consistency constraints_ include the openings ${ v_j}$ of the commitments $V_j$", 
refer to page 24 of [[10]]. Their definition of a constraint system incorporates a secret vector $\mathbf{v}$ 
and its weight $\mathbf{W_V}$, because commitments $V_i$ of components 
${ v_i}$ of $\mathbf{v} = {(v_1, v_2, \dots , v_m )}$ are included among the inputs. We note that 
Bulletproofs use the Pedersen commitment scheme. 



### Easy-to-build Constraint Systems 

In this bulletproofs framework, a _prover_ can build a constraint system in two steps:

- Firstly, by _committing_ to secret 
  inputs and _allocating_ high-level variables corresponding to the inputs.
- Secondly, by _selecting_ a suitable combination 
  of _multiplicative constraints_ and _linear constraints_, as well as requesting a _random scalar_ in response to the 
  high-level variables already committed [[13]]. 

Reference 
[[14]] gives an excellent outline of _ZK proofs_ that use _Bulletproofs_: 

1. The prover _commits_ to a value(s) that they want to prove knowledge of. 
2. The prover _generates_ the _proof_ by enforcing the _constraints over the committed values_ and any additional 
   _public values_. The constraints might require the prover to commit to some _additional variables_. 
3. The Prover _sends_ the verifier _all the commitments_ made in step 1 and step 2 along with the _proof_ from step 2. 
4. The verifier now _verifies the proof_ by enforcing the _same constraints_ over _the commitments_ plus any public 
   values.



### About Gadgets 

Consider a _verifiable shuffle_: given two lists of _committed values_ ${ x_1, x_2, . . . , x_n}$ and 
${ y_1, y_2, . . . , y_n} ,$ prove that the second list is a permutation of the first. Bunz et al. (page 5 of [[9]]) mention that 
the use of bulletproofs improves the complexity of such a _verifiable shuffle_ to size $\mathcal{O}(log(n))$ compared to 
previous implementation results. Although not referred to as a _gadget_ in the paper, this is in 
fact a _shuffle gadget_. The term _gadget_ was used and popularized by the Interstellar team, who introduced _gadgets_ 
as building blocks of constraint systems; refer to [[1]]. 

A _shuffle gadget_ is any function whose outputs are but a permutation of its inputs. By definition of a permutation, the 
_number of inputs_ to a shuffle gadget is always the same as the _number of outputs_. 


<div align="center"><b> 
<img src="sources/simple-shuffle-diagram.png" style="zoom:67%;" /> 
</b></div> 
​				

<div align="center"><b>Figure 3: Simple Shuffle Gadgets with Two Inputs [[1]]</b></div> 
Find a RUST example code for a _shuffle gadget_ in the _Bulletproofs Constraint System_ framework 
[here](https://github.com/lovesh/bulletproofs/blob/e477511a20bdb8de8f4fa82cb789ba71cc66afd8/docs/r1cs-docs-example.md), 
written by Lovesh Harchandani. 

The Interstellar team mentions other _gadgets_: “merge”, “split” and a “range proof”, that are implemented in their 
_Confidential Assets scheme_ called the _Cloak_. Just as a _shuffle gadget_ creates constraints which prove that 
_two sets of variables_ are equal up to a permutation, a _range proof gadget_ checks that a given _value_ is in the 
interval ${ [0, 2^n]}$, where ${ n}$ is the size of the input vector [[13]]. 

Gadgets in their simplest form merely receive some _variables_ as inputs and produce corresponding _output_ values. However, they 
may _allocate_ more _variables_ for internal use, sometimes called _auxiliary variables_, and _produce constraints_
 involving all these variables. The main advantage of gadgets is that they are _composable_, thus a more _complex_ 
 gadget can always be created from a number of _single_ gadgets. Interstellar's Bulletproofs API allows developers to 
 choose their own collection of gadgets suitable for the protocol they wish to develop. 



### Previous Work on Verifiable Shuffles

It would be interesting to see how some of the applications of _verifiable shuffles_ mentioned in [[9]], i.e. _voting_, 
_mix-nets_ and _solvency proofs_, could be enhanced by frameworks similar to Interstellar's Bulletproofs. The 
_first_ and _third_ applications seem more relevant to _cryptocurrencies_ and _confidential asset schemes_. 



### Interstellar's Concluding Remarks 

Cathie Yun reports in [[11]] that their "_work on Cloak and Spacesuit is far from complete_" and mentions that they 
still have two more goals to achieve: 

- Firstly, in order to "_ensure that challenge-based variables cannot be inspected_" and _prevent_ the user from 
  accidentally breaking soundness of their gadgets, the Bulletproofs protocol needs to be slightly extended, enabling it 
  to commit "to a portion of low-level variables using a single vector Pedersen commitment without an overhead of 
  additional individual high-level Pedersen commitments" [[11]]. 
- Secondly, to "_improve privacy in Cloak"_ by enabling "_multi-party proving of a single constraint system_". That is, 
  "_building a joint proof of a single constraint system by multiple parties_, without sharing their secret inputs with 
  each other".

All-in-all, Yun regards _constraint systems_ as "_very powerful_ because they can represent any efficiently 
verifiable program" [[1]]. 





### R1CS Factorization Example for Bulletproofs 

In the same article [[14]], Harchandani explores the Dalek Bulletproofs API with various examples. Having 
looked at an R1CS _zk-SNARK_ example by Alex Pinto, where he looked at a _factorization problem_, we now look at 
the counterpart R1CS _Bulletproof_ example. The computational challenge is to "_prove knowledge of factors_ p 
_and_ q _of a given number_ r _without revealing the factors_". It is one of six examples discussed in the article [[14]]. 

Table 3 gives an _outline of the description and the code lines_ of the example. Harchandani's complete 
code of this example can be found [here](https://github.com/lovesh/bulletproofs/blob/e477511a20bdb8de8f4fa82cb789ba71cc66afd8/tests/basic_r1cs.rs#L17). 

<div align="center"><b>Table 3: Example of Bulletproof Constraint</b></div> 


| No.  | Description                                                  | Code Lines                                                   |
| ---- | :----------------------------------------------------------- | ------------------------------------------------------------ |
| 1.   | Create two pairs of generators; one pair for the <br/>Pedersen commitments and the other for the <br/>Bulletproof. | `let pc_gens = PedersenGens::default();`<br/>`let bp_gens = BulletproofGens::new(128, 1);` |
| 2.   | Instantiate the prover using the commitment <br/>and Bulletproofs generators of Step 1, to <br/>produce the prover's transcript. | `let mut prover_transcript = Transcript::new(b"Factors");`<br/>`let mut prover = Prover::new(&bp_gens, &pc_gens, &mut prover_transcript);` |
| 3.   | Prover commits to variables using the Pedersen <br/>commitments, creates variables corresponding to <br/>each commitment and adds the variables to the <br/>transcript. | `let x1 = Scalar::random(&mut rng);`<br/>`let (com_p, var_p) = prover.commit(p.into(), x1);`<br/>`let x2 = Scalar::random(&mut rng);`<br/>`let (com_q, var_q) = prover.commit(q.into(), x2);` |
| 4.   | Prover constrains the variables in two steps: <br/>a) Prover multiplies the variables of step 3 and captures <br/>the product in the "output" variable O. <br/>b) Prover wants to ensure the difference of the product O and r is zero. | `let (_, _, o) =  prover.multiply(var_p.into(), var_q.into());`<br/>`let r_lc: LinearCombination = vec![(Variable::One(),      r.into())].iter().collect();`<br/>`prover.constrain(o -  r_lc);` |
| 5.   | Prover creates the proof.                                    | `let proof = prover.prove().unwrap();`                       |
| 6.   | Instantiation of the Verifier using the Pedersen <br/>commitments and Bulletproof generators, and <br/>creates its own transcript. | `let mut verifier_transcript = Transcript::new(b"Factors");`<br/>`let mut verifier = Verifier::new(&bp_gens, &pc_gens, &mut verifier_transcript);` |
| 7.   | Verifier records commitments for p and q sent by <br/>prover in the transcript, and creates variables for <br/>them similar to the prover's. | `let var_p = verifier.commit(commitments.0);`<br/>`let var_q = verifier.commit(commitments.1);` |
| 8.   | Verifier constrains variables corresponding to <br/>the commitments. | `let (_, _, o) =  verifier.multiply(var_p.into(), var_q.into());`<br/>`let r_lc: LinearCombination = vec![(Variable::One(), r.into())].iter().collect();`<br/>`verifier.constrain(o -  r_lc);` |
| 9.   | Verifier verifies the proof.                                 | `verifier.verify(&proof)`                                    |


## Conclusions, Observations and Recommendations 

In conclusion, this report has: 

- highlighted the connection between arithmetic circuits and R1CSs; 
- clarified the difference R1CSs make in Bulletproofs and in range proofs; 
- compared R1CSs in zk-SNARKs and Bulletproofs; 
- explained what is meant by the following in [[12]] - "We note that a range proof 
  using the protocol of [[BCC+16]](Bootle et al.'s paper [[4]]) would have required implementing the commitment opening 
  algorithm as part of the verification circuit, which we are able to eliminate." 

Constraint systems indeed form a natural language for most computational problems expressed as arithmetic circuits. No 
wonder they have found such ample application in both zk-SNARKs and Bulletproofs. 
Although much work still needs to be done, Bulletproofs with constraint systems built on them promise to be powerful 
tools for efficient handling of verifiable programs. The leverage that developers have, in choosing whatever gadgets 
they wish to implement, leaves enough room to build proof systems that have some degree of modularity. 
The possibility of using this Bulletproofs framework in building confidential digital asset schemes provides 
greater opportunities or options to consider when building Tari's envisaged Digital Assets Network. 
With regard to validation of transactions or verification proofs, any voting could easily be split between zk-SNARKs and 
Bulletproofs. Only time will tell which of these two will yield better practicality. 






## References 

[[1]] C. Yun, "Building on Bulletproofs" [online]. Available: <https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8>. 
Date accessed: 2020&#8209;01&#8209;03.

[1]: https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8
"Building on Bulletproofs"

[[2]] A. Szepieniec and B. Preneel, "Generic Zero-knowledge and Multivariate Quadratic Systems", 2016 [online]. 
Available: <https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591>. 
Date accessed: 2020&#8209;01&#8209;08. 

[2]: https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591
"Generic Zero-knowledge and 
Multivariate Quadratic Systems"

[[3]] A. Shpilka and A. Yehudayoff, "Arithmetic Circuits: A Survey of Recent Results and Open Questions", 
Technion-Israel Institute of Technology, Haifa, Israel, 2010 [online]. Available: 
<https://ieeexplore.ieee.org/document/8186881/metrics#metrics>. 
Date accessed: 2019&#8209;12&#8209;21. 

[3]: https://ieeexplore.ieee.org/document/8186881/metrics#metrics
"Arithmetic Circuits: 
A Survey of Recent Results 
and Open Questions" 

[[4]] J. Bootle, A. Cerulli, P. Chaidos, J. Groth and C. Petit, "Efficient Zero-knowledge Arguments for Arithmetic 
Circuits in the Discrete Log Setting", *Annual International Conference on the Theory and Applications of Cryptographic* 
*Techniques*, pp. 327‑357. Springer, 2016 [online]. Available: <https://eprint.iacr.org/2016/263.pdf> Date accessed: 
2019&#8209;12&#8209;21.

[4]: https://eprint.iacr.org/2016/263.pdf
"Efficient Zero-knowledge 
Arguments for Arithmetic Circuits 
in the Discrete Log Setting" 

[[5]] A. Pinto, "Constraint Systems for ZK SNARKs," 2019-03-06 [online]. Available: 
<http://coders-errand.com/constraint-systems-for-zk-snarks/>. 
Date accessed: 2019&#8209;12&#8209;23.

[5]: http://coders-errand.com/constraint-systems-for-zk-snarks/
"Constraint Systems for ZK SNARKs"

[[6]] H. Wu, W. Zheng, A. Chiesa, R. Ada Popa, and I. Stoica, "DIZK: A Distributed Zero Knowledge 
Proof System", Proceedings of the 27th USENIX Security Symposium, August 15–17, 2018 [online]. Available: 
<https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf>. Date accessed: 2019&#8209;12&#8209;14. 

[6]: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf
"DIZK: A Distributed Zero 
Knowledge Proof System"

[[7]] A. Szepieniec and B. Preneel, "Generic Zero-knowledge and Multivariate Quadratic Systems" [online]. Available: 
<https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591>. 
Date accessed: 2019&#8209;12&#8209;31.

[7]: https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591
"Generic Zero-knowledge and 
Multivariate Quadratic Systems" 

[[8]] E. Ben-Sasson, A. Chiesa, D. Genkin, E. Tromer and M. Virza, "SNARKs for C: Verifying Program 
Executions Succinctly and in Zero Knowledge (extended version)," October 2013 [online]. Available: 
<https://eprint.iacr.org/2013/507.pdf>. Date accessed: 2019&#8209;12&#8209;17. 

[8]: https://eprint.iacr.org/2013/507.pdf
"SNARKs for C: 
Verifying Program Executions 
Succinctly and in Zero 
Knowledge (extended version)"

[[9]] V. Buterin, "Quadratic Arithmetic Programs: from Zero to Hero," 12 December 2016 [online]. Available: 
<https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649>. Date accessed: 
2019&#8209;12&#8209;19.

[9]: https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649

[[10]] Dalek's documents, "Constraint System" [online]. Available: <https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html>. 
Date accessed: 2020&#8209;01&#8209;03.

[10]: https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html#constraint-system
"Constraint System" 

[[11]] C. Yun, "Programmable Constraint Systems for Bulletproofs" [online]. Available: 
<https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7>. Date accessed: 
2019&#8209;12&#8209;04.

[11]: https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7
"Programmable Constraint Systems 
for Bulletproofs" 

[[12]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential 
Transactions and More", Blockchain Protocol Analysis and Security Engineering 2018 [online]. 
Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2019&#8209;11&#8209;21.

[12]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf
"Bulletproofs: Short Proofs for Confidential Transactions and More"

[[13]] Dalek's documents, "Module Bulletproofs::notes::r1cs_proof" [online]. Available: 
<https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html>. Date accessed: 2020&#8209;01&#8209;07. 

[13]: https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html
"Module Bulletproofs::Notes::r1cs_proof"

[[14]] L. Harchandani, "Zero Knowledge Proofs using Bulletproofs" [online]. Available: 
<https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82>. Date accessed: 2020&#8209;01&#8209;03.

[14]: https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82
"Zero Knowledge Proofs 
using Bulletproofs"

[[15]] J. Groth, "Linear Algebra with Sub-linear Zero-knowledge Arguments", Advances in Cryptology – CRYPTO 2009, 
pages 192–208, 2009 [online]. Available: <https://iacr.org/archive/crypto2009/56770190/56770190.pdf>. Date accessed: 
2019&#8209;12&#8209;04.

[15]: https://iacr.org/archive/crypto2009/56770190/56770190.pdf
"Linear Algebra with 
Sub-linear Zero-knowledge 
Arguments"

[[16]] "Zero-knowledge Proofs: _What are they, how do they work, and are they fast yet?_" [online]. Available: 
<https://zkp.science/>. Date accessed: 2020&#8209;01&#8209;07. 

[16]: https://zkp.science/
"Zero-knowledge Proofs: 
What are they, 
how do they work, and 
are they fast yet?"



## Contributors 

- <https://github.com/Empiech007>
- <https://github.com/hansieodendaal> 
- <https://github.com/anselld> 




