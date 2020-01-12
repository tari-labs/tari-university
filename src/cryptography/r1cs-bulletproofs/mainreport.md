


# **Rank-1 Constraint System with Application to Bulletproofs**



- [Introduction](introduction) 
- [Arithmetic Circuits](arithmetic-circuits) 
- [R1CS](r1cs) 
- [ZK Proofs for Arithmetic Circuits _vs._ Programmable Constraint Systems for Bulletproofs](zkproofs-for-arithmetic-circuits-vs-programmable-constraint-systems-for-bulletproofs)
- [Constraint Systems for ZK SNARKs](constraint-systems-for-zksnarks) 
- [Interstellar’s Constraint System](interstellars-constraint-system) 
  - [Easy to Build Constraint Systems](easy-to-build-constraint-system) 
    -  [About Gadgets](about-gadgets) 
    -  [Interstellar's Concluding Remarks](interstellar-concluding-remarks) 
    -  [Previous Work on Verifiable Shuffles](previous-work-on-verifiable-shuffles) 
- [R1CS: zkSNARKs vs. Bulletproofs](r1cs-zksnarks-vs-bulletproofs)
- [Zero-Knowledge Proofs : Privacy & Scaling](zero-knowledge-proofs-privacy-&-scaling)
- [Implementations of R1CS](implementations-of-r1cs) 
- [References](references) 





## Introduction 

The main focus of this report is to elucidate technical underpinnings of Rank-1 Constraints Systems or R1CS as applied to zkSNARKs and Bulletproofs. 

When reading the literature on the use of R1CS in zkSNARKs, this mathematical tool is used simply as one part of many in a complex process towards achieving a zkSNARK-proof. In fact, not much attention is given to it, not even in explaining what "rank-1" actually means. Although the terminology has a similar notion as the traditional _rank of a matrix_ in Linear Algebra, examples given on the internet do not yield a _reduced matrix with only one non-zero row or column_. 

R1CS became more prominent due to research work done by Cathie Yun and her colleagues at Interstellar. The _Constraint systems_, or in particular _Rank-1 Constraint Systems_, are used as an _add-on_ to Bulletproofs protocols. The title of Cathie Yun's article, "[Building on Bulletproofs](https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8)" [[1.]] suggests this to be true. One of the Interstellar team's goals is to use the constraints system in their _Confidential Asset Protocol_ called the CLOAK and the envisaged [_Spacesuit_](https://github.com/interstellar/spacesuit). Despite their work on using R1CS being _research-in-progress_, their detailed notes on _constraints systems_ and their implementation in RUST are made available [here](https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html#constraint-system). 

Most recent constructions of zkSNARKs and Bulletproofs involve _arithmetic circuits_ and _rank-1 constraint systems_ either explicitly or implicitly.  

In this report we intend to; 

- highlight the connection between _Arithmetic circuits_ and R1CS, 
- clarify the difference R1CS makes in bulletproofs compared to range proofs, 
- the link between R1CS in _zkSNARK_ and R1CS in _Bulletproofs_.   

We refer the technical reader [here](https://tlu.tarilabs.com/cryptography/bulletproofs-protocols/MainReport.html#notation-used) for proper notations as used in the Bulletproofs Protocols [report](https://tlu.tarilabs.com/cryptography/bulletproofs-protocols/MainReport.html#the-bulletproof-protocols). 

And talking about Bulletproofs, we would like to understand what Bunz et al. meant by ... "We note that a range proof using the protocol of [[BCC+16]](Bootle et al's paper [[4.]]) would have required implementing the commitment opening algorithm as part of the verification circuit, which we are able to eliminate."  







## Arithmetic Circuits  

Arithmetic circuits are said to be the most natural and standard model for computing polynomials. In fact, every function  ${\large \mathcal{H} : \{ 0 , 1 \}^n → \{ 0 , 1 \}^m} $  of fixed input and output _length_ can be represented as an arithmetic circuit over any finite field  $ \mathcal{F}_p$ ,  [[2.]]. 

According to Amir Shpilka and Amir Yehudayoff [[3.]], arithmetic circuits are a highly structured model of computation compared to Boolean circuits. For instance, when studying arithmetic circuits one is interested in _syntactic_ computation of polynomials, whereas in studying Boolean circuits one is interested in the _semantics_ of the computation. In other words, in the Boolean case one is not interested in any specific _polynomial representation_ of the function but rather one just wants to compute some representation of it, while in the arithmetic world one focuses on a _specific representation_ of the function. 

For our purposes, arithmetic circuits have as inputs variables  $\large{ x_1, ... , x_n }$, and computations are performed using the arithmetic operations  $ + $  and  $ × $, and may involve constants from a field  $\mathcal{F}_p$. Complexity measures associated with such circuits are _size_ and _depth_, which capture _the number of gates_ in a circuit and _the maximal distance between an input and an output_, respectively [[3.]].



### Definition 1 (Arithmetic circuits) [[3.]]

An ***arithmetic circuit***  $\mathcal{A}$  over the field  $\mathcal{F}$  and the set of variables  $X = \{ \large{x_1,...,x_n} \}$  is a _directed acyclic graph_ such that the vertices of  $\mathcal{A}$  are called _gates_, while the edges are called _wires_. 

(a)  A gate is said to be of _in-degree_  $l$  if it has  $l$  incoming wires, and similarly, of _out-degree_  $k$  if it has  $k$  out going wires. 

(b)  Every gate in  $\mathcal{A}$  of _in-degree_  0  is labelled by either a variable  $\large{x_i}$  or some field element from  $\mathcal{F}$ , and such a gate is called an _input gate_. Every gate of _out-degree_ 0 is called an _output gate_. 

(c)  Every other gate in  $\mathcal{A}$  is labeled by either  ${\Large{\otimes}}$  or  $\Large{\oplus}$ , and called a _multiplication gate_ or _addition gate_, respectively. We are interested in _binary_ circuits, that is, circuits where each gate has two _input wires_ and one _output wire_. 

(d)  An _arithmetic circuit_ is called a _formula_ if it is a _directed tree_ whose edges are _directed_ from the leaves to the root. 

(e)  The size of  $\mathcal{A}$ , denoted |$\mathcal{A}$|, is the number of _wires_ (i.e. _edges_) in  $\mathcal{A}$. 



A typical multiplication gate is thought of as having a _left input_  $\large{a_L}$, a _right input_  $\large{a_R}$, and an _output_  $\large{a_O}$. Also, we note that,  $$ \large{a_L \cdot a_R  -  a_O  =  0} .$$ 

<img src="sources/basic-multiplication-gate.png" alt="basic-multiplication-gate" style="zoom:67%;" />



We note that in cases where the inputs and outputs are all _vectors_ of  ${\large n}$  components, i.e., 

​	${\large \mathbf{a_L} = ( a_{L, 1}, a_{L, 2} , ... , a_{L, n}) }$, $ {\large \mathbf{a_R} = ( a_{R, 1}, a_{R, 2} , ... , a_{R, n}) } $, ${\large \mathbf{a_O} = ( a_{O, 1}, a_{O, 2} , ... , a_{O, n}) } ,$ 

then _multiplication_ between  ${\large \mathbf{a_L}}$  and  ${\large \mathbf{a_R} }$  is defined as an _entry-wise_ product called the _**Hadamard product**_; 

 ${\large \mathbf{a_L}\circ \mathbf{a_R} = (( a_{L, 1} \cdot a_{R, 1} ) , ( a_{L, 2}  \cdot a_{R, 2} ) , ... , ( a_{L, n} \cdot a_{R, n} ) )}   =   \large \mathbf{a_O}  .$

The equation  ${\large \mathbf{a_L}\circ \mathbf{a_R} =  \mathbf{a_O} }$  is referred to as a _multiplicative constraint_ but also known as the _Hadamard relation_, [[4.]]. 



An arithmetic circuit computes a polynomial in a natural way, as seen in the example below. 

###Example 1 

Consider the following Arithmetic circuit  $\mathcal{A}$  with inputs  $\{ {\large{ x_1 , x_2 , 1 }} \}$  over some field  $\mathcal{F}$ , 

<img src="sources/polynomial-eg-ac.png" alt="polynomial-eg-ac" style="zoom:67%;" />

The output of  $\mathcal{A}$  above is the polynomial   $\large{x^2_1 x_2 + x_1 + 1 }$  of _total degree_ three. 

A typical _computational problem_ would involve finding the solution to, let's say,  $\large{x^2_1 x_2 + x_1 + 1 = 22}$.  Or, in a proof of knowledge scenario, the prover has to prove to the verifier that he has the correct solution to such an equation. 

Following the wires, in Example 1.2, shows that an arithmetic circuit actually _breaks down_ the given computation into _smaller equations_ corresponding to each _gate_; 

​				$\large{ u = x_1*x_1} $ ,   $ \large{ v = u*x_2 }$ ,   $ \large{ y = x_1 + 1 } $ , and  $ \large{ z = v + y } $ . 

The variables  $ \large{ u , v }$  and $ \large{ y }$  are called _auxiliary variables_ or _low level variables_, while  $\large{ z }$  is the _output_ of  $ \mathcal{A} $.  Thus, in addition to computing polynomials naturally, an arithmetic circuit helps in reducing a computation to a _low level_ language involving only _two_ variables, _one operation_, and an _output_. 

Although "Arithmetic circuits are a highly structured model of computation compared to Boolean circuits," according to Amir Shpilka and Amir Yehudayoff [[3.]], "we do not know how to efficiently reconstruct a circuit using only queries to the polynomial it computes." 

Zero-knowledge proofs, such as zkSNARKs and Bulletproofs, require _statements to be proved_ be expressed in their simplest terms for efficiency. Alex Pinto [[5.]] mentions that "the ZK SNARK end-to-end journey is to create a _function_ to write proofs about," but they "must work with specific constructs." That is why arithmetic circuits are so important in making zero-knowledge more efficient, "these _functions_ have to be specified as sequences of very simple terms, namely, additions and multiplications of only two terms in a particular field" [[5.]].

In verifying a zkSNARK proof, the verifier needs to carry out a step-by-step check of the computations, that is, for each gate the verifier has to check if the _output_  $ \large{ a_O }$  is correct with respect to the given _inputs_  $\large{ a_L }$  and  $ \large{ a_R } $. Testing if   $ \large{a_L \cdot a_R  -  a_O  =  0}$ ,  for each multiplication gate. This requires that an _addition gate_ be treated as some form of a _multiplication gate_, we explain this later in this report. 







## R1CS 

Although one typically expresses a computation problem in terms of a high-level programming language, a zkSNARK requires expressing it in terms of a set of _quadratic constraints_, which is closely related to circuits of logical gates. 

R1CS is short for _Rank-1 Constraint System_. These type of constraint systems have been featured in several constructions of zkSNARKs and were at times simply referred to as _quadratic constraints_ or _quadratic equations_, see [[6.]], [[7.]] and [[8.]]. 

Here's a simplified definition of an R1CS as it applies to zkSNARKs by Vitalik Buterin, 

 ### Definition 2 (R1CS) [[9.]]   

An R1CS is a sequence of groups of three vectors  ${\large \bf{a_L}}, {\large \bf{a_R}}, {\large \bf{a_O}} ,$  and the solution to an R1CS is a vector  ${\large \bf{s}}$  that satisfies the equation 

​					${\large \langle {\bf{a_L , s}} \rangle * \langle {\bf{a_R , s }} \rangle - \langle {\bf{a_O , s }} \rangle = 0 }$. 




### Example 2 

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



${\large \bf{A_L}} = {\bf{\begin{bmatrix} 0&1&0&0&0&0&0 <br/>\\ 0&0&0&0&1&0&0 <br/>\\ 1&1&0&0&0&0&0 <br/>\\ 0&0&0&0&0&1&1\\ \end{bmatrix}}},$ ${\large \bf{A_R}} = {\bf{\begin{bmatrix} 0&1&0&0&0&0&0 <br/>\\ 0&1&0&0&0&0&0 <br/>\\ 1&0&0&0&0&0&0 <br/>\\ 1&0&0&0&0&0&0 \\ \end{bmatrix}}}$ , ${\large \bf{ A_O }} = {\bf{  \begin{bmatrix} 0&0&0&0&1&0&0 <br/>\\ 0&0&0&0&0&1&0 <br/>\\ 0&0&0&0&0&0&1 <br/>\\ 0&0&0&1&0&0&0\\ \end{bmatrix}}}.$



We observe that   ${\large \bf{ (A_L\cdot s^T) * (A_R\cdot s^T ) - (A_O\cdot s^T)} = 0 }$ ,  where  "${\bf{ \cdot }}$"  is _matrix multiplication_ and  ${\large \bf s^T}$  is the transpose of the solution vector  ${\large \bf{s}}$ .    

A more general definition of a Constraint System was given by the Dalek team in [[10.]], but originally defined by Bootle et al. [[4.]].   

### Definition 3 (Constraint System) [[10.]]

A **constraint system** is a collection of arithmetic constraints over a set of variables. There are two kinds of variables in the constraint system:

(a)   $\large{m}$  **high-level** variables, the input secrets  ${\large \mathbf{v}}$,  

(b)   $\large n$  **low-level** variables, the internal input vectors  ${\large \mathbf{a}_L}$ and  ${\large \mathbf{a}_R},$  and output vectors  ${\large{ \mathbf{a}_O }} $  of the multiplication gates. 

Specifically, a **rank-1 constraint system** or **R1CS** is a system that consists of two sets of constraints; 

​	  ${\large n}$   _multiplicative constraints_,   ${\large \mathbf{ a_L \circ a_R = a_O }} $ ,  

where  "${\large \circ }$"  is the Hadamard product, and

​	  ${\large q}$    _linear constraints_,   $\mathbf{W_L\cdot {\large a_L}  +  W_R\cdot {\large a_R}  +  W_O\cdot {\large a_O } =  W_V\cdot {\large v  +  c} } $ , 

 where  $\mathbf{W_L, W_R, W_O}$  and  $\mathbf{W_V}$  are weights applied to respective input vectors and output vectors. 

Note that it is Bootle et al. who first expressed _arithmetic circuit satisfiability_ in terms of the _Hadamard relation_ and linear constraints (see [[10.]]). In their definition, the above linear constraints are written as:

  $\mathbf{W_L\cdot {\large a_L}  +  W_R\cdot {\large a_R}  +  W_O\cdot {\large a_O } =  c } $. 

That is, without the vector  $\mathbf{\large{v}}$  and its weight  $\mathbf{W_V} $. We explain this later.  







## ZK Proofs for Arithmetic Circuits _vs._ Programmable Constraint Systems for Bulletproofs 

As noted in previous reports [here](https://tlu.tarilabs.com/cryptography/bulletproofs-protocols/MainReport.html#evolving-bulletproof-protocols), Interstellar's [_Programmable Constraint Systems for Bulletproofs_](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7) [[11.]] is an extension of _Zero-knowledge Proofs for Arithmetic Circuits_ by Bootle et al [[4.]], enabling protocols that support _proving of arbitrary statements_ in zero-knowledge using constraint systems. Although our focus here is on the above two works of research [[4.]] and [[11.]], we recognize the _Bulletproofs paper_ by Bunz et al [[12.]] as a _bridge_ between the two. We thus split the comparison to be among **three** works of research as shown in the table below. 

All these are _zero-knowledge proofs_ based on the difficulty of the _discrete logarithm problem_. 



| No.  | ZKProofs for Arithmetic Circuits by Bootle et al. [[4.]] (2016) | Bulletproofs & Constraints by Benedikt Buenz et al. [[12.]] (2017) | Programmable Constraints by Cathie Yun et al. [[11.]] (2018) |
| ---- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| 1.   | Introduces the Hadamard relation and linear constraints.     | Turns the Hadamard relation and linear constraints into a single linear constraint, and these are in fact the R1CS. | Generalizes Constraint systems and uses what is called _gadgets_ as building blocks for Constraint systems. |
| 2    | Improves on Groth's work [[17.]] on ZK Proofs. Reducing  a $\sqrt{N}$  complexity to  $6log_2(N) + 13$,  where  $N$  is the circuit size. | Improves on Bootle et al.'s work [[4.]]. Reducing a $2log_2(N) + 13$  complexity to  $6log_2(N) + 13$,  where  $N$  is the circuit size. | Adds Constraint systems to Bunz et al.'s work on Bulleproofs, which are _short proofs_ and the complexity advantage is seen in _proving_ several statements at once. |
| 3.   | Introduces logarithm-sized inner-product proofs zero-knowledge proofs. | Introduces Bulletproofs, extending proofs to proofs of arbitrary statements. The _halving method_ is used on the inner-products _resulting_ in the above reduction in complexity. | Introduces _Gadgets_ that are actually _add-ons_ to an ordinary zero-knowledge proofs. A _range proof_  is but an example of a gadget. |
| 4.   | Uses _Fiat-Shamir heuristics_ in order to achieve _non-interactive_ zero-knowledge proofs. | Bulletproofs have no trusted setup as they use of the _Fiat Shamir heuristics_ to achieve non-interactive zero-knowledge proofs. | _Merlin transcripts_ are specifically used for a Fiat-Shamir transformation to achieve _non-interactive_ zero-knowledge proofs. |
| 5.   | The Pedersen Commitments are used in order to achieve _zero-knowledge_ property. | Eliminates the need for a _commitment algorithm_ by including _Pedersen commitments_ among the inputs to the verification proof. | Low-level variables, representing _inputs_ and _outputs_ to _multiplication gates_, are computed per-proof and committed using a single _vector Pedersen commitment_. |
| 6.   | The zero-knowledge proof involves _conversion_ of the Arithmetic circuit into a Rank-1 Constraint system. | The mathematical expression of a _Hadamard relation_ is closely related to an _Arithmetic circuit_. The use of this relation plus linear constraints as a single constraint amounts to using a constraint system. | Although _Arithmetic circuits_ are not explicitly used here, the Hadamard relation remains the same as first seen in Bulletproofs, moreso in the _inner-product proof_. |
| 7.   | The zero-knowledge proof here is for NP statements based on the _difficulty_ of _the discrete logarithm problem_. | As mentioned above, Bulletproofs extend zero-knowledge proofs (such as _range proofs_) to proofs on arbitrary statements. They are also described as _short non-Interactive proofs for Arithmetic Circuits without a Trusted Setup_. | Interstellar is building an API that allows developers to choose their own collection of gadgets suitable for the protocol they wish to develop. |

​				**Table 2 : Comparison of three Research Works Zero Knowledge Proofs**   







## Constraint Systems for ZK SNARKs 

<img src="sources/alex-pinto.png" alt="alex-pinto" style="zoom: 15%;" /> 

**Alex Pinto**

Developer and researcher in the blockchain space



#### Overview 

Now, how exactly does one go _**from an Arithmetic circuit to an R1CS**_ ? 

Eli Ben-Sasson et al. mentioned that the difference in computational power among the parties makes _succinct verification_ a requirement [[8.]]. Hence the pursuit of short and fast proofs systems. These make it possible for a verifier to check a non-deterministic polynomial-time computation in a much shorter time than the time required to run the computation when given a valid NP witness. They also state that the difficulty in studying the efficiency of proofs for NP statements is the _problem of representation_. Arithmetic circuits and the R1CS are some of such _representations_ of statements to be proved. 

#### Summary 

In his blogpost entitled "_Constraint Systems for ZK SNARKs_" found [here](http://coders-errand.com/constraint-systems-for-zk-snarks/), Alex Pinto wrote with the intention to elucidate the difficulties he came across while making "DIZK, a library written in JAVA, work with the same zkSNARK scheme as used by ZoKrates and libsnark, which are written in C++." He says the biggest obstacle was "proper encoding of the function that we want to prove statements for." So he focused specifically on how to express a computational problem in terms of an _Arithmetic circuit_, as well as how to _convert the Arithmetic circuit into an R1CS_. Although Alex Pinto covers the same concepts Vitalik Buterin covers in [[9.]] (see a summary of Buterin's article in [zkSNARKs](https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html#quadratic-arithmetic-programs---from-zero-to-hero) [[13.]]), Pinto gives some finer details. 

The _first step_ is to express the given computational problem in terms of _additions_ and _multiplications_ of only two terms. This involves introducing _auxilliary variables_ in order to express the given computational problem in terms of _equations_ with a single operation between two variables. That is, the result is but an Arithmetic circuit. The _second step_ is to encode the Arithmetic circuit as an R1CS. Pinto explains how this is achieved; He makes it easy to understand how the _solution vector_ as seen in Example 2 above is constructed, as well as how addition gates need to be expressed as some _multiplication gate_ in order to understand the resulting R1CS. Pinto uses an example inspired by RSA's well-known _difficulty of factorisation problem_; "Proving that one knows prime factors  ${\large p}$  and  ${\large q}$  such that  ${\large n + 1  =  (p + 3)(q + 2)}$."  







## Interstellar’s Constraint System 

The Interstellar team paved the way towards implementations of several cryptographic primitives in the RUST language, including [_Ristretto_](https://docs.rs/curve25519-dalek/0.15.1/curve25519_dalek/ristretto/index.html), a construction of a prime-order group using a cofactor-8 curve known as Curve25519. They reported on how they implemented Bulletproofs in Henry de Valence's article entitled "[Bulletproofs pre-release](https://medium.com/interstellar/bulletproofs-pre-release-fcb1feb36d4b)". An update on their progress in extending the [Bulletproofs implementation](http://github.com/dalek-cryptography/bulletproofs/) to a _constraint system_ API which enables zero-knowledge proofs of arbitrary statements was given in Cathie Yun's article, [Programmable Constraint Systems for Bulletproofs](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7) [[11.]] However, it was Bootle et al [[4.]] who first used the _Hadamard relation_ and _linear constraints_ which together form the _constraints system_ as formalised by the Interstellar team, and most of the Mathematical background of these _constraints_ and _bulletproofs_ is contained Bunz et al.'s paper [[12.]]. For an extensive study on Bulletproofs protocols see previous report [here](https://tlu.tarilabs.com/preface/learning/bulletproofs.html). 

Dalek's constraint system, as seen in Definition 3 above, is a collection of _arithmetic constraints_ of two types, _multiplicative constraints_ and _linear constraints_ over a set of high-level variables and low-level variables.  

As to why there is a vector  $\mathbf{\large{v}}$  and its weight  $\mathbf{W_V}$  in Definition 3 above, Bunz et al explain that "we include additional commitments  $V_i$  as part of our statement, and give a protocol for a more general relation, where the _linear consistency constraints_ include the openings  ${\large v_j}$  of the commitments  $V_j$", see Page 24 of [[10.]]. That is, their definition of a _constraint system_ incorporates a secret vector  $\mathbf{\large{v}}$  and its weight  $\mathbf{W_V}$  because commitments  $V_i$  of components  ${\large v_i}$  of  $\mathbf{\large{v}} = \large{(v_1, v_2, ... , v_m )}$  are included among the inputs. We note that Bulletproofs use the _Pedersen commitment scheme_.  



#### Easy to Build Constraint Systems  

In this bulletproofs framework, a _prover_ can build a constraint system in two steps. Firstly, she _commits_ to secret inputs and _allocates_ high-level variables corresponding the inputs. Secondly, she _selects_ a suitable combination of _multiplicative constraints_ and _linear constraints_, as well as requesting a _random scalar_ in response to the high-level variables already committed [[14.]]. 

Lovesh Harchandani in the article "[Zero knowledge proofs using Bulletproofs](https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82)" [[15.]] captures an excellent outline of _zero-knowledge proofs_ that use _Bulletproofs_;  

(a)  The prover _commits_ to values that he wants to prove knowledge of.  

(b)  The prover _generates_ the _proof_ by enforcing the _constraints over the committed values_ and any additional _public values_. The constraints might require the prover to commit to some _additional variables_. 

(c)  Prover _sends_ the verifier _all the commitments_ he made in step 1 and step 2 along with the _proof_ from step 2. 

(d)  The verifier now _verifies the proof_ by enforcing the _same constraints_ over _the commitments_ plus any public values.



##### _About Gadgets_ 

Consider a _verifiable shuffle_: Given two lists of _committed values_  ${\large x_1, x_2, . . . , x_n}$  and  ${\large y_1, y_2, . . . , y_n} ,$  prove that the second list is a permutation of the first. Bunz et al mention that the use of bulletproofs improves the complexity of such a _verifiable shuffle_ to size $\mathcal{O}(log(n))$ compared to previous implementation results, see Page 5 of [[9.]]  Although not referred to as a _gadget_ in the paper, this is in fact a _shuffle gadget_. The term _gadget_ was used and popularised by the Interstellar team, who introduced _gadgets_ as building blocks of constraints systems, see Cathie Yun's article entitled [_Building on Bulletproofs_](https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8).  

A _shuffle gadget_ is any function whose outputs are but a permutation of its inputs. By definition of a permutation the _number of inputs_ to a shuffle gadget is always the same as the _number of outputs_. 



<img src="sources/simple-shuffle-diagram.png" style="zoom:67%;" />

​				**Diagram 3 : Simple Shuffle Gadgets with 2 inputs** [[1.]]



Find a RUST example code for a _shuffle gadget_ in the _Bulletproofs Constraint System_ framework [here](https://github.com/lovesh/bulletproofs/blob/e477511a20bdb8de8f4fa82cb789ba71cc66afd8/docs/r1cs-docs-example.md), written  by Lovesh Harchandani. 

The Interstellar team mentions other _gadgets_; “merge”, “split” and a “range proof”; that are implemented in their _Confidential Assets scheme_ called the _Cloak_. Just as a _shuffle gadget_ creates constraints that prove that _two sets of variables_ are equal up to a permutation, a _range proof gadget_ checks that a given _value_ is in the interval  ${\large [0, 2^n]}$  where  ${\large n}$  is the size of input vector [[14.]]. 

Gadgets in their simplest form merely receive some _variables_ as inputs and produce corresponding _output_ values. They however may _allocate_ more _variables_ for internal use, sometimes called _auxilliary variables_, and _produce constraints_ involving all these variables. The main advantage with gadgets is that they are _composable_, and thus a more _complex_ gadget can always be created from a number of _single_ gadgets. Interstellar's Bulletproofs API allows developers to choose their own collection of gadgets suitable for the protocol they wish to develop. 



##### Interstellar's _Concluding Remarks_ 

Cathie Yun reported in [[11.]] that their "_work on Cloak and Spacesuit is far from complete_" and mentions that they still have two more goals to achieve; 

Firstly, in order to "_ensure that challenge-based variables cannot be inspected_" and _prevent_ the user from accidentally breaking soundness of their gadgets, the Bulletproofs protocol need to be slightly extended, enabling it to commit "to a portion of low-level variables using a single vector Pedersen commitment without an overhead of additional individual high-level Pedersen commitments" [[11.]].  

Secondly, to "_improve privacy in Cloak"_ by enable "_multi-party proving of a single constraint system_". That is, "_building a joint proof of a single constraint system by multiple parties_, without sharing their secret inputs with each other."

All-in-all Cathie Yun regards _constraint systems_ as "_very powerful_ because they can represent any efficiently verifiable program," [[13.]].  



##### Previous Work on _Verifiable Shuffles_

It would be interesting to see how some of the applications of _verifiable shuffles_ mentioned in [[9.]]; _voting_, _mix-nets_, and _solvency proofs_; could be enhanced by frameworks similar to the Interstellar's Bulletproofs. The _first_ and _third_ applications seem more relevant to _cryptocurrencies_ and _confidential asset schemes_. 







## R1CS: zkSNARKs vs. Bulletproofs 

In the context of comparing  zkSNARKs and Bulletproofs, Lovesh Harchandani in [Zero knowledge proofs using Bulletproofs](https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82) [[15.]] mentions the two main disadvantages of zkSNARKs; 

- First is the presence of a _trusted setup_ used for a one-time generation of protocol parameters (for a so-called _common reference string_ (CRS)). The problem with such a setup is that the generated parameters (or the CRS) could be used to leak some secrets. One way, however, to avoid such possible leakage of secrets is to use a multi-party computation (MPC) as utilised in crypto-currencies like Zcash. 

- The second disadvantage of zkSNARKs is that a new _trusted setup_ (i.e., a fresh CRS) is needed for each circuit simply because each new computational challenge will have different constraints. 

The Bulletproofs constraint system, on the other hand, has no trusted setup. 

In the same article [[15.]], Lovesh Harchandani explores the Dalek's Bulletproofs API with various examples. Having looked at an R1CS _zkSNARK_ example by Alex Pinto above, where he looked at a _factorisation problem_, we now look at the counterpart R1CS _Bulletproof_ example. That is, the computational challenge is "_prove knowledge of factors_  p  _and_  q  _of a given number_  r  _without revealing the factors_." It is one of six examples discussed in the article [[15.]]. 

Here's an _outline of the description and the code lines_ of the example in tabular form. Lovesh Harchandani's complete code of this example can be found [here](https://github.com/lovesh/bulletproofs/blob/e477511a20bdb8de8f4fa82cb789ba71cc66afd8/tests/basic_r1cs.rs#L17). 

|      | Description                                                  | Code Lines                                                   |
| ---- | :----------------------------------------------------------- | ------------------------------------------------------------ |
| 1.   | Create _two_ pairs of generators; one pair <br/>for the Pedersen commitments and <br/>the other for the Bulletproof. | `let pc_gens = PedersenGens::default();`<br/>`let bp_gens = BulletproofGens::new(128, 1);` |
| 2.   | Instantiate the prover using the commitment<br/> and Bulletproofs generators of Step 1, to <br/>produce the prover's transcript. | `let mut prover_transcript = Transcript::new(b"Factors");`<br/>`let mut prover = Prover::new(&bp_gens, &pc_gens, &mut prover_transcript);` |
| 3.   | Prover commits to variables using <br/>the Pedersen commitments,<br/>creates variables corresponding<br/>to each commitment, and adds<br/>the variables to the transcript. | `let x1 = Scalar::random(&mut rng);`<br/>`let (com_p, var_p) = prover.commit(p.into(), x1);`<br/>`let x2 = Scalar::random(&mut rng);`<br/>`let (com_q, var_q) = prover.commit(q.into(), x2);` |
| 4.   | The prover _constrains_ the variables in _two_ steps;<br/><br/>a) Prover _multiplies_ the variables of <br/>step 3 and captures the product in <br/>the "output" variable O, <br/>b) Prover wants to ensure the <br/>_difference of_ the product  O  and  r  <br/> is zero. | `let (_, _, o) =  prover.multiply(var_p.into(), var_q.into());`<br/> <br/>`let r_lc: LinearCombination = vec![(Variable::One(),      r.into())].iter().collect();`<br/>`prover.constrain(o -  r_lc);` |
| 5.   | Prover creates the proof.                                    | `let proof = prover.prove().unwrap();`                       |
| 6.   | Instantiation of the Verifier using the Pedersen<br/> commitments and Bulletproof generators, <br/>and creates its own transcript. | `let mut verifier_transcript = Transcript::new(b"Factors");`<br/>`let mut verifier = Verifier::new(&bp_gens, &pc_gens, &mut verifier_transcript);` |
| 7.   | Verifier _records_ commitments<br/>for p and q sent by prover in<br/>the transcript, and creates variables for them similar to the prover's. | `let var_p = verifier.commit(commitments.0);`<br/>`let var_q = verifier.commit(commitments.1);` |
| 8.   | Verifier _constrains_ variables corresponding to the<br/>commitments. | `let (_, _, o) =  verifier.multiply(var_p.into(), var_q.into());`<br/>`let r_lc: LinearCombination = vec![(Variable::One(), r.into())].iter().collect();`<br/>`verifier.constrain(o -  r_lc);` |
| 9.   | Verifier _verifies_ the _proof_.                             | `verifier.verify(&proof)`                                    |

​			**Table 3 : Bulletproof Constraint Example**







## Zero-Knowledge Proofs: Privacy & Scaling

<img src="sources/mediodemarco1.png" alt="mediodemarco1" style="zoom:67%;" />  <img src="sources/anillulla.png" alt="anillulla" style="zoom: 25%;" />

Medio Demarco (Principal at Delphi Digital) and Anil Lulla (co-founder of Delphi Digital) 



#### Summary 

"_Zero-Knowledge Proofs: Privacy & Scaling_" is a synoptic infographic that contains _quick-to-read_ information on zkSNARKs and Bulletproofs [[16.]]. It includes a summative explanation on what ZK proofs are, takes a glance at four main ZK Proofs (zkSNARKs, zkSTARKs, Bulletproofs and Mimblewimble, and the AZTEC Protocol), gives a brief overview on ZK Proofs features and trade-offs that have driven recent research (speed, proof-size, trusted setups, and quantum resistance). Terse descriptions of a few known implementations are included those that in line with this report are; 

- zkSNARKs implementations; Zcash, Coda, and Matter Labs, 
- Bulletproofs implementations; Monero, Grin & Beam, and a possible implementation to Bitcoin. 








## Implementations of R1CS  

A lot of theory on zero-knowledge proofs has seen practical and almost practical implementation. Most of which are made efficient, secure, and achieve privacy thanks to _zero-knowledge_ techniques such as _zkSNARKs_ and _Bulletproofs_. 


For further information and a more inclusive list of references on zero-knowledge proofs (with links) see [[18.]]." It contains a tabular list of "Implementations of proving systems" which includes [bellman](https://github.com/zkcrypto/bellman), [dalek bulletproofs](https://github.com/dalek-cryptography/bulletproofs) and [adjoint-io bulletproofs](https://github.com/adjoint-io/bulletproofs). In line with this report, one also finds a list of "Libraries for writing circuits (or gadgets)" such as [libsnark](https://github.com/scipr-lab/libsnark),  [ZoKrates](https://github.com/Zokrates/ZoKrates), [snarky](https://github.com/o1-labs/snarky), and [ZkMV](https://github.com/stellar/slingshot/tree/main/zkvm). 

There are standardization efforts made by the community with [the third _ZKProof workshop_ coming up in April 4 - 6, 2020 in London](https://zkproof.org/), and communication made with NIST in this regard as seen in the letter [here](https://zkp.science/docs/Letter-to-NIST-20160613-Advanced-Crypto.pdf).  







## References 

[[1.](https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8)] Cathie Yun, "Building on Bulletproofs," https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8 Date accessed: 2019‑01‑03 

[1.]: https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8
"Building on Bulletproofs"


[[2.]]  Alan Szepieniec and Bart Preneel, "Generic Zero-Knowledge and Multivariate Quadratic Systems," 2016. https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591 Date accessed: 2020‑01‑08. 

[2]: https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591
"Generic Zero-Knowledge and 
Multivariate Quadratic Systems"
"Generic Zero-Knowledge and Multivariate Quadratic Systems,"


[[3.]]  Amir Shpilka and Amir Yehudayoff, "Arithmetic Circuits: a survey of recent results and open questions," Technion-Israel Institute of Technology, Haifa, Israel, 2010. https://ieeexplore.ieee.org/document/8186881/metrics#metrics Date accessed: 2019‑12‑21

[3.]: https://ieeexplore.ieee.org/document/8186881/metrics#metrics
"Arithmetic Circuits: a survey of recent results and open questions," 


[[4](https://eprint.iacr.org/2016/263.pdf)] J. Bootle, A. Cerulli, P. Chaidos, J. Groth and C. Petit, "Efficient Zero-knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting", *Annual International Conference on the Theory and Applications of Cryptographic* *Techniques*, pp. 327‑357. Springer, 2016 [online]. Available: https://eprint.iacr.org/2016/263.pdf. Date accessed: 2019‑12‑21. 

[4.]: https://eprint.iacr.org/2016/263.pdf
"Efficient Zero-knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting" 


[[5.]]  Alex Pinto, "Constraint Systems for ZK SNARKs," 2019-03-06, http://coders-errand.com/constraint-systems-for-zk-snarks/ Date accessed: 2019‑12‑23

[5.]: http://coders-errand.com/constraint-systems-for-zk-snarks/
"Constraint Systems for ZK SNARKs"


[[6.]]  Howard Wu, Wenting Zheng, Alessandro Chiesa, Raluca Ada Popa, and Ion Stoica, "DIZK: A Distributed Zero Knowledge Proof System", Proceedings of the 27th USENIX Security Symposium, August 15–17, 2018. https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf Date accessed: 2019‑12‑14  

[6.]: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf
"DIZK: A Distributed Zero Knowledge Proof System"

[[7.]] Alan Szepieniec and Bart Preneel, "Generic Zero-Knowledge and Multivariate Quadratic Systems". https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591 Date accessed: 2019‑12‑31

[7.]: https://pdfs.semanticscholar.org/06c8/ea507b2c4aaf7b421bd0c93e6145e3ff7517.pdf?_ga=2.124585865.240482160.1578465071-151955209.1571053591
"Generic Zero-Knowledge and Multivariate Quadratic Systems" 


[[8.]]  Eli Ben-Sasson, Alessandro Chiesa, Daniel Genkin, Eran Tromer and Madars Virza, "SNARKs for C : Verifying Program Executions Succinctly and in Zero Knowledge (extended version)," October, 2013. https://eprint.iacr.org/2013/507.pdf Date accessed: 2019‑12‑17

[8.]: https://eprint.iacr.org/2013/507.pdf
"SNARKs for C : Verifying Program Executions Succinctly and in Zero Knowledge (extended version)"


[[9.]]  Vitalik Buterin, "Quadratic Arithmetic Programs: from Zero to Hero," Dec 12, 2016. https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649 Date accessed: 2019‑12‑19

[9.]: https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649


[[10.]] Dalek's documents, "Constraint System" https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html Date accessed: 2020‑01‑03

[10.]: https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html#constraint-system
"Constraint System" 


[[11](https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7)] Cathie Yun, "Programmable Constraint Systems for Bulletproofs" [online]. Available: https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7. Date accessed: 2019‑12-04.

[11.]: https://medium.com/interstellar/programmable-constraint-systems-for-bulletproofs-365b9feb92f7
"Programmable Constraint Systems for Bulletproofs" 


[[12.](http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf)] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More", *Blockchain Protocol Analysis and Security Engineering 2018* [online]. Available: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf. Date accessed: 2019‑11‑21.

[12.]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf
"Bulletproofs: Short Proofs for Confidential Transactions and More"

[[13.](https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html )]  Tari Labs University, "zkSNARKs," https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html Date accessed: 2020‑01‑06. 

[13.]: https://tlu.tarilabs.com/cryptography/zksnarks/mainreport.html

[[14.](https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html)]  Dalek's documents, "Module bulletproofs::notes::r1cs_proof." https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html Date accessed: 2020‑01‑07. 

[14.]: https://doc-internal.dalek.rs/bulletproofs/notes/r1cs_proof/index.html

[[15.](https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82)]  Lovesh Harchandani, "Zero knowledge proofs using Bulletproofs," Feb. 2019. https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82 Date accessed: 2020‑01‑03. 

[15.]: https://medium.com/coinmonks/zero-knowledge-proofs-using-bulletproofs-4a8e2579fc82

[16.]  Medio Demarco and Anil Lulla, "Zero-Knowledge Proofs: Privacy & Scaling Thematic Insights", Delphi Digital, April 2019. Date downloaded: 2020‑01‑06. 

[[17.](https://iacr.org/archive/crypto2009/56770190/56770190.pdf)]  Jens Groth, "Linear algebra with sub-linear zero-knowledge arguments," Advances in Cryptology – CRYPTO 2009, pages 192–208, 2009. https://iacr.org/archive/crypto2009/56770190/56770190.pdf Date accessed 2019-12-04

[17.]: https://iacr.org/archive/crypto2009/56770190/56770190.pdf
"Linear algebra with sub-linear zero-knowledge arguments"

[[18.](https://zkp.science/)]  "Zero-Knowledge Proofs: _What are they, how do they work, and are they fast yet?_" https://zkp.science/. Date accessed 2020-01-07.  

[18.]: https://zkp.science/
"Zero-Knowledge Proofs: What are they, how do they work, and are they fast yet?"



