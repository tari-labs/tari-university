

# **Rank-1 Constraint System with Application to Bulletproofs**



The main focus of this report is to explain the Mathematics behind Rank-1 Constraints System or R1CS as it applies to zkSNARKs and Bulletproofs. 

When reading the literature on the use of R1CS in zkSNARKs, this mathematical tool is used simply as one of many parts of the process towards achieving a zkSNARK-proof. In fact, not much attention is given to it, not even in explaining what "rank-1" actually means. Although the terminology has a similar notion of the traditional _rank of a matrix_ as known in Linear Algebra, examples given on the internet do not yield a _reduced matrix with only one non-zero row or column_. 



## Introduction 

R1CS became more prominent due to research work done by Cathie Yun and her colleagues at Interstellar. The _Constraint systems_, or in particular _Rank-1 Constraint Systems_, are used as an _add-on_ to Bulletproofs protocols. The title of Cathie Yun's article, "[Building on Bulletproofs](https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8)" [1b.] suggests this to be true. One of the Interstellar team's goals is to use the constraints system in their envisaged _Confidential Asset Protocol_ called the CLOAK. Despite their work on using R1CS being _research-in-progress_, their detailed notes on _constraints systems_ and their implementation in RUST are made available [here](https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html#constraint-system). (Also https://doc.dalek.rs/bulletproofs/index.html)

Most recent constructions of zkSNARKs and Bulletproofs involve _arithmetic circuits_ and _rank-1 constraint systems_ either explicitly or implicitly.  

In this report we intend to; 

- highlight the connection between _Arithmetic circuits_ and R1CS, 
- clarify the difference R1CS makes in bulletproofs compared to range proofs (or inner-product proofs),  
- the link between R1CS in _zkSNARK_ and R1CS in _Bulletproofs_.   

We refer the reader to the [notation used](https://tlu.tarilabs.com/cryptography/bulletproofs-protocols/MainReport.html#notation-used) in the Bulletproofs Protocols [report](https://tlu.tarilabs.com/cryptography/bulletproofs-protocols/MainReport.html#the-bulletproof-protocols).

And talking about Bulletproofs, we would like to understand what Bunz et al. meant by ... "We note that a range proof using the protocol of [BCC+16] would have required implementing the commitment opening algorithm as part of the verification circuit, which we are able to eliminate."  





## Arithmetic Circuits  

Arithmetic circuits are said to be the most natural and standard model for computing polynomials. In fact, every function  ${\large \mathcal{H} : \{ 0 , 1 \}^n → \{ 0 , 1 \}^m} $  of fixed input and output _length_ can be represented as an arithmetic circuit over any finite field  $ \mathcal{F}_p$ , cf. [[7.]](Alan Szepieniec and Bart Preneel, "Generic Zero-Knowledge and Multivariate Quadratic Systems"). 

According Amir Shpilka and Amir Yehudayoff [1.], arithmetic circuits are a highly structured model of computation compared to Boolean circuits. For instance, when studying arithmetic circuits one is interested in _syntactic_ computation of polynomials, whereas in studying Boolean circuits one is interested in the _semantics_ of the computation. In other words, in the Boolean case one is not interested in any specific _polynomial representation_ of the function but rather one just wants to compute some representation of it, while in the arithmetic world one focuses on a _specific representation_ of the function. 

For our purposes, arithmetic circuits have as inputs variables  $\large{ x_1, ... , x_n }$, and computations are performed using the arithmetic operations  $ + $ , $ × $  and may involve constants from a field  $\mathcal{F}_p$. Complexity measures associated with such circuits are _size_ and _depth_, which capture _the number of gates_ in a circuit and _the maximal distance between an input and an output_, respectively [[1.]](Arithmetic Circuits-survey of recent results -open questions__SY10.pdf)



### Definition 1 (Arithmetic circuits) [[1.]](Arithmetic Circuits-survey of recent results -open questions__SY10.pdf) 

An ***arithmetic circuit***  $\mathcal{A}$  over the field  $\mathcal{F}$  and the set of variables  $X = \{ \large{x_1,...,x_n} \}$  is a _directed acyclic graph_ such that the vertices of  $\mathcal{A}$  are called _gates_, while the edges are called _wires_. 

(a)  A gate is said to be of _in-degree_  $l$  if it has  $l$  incoming wires, and similarly, of _out-degree_  $k$  if it has  $k$  out going wires. 

(b)  Every gate in  $\mathcal{A}$  of _in-degree_  0  is labelled by either a variable  $\large{x_i}$  or some field element from  $\mathcal{F}$ , and such a gate is called an _input gate_. Every gate of _out-degree_ 0 is called an _output gate_. 

(c)  Every other gate in  $\mathcal{A}$  is labeled by either  ${\Large{\otimes}}$  or  $\Large{\oplus}$ , and called a _multiplication gate_ or _addition gate_, respectively. We are interested in _binary_ circuits, that is, circuits where each gate has two _input wires_ and one _output wire_. 

(d)  An _arithmetic circuit_ is called a _formula_ if it is a _directed tree_ whose edges are _directed_ from the leaves to the root. 

(e)  The size of  $\mathcal{A}$ , denoted |$\mathcal{A}$|, is the number of _wires_ (i.e. _edges_) in  $\mathcal{A}$. 



A typical multiplication gate is thought of as having a _left input_  $\large{a_L}$, a _right input_  $\large{a_R}$, and an _output_  $\large{a_O}$. Also, we note that,  $$ \large{a_L \cdot a_R  -  a_O  =  0} .$$ 

![basic-multiplication-gate](/Users/anthonymatlala/Downloads/basic-multiplication-gate.png)



We note that in cases where the inputs and outputs are all _vectors_ of  ${\large n}$  components, i.e., 

​	${\large \mathbf{a_L} = ( a_{L, 1}, a_{L, 2} , ... , a_{L, n}) }$, $ {\large \mathbf{a_R} = ( a_{R, 1}, a_{R, 2} , ... , a_{R, n}) } $, ${\large \mathbf{a_O} = ( a_{O, 1}, a_{O, 2} , ... , a_{O, n}) } ,$ 

then _multiplication_ between  ${\large \mathbf{a_L}}$  and  ${\large \mathbf{a_R} }$  is defined as an _entry-wise_ product called the _**Hadamard product**_; 

  			${\large \mathbf{a_L}\circ \mathbf{a_R} = (( a_{L, 1} \cdot a_{R, 1} ) , ( a_{L, 2}  \cdot a_{R, 2} ) , ... , ( a_{L, n} \cdot a_{R, n} ) )}   =   \large \mathbf{a_O}  .$

The equation  ${\large \mathbf{a_L}\circ \mathbf{a_R} =  \mathbf{a_O} }$  referred to as a _multiplicative constraint_ is also known as the _Hadamard relation_, [10.]. 



An arithmetic circuit computes a polynomial in a natural way, as seen in the example below. 

###Example 1 

Consider the following Arithmetic circuit  $\mathcal{A}$  with inputs  $\{ {\large{ x_1 , x_2 , 1 }} \}$  over some field  $\mathcal{F}$ , 

<img src="/Users/anthonymatlala/Downloads/polynomial-eg-ac.png" alt="polynomial-eg-ac"  />

The output of  $\mathcal{A}$  above is the polynomial   $\large{x^2_1 x_2 + x_1 + 1 }$  of _total degree_ three. 

A typical _computational problem_ would involve finding the solution to, let's say,  $\large{x^2_1 x_2 + x_1 + 1 = 22}$.  Or, in a proof of knowledge scenario, the prover has to prove to the verifier that he has the correct solution to such an equation. 

Following the wires, in Example 1.2, shows that an arithmetic circuit actually _breaks down_ the given computation into _smaller equations_ corresponding to each _gate_; 

​				$\large{ u = x_1*x_1} $ ,   $ \large{ v = u*x_2 }$ ,   $ \large{ y = x_1 + 1 } $ , and  $ \large{ z = v + y } $ . 

The variables  $ \large{ u , v }$  and $ \large{ y }$  are called _auxiliary variables_ or _low level variables_, while  $\large{ z }$  is the _output_ of  $ \mathcal{A} $.  Thus, in addition to computing polynomials naturally, an arithmetic circuit helps in reducing a computation to a _low level_ language involving only _two_ variables, _one operation_, and an _output_. 

Although "Arithmetic circuits are a highly structured model of computation compared to Boolean circuits," according to Amir Shpilka and Amir Yehudayoff [[1.]](Arithmetic Circuits-survey of recent results -open questions__SY10.pdf), "we do not know how to efficiently reconstruct a circuit using only queries to the polynomial it computes." 

Computational problems that zero-knowledge proofs such as zkSNARKs and Bulletproofs handle need to be expressed in the simplest terms for efficiency. Alex Pinto [[2.]](http://coders-errand.com/constraint-systems-for-zk-snarks/) mentions that "the ZK SNARK end-to-end journey is to create a _function_ to write proofs about," but they "must work with specific constructs." That is why arithmetic circuits are so important in making zero-knowledge more efficient, "these _functions_ have to be specified as sequences of very simple terms, namely, additions and multiplications of only two terms in a particular field" [2.]

In verifying a zkSNARK proof, the verifier needs to carry out a step-by-step check of the computations, that is, for each gate the verifier has to check if the _output_  $ \large{ a_O }$  is correct with respect to the given _inputs_  $\large{ a_L }$  and  $ \large{ a_R } $. Testing if   $ \large{a_L \cdot a_R  -  a_O  =  0}$ , for each multiplication gate. This requires that an _addition gate_ be treated as some form of a _multiplication gate_, we explain this later in this report. 

For further details on Arithmetic circuits see previously curated content at  [[4.]](https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html), in particular, Vitalik Buterin's article [[5.]](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649).  





## R1CS 

Although one typically expresses a computation problem in terms of a high-level programming language, a zkSNARK requires expressing it in terms of a set of _quadratic constraints_, which is closely related to circuits of logical gates. 

R1CS is short for _Rank-1 Constraint System_. These type of constraint systems have been featured in several constructions of zkSNARKs and were at times simply referred to as _quadratic constraints_ or _quadratic equations_, see [6.], [[7.]](Alan Szepieniec and Bart Preneel, "Generic Zero-Knowledge and Multivariate Quadratic Systems") and [[8.]](Eli Ben-Sasson, et al., "SNARKs for C : Verifying Program Executions Succinctly and in Zero Knowledge" https://eprint.iacr.org/2013/507.pdf ). 

Here's a simplified definition of an R1CS as it applies to zkSNARKs by Vitalik Buterin, 

 ### Definition 2 (R1CS) [[5.]]( https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649 )   

An R1CS is a sequence of groups of three vectors  ${\large \bf{a_L}}, {\large \bf{a_R}}, {\large \bf{a_O}} ,$  and the solution to an R1CS is a vector  ${\large \bf{s}}$  that satisfies the equation 

​					${\large \langle {\bf{a_L , s}} \rangle * \langle {\bf{a_R , s }} \rangle - \langle {\bf{a_O , s }} \rangle = 0 }$. 




###Example 2 

One solution to the equation  $\large{x^2_1 x_2 + x_1 + 1 = 22}$  , from Example 1.2 above, is  ${\large{ x_1 = 3} }$  and  ${ \large{ x_2 = 2 }}$  belonging to the appropriate field  $\large{ \mathcal{F}}$.  Thus the solution vector  ${ \large{ s = ( const , x_1 , x_2 , z , u , v , y )}}$  becomes   ${ \large{ s = ( 1 , 3 , 2 , 22 , 9 , 18 , 4 )}}$ . 

It is easy to check that the R1CS for the computation problem in **Example 1** is as follows; (i.e., one need only test if  ${\large \langle {\bf{a_L , s}} \rangle * \langle {\bf{a_R , s }} \rangle - \langle {\bf{a_O , s }} \rangle = 0}$  for each equation),  



| Equation                        | R1CS Vectors                                                 |
| ------------------------------- | ------------------------------------------------------------ |
| $\large{ u = x_1*x_1}$          | $ {\bf{a_L}} = ( 0 , 1 , 0 , 0 , 0 , 0 , 0 ) , \ \ {\bf{a_R}} = ( 0 , 1 , 0 , 0 , 0 , 0 , 0  ) ,\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 0 , 1 , 0 , 0  ) $ |
| $ \large{ v = u*x_2 }$          | $ {\bf{a_L}} = ( 0 , 0 , 0 , 0 , 1 , 0 , 0 ) ,\ \ {\bf{a_R}} = ( 0 , 1 , 0 , 0 , 0 , 0 , 0  ),\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 0 , 0 , 1 , 0 )  $ |
| $ \large{ y = 1*( x_1 + 1 ) } $ | ${\bf{a_L}} = ( 1 , 1 , 0 , 0 , 0 , 0 , 0 ),\ \ {\bf{a_R}} = ( 1 , 0 , 0 , 0 , 0 , 0 , 0 ),\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 0 , 0 , 0 , 1 ) $ |
| $ \large{ z = 1*( v + y )} $    | ${\bf{a_L}} = ( 0 , 0 , 0 , 0 , 0 , 1 , 1 ),\ \ {\bf{a_R}} = ( 1 , 0 , 0 , 0 , 0 , 0 , 0 ),\ \ {\bf{a_O}} = ( 0 , 0 , 0 , 1 , 0 , 0 , 0 )$ |

​				**Table 1 : Equations and R1CS Vectors**  



In a more formal definition, an **R1CS** is in fact a set of three matrices  ${\bf{ A_L , A_R }}$ and  ${\bf A_O}$  where the rows of each matrix is formed by the corresponding vectors  $\large {\bf{a_L }}$ , ${\large \bf{a_R }}$ and  ${\large \bf{a_O}} $, respectively, as in Example 2 above;  



$ {\large \bf{A_L}} = {\bf{\begin{bmatrix} 0&1&0&0&0&0&0\\ 0&0&0&0&1&0&0\\ 1&1&0&0&0&0&0\\ 0&0&0&0&0&1&1\\ \end{bmatrix}}}, $ ${\large \bf{A_R}} = {\bf{\begin{bmatrix} 0&1&0&0&0&0&0\\ 0&1&0&0&0&0&0\\ 1&0&0&0&0&0&0\\ 1&0&0&0&0&0&0\\ \end{bmatrix}}}$ , ${\large \bf{ A_O }} = {\bf{  \begin{bmatrix} 0&0&0&0&1&0&0\\ 0&0&0&0&0&1&0\\ 0&0&0&0&0&0&1\\ 0&0&0&1&0&0&0\\ \end{bmatrix}}}.$



We observe that   ${\large \bf{ (A_L\cdot s^T) * (A_R\cdot s^T ) - (A_O\cdot s^T)} = 0 }$ ,  where  "${\bf{ \cdot }}$"  is _matrix multiplication_ and  ${\large \bf s^T}$  is the transpose of the solution vector  ${\large \bf{s}}$ .    

A more general definition of an Constraint System was given by the Dalek team in [[7.]](https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html#constraint-system), but originally defined by Bootle et al. [[10.]](https://eprint.iacr.org/2016/263.pdf)    

### Definition 3 (Constraint System) [[7.]](https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html#constraint-system) 

A **constraint system** is a collection of arithmetic constraints over a set of variables. There are two kinds of variables in the constraint system:

(a)   $\large{m}$  **high-level** variables, the input secrets  ${\large \mathbf{v}}$,  

(b)   $\large n$  **low-level** variables, the internal input vectors  ${\large \mathbf{a}_L}$ and  ${\large \mathbf{a}_R},$  and output vectors  ${\large{ \mathbf{a}_O }} $  of the multiplication gates. 

Specifically, a  **rank-1 constraint system**  or  **R1CS**  is a system that consists of two sets of constraints; 

​	  ${\large n}$   _multiplicative constraints_,   ${\large \mathbf{ a_L \circ a_R = a_O }} $ ,  

where  "${\large \circ }$"  is the Hadamard product, and

​	  ${\large q}$    _linear constraints_,   $\mathbf{W_L\cdot {\large a_L}  +  W_R\cdot {\large a_R}  +  W_O\cdot {\large a_O } =  W_V\cdot {\large v  +  c} } $ , 

 where  $\mathbf{W_L, W_R, W_O}$  and  $\mathbf{W_V}$  are weights applied to respective input vectors and output vectors. 

It is Bootle et al. who first expressed _arithmetic circuit satisfiability_ in terms of the _Hadamard relation_ and linear constraints (see [10.]). In their definition, the above linear constraints is written without as:

  $\mathbf{W_L\cdot {\large a_L}  +  W_R\cdot {\large a_R}  +  W_O\cdot {\large a_O } =  c } $. 

That is, without the vector  $\mathbf{\large{v}}$  and its weight  $\mathbf{W_V} $. We explain this later.  



## ZK Proofs for Arithmetic Circuits _vs._ Programmable Constraint Systems for Bulletproofs 

As noted [here](https://tlu.tarilabs.com/cryptography/bulletproofs-protocols/MainReport.html#evolving-bulletproof-protocols), Interstellar's [_Programmable Constraint Systems for Bulletproofs_](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7) [11.] is an extension of _Zero-knowledge Proofs for Arithmetic Circuits_ by Bootle et al [10.], enabling protocols that support _proving of arbitrary statements_ in zero-knowledge using constraint systems. Although our focus here is on the above two works of research [10.] and [11.], we recognize the _Bulletproofs paper_ by Bunz et al [9.] as a _bridge_ between the two. We thus split the comparison to be among **three** works of research as shown in the table below. 

All these are _zero-knowledge proofs_ based on the difficulty of the _discrete logarithm problem_. 



| No.  | ZKProofs for Arithmetic Circuits by Bootle et al. [10.] (2016) | Bulletproofs & Constraints by Benedikt Buenz et al. [9.] (2017) | Programmable Constraints by Cathie Yun et al. [11.] (2018)   |
| ---- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| 1.   | Introduces the Hadamard relation and linear constraints.     | Turns the Hadamard relation and linear constraints into a single linear constraint. | Generalizes Constraint systems and uses what is called _gadgets_ as building blocks for Constraint systems. |
| 2    | Improves on Groth's work [[12.]](https://iacr.org/archive/crypto2009/56770190/56770190.pdf) on ZK Proofs. Reducing  a $\sqrt{N}$  complexity to  $6log_2(N) + 13$,  where  $N$  is the circuit size. | Improves on Bootle et al.'s work [10.]. Reducing a $2log_2(N) + 13$  complexity to  $6log_2(N) + 13$,  where  $N$  is the circuit size. | Adds Constraint systems to Bunz et al.'s work on Bulleproofs, which are _short proofs_ and the complexity advantage is seen in _proving_ several statements at once. |
| 3.   | Introduces logarithm-sized inner-product proofs zero-knowledge proofs. | Introduces Bulletproofs, extending proofs to proofs of arbitrary statements. The _halving method_ is used on the inner-products _resulting_ in the above reduction in complexity. | Introduces _Gadgets_ that are actually _add-ons_ to an ordinary zero-knowledge proofs. A _range proof_  is but an example of a gadget. |
| 4.   | Uses _Fiat-Shamir heuristics_ in order to achieve _non-interactive_ zero-knowledge proofs. | Bulletproofs have no trusted setup as they use of the _Fiat Shamir heuristics_ to achieve non-interactive zero-knowledge proofs. | The _Merlin transcripts_ are specifically used for a Fiat-Shamir transformation to achieve _non-interactive_ zero-knowledge proofs. |
| 5.   | The Pedersen Commitments are used in order to achieve _zero-knowledge_ property. | Eliminates the need for a _commitment algorithm_ by including _Pedersen commitments_ among the inputs to the verification proof. | Low-level variables, representing _inputs_ and _outputs_ to _multiplication gates_, are computed per-proof and committed using a single _vector Pedersen commitment_. |
| 6.   | The zero-knowledge proof involves _conversion_ of the Arithmetic circuit into a Rank-1 Constraint system. | The mathematical expression of a _Hadamard relation_ is closely related to an _Arithmetic circuit_. The use of this relation plus linear constraints as a single constraint amounts to using a constraint system. | Although _Arithmetic circuits_ are not explicitly used here, the Hadamard relation remains the same as first seen in Bulletproofs, moreso in the _inner-product proof_. |
| 7.   | The zero-knowledge proof here is for NP statements based on the _difficulty_ of _the discrete logarithm problem_. | As mentioned above, Bulletproofs extend zero-knowledge proofs (such as _range proofs_) to proofs on arbitrary statements. They are also described as _short non-Interactive proofs for Arithmetic Circuits without a Trusted Setup_. | Interstellar is building an API that allows developers to choose their own collection of gadgets suitable for the protocol they wish to develop. |

​				**Table 2 : Comparison of three Research Works Zero Knowledge Proofs**   





## Constraint Systems for ZK SNARKs 

<img src="/Users/anthonymatlala/Downloads/alex-pinto.png" alt="alex-pinto" style="zoom: 15%;" /> 

**Alex Pinto**

Developer and researcher in the blockchain space



#### Overview 

Now, how exactly does one go _**from an Arithmetic circuit to an R1CS**_ ? 

Eli Ben-Sasson et al. mentioned that the difference in computational power among the parties makes _succinct verification_ a requirement [8.]. Hence the pursuit of short and fast proofs systems. These make it possible for a verifier to check a non-deterministic polynomial-time computation in a much shorter time than the time required to run the computation when given a valid NP witness. They also state that the difficulty in studying the efficiency of proofs for NP statements is the _problem of representation_. Arithmetic circuits and the R1CS are some of such _representations_ of statements to be proved. 

#### Summary 

In his blogpost entitled ["Constraint Systems for ZK SNARKs"](http://coders-errand.com/constraint-systems-for-zk-snarks/), Alex Pinto wrote with the intention to elucidate the difficulties he came across while making "DIZK, a library written in JAVA, work with the same zkSNARK scheme as used by ZoKrates and libsnark, which are written in C++." He says the biggest obstacle was "proper encoding of the function that we want to prove statements for" [2.] So he focused specifically on how to express a computational problem in terms of an _Arithmetic circuit_, as well as how to _convert the Arithmetic circuit into an R1CS_. Although Alex Pinto covers the same concepts Vitalik Buterin covers in [5.] (see a summary of Buterin's article in [zkSNARKs](https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html#quadratic-arithmetic-programs---from-zero-to-hero)), Pinto gives some finer details. 

The _first step_ is to express the given computational problem in terms of _additions_ and _multiplications_ of only two terms. This involves introducing _auxilliary variables_ in order to express the given computational problem in terms of _equations_ with a single operation between two variables. That is, the result is but an Arithmetic circuit. The _second step_ is to encode the Arithmetic circuit as an R1CS. Pinto explains how this is achieved; He makes it easy to understand how the _solution vector_ as seen in Example 2 above is constructed, as well as how addition gates need to be expressed as some _multiplication gate_ in order to understand the resulting R1CS. Pinto uses an example inspired by RSA's well-known _difficulty of factorisation problem_; "Proving that one knows prime factors  ${\large p}$  and  ${\large q}$  such that  ${\large n + 1  =  (p + 3)(q + 2)}$."  





## Interstellar’s Constraint System 

The Interstellar team paved the way towards implementations of several cryptographic primitives in the RUST language, including [_Ristretto_](https://docs.rs/curve25519-dalek/0.15.1/curve25519_dalek/ristretto/index.html), a construction of a prime-order group using a cofactor-8 curve known as Curve25519. They reported on how they implemented Bulletproofs in Henry de Valence's article entitled "[Bulletproofs pre-release](https://medium.com/interstellar/bulletproofs-pre-release-fcb1feb36d4b)". An update on their progress in extending the [Bulletproofs implementation](http://github.com/dalek-cryptography/bulletproofs/) to a _constraint system_ API which enables zero-knowledge proofs of arbitrary statements was given in Cathie Yun's article, [Programmable Constraint Systems for Bulletproofs](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7) [11.] However, it was Bootle et al [9.] who first used the _Hadamard relation_ and _linear constraints_ which together form the _constraints system_ as formalised by the Interstellar team, and most of the Mathematical background of these _constraints_ and _bulletproofs_ is contained Bunz et al paper [9.] For an extensive study on Bulletproofs protocols see previous report [here](https://tlu.tarilabs.com/preface/learning/bulletproofs.html). 

Dalek's constraint system, as seen in Definition 3 above, is a collection of _arithmetic constraints_ of two types, _multiplicative constraints_ and _linear constraints_ over a set of high-level variables and low-level variables.  

As to why there is a vector  $\mathbf{\large{v}}$  and its weight  $\mathbf{W_V}$  in Definition 3 above, Bunz et al explain that "we include additional commitments  $V_i$  as part of our statement, and give a protocol for a more general relation, where the _linear consistency constraints_ include the openings  ${\large v_j}$  of the commitments  $V_j$", see Page24 [9.] That is, their definition of a _constraint system_ incorporates a secret vector  $\mathbf{\large{v}}$  and its weight  $\mathbf{W_V}$  because commitments  $V_i$  of components  ${\large v_i}$  of  $\mathbf{\large{v}} = \large{(v_1, v_2, ... , v_m )}$  are included among the inputs. We note that Bulletproofs use the _Pedersen commitment scheme_.  



#### Easy to Build Constraint Systems  

In this bulletproofs framework, a _prover_ can build a constraint system in two steps. Firstly, she _commits_ to secret inputs and _allocates_ high-level variables corresponding the inputs. Secondly, she _selects_ a suitable combination of _multiplicative constraints_ and _linear constraints_, as well as requesting a _random scalar_ in response to the high-level variables already committed. 

##### _About Gadgets_ 

Consider a _verifiable shuffle_: Given two lists of _committed values_  ${\large x_1, x_2, . . . , x_n}$  and  ${\large y_1, y_2, . . . , y_n} ,$  prove that the second list is a permutation of the first. Bunz et al mention that the use of bulletproofs improves the complexity of such a _verifiable shuffle_ to size $\mathcal{O}(log(n))$ compared to previous implementation results, see Page 5 of [9.]  Although not referred to as a _gadget_ in the paper, this is in fact a _shuffle gadget_. The term _gadget_ was used and popularised by the Interstellar team, who introduced _gadgets_ as building blocks of constraints systems, see Cathie Yun's article entitled [_Building on Bulletproofs_](https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8).  

A _shuffle gadget_ is any function whose outputs are but a permutation of its inputs. By definition of a permutation the _number of inputs_ to a shuffle gadget is always the same as the _number of outputs_. 



<img src="https://miro.medium.com/max/1772/0*PYz5oJnePHRxfZJi" alt="img" style="zoom: 67%;" />

​				**Diagram 3 : Simple Shuffle Gadgets with 2 inputs** [[13.]](Building on Bulletproofs, https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8)



The Interstellar team mentions other _gadgets_; “merge”, “split” and a “range proof”; that are implemented in their _Confidential Assets scheme_ called the _Cloak_. Just as a _shuffle gadget_ creates constraints that prove that _two sets of variables_ are equal up to a permutation, a _range proof gadget_ checks that a given _value_ is composed of a specific number of bits. 

Gadgets in their simplest form merely receive some _variables_ as inputs and produce corresponding _output_ values. They however may _allocate_ more _variables_ for internal use, sometimes called _auxilliary variables_, and _produce constraints_ involving all these variables. The main advantage with gadgets is that they are _composable_, and thus a more _complex_ gadget can always be created from a number of _single_ gadgets. 

Cathie Yun regards _constraint systems_ as "_very powerful_ because they can represent any efficiently verifiable program," [[13.]](Cathie Yun, "Building on Bulletproofs," https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8).   

##### Previous Work on _Verifiable Shuffles_

It would be interesting to see how some of the applications of _verifiable shuffles_ mentioned in [9.]; _voting_, _mix-nets_, and _solvency proofs_; could be enhanced by the Interstellar's Bulletproofs framework. The _first_ and _third_ seem more relevant to _cryptocurrencies_ and _confidential asset schemes_. 





## R1CS in two contexts; zkSNARKs and Bulletproofs 



(reference the examples, … [Vitalik’s e.g.](https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html#quadratic-arithmetic-programs---from-zero-to-hero)  & [Lovesh’s Rust e.g.](https://github.com/lovesh/bulletproofs-r1cs-gadgets/blob/master/src/gadget_bound_check.rs) or the _shuffle_ e.g.  



Example by Lovesh ... shuffle Rust example https://github.com/lovesh/bulletproofs/blob/e477511a20bdb8de8f4fa82cb789ba71cc66afd8/docs/r1cs-docs-example.md



Bellman's RUST example of a zkSNARK circuit [here](https://docs.rs/bellman/0.2.0/bellman/#example-circuit). 







## Implementations of R1CS, 



1. Write a summary of this infographic; 

Medio Demarco and Anil Lulla, "Zero-Knowledge Proofs : Privacy & Scaling - Thematic Insights," Delphi Digital, April 2019.	See Delphi Digital's article [[y.]](Zero-Knowledge Proofs: Privacy & Scaling Thematic Insights, Delphi Digital, April, 2019) for a sypnosis on implementations of, and some historical facts on, zkSNARKs, zkSTARKs, and Bulletproofs. The bulletproofs paper, by Benedikt Bunz et al., can actually be seen as part of that great research work on Zero-knowledge proofs. 

2. Write a summary, ... who did what (e.g. as in https://zkp.science/)

DIZK, PINNOCHIO, ... 

CLOAK is a Confidential Assets scheme built using Bulletproofs ... The Cloak was inspired by the Bellman's API ... see [11.]  

ZkVM aim is to create a smart contracts language that allows for confidentiality.  







## References 

[1.]  Amir Shpilka and Amir Yehudayoff, "Arithmetic Circuits: a survey of recent results and open questions," Technion-Israel Institute of Technology, Haifa, Israel, 2010.   

[2.] Alex Pinto, "Constraint Systems for ZK SNARKs," 2019-03-06, http://coders-errand.com/constraint-systems-for-zk-snarks/ 

[3.] Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer and Madars Virza, "SNARKs for C : Verifying Program Executions Succinctly and in Zero Knowledge (extended version)," October 7, 2013. 

[4.] Tari Labs University, "zkSNARKs," https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html 

[5.] Vitalik Buterin, "Quadratic Arithmetic Programs: from Zero to Hero," Dec 12, 2016. https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649 

[6.] Howard Wu, Wenting Zheng, Alessandro Chiesa, Raluca Ada Popa, and Ion Stoica, "DIZK: A Distributed Zero Knowledge Proof System," Proceedings of the 27th USENIX Security Symposium, August 15–17, 2018. https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf  

[7.] [[x.]] Alan Szepieniec and Bart Preneel, "Generic Zero-Knowledge and Multivariate Quadratic Systems".  

[8.]  Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer and Madars Virza, "SNARKs for C : Verifying Program Executions Succinctly and in Zero Knowledge (extended version)," October, 2013. 

[9.]  Benedikt Buenz, Jonathan Bootle, Dan Boneh, Andrew Poelstra, Pieter Wuille and Greg Maxwell; "Bulletproofs: Short Proofs for Confidential Transactions and More," 2017.  

[10.]  Jonathan Bootle, Andrea Cerulli, Pyrros Chaidos, Jens Groth, and Christophe Petit; "Efficient Zero-Knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting," 2016. 

[11.] Cathie Yun, "Programmable Constraint Systems for Bulletproofs," https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7  

[12.]  Jens Groth, "Linear algebra with sub-linear zero-knowledge arguments," Advances in Cryptology – CRYPTO 2009, pages 192–208, 2009. https://iacr.org/archive/crypto2009/56770190/56770190.pdf 

[13.] Cathie Yun, "Building on Bulletproofs," https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8)  

[14.]  



 



