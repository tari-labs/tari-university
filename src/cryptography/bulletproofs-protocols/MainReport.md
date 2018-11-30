# The Bulletproof Protocols

## Introduction

An overview of Bulletproofs have been given in [Bulletproofs and Mimblewimble]("../bulletproofs-and-mimblewimble/MainReport.md"), which has largely been based on the original work done by Bünz et al. [[1]]. They documented a number of different Bulletproof protocols, but not all of them in an obvious manner. This report chronologically numbers, summarizes and explain the different Bulletproof protocols in as simple terms as possible. It also simplifies the logic and explains the base mathematical concepts in more detail where prior knowledge was assumed. The report concludes with a discussion on an improved Bulletproof zero-knowledge proof protocol by some community members following an evolutionary approach.



## Contents

- [The Bulletproof Protocols](#the-bulletproof-protocols)
  - [Introduction](#introduction)
  - [Contents](#contents)
  - [Bulletproof Protocols](#bulletproof-protocols)
    - [Protocol 1 - Inner-product Argument](#protocol-1---inner-product-argument)
    - [Protocol 2 - Inner-Product Verification through Multi-Exponentiation](#protocol-2---inner-product-verification-through-multi-exponentiation)
    - [Protocol 2! - Range Proof Protocol with Logarithmic Size](#protocol-2---range-proof-protocol-with-logarithmic-size)
      - [Protocol 2.1! - Inner-Product Range Proof](#protocol-21---inner-product-range-proof)
      - [Protocol 2.2! - Logarithmic Range Proof](#protocol-22---logarithmic-range-proof)
      - [Protocol 2.3! - Aggregating Logarithmic Proofs](#protocol-23---aggregating-logarithmic-proofs)
      - [Protocol 2.4! - Non-Interactive Proof through Fiat-Shamir](#protocol-24---non-interactive-proof-through-fiat-shamir)
      - [Protocol 2.5! - MPC Protocol for Bulletproofs](#protocol-25---mpc-protocol-for-bulletproofs)
    - [Protocol 3! - Zero-Knowledge Proof for Arithmetic Circuits](#protocol-3---zero-knowledge-proof-for-arithmetic-circuits)
      - [Protocol 3 - Inner-Product Proof for Arithmetic Circuits](#protocol-3---inner-product-proof-for-arithmetic-circuits)
      - [Protocol 3.1! - Logarithmic-Sized Non-Interactive Protocol for Arithmetic Circuits](#protocol-31---logarithmic-sized-non-interactive-protocol-for-arithmetic-circuits)
    - [Protocol 4! - Optimized Verifier using Multi-Exponentiation and Batch Verification](#protocol-4---optimized-verifier-using-multi-exponentiation-and-batch-verification)
  - [Evolving Bulletproof Protocols](#evolving-bulletproof-protocols)
  - [Conclusions, Observations, Recommendations](#conclusions-observations-recommendations)
  - [References](#references)
  - [Appendices](#appendices)
    - [Appendix A: Definition of Terms](#appendix-a-definition-of-terms)
    - [Appendix B: Notations Used](#appendix-b-notations-used)
  - [Contributors](#contributors)



## Bulletproof Protocols

Protocols 1, 2 and 3 are numbered consistently with [[1]], whereas the rest of the protocols are numbered to fit chronologically with a faculty sign "!" to differentiate them. See [Appendix&nbsp;B](#appendix-b-notations-used) for notations used. 

<i>**Note:** Full mathematical definitions and terms not defined are available in [[1]].</i>
<br>

#### Protocol 1 - Inner-product Argument

Protocol 1 is an argument of knowledge that the *prover* $ \mathcal{P} $ knows the openings of two binding Pedersen vector commitments that satisfy a given inner product relation. Let inputs to the inner-product argument be independent generators $ g,h \in \mathbb G^n $, a scalar $ c \in \mathbb Z_p $ and $ P \in \mathbb G $. The argument lets the *prover* $ \mathcal{P} $ convince a *verifier* $ \mathcal{V} $ that the *prover* $ \mathcal{P} $ knows two vectors $ \mathbf a, \mathbf b \in \mathbb Z^n_p $ such that

$$
P =g^ah^b \mspace{30mu} \mathrm{and} \mspace{30mu} c = \langle \mathbf {a} \mspace{3mu}, \mspace{3mu} \mathbf {b} \rangle
$$

$ P $ is referred to as the binding vector commitment to $ \mathbf a, \mathbf b $. The inner product argument is an efficient proof system for the following relation:

$$
\{ (\mathbf {g},\mathbf {h} \in \mathbb G^n , \mspace{12mu} P \in \mathbb G , \mspace{12mu} c \in \mathbb Z_p ; \mspace{12mu} \mathbf {a}, \mathbf {b} \in \mathbb Z^n_p ) \mspace{3mu} : \mspace{15mu} P = g^\mathbf {a} h^\mathbf {b} \mspace{3mu} \wedge \mspace{3mu} c = \langle \mathbf {a} \mspace{3mu}, \mspace{3mu} \mathbf {b} \rangle \} \mspace{100mu} (1)
$$

Relation (1) requires sending $ 2n $ elements to the *verifier* $ \mathcal{V} $. In order to send only $ 2 \log 2 (n) $ elements to the *verifier* $ \mathcal{V} $ for a given $ P \in \mathbb G $ the *prover* $ \mathcal{P} $ proves that it has vectors $ \mathbf {a}, \mathbf {b} \in \mathbb Z^n_p $ for which $ P =g^ah^b \cdot u^{ \langle \mathbf {a}, \mathbf {b} \rangle } $. Here $ u \in \mathbb G $ is a fixed group element with an unknown discrete-log relative to $ g,h \in \mathbb G^n $. 

$$
\{ (\mathbf {g},\mathbf {h} \in \mathbb G^n , \mspace{12mu} u,P \in \mathbb G ; \mspace{12mu} \mathbf {a}, \mathbf {b} \in \mathbb Z^n_p ) : \mspace{15mu} P =g^ah^b \cdot u^{ \langle \mathbf {a}, \mathbf {b} \rangle } \} \mspace{100mu} (2)
$$

A proof system for relation (2) gives a proof system for (1) with the same complexity, thus only a proof system for relation (2) is required. 

Protocol 1 is then defined as the proof system for relation (2) as shown in Figure&nbsp;1. The element $ u $ is raised to a random power $ x $ chosen by the *verifier* $ \mathcal{V} $ to ensure that the extracted vectors $ \mathbf {a}, \mathbf {b} $ from [Protocol&nbsp;2](#protocol-2---inner-product-verification-through-multi-exponentiation) satisfy $ c = \langle \mathbf {a} \mspace{3mu} , \mspace{3mu} \mathbf {b} \rangle $.

<p align="center"><img src="sources/Protocol-1.png" width="470" /></p>
<div align="center"><b>Figure&nbsp;1: Bulletproofs' Protocol 1 [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>


The argument presented in Protocol 1 has the following Commitment Scheme<sup>[def][cs~]</sup> properties:

- <u>Perfect completeness (hiding)</u>: Every validity/truth is provable, also see Definition&nbsp;9 in [[1]];
- <u>Statistical witness extended emulation (binding)</u>: Robust against either extracting a non-trivial discrete logarithm relation between $ \mathbf {g} , \mathbf {h} , u $ or extracting a valid witness $ \mathbf {a}, \mathbf {b} $.

#### Protocol 2 - Inner-Product Verification through Multi-Exponentiation

Protocol 2 performs inner-product verification through multi-exponentiation, the latter being a technique to reduce the number of computationally expensive exponentiations. The number of exponentiations is reduced to a single multi-exponentiation by delaying all the exponentiations until the last round. Protocol 2 has a logarithmic number of rounds and in each round the *prover* $ \mathcal{P} $ and *verifier* $ \mathcal{V} $ compute a new set of generators. By unrolling the recursion the, final $ g $ and $ h $ can be expressed in terms of the input generators $ \mathbf {g},\mathbf {h} \in \mathbb G^n $ as:

$$
g = \prod _{i=1}^n g_i^{s_i} \in \mathbb{G}, \mspace{21mu} h=\prod _{i=1}^n h_i^{1/s_i} \in \mathbb{G}
$$

where $ \mathbf {s} = (s_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} s_n) \in \mathbb Z_p^n $ only depends on the challenges $ (x_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} x_{\log_2(n)}) \in \mathbb Z_p^n $. The entire verification check in the protocol reduces to a single multi-exponentiation of size $ 2n + 2 \log_2(n) + 1 $:

$$
\mathbf g^{a \cdot \mathbf{s}} \cdot \mathbf h^{b \cdot\mathbf{s^{-1}}} \cdot u^{a \cdot b} \mspace{12mu} \overset{?}{=} \mspace{12mu} P \cdot \prod _{j=1}^{\log_2(n)} L_j^{x_j^2} \cdot R_j^{x_j^{-2}}
$$

with $ L $ and $R $ as defined in the original reference.

Protocol 2 is shown in Figure&nbsp;2. 

<p align="center"><img src="sources/Protocol-2.png" width="570" /></p>
<div align="center"><b>Figure&nbsp;2: Bulletproofs' Protocol 2 [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>




#### Protocol 2! - Range Proof Protocol with Logarithmic Size

Protocol 2! provides short and aggregatable range proofs, using the improved inner product argument from Protocol 1. It is build up in 5 parts: 

- how to construct a range proof that requires the *verifier* $ \mathcal{V} $ to check an inner product between two vectors;
- how to replace the inner product argument with an efficient inner-product argument;
- how to efficiently aggregate $ m $ range proofs into one short proof;
- how to make interactive public coin protocols non-interactive by using the Fiat-Shamir Heuristic<sup>[def](#fsh)</sup> and 
- how to allow multiple parties to construct a single aggregate range proof. 

A diagrammatic overview of a range proof protocol implementation is given in Figure&nbsp;3.

<p align="center"><img src="sources/RangeProofDiagram.PNG" width="1000" /></p>
<div align="center"><b>Figure&nbsp;3: Range Proof Protocol Implementation Example [<a href="https://doc.dalek.rs/bulletproofs/index.html" title="Dalek Cryptography - 
Crate Bulletproofs">22</a>]</b></div>



##### Protocol 2.1! - Inner-Product Range Proof

This protocol provides the ability to construct a range proof that requires the *verifier* $ \mathcal{V} $ to check an inner product between two vectors. The range proof is constructed by exploiting the fact that a Pedersen commitment $ V $ is an element in the same group $ \mathbb G $ that is used to perform the inner product argument. Let $ v \in \mathbb Z_p $ and let $ V \in \mathbb G $ be a Pedersen commitment to $ v $ using randomness $ \gamma $. The proof system will convince the *verifier* $ \mathcal{V} $ that commitment $ V $ contains a number $ v \in [0,2^n - 1] $ such that

$$
\{ (g,h \in \mathbb{G}) , V , n \mspace{3mu} ; \mspace{12mu} v, \gamma \in \mathbb{Z_p} ) \mspace{3mu} : \mspace{3mu} V =h^\gamma g^v \mspace{5mu} \wedge \mspace{5mu} v \in [0,2^n - 1] \} \mspace{100mu} (3)
$$

without revealing $ v $. Let $ \mathbf {a}_L = (a_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} a_n) \in \{0,1\}^n $ be the vector containing the bits of $ v, $ so that $ \langle \mathbf {a}_L, \mathbf {2}^n \rangle = v $. The *prover* $ \mathcal{P} $ commits to $ \mathbf {a}_L $ using a constant size vector commitment $ A \in \mathbb{G} $. It will convince the *verifier* $ \mathcal{V} $ that $ v $ is in $ [0,2^n - 1] $ by proving that it knows an opening $ \mathbf {a}_L \in \mathbb Z_p^n $ of $ A $ and $ v, \gamma \in \mathbb{Z_p} $ such that $ V =h^\gamma g^v $ and

$$
\langle \mathbf {a}_L \mspace{3mu} , \mspace{3mu} \mathbf {2}^n \rangle = v \mspace{20mu} \mathrm{and} \mspace{20mu} \mathbf {a}_R = \mathbf {a}_L - \mathbf {1}^n \mspace{20mu} \mathrm{and} \mspace{20mu} \mathbf {a}_L \circ \mathbf {a}_R = \mathbf{0}^n \mspace{20mu} \mspace{100mu} (4)
$$

This proves that $ a_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} a_n $ are all in $ \{0,1\} $ and that $ \mathbf {a}_L $ is composed of the bits of $ v $. However, the $ 2n + 1 $ constraints needs to be expressed as a single inner-product constant so that [Protocol&nbsp;1](#protocol-1---inner-product-argument) can be used, by letting the *verifier* $ \mathcal{V} $ choose a random linear combination of the constraints. To prove that a committed vector $ \mathbf {b} \in \mathbb Z_p^n $ satisfies $ \mathbf {b} = \mathbf{0}^n $ it suffices for the *verifier* $ \mathcal{V} $ to send a random $ y \in \mathbb{Z_p} $ to the *prover* $ \mathcal{P} $ and for the *prover* $ \mathcal{P} $ to prove that $ \langle \mathbf {b}, \mathbf {y}^n \rangle = 0 $, which will convince the *verifier* $ \mathcal{V} $ that $ \mathbf {b} = \mathbf{0}^n $. The *prover* $ \mathcal{P} $ can thus prove relation (4) by proving that

$$
\langle \mathbf {a}_L \mspace{3mu} , \mspace{3mu} \mathbf {2}^n \rangle = v \mspace{20mu} \mathrm{and} \mspace{20mu} \langle \mathbf {a}_L - 1 - \mathbf {a}_R \mspace{3mu} , \mspace{3mu} \mathbf {y}^n \rangle=0 \mspace{20mu} \mathrm{and} \mspace{20mu} \langle \mathbf {a}_L \mspace{3mu} , \mspace{3mu} \mathbf {a}_R \circ \mathbf {y}^n \rangle = \mathbf{0}^n \mspace{20mu} \mspace{100mu} (5)
$$

Building on this, the *verifier* $ \mathcal{V} $ chooses a random $ z \in \mathbb{Z_p} $ and let the *prover* $ \mathcal{P} $ proves that

$$
z^2 \cdot \langle \mathbf {a}_L \mspace{3mu} , \mspace{3mu} \mathbf {2}^n \rangle + z \cdot \langle \mathbf {a}_L - 1 - \mathbf {a}_R \mspace{3mu} , \mspace{3mu} \mathbf {y}^n \rangle + \langle \mathbf {a}_L \mspace{3mu} , \mspace{3mu} \mathbf {a}_R \circ \mathbf {y}^n \rangle = z^2 \cdot v \mspace{20mu} \mspace{100mu} (6)
$$

Relation (6) can be rewritten as

$$
\langle \mathbf {a}_L - z \cdot \mathbf {1}^n \mspace{3mu} , \mspace{3mu} \mathbf {y}^n \circ (\mathbf {a}_R + z \cdot \mathbf {1}^n) +z^2 \cdot \mathbf {2}^n \rangle = z^2 \cdot v + \delta (y,z) \mspace{100mu} (7)
$$

where

$$
\delta (y,z) = (z-z^2) \cdot \langle \mathbf {1}^n \mspace{3mu} , \mspace{3mu} \mathbf {y}^n\rangle -z^3 \cdot \langle \mathbf {1}^n \mspace{3mu} , \mspace{3mu} \mathbf {2}^n\rangle \in \mathbb{Z_p}
$$

can be easily calculated by the *verifier* $ \mathcal{V} $. The proof that relation (4) holds was thus reduced to a single inner-product identity.

Relation (7) cannot be used in its current form without revealing information about $ \mathbf {a}_L $. Two additional blinding vectors $ \mathbf {s}_L , \mathbf {s}_R \in \mathbb Z_p^n $ are introduced with the *prover* $ \mathcal{P} $ and *verifier* $ \mathcal{V} $ engaging in the following zero-knowledge protocol (Figure&nbsp;4):

<p align="center"><img src="sources/Protocol-2b-part-a.png" width="550" /></p>
<div align="center"><b>Figure&nbsp;4: Bulletproofs' Protocol 2.1! Part A [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>



Two linear vector polynomials $ l(X), r(X) $ in $ \mathbb Z^n_p[X] $ are defined as the inner-product terms for relation (7), also containing the blinding vectors $ \mathbf {s}_L $ and $ \mathbf {s}_R $. A quadratic polynomial $ t(X) \in \mathbb Z_p[X] $ is then defined as the inner product between the two vector polynomials $ l(X), r(X) $ such that

$$
t(X) = \langle l(X) \mspace{3mu} , \mspace{3mu} r(X) \rangle = t_0 + t_1 \cdot X + t_2 \cdot X^2 \mspace{10mu} \in \mathbb {Z}_p[X]
$$

The blinding vectors $ \mathbf {s}_L $ and $ \mathbf {s}_R $ ensure that the *prover* $ \mathcal{P} $ can publish $ l(x) $ and $ r(x) $ for one $ x \in \mathbb Z_p^* $ without revealing any information about $ \mathbf {a}_L $ and $ \mathbf {a}_R $. The constant term $ t_0 $ of the quadratic polynomial $ t(X) $ is then the result of the inner product in relation (7), and the *prover* $ \mathcal{P} $ needs to convince the *verifier* $ \mathcal{V} $ that 

$$
t_0 = z^2 \cdot v + \delta (y,z)
$$

In order to do so, the *prover* $ \mathcal{P} $ convinces the *verifier* $ \mathcal{V} $ that it has a commitment to the remaining coefficients of $ t(X) $, namely $ t_1,t_2 \in \mathbb Z_p $ by checking the value of $ t(X) $ at a random point $ x \in \mathbb Z_p^* $. This is illustrated in Figure&nbsp;5.

<p align="center"><img src="sources/Protocol-2b-part-b.png" width="655" /></p>
<div align="center"><b>Figure&nbsp;5: Bulletproofs' Protocol 2.1! Part B [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>



The *verifier* $ \mathcal{V} $ now needs to check that $ l $ and $ r $ are in fact $ l(x) $ and $ r(x) $ and that $ t(x) = \langle l \mspace{3mu} , \mspace{3mu} r \rangle $. A commitment for $ \mathbf {a}_R \circ \mathbf {y}^n $ is needed and to do so the commitment generators are switched from $ h \in \mathbb G^n $ to $ h ^\backprime = h^{(\mathbf {y}^{-1})}$. Thus $ A $ and $ S $ now become vector commitments to $ ( \mathbf {a}_L \mspace{3mu} , \mspace{3mu} \mathbf {a}_R \circ \mathbf {y}^n ) $ and $ ( \mathbf {s}_L \mspace{3mu} , \mspace{3mu} \mathbf {s}_R \circ \mathbf {y}^n ) $ respectively with respect to the new generators $ (g, h ^\backprime, h) $. This is illustrated in Figure&nbsp;6.



<p align="center"><img src="sources/Protocol-2b-part-c.png" width="640" /></p>
<div align="center"><b>Figure&nbsp;6: Bulletproofs' Protocol 2.1! Part C [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>



The range proof presented here has the following Commitment Scheme<sup>[def][cs~]</sup> properties:

- <u>Perfect completeness (hiding)</u>: Every validity/truth is provable, also see Definition&nbsp;9 in [[1]];
- <u>Perfect special honest verifier zero-knowledge</u>: The *verifier* $ \mathcal{V} $ behaves according to the protocol, also see Definition&nbsp;12 in [[1]];
- <u>Computational witness extended emulation (binding)</u>: A witness can be computed in time closely related to time spent by the *prover* $ \mathcal{P} $, also see Definition&nbsp;10 in [[1]].



##### Protocol 2.2! - Logarithmic Range Proof

This protocol replaces the inner product argument with an efficient inner-product argument. In step&nbsp;(63) Figure&nbsp;5 the *prover* $ \mathcal{P} $ transmits $ \mathbf {l} $ and $ \mathbf {r} $ to the *verifier* $ \mathcal{V} $, but their size is linear in $ n $. To make this efficient a proof size that is logarithmic in $ n $ is needed. The transfer of $ \mathbf {l} $ and $ \mathbf {r} $ can be eliminated with an inner-product argument. Checking correctness of $ \mathbf {l} $ and $ \mathbf {r} $ (step&nbsp;(67) Figure&nbsp;6) and $ \hat {t} $ (step&nbsp;(68) Figure&nbsp;6) is the same as verifying that the witness $ \mathbf {l} , \mathbf {r} $ satisfies the inner product of relation (2) on public input $ (\mathbf {g} , \mathbf {h} ^ \backprime , P \cdot h^{-\mu}, \hat t) $. Transmission of vectors $ \mathbf {l} $ and $ \mathbf {r} $ to the *verifier* $ \mathcal{V} $ (step&nbsp;(63) Figure&nbsp;5) can then be eliminated and transfer of information limited to the scalar properties $ ( \tau _x , \mu , \hat t ) $ alone, thereby archiving a proof size that is logarithmic in $ n $.



##### Protocol 2.3! - Aggregating Logarithmic Proofs

This protocol efficiently aggregate $ m $ range proofs into one short proof with a slight modification to the protocol presented in [Protocol 2.1!](#protocol-21---inner-product-range-proof). For aggregate range proofs, the inputs of one range proof do not affect the output of another range proof. Aggregating logarithmic range proofs is especially helpful if a single *prover* $ \mathcal{P} $ needs to perform multiple range proofs at the same time.

A proof system must be presented for the following relation:
$$
\{ (g,h \in \mathbb{G}) , \mspace{9mu} \mathbf {V} \in \mathbb{G}^m \mspace{3mu} ; \mspace{9mu} \mathbf {v}, \gamma \in \mathbb Z_p^m ) \mspace{6mu} : \mspace{6mu} V_j =h^{\gamma_j} g^{v_j} \mspace{6mu} \wedge \mspace{6mu} v_j \in [0,2^n - 1] \mspace{15mu} \forall \mspace{15mu} j \in [1,m] \} \mspace{100mu} (8)
$$
The *prover* $ \mathcal{P} $ should now compute $ \mspace{3mu} \mathbf a_L \in \mathbb Z_p^{n \cdot m} $ as the concatenation of all of the bits for every $ v_j $ such that
$$
\langle \mathbf{2}^n \mspace{3mu} , \mspace{3mu} \mathbf a_L[(j-1) \cdot n : j \cdot n-1] \rangle = v_j \mspace{9mu} \forall \mspace{9mu} j \in [1,m] \mspace{3mu}
$$
The quantity $ \delta (y,z) $ is adjusted to incorporate more cross terms $ n \cdot m $ , the linear vector polynomials $ l(X), r(X) $ are adjusted to be in $ \mathbb Z^{n \cdot m}_p[X] $ and the blinding factor $ \tau_x $ for the inner product $ \hat{t} $ (step&nbsp;(61) Figure&nbsp;5) is adjusted for the randomness of each commitment $ V_j $. The verification check (step&nbsp;(65) Figure&nbsp;6) is updated to include all $ V_j $ commitments and the definition of $ P $ (step&nbsp;(66) Figure&nbsp;6) is changed to be a commitment to the new $ r $.

This aggregated range proof that makes use of the inner product argument only uses $ 2 \cdot [ \log _2 (n \cdot m)] + 4 $ group elements and $ 5 $ elements in $ \mathbb Z_p $. The growth in size is limited to an additive term $ 2 \cdot [ \log _2 (m)] $ as opposed to a multiplicative factor $ m $ for $ m $ independent range proofs.

The aggregate range proof presented here has the following Commitment Scheme<sup>[def][cs~]</sup> properties:

- <u>Perfect completeness (hiding)</u>: Every validity/truth is provable, also see Definition&nbsp;9 in [[1]];
- <u>Perfect special honest verifier zero-knowledge</u>: The *verifier* $ \mathcal{V} $ behaves according to the protocol, also see Definition&nbsp;12 in [[1]];
- <u>Computational witness extended emulation (binding)</u>: A witness can be computed in time closely related to time spent by the *prover* $ \mathcal{P} $, also see Definition&nbsp;10 in [[1]].



##### Protocol 2.4! - Non-Interactive Proof through Fiat-Shamir

So far the *verifier* $ \mathcal{V} $ behaves as an honest verifier and all messages are random elements from $ \mathbb Z_p^* $. These are the pre-requisites needed to convert the protocol presented so far into a non-interactive protocol that is secure and has full zero-knowledge in the random oracle model (thus without a trusted setup) using the Fiat-Shamir Heuristic<sup>[def][fsh~]</sup>. 



##### Protocol 2.5! - MPC Protocol for Bulletproofs

This protocol allows multiple parties to construct a single simple efficient aggregate range proof designed for Bulletproofs. This is valuable when multiple parties want to create a single joined confidential transaction, where each party knows some of the inputs and outputs and needs to create range proofs for their known outputs. In Bulletproofs, $ m $ parties each having a Pedersen commitment $ (V_k)_{k=1}^m $ can generate a single Bulletproof that each $ V_k $ commits to a number in some fixed range.

Let $ k $ denote the $ k $th party's message, thus $ A^{(k)} $ is generated using only inputs of party $ k $. A set of distinct generators $ (g^{(k)}, h^{(k)})^m_{k=1} $ is assigned to each party, and $ \mathbf g,\mathbf h $ is defined as the interleaved concatenation of all $ g^{(k)} , h^{(k)} $ such that 

$$
g_i=g_{[{i \over{m}}]}^{((i-1) \mod m+1)} \mspace{15mu} \mathrm{and} \mspace{15mu} h_i=h_{[{i \over{m}}]}^{((i-1) \mod m+1)}
$$

The protocol either uses three rounds with linear communication in both $ m $ and the binary encoding of the range, or it uses a logarithmic number of rounds and communication that is only linear in $ m $. For the linear communication case the protocol in [Protocol 2.1!](#protocol-21---inner-product-range-proof) is followed with the difference that each party generates its part of the proof using its own inputs and generators, that is

$$
A^{(k)} , S^{(k)}; \mspace{15mu} T_1^{(k)} , T_2^{(k)}; \mspace{15mu} \tau_x^{(k)} , \mu^{(k)} , \hat{t}^{(k)} , \mathbf{l}^{(k)} , \mathbf{r}^{(k)}
$$

These shares are sent to a dealer (could be anyone, even one of the parties) who adds them homomorphically to generate the respective proof components, for example

$$
A = \prod^m_{k=1} A^{(k)} \mspace{15mu} \mathrm{and} \mspace{15mu} \tau_x = \prod^m_{k=1} \tau_x^{(k)}
$$

In each round, the dealer generates the challenges using the Fiat-Shamir Heuristic<sup>[def](#fsh)</sup> and the combined proof components and sends them to each party. In the end each party send $ \mathbf{l}^{(k)},\mathbf{r}^{(k)} $ to the dealer who computes $ \mathbf{l},\mathbf{r} $ as the interleaved concatenation of all shares. The dealer runs the inner product argument ([Protocol&nbsp;1](#protocol-1---inner-product-argument)) to generate the final proof. Each proof component is the (homomorphic) sum of each parties' proof components and each share constitutes part of a separate zero-knowledge proof. An example of the MPC protocol implementation using three rounds with linear communication is shown in Figure&nbsp;7. 

<p align="center"><img src="sources/MPC-diagram.PNG" width="850" /></p>
<div align="center"><b>Figure&nbsp;7: MPC Implementation Example [<a href="https://doc-internal.dalek.rs/bulletproofs/aggregation/index.html" title="Dalek Cryptography - 
Module bulletproofs::aggregation">34</a>]</b></div>

The communication can be reduced by running a second MPC protocol for the inner product argument, reducing the rounds to $ \log_2(l) $. Up to the last $ \log_2(l) $ round each parties' witnesses are independent and the overall witness is the interleaved concatenation of the parties' witnesses. The parties compute $ L^{(k)}, R^{(k)} $ in each round and the dealer computes $ L, R $ as the homomorphic sum of the shares. In the final round the dealer generates the final challenge and sends it to each party who in turn send their witness to the dealer who completes [Protocol&nbsp;2](#protocol-2---inner-product-verification-through-multi-exponentiation). 



#### Protocol 3! - Zero-Knowledge Proof for Arithmetic Circuits

Bulletproofs present an efficient zero-knowledge argument for arbitrary Arithmetic Circuits<sup>[def][ac~]</sup> with a proof size of $ 2 \cdot [ \log _2 (n)+13] $ elements with $ n $ denoting the multiplicative complexity (number of multiplication gates) of the circuit. 

Bootle et al. [[2]] showed how an arbitrary arithmetic circuit with $ n $ multiplication gates can be converted into a relation containing a Hadamard Product<sup>[def][hdmp~]</sup> relation with additional linear consistency constraints. The communication cost of the addition gates in the argument was removed by providing a technique that can directly handle a set of Hadamard products and linear relations together. For a two-input multiplication gate let $ \mathbf a_L , \mathbf a_R $ be the left and right input vectors respectively, then $ \mathbf a_L + \mathbf a_R = \mathbf a_O $ is the vector of outputs. Let $ Q \leqslant 2 \cdot n $ be the number of linear consistency constraints, $ \mathbf W_{L,q} \mspace{3mu} , \mathbf W_{R,q} \mspace{3mu}, \mathbf W_{O,q} \in \mathbb Z_p^n $ be the gate weights and $ c_q \in \mathbb Z_p $ for all $ q \in [1,Q] $, then the linear consistency constraints has the form

$$
\langle \mathbf W_{L,q}, \mathbf a_L \rangle + \langle \mathbf W_{R,q}, \mathbf a_R \rangle +\langle \mathbf W_{O,q}, \mathbf a_O \rangle = c_q
$$

The high-level idea of this protocol is to convert the Hadamard-product relation along with the linear consistency constraints into a single inner product relation. Pedersen commitments $ V_j ​$ are also included as input wires to the arithmetic circuit, which is an important refinement otherwise the arithmetic circuit would need to implement a commitment algorithm. The linear constraints also include openings $ v_j ​$ of $ V_j ​$. 

##### Protocol 3 - Inner-Product Proof for Arithmetic Circuits

Similar to [Protocol 2.1!](#protocol-21---inner-product-range-proof) the *prover* $ \mathcal{P} $ produces a random linear combination of the Hadamard Product<sup>[def][hdmp~]</sup> and linear constraints to form a single inner product constraint. If the combination is chosen randomly by the *verifier* $ \mathcal{V} $ then with overwhelming probability the inner-product constraint implies the other constraints. A proof system must be presented for relation (9) below:

$$
\begin{aligned} 
\mspace{3mu} (g,h \in \mathbb{G} \mspace{3mu} ; \mspace{3mu} \mathbf g,\mathbf h \in \mathbb{G}^n \mspace{3mu} ; \mspace{3mu} \mathbf V \in \mathbb{G}^m \mspace{3mu} ; \mspace{3mu} \mathbf W_{L} , \mathbf W_{R} , \mathbf W_{O} \in \mathbb Z_p^{Q \times n} \mspace{3mu} ; \\\ 
\mathbf W_{V} \in \mathbb Z_p^{Q \times m} \mspace{3mu} ; \mspace{3mu} \mathbb{c} \in \mathbb Z_p^{Q} \mspace{3mu} ; \mspace{3mu} \mathbf a_L , \mathbf a_R , \mathbf a_O \in \mathbb Z_p^{n} \mspace{3mu} ; \mspace{3mu} \mathbf v , \mathbf \gamma \in \mathbb Z_p^{m}) \mspace{3mu} : \mspace{15mu} \\\ 
V_j =h^{\gamma_j} g^{v_j} \mspace{6mu} \forall \mspace{6mu} j \in [1,m] \mspace{6mu} \wedge \mspace{6mu} \mathbf a_L + \mathbf a_R = \mathbf a_O \mspace{6mu} \wedge \mspace{50mu} \\\ 
\mathbf W_L \cdot \mathbf a_L + \mathbf W_R \cdot \mathbf a_R + \mathbf W_O \cdot \mathbf a_O = \mathbf W_V \cdot \mathbf v + \mathbf c \mspace{50mu}
\end{aligned}
\mspace{70mu} (9)
$$



Let $ \mathbf W_V \in \mathbb Z_p^{Q \times m} $ be the weights for a commitment $ V_j $. Relation (9) only holds when $ \mathbf W_{V} $ is of rank $ m $, i.e. if the columns of the matrix are all linearly independent. 

Part 1 of the protocol is presented in Figure&nbsp;8 where the *prover* $ \mathcal{P} $ commits to $ l(X),r(X),t(X) $.

<p align="center"><img src="sources/Protocol-3-part-1.png" width="690" /></p>
<div align="center"><b>Figure&nbsp;8: Bulletproofs' Protocol 3 (Part 1) [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>



Part 2 of the protocol is presented in Figure&nbsp;9 where the *prover* $ \mathcal{P} $ convinces the *verifier* $ \mathcal{V} $ that the polynomials are well formed and that $ \langle l(X),r(X) \rangle = t(X) $.

<p align="center"><img src="sources/Protocol-3-part-2.png" width="690" /></p>
<div align="center"><b>Figure&nbsp;9: Bulletproofs' Protocol 3 (Part 2) [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>



The proof system presented here has the following Commitment Scheme<sup>[def][cs~]</sup> properties:

- <u>Perfect completeness (hiding)</u>: Every validity/truth is provable, also see Definition&nbsp;9 in [[1]];
- <u>Perfect honest verifier zero-knowledge</u>: The *verifier* $ \mathcal{V} $ behaves according to the protocol, also see Definition&nbsp;12 in [[1]];
- <u>Computational witness extended emulation (binding)</u>: A witness can be computed in time closely related to time spent by the *prover* $ \mathcal{P} $, also see Definition&nbsp;10 in [[1]].



##### Protocol 3.1! - Logarithmic-Sized Non-Interactive Protocol for Arithmetic Circuits

Similar to [Protocol 2.2!](#protocol-22---logarithmic-range-proof) the communication cost of [Protocol 3](#protocol-3---inner-product-proof-for-arithmetic-circuits) can be reduced by using the efficient inner product argument. Transmission of vectors $ \mathbf {l} $ and $ \mathbf {r} $ to the *verifier* $ \mathcal{V} $ (step&nbsp;(82) Figure&nbsp;9) can be eliminated and transfer of information limited to the scalar properties $ ( \tau _x , \mu , \hat t ) $ alone. The *prover* $ \mathcal{P} $ and *verifier* $ \mathcal{V} $ engage in an inner product argument on public input $ (\mathbf {g} , \mathbf {h} ^ \backprime , P \cdot h^{-\mu}, \hat t) $ to check correctness of $ \mathbf {l} $ and $ \mathbf {r} $ (step&nbsp;(92) Figure&nbsp;9) and $ \hat {t} $ (step&nbsp;(88) Figure&nbsp;9); this is the same as verifying that the witness $ \mathbf {l} , \mathbf {r} $ satisfies the inner product of relation. Communication is now reduced to $ 2 \cdot [ \log_22(n)] + 8 $ group elements and $ 5 $ elements in $ \mathbb Z $ instead of $ 2 \cdot n $ elements, thereby archiving a proof size that is logarithmic in $ n $.

Similar to [Protocol 2.4!](#protocol-24---non-interactive-proof-through-fiat-shamir) the protocol presented so far can be turned into an efficient non interactive proof that is secure and full zero-knowledge in the random oracle model (thus without a trusted setup) using the Fiat-Shamir Heuristic<sup>[def][fsh~]</sup>.

The proof system presented here has the following Commitment Scheme<sup>[def][cs~]</sup> properties:

- <u>Perfect completeness (hiding)</u>: Every validity/truth is provable, also see Definition&nbsp;9 in [[1]];
- <u>Statistical zero-knowledge</u>: The *verifier* $ \mathcal{V} $ behaves according to the protocol and $ \mathbf {l} , \mathbf {r} $ can be efficiently simulated;
- <u>Computational soundness (binding)</u>: if the generators $ \mathbf {g} , \mathbf {h} , g , h $ are independently generated, then finding a discrete logarithm relation between them is as hard as breaking the Discrete Log Problem.



#### Protocol 4! - Optimized Verifier using Multi-Exponentiation and Batch Verification

In many of the Bulletproofs' [Use Cases](../bulletproofs-and-mimblewimble/MainReport.md#applications-for-bulletproofs) the *verifier's* runtime is of particular interest. This protocol presents optimizations for a single range proof that is also extendable to aggregate range proofs ([Protocol 2.3!](#protocol-23---aggregating-logarithmic-proofs)) and the arithmetic circuit protocol ([Protocol 3!](#protocol-3---zero-knowledge-proof-for-arithmetic-circuits)).

<u>Multi-exponentiation</u>

In [Protocol 2](#protocol-2---inner-product-verification-through-multi-exponentiation) verification of the inner-product is reduced to a single multi-exponentiation. This can be extended to verify the whole range proof using a single multi-exponentiation of size $ 2n + \log_2(n) + 7 $. In [Protocol 2](#protocol-2---inner-product-verification-through-multi-exponentiation) the Bulletproof *verifier* $ \mathcal{V} $ only performs two checks, that is step&nbsp;(68) Figure&nbsp;6 and step&nbsp;(16) Figure&nbsp;2.

In the protocol presented in Figure&nbsp;10, that is processed by the *verifier* $ \mathcal{V} $, $ x_u $ is the challenge from [Protocol 1](#protocol-1---inner-product-argument), $ x_j $ the challenge from round $ j $ of [Protocol 2](#protocol-2---inner-product-verification-through-multi-exponentiation), and $ L_j , R_j $ the $ L , R $ values from round $ j $ of [Protocol 2](#protocol-2---inner-product-verification-through-multi-exponentiation). 

<p align="center"><img src="sources/Protocol-4.png" width="570" /></p>
<div align="center"><b>Figure&nbsp;10: Bulletproofs' Protocol 4! [<a href="http://web.stanford.edu/%7Ebuenz/pubs/bulletproofs.pdf" title="Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al">1</a>]</b></div>



A further idea is that multi-exponentiation (steps (98) and (105) in Figure&nbsp;10) be delayed until those checks are performed and that they are also combined into a single check using a random value $ c \xleftarrow[]{$} \mathbf Z_p $. This follows from the fact that if $ A^cB = 1 $ for a random $ c $ then with high probability $ A = 1 \mspace 3mu \wedge \mspace 3mu B = 1 $. Various algorithms are known to compute the multi-exponentiations and scalar quantities (steps (101) and (102) in Figure&nbsp;10) efficiently (sub-linearly), thereby further improving the speed and efficiency of the protocol.

<u>Batch verification</u>

A further important optimization concerns the verification of multiple proofs. The essence of the verification is to calculate a large multi-exponentiation. Batch verification is applied in order to reduce the number of expensive exponentiations. This is based on the observation that checking $ g^x = 1 \mspace 3mu \wedge \mspace 3mu g^y = 1 $ can be checked by drawing a random scalar $ \alpha $ from a large enough domain and checking that $ g^{\alpha x + y} = 1 $. With high probability, the latter equation implies the first. When applied to multi-exponentiations, $ 2n $ exponentiations can be saved per additional proof. Verifying $ m $ distinct range proofs of size $ n $ only requires a single multi-exponentiation of size $ 2n+2+m \cdot (2 \cdot \log (n) + 5 ) $ along with $ O ( m \cdot n ) $ scalar operations.



## Evolving Bulletproof Protocols

Interstellar [[24]] recently introduced the Programmable Constraint Systems for Bulletproofs [[23]], an evolution of [Protocol 3!](#protocol-3---zero-knowledge-proof-for-arithmetic-circuits), extending it to support proving arbitrary statements in zero-knowledge using a constraint system, bypassing arithmetic circuits altogether. They provide an Application Programmers Interface (API) for building a constraint system directly, without the need to construct arithmetic expressions and then transform them into constraints. The Bulletproof constraint system proofs are then used as building blocks for a confidential assets protocol called Cloak.

The constraint system has three kinds of variables: 

- High-level witness variables:
  - Known only to the *prover* $ \mathcal{P} $, as external inputs to the constraint system;
  - Represented as individual Pedersen commitments to the external variables in Bulletproofs.
- Low-level witness variables:
  - Known only to the *prover* $ \mathcal{P} $, as internal to the constraint system;
  - Representing the inputs and outputs of the multiplication gates.
- Instance variables:
  - Known to both the *prover* $ \mathcal{P} $ and the *verifier* $ \mathcal{V} $, as public parameters;
  - Represented as a family of constraint systems parameterized by public inputs (compatible with Bulletproofs);
  - Folding all instance variables into a single constant parameter internally.

Instance variables can select the constraint system out of a family for each proof. The constraint system becomes a challenge from a *verifier* $ \mathcal{V} $ to a *prover* $ \mathcal{P} $, where some constraints are generated randomly in response to the *prover*'s $ \mathcal{P} $ commitments. Challenges to parametrize constraint systems makes the resulting proof smaller, requiring only $ O(n) $ multiplications instead of $ O(n^2) $ in the case of verifiable shuffles when compared to a static constraint system.

Merlin transcripts [[25]] employing the Fiat-Shamir Heuristic<sup>[def][fsh~]</sup> are used to generate the challenges. The challenges are bound to the high-level witness variables (the external inputs to the constraint system) which are added to the transcript before any of the constraints are created. The *prover* $ \mathcal{P} $ and *verifier* $ \mathcal{V} $ can then compute weights for some constraints with the use of the challenges.

Because the challenges are not bound to low-level witness variables the resulting construction can be unsafe. Interstellar are working on an improvement to the protocol that would allow challenges to be bound to a subset of the low-level witness variables, and have a safer API using features of Rust’s type system.

The resulting API provides a single code path used by both the *prover* $ \mathcal{P} $ and *verifier* $ \mathcal{V} $ to allocate variables and define constraints. This is organized into a hierarchy of task-specific gadgets, which manages allocation, assignment and constraints on the variables, ensuring that all variables are constrained. Gadgets interact with mutable constraint system objects, which are specific to the *prover* $ \mathcal{P} $ and *verifier* $ \mathcal{V} $. They also receive secret variables and public parameters and generate challenges.

The Bulletproofs library [[22]] does not provide any standard gadgets, but only an API for the constraint system. Each protocol built on top of the Bulletproofs library must create its own collection of gadgets to enable building a complete constraint system out of them. The Interstellar Bulletproof zero-knowledge proof protocol built with their programmable constraint system is shown in Figure&nbsp;11.

<p align="center"><img src="sources/InterstellarConstraintSystem.png" width="870" /></p>
<div align="center"><b>Figure&nbsp;11: Interstellar Bulletproof Zero-Knowledge Proof Protocol [<a href="https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7" title="Programmable Constraint Systems for Bulletproofs,
Interstellar,
Cathie Yun">24</a>]</b></div>




## Conclusions, Observations, Recommendations

- Bulletproofs have many potential use cases or [applications](../bulletproofs-and-mimblewimble/MainReport.md#applications-for-bulletproofs), but are still under [development](../bulletproofs-and-mimblewimble/MainReport.md#current--past-efforts). A new confidential blockchain protocol like Tari should carefully consider expanded use of Bulletproofs to maximally leverage functionality of the code base.
- Bulletproofs are not done yet, as illustrated in [Evolving Bulletproof Protocols](#evolving-bulletproof-protocols), and its further development and efficient implementation has a lot of traction in the community.
- Bünz et al. [[1]] proposed that the switch commitment scheme defined by Ruffing et al. [[10]] can be used for Bulletproofs if doubts in the underlying cryptographic hardness (discrete log) assumption arise in future. The switch commitment scheme allows for a blockchain with proofs that are currently only computationally binding to later switch to a proof system that is perfectly binding and secure against quantum adversaries; this will weaken the perfectly hiding property as a drawback and slow down all proof calculations. In the Bünz et al. [[1]] proposal all Pedersen commitments will be replaced with ElGamal Commitments<sup>[def][egc~]</sup> to move from computationally binding to perfectly binding. They also gave further ideas about how the ElGamal commitments can possibly be enhanced to improve the hiding property to be statistical or perfect. (*See the Grin projects' implementation [here](../bulletproofs-and-mimblewimble/MainReport.md#wallet-reconstruction-and-switch-commitment---grin).*)
- It is important that developers understand more about the fundamental underlying mathematics when implementing something like Bulletproofs, even if they just re-use libraries developed by someone else.



## References

[[1]] Bulletproofs: Short Proofs for Confidential Transactions and More, Blockchain Protocol Analysis and Security Engineering 2018, Bünz B., Bootle J., Boneh D., Poelstra A., Wuille P. and Maxwell G., http://web.stanford.edu/~buenz/pubs/Bulletproofs.pdf, Date accessed: 2018-09-18.

[1]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf
"Bulletproofs: Short Proofs for Confidential Transactions 
and More, Blockchain Protocol Analysis and Security 
Engineering 2018, 
Bünz B. et al"

[[2]] Efficient zero-knowledge arguments for arithmetic circuits in the discrete log setting, Bootle J., Cerulli A., Chaidos P., Groth J. and Petit C., Annual International Conference on the Theory and Applications of Cryptographic Techniques, pages 327-357. Springer, 2016., https://eprint.iacr.org/2016/263.pdf, Date accessed: 2018-09-21.

[2]: https://eprint.iacr.org/2016/263.pdf
"Efficient zero-knowledge arguments for arithmetic 
circuits in the discrete log setting, Bootle J et al."

[[3]] Confidential Assets, Poelstra A., Back A., Friedenbach M., Maxwell G. and Wuille P., Blockstream, https://blockstream.com/bitcoin17-final41.pdf, Date accessed: 2018-09-25.

[3]: https://blockstream.com/bitcoin17-final41.pdf
"Confidential Assets,
Poelstra A. et al.,
Blockstream"

[[4]] Wikipedia: Zero-knowledge Proof, https://en.wikipedia.org/wiki/Zero-knowledge_proof, Date accessed: 2018-09-18. 

[4]: https://en.wikipedia.org/wiki/Zero-knowledge_proof
"Wikipedia - Zero-knowledge Proof"

[[5]] Wikipedia: Discrete logarithm, https://en.wikipedia.org/wiki/Discrete_logarithm, Date accessed: 2018-09-20.

[5]: https://en.wikipedia.org/wiki/Discrete_logarithm
"Wikipedia: Discrete logarithm"

[[6]] How to Prove Yourself: Practical Solutions to Identification and Signature Problems, Fiat A. and Shamir A., CRYPTO 1986: pp. 186-194, https://link.springer.com/content/pdf/10.1007%2F3-540-47721-7_12.pdf, Date accessed: 2018-09-20.

[6]: https://link.springer.com/content/pdf/10.1007%2F3-540-47721-7_12.pdf
"How to Prove Yourself: Practical Solutions to 
Identification and Signature Problems, 
Fiat A. et al."

[[7]] How not to Prove Yourself: Pitfalls of the Fiat-Shamir Heuristic and Applications to Helios, Bernhard D., Pereira O. and Warinschi B., https://link.springer.com/content/pdf/10.1007%2F978-3-642-34961-4_38.pdf, Date accessed: 2018-09-20.

[7]: https://link.springer.com/content/pdf/10.1007%2F978-3-642-34961-4_38.pdf
"How not to Prove Yourself: Pitfalls of the 
Fiat-Shamir Heuristic and Applications to Helios, 
Bernhard D. et al."

[[8]] Pedersen-commitment: An implementation of Pedersen commitment schemes, https://hackage.haskell.org/package/pedersen-commitment, Date accessed: 2018-09-25.

[8]: https://hackage.haskell.org/package/pedersen-commitment
"Pedersen-commitment: An implementation
of Pedersen commitment schemes"

[[9]] Zero Knowledge Proof Standardization - An Open Industry/Academic Initiative, https://zkproof.org/documents.html, Date accessed: 2018-09-26.

[9]: https://zkproof.org/documents.html
"Zero Knowledge Proof Standardization - 
An Open Industry/Academic Initiative"

[[10]] Switch Commitments: A Safety Switch for Confidential Transactions, Ruffing T. and Malavolta G., https://eprint.iacr.org/2017/237.pdf, Date accessed: 2018-09-26.

[10]: https://eprint.iacr.org/2017/237.pdf
"Switch Commitments: A Safety Switch 
for Confidential Transactions, 
Ruffing T. et al."

[[11]] GitHub: adjoint-io/Bulletproofs, Bulletproofs are Short Non-interactive Zero-knowledge Proofs that Require no Trusted Setup, https://github.com/adjoint-io/Bulletproofs, Date accessed: 2018-09-10.

[11]: https://github.com/adjoint-io/bulletproofs
"GitHub: adjoint-io/Bulletproofs, Bulletproofs are Short
Non-interactive Zero-knowledge Proofs that Require 
no Trusted Setup"

[[12]] Wikipedia: Commitment scheme, https://en.wikipedia.org/wiki/Commitment_scheme, Date accessed: 2018-09-26.

[12]: https://en.wikipedia.org/wiki/Commitment_scheme
"Wikipedia: Commitment scheme"

[[13]] Cryptography Wikia: Commitment scheme, http://cryptography.wikia.com/wiki/Commitment_scheme, Date accessed: 2018-09-26.

[13]: http://cryptography.wikia.com/wiki/Commitment_scheme
"Cryptography Wikia: Commitment scheme"

[[14]] Adjoint Inc. Documentation: Pedersen Commitment Scheme, https://www.adjoint.io/docs/cryptography.html#pedersen-commitment-scheme, Date accessed: 2018-09-27.

[14]: https://www.adjoint.io/docs/cryptography.html#pedersen-commitment-scheme
"Adjoint Inc. Documentation: 
Pedersen Commitment Scheme"

[[15]] Non-interactive and information-theoretic secure verifiable secret sharing, Pedersen T., https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf, Date accessed: 2018-09-27.

[15]: https://www.cs.cornell.edu/courses/cs754/2001fa/129.pdf
"Non-interactive and information-theoretic
secure verifiable secret sharing, 
Pedersen T."

[[16]] Assumptions Related to Discrete Logarithms: Why Subtleties Make a Real Difference, Sadeghi A. and Steiner M., http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf, Date accessed: 2018-09-24.

[16]: http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf
"Assumptions Related to Discrete Logarithms: 
Why Subtleties Make a Real Difference, 
Sadeghi A et al." 

[[17]] Intensified ElGamal Cryptosystem (IEC), Sharma P., Gupta A. and Sharma S., International Journal of Advances in Engineering & Technology, Jan 2012, http://www.e-ijaet.org/media/58I6-IJAET0612695.pdf, Date accessed: 2018-10-09.

[17]: http://www.e-ijaet.org/media/58I6-IJAET0612695.pdf
"Intensified ElGamal Cryptosystem (IEC), Sharma P. et al.
International Journal of Advances in Engineering & Technology,
Jan 2012"

[[18]] On the Security of ElGamal Based Encryption, Tsiounis Y. and Yung M., https://drive.google.com/file/d/16XGAByoXse5NQl57v_GldJwzmvaQlS94/view, Date accessed: 2018-10-09.

[18]: https://drive.google.com/file/d/16XGAByoXse5NQl57v_GldJwzmvaQlS94/view
"On the Security of ElGamal Based Encryption,
Tsiounis Y. et al."

[[19]] Wikipedia: Decisional Diffie–Hellman assumption, https://en.wikipedia.org/wiki/Decisional_Diffie%E2%80%93Hellman_assumption, Date accessed: 2018-10-09.

[19]: https://en.wikipedia.org/wiki/Decisional_Diffie%E2%80%93Hellman_assumption
"Wikipedia: Decisional Diffie–Hellman assumption"

[[20]] Wikipedia: Arithmetic circuit complexity, https://en.wikipedia.org/wiki/Arithmetic_circuit_complexity, Date accessed: 2018-11-08.

[20]: https://en.wikipedia.org/wiki/Arithmetic_circuit_complexity
"Wikipedia: Arithmetic circuit complexity"

[[21]] Wikipedia: Hadamard product (matrices), https://en.wikipedia.org/wiki/Hadamard_product_(matrices), Date accessed: 2018-11-12.

[21]: https://en.wikipedia.org/wiki/Hadamard_product_(matrices)
"Wikipedia: Hadamard product (matrices)"

[[22]] Dalek Cryptography - Crate Bulletproofs, https://doc.dalek.rs/bulletproofs/index.html, Date accessed: 2018-11-12.

[22]: https://doc.dalek.rs/bulletproofs/index.html
"Dalek Cryptography - 
Crate Bulletproofs"

[[23]] Programmable Constraint Systems for Bulletproofs, https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7, Date accessed: 2018-11-22.

[23]: https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7
"Programmable Constraint Systems for Bulletproofs,
Interstellar,
Cathie Yun"

[[24]] Inter/stellar website, https://interstellar.com, Date accessed: 2018-11-22.

[24]: https://interstellar.com
"Inter/stellar Website"

[[25]] Dalek Cryptography - Crate merlin, https://doc.dalek.rs/merlin/index.html, Date accessed: 2018-11-22.

[25]: https://doc.dalek.rs/merlin/index.html
"Dalek Cryptography - 
Crate merlin"




## Appendices

### Appendix A: Definition of Terms

Definitions of terms presented here are high level and general in nature. Full mathematical definitions are available in the cited references. 

- <u><i>Arithmetic Circuits</i></u>:<a name="ac"> </a>An arithmetic circuit $ C ​$ over a field $ F ​$ and variables $ (x_1, ..., x_n) ​$ is a directed acyclic graph whose vertices are called gates. Arithmetic circuits can alternatively be described as a list of addition and multiplication gates with a collection of linear consistency equations relating the inputs and outputs of the gates. The size of an arithmetic circuit is the number of gates in it, with the depth being the length of the longest directed path. *Upper bounding* the complexity of a polynomial $ f ​$ is to find any arithmetic circuit that can calculate $ f ​$, whereas *lower bounding* is to find the smallest arithmetic circuit that can calculate $ f ​$. An example of a simple arithmetic circuit with size six and depth two that calculates a polynomial is shown below. ([[11]], [[20]])

 <p align="center"><img src="sources/ArithmiticCircuit.PNG" width="300" /></p>

[ac~]: #ac
"An arithmetic circuit C over a 
field F and variables (x_1, ..., x_n) 
is a directed acyclic graph ..."

- <u><i>Commitment Scheme</i></u>:<a name="cs"> </a>A commitment scheme in a Zero-knowledge Proof<sup>[def][zk~]</sup> is a cryptographic primitive that allows a prover to commit to only a single chosen value/statement from a finite set without the ability to change it later (*binding* property) while keeping it hidden from a verifier (*hiding* property). Both *binding* and *hiding* properties are then further classified in increasing levels of security to be computational, statistical or perfect. No commitment scheme can at the same time be perfectly *binding* and perfectly *hiding*. ([[12]], [[13]])

[cs~]: #cs
"A commitment scheme in a 
zero-knowledge proof is a 
cryptographic primitive ..."

- <i><u>Discrete Logarithm/Discrete Logarithm Problem (DLP)</u></i>:<a name="dlp"> </a>In the mathematics of real numbers, the logarithm $ \log_b^a ​$ is a number $ x ​$ such that $ b^x=a ​$, for given numbers $ a ​$ and $ b ​$. Analogously, in any group $ G ​$ , powers $ b^k ​$ can be defined for all integers $ k ​$, and the discrete logarithm $ \log_ba ​$ is an integer $ k ​$ such that $ b^k=a ​$. Algorithms in public-key cryptography base their security on the assumption that the discrete logarithm problem over carefully chosen cyclic finite groups and cyclic subgroups of elliptic curves over finite fields has no efficient solution. ([[5]], [[16]])

[dlp~]: #dlp
"In the mathematics of real 
numbers, the logarithm log_b(a) 
is a number x such that ..."

- <u><i>ElGamal Commitment/Encryption</i></u>:<a name="egc"> </a>An ElGamal commitment is a Pedersen Commitment<sup>[def][pc~]</sup> with an additional commitment $ g^r $ to the randomness used. The ElGamal encryption scheme is based on the Decisional Diffe-Hellman (DDH) assumption and the difficulty of the DLP for finite fields. The DDH assumption states that it is infeasible for a Probabilistic Polynomial-time (PPT) adversary to solve the DDH problem. (<i>**Note:** The ElGamal encryption scheme should not be confused with the ElGamal signature scheme.</i>) ([[1]], [[17]], [[18]], [[19]])

[egc~]: #egc
"An ElGamal Commitment is a 
Pedersen Commitment with
additional commitment ..."

- <u><i>Fiat–Shamir Heuristic/Transformation</i></u>:<a name="fsh"> </a>The Fiat–Shamir heuristic is a technique in cryptography to convert an interactive public-coin protocol (Sigma protocol) between a *prover* and a *verifier* into a one-message (non-interactive) protocol using a cryptographic hash function. ([[6]], [[7]])
  - The *prover* will use a <code>Prove()</code> algorithm to calculate a commitment $ A $ with a statement $ Y $ that is shared with the *verifier* and a secret witness value $ w $ as inputs. The commitment $ A $ is then hashed to obtain the challenge $ c $, which is further processed with the <code>Prove()</code> algorithm to calculate the response $ f $. The single message sent to the *verifier* then contains the challenge $ c $ and response $ f $.

  - The *verifier* is then able to compute the commitment $ A $ from the shared statement $ Y $, challenge $ c $ and response $ f $. The *verifier* will then use a <code>Verify()</code> algorithm to verify the combination of shared statement $ Y $, commitment $ A $, challenge $ c $ and response $ f $.

  - A weak Fiat–Shamir transformation can be turned into a strong Fiat–Shamir transformation if the hashing function is applied to the commitment $ A $ and shared statement $ Y $ to obtain the challenge $ c $ as opposed to only the commitment $ A $.

[fsh~]: #fsh
"The Fiat–Shamir heuristic is a 
technique in cryptography to 
convert an interactive ..."

- <u>*Hadamard Product*</u>:<a name="hdmp"> </a>In mathematics, the Hadamard product is a binary operation that takes two matrices $ \mathbf {A} , \mathbf {B} $ of the same dimensions, and produces another matrix of the same dimensions where each element $ i,j $ is the product of elements $ i,j $ of the original two matrices. The Hadamard product $ \mathbf {A} \circ \mathbf {B} $ is different from normal matrix multiplication most notably because it is also commutative $ [ \mathbf {A} \circ \mathbf {B} = \mathbf {B} \circ \mathbf {A} ] $ along with being associative $ [ \mathbf {A} \circ ( \mathbf {B} \circ \mathbf {C} ) = ( \mathbf {A} \circ \mathbf {B} ) \circ \mathbf {C} ] $ and distributive over addition $ [ \mathbf {A} \circ ( \mathbf {B} + \mathbf {C} ) = \mathbf {A} \circ \mathbf {B} + \mathbf {A} \circ \mathbf {C} ] $. ([[21]])

$$
\mathbf {A} \circ \mathbf {B} = \mathbf {C} = (a_{11} \cdot b_{11} \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} a_{1m} \cdot b_{1m} \mspace{6mu} ; \mspace{6mu} . . . \mspace{6mu} ; \mspace{6mu} a_{n1} \cdot b_{n1} \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} a_{nm} \cdot b_{nm} )
$$

[hdmp~]: #hdmp
"In mathematics, the Hadamard product 
is a binary operation that takes two 
matrices A,B of the same dimensions ..."

- <u><i>Pedersen Commitment</i></u>:<a name="pc"> </a>Pedersen commitments are a system for making blinded non-interactive commitments to a value. ([[1]], [[3]], [[8]], [[14]], [[15]]).

  - The generalized Pedersen commitment definition follows (*see [Appendix B](#appendix-b-notations-used) for notations used*):
    - Let $ q $ be a large prime and $ p $ be a large safe prime such that $ p = 2q + 1 $ 

    - Let $ h $ be a random generator of cyclic group $ \mathbb G $ such that $ h $ is an element of $ \mathbb Z_q^* $ 

    - Let $ a $ be a random value and element of $ \mathbb Z_q^* $ and calculate $ g $ such that $ g = h^a $ 

    - Let $ r $ (the blinding factor) be a random value and element of $ \mathbb Z_p^* $ 

    - The commitment of value $ x $ is then determined by calculating $ C(x,r) = h^r g^x $ 

    - The generator $ h $ and resulting number $ g $ are known as the commitment bases and should be shared along with $ C(x,r) $ with whomever wishes to open the value.

  - Pedersen commitments are also additionally homomorphic, such that for messages $ x_0 $ and $ x_1 $ and blinding factors $ r_0 $ and $ r_1 $ we have $ C(x_0,r_0) \cdot C(x_1,r_1) = C(x_0+x_1,r_0+r_1) $ 

  - Security attributes of the Pedersen Commitment scheme are perfectly *hiding*<sup>[def][cs~]</sup> and computationally *binding*<sup>[def][cs~]</sup>. An efficient implementation of the Pedersen Commitment will use secure Elliptic Curve Cryptography (ECC), which is based on the algebraic structure of elliptic curves over finite (prime) fields. 

  - Practical implementations usually consist of three algorithms: <code>Setup()</code> to set up the commitment parameters; <code>Commit()</code> to commit to the message using the commitment parameters and <code>Open()</code> to open and verify the commitment.

[pc~]: #pc
"A Pedersen commitments are a system 
for making blinded non-interactive 
commitments to a value ..."

- <u><i>Zero-knowledge Proof/Protocol</i></u>:<a name="zk"> </a>In cryptography, a zero-knowledge proof/protocol is a method by which one party (the prover) can convince another party (the verifier) that a statement $ Y $ is true, without conveying any information apart from the fact that the prover knows the value of $ Y $. The proof system must be complete, sound and zero-knowledge. ([[4]], [[9]])
  - Complete: If the statement is true and both prover and verifier follow the protocol; the verifier will accept.
  - Sound: If the statement is false, and the verifier follows the protocol; the verifier will not be convinced.

  - Zero-knowledge: If the statement is true and the prover follows the protocol, the verifier will not learn any confidential information from the interaction with the prover apart from the fact that the statement is true.

[zk~]: #zk
"In cryptography, a zero-knowledge 
proof/protocol is a method by which 
one party (the prover) can convince ..."



### Appendix B: Notations Used

The general notation of mathematical expressions when specifically referenced are listed here, based on [[1]].

- Let $ \mathbb G $ and $ \mathbb Q $ denote cyclic groups of prime order $ p $ and $ q $ respectively

- let $ \mathbb Z_p $ and $ \mathbb Z_q $ denote the ring of integers $ modulo \mspace{4mu} p $ and $ modulo \mspace{4mu} q $ respectively

- Let $ \mathbb Z_p^* $ denote $ \mathbb Z_p \setminus \lbrace 0 \rbrace $ and $ \mathbb Z_q^* $ denote $ \mathbb Z_q \setminus \lbrace 0 \rbrace $ 

- Let generators of $ \mathbb G $ be denoted by $ g, h, v, u \in \mathbb G $ 

- Let $ \mathbb G^n $ and $ \mathbb Z^n_p $ be vector spaces of dimension $ n $ over $ \mathbb G $ and $ \mathbb Z_p $ respectively

- Let $ h^r \mathbf g^\mathbf x = h^r \prod_i g_i^{x_i} \in \mathbb G $ be the vector Pedersen Commitment<sup>[def][pc~]</sup> with $ \mathbf {g} = (g_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} g_n) \in \mathbb G^n $ and $ \mathbf {x} = (x_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} x_n) \in \mathbb G^n $ 

- Let $ \mathbf {a} \in \mathbb F^n $ be a vector with elements $ a_1 \cdot b_1 \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} a_n \cdot b_n \in F^n $ 
- Let $ \langle \mathbf {a}, \mathbf {b} \rangle = \sum _{i=1}^n {a_i \cdot b_i} ​$ denote the inner-product between two vectors $ \mathbf {a}, \mathbf {b} \in \mathbb F^n ​$ 
- Let $ \mathbf {a} \circ \mathbf {b} = (a_1 \cdot b_1 \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} a_n \cdot b_n) \in \mathbb F^n $ denote the entry wise multiplication of two vectors $ \mathbf {a}, \mathbf {b} \in \mathbb F^n $ 
- Let $ \mathbf {A} \circ \mathbf {B} = (a_{11} \cdot b_{11} \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} a_{1m} \cdot b_{1m} \mspace{6mu} ; \mspace{6mu} . . . \mspace{6mu} ; \mspace{6mu} a_{n1} \cdot b_{n1} \mspace{3mu} , \mspace{3mu} . . . \mspace{3mu} , \mspace{3mu} a_{nm} \cdot b_{nm} ) $ denote the entry wise multiplication of two matrixes, also known as the Hadamard Product<sup>[def][hdmp~]</sup> 
- Let $ \mathbf {a} \parallel \mathbf {b} $ denote the concatenation of two vectors; if $ \mathbf {a} \in \mathbb Z_p^n $ and $ \mathbf {b} \in \mathbb Z_p^m $ then $ \mathbf {a} \parallel \mathbf {b} \in \mathbb Z_p^{n+m} $ 
- Let $ p(X) = \sum _{i=0}^d { \mathbf {p_i} \cdot X^i} \in \mathbb Z_p^n [X] $ be a vector polynomial where each coefficient $ \mathbf {p_i} $ is a vector in $ \mathbb Z_p^n $ 
- Let $ \langle l(X),r(X) \rangle = \sum _{i=0}^d { \sum _{j=0}^i { \langle l_i,r_i \rangle \cdot X^{i+j}}} \in \mathbb Z_p [X] $ denote the inner-product between two vector polynomials $ l(X),r(X) $ 
- Let $ t(X)=\langle l(X),r(X) \rangle $, then the inner-product is defined such that $ t(x)=\langle l(x),r(x) \rangle $ holds for all $ x \in \mathbb{Z_p} $ 
- Let $ C=g^a = \prod _{i=1}^n g_i^{a_i} \in \mathbb{G} $ be a binding (but not hiding) commitment to the vector $ \mathbf {a} \in \mathbb Z_p^n $ where $ \mathbf {g} = (g_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} g_n) \in \mathbb G^n $. Given vector $ \mathbf {b} \in \mathbb Z_p^n $ with non-zero entries, $ \mathbf {a} \circ \mathbf {b} $ is treated as a new commitment to $ C $. For this let $ g_i^\backprime =g_i^{(b_i^{-1})} $ such that $ C= \prod _{i=1}^n (g_i^\backprime)^{a_i \cdot b_i} $. The binding property of this new commitment is inherited from the old commitment.

- Let slices of vectors be defined as $ \mathbf {a_{[:l]}} = (a_1 \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} a_l) \in \mathbb F^l \mspace{3mu} , \mspace{12mu}\ \mathbf {a_{[l:]}} = (a_{l+1} \mspace{3mu} , \mspace{3mu} ... \mspace{3mu} , \mspace{3mu} a_n) \in \mathbb F^{n-l}$ 
- Let $ \mathbf {k}^n $ denote the vector containing the first $ n $ powers of $ k \in \mathbb Z_p^* $ such that $ \mathbf {k}^n = (1,k,k^2, \mspace{3mu} ... \mspace{3mu} ,k^{n-1}) \in (\mathbb Z_p^*)^n $ 
- Let $ \mathcal{P} $ and $ \mathcal{V} $ denote the *prover* and *verifier* respectively
- Let $ \mathcal{P_{IP}} $ and $ \mathcal{V_{IP}} $ denote the *prover* and *verifier* in relation to inner-product calculations respectively



## Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- [https://github.com/neonknight64](https://github.com/neonknight64)
- [https://github.com/CjS77](https://github.com/CjS77)
- [https://github.com/philipr-za](https://github.com/philipr-za)
- [https://github.com/Kevoulee](https://github.com/Kevoulee) ???
