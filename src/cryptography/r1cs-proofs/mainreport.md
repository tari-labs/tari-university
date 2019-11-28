
# Rank-1 Constraint Systems 


## Introduction 

Zero-knowledge proofs and arguments are ubiquitous in cryptography today, with prominent applications in authentication protocols, multi-party computation, encryption primitives, electronic voting systems and verifiable computation protocols [11.] A lot work has been done on improving the efficiency of zero-knowledge proofs. 

Rank-1 Constraints Systems have recently been used to enhance zero-knowledge proofs such as zkSNARKs as well as bulletproofs. 

The aim of this report is to study what a Rank-1 Constraint Systems are, and how they have been used in zk-SNARKs (briefly) and in bulletproofs (more elaborately). 





## Arithmetic Circuits 


An arithmetic circuit over a field  $Z_p$  and variables  $ ( a_1 , . . . , a_m ) $  is a directed acyclic graph whose vertices are called _gates_ [11.] Gates of in-degree  $0$  are inputs to the circuit and labelled with some  $a_i$  or a constant field element. All other gates are labelled   $ + $  or $ × $. We consider fan-in 2 circuits, in which case all of the  $ + $  and  $ × $  gates have in-degree 2, or arbitrary fan-in circuits.
Arithmetic circuits can be described alternatively as a list of multiplication gates with a collection of linear consistency equations relating the inputs and outputs of the gates. Our zero-knowledge protocols for circuit satisfiability use circuits in this form. Any circuit described as an acyclic graph can be efficiently converted into the alternative description. 


Here is an example of what an arithmetic circuit looks like for computing the expression  $(a+b)*(b*c)$ :

  ![img](https://z.cash/wp-content/uploads/2018/09/arithmetic-circuit_720_2x.png)

Looking at such a circuit, we can think of the input values a, b, c as “traveling” left-to-right on the wires towards the output wire. Our next step is to build what is called a Rank 1 Constraint System, or R1CS, to check that the values are “traveling correctly”. In this example, the R1CS will confirm, for instance, that the value coming out of the multiplication gate where b and c went in is b*c.



## What is a Rank-1 Constraint System? 


  **Definition**: [[2.]](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)  
  An R1CS is a sequence of groups of three vectors  $( A, B, C )$ , and the solution to an R1CS is a vector  $s$  that satisfies the equation 
  $$ ( A \cdot s ) * ( B \cdot s ) - ( C \cdot s ) = 0 $$   where  "$\cdot$"  is the dot-product of vectors. 
  
In the context of verification using zero-knowledge proofs, the solution  $s$  in the above definition would be the solution to a computational problem posed by the Verifier. The Prover needs to have  $s$, and prove to the verifier that they are in possession of  $s$.  




# PART I: R1CS  in *zkSNARKs*  
 


  ## Zero-Knowledge SNARKs (zk-SNARKs)
  
A *zero-knowledge proof* protocol allows two players, the Verifier and the Prover, to carry out a 3-step process,  
   - the Verifier challenges the Prover, say to carry out a certain computational problem, 
   - the Prover solves the problem and sends back proof that they have successfully completed the solution, 
   - the Verifier checks if indeed the proof is correct for the original challenge. 

A lot of work has been done in making these type of protocols practical, making them short and efficient. Research work over the last decade has produced several refinements of these proofs, even mechanisms to transform interactive zero-knowledge proofs to non-interactive zero-knowledge proofs (NIZKs). 

Of interest to us is the so called *zero-knowledge succinct non-interactive arguments of knowledge*, or zk-SNARKs. If we drop the technical detail that *arguments of knowledge* must be *decidable* and *sound*, zk-SNARKs can simply be classified as *proofs*, zero-knowledge proofs, and hence a special class of NIZKs (See  [1.]  for a brief landscape of NIZKs, which includes zk-SNARKs).  

 
 **Definition:** (zk-SNARKs)
 
 A typical zk-SNARK consists of three components  ( G, P, V ),  where  G  is the Key Generator,  P  is the Prover, and  V  is the Verifier. 
 
		 G  the Key Generator; 
			•   G  takes as inputs, 
				$\lambda$ , a secret parameter, and 
				C , a given program. 
			•  The output of  G  is a pair of public keys, 
				pk  the proving key to be used by the Prover, and 
				vk  the verification key to be used by the Verifier. 
	
		 P  the Prover; 
			•  P  takes 3 inputs,  
				pk  the prover key, 
				x  a public input, 
				w  a private witness 
			•  The output of  P  is a proof, 
				prf  =  P( pk , x, w ).
   
		V  the Verifier;   
			•   V  takes 3 inputs 
				vk  the prover key, 
				x  a public input, 
				prf  a private witness. 
			•   V  computes two values and checks equality, 
				First value  V( vk, x, prf ) , 
				Second value  C( x, \eta )  for some  \eta  in the range, 
				Checks if  V( vk, x,  prf )  equals   C( x, \eta ).    


 Note that  $pk$ and $vk$ are generated once and only for a given program  $C$. i.e. $(pk, vk) = \bf{G}( \lambda, C )$. 
 
 
 Christian Lundkvist tweeted the definition below, @ChrisLundkvist, in order to prove how short a zk-SNARK is, [[10.]](https://media.consensys.net/introduction-to-zksnarks-with-examples-3283b554fc3b)

						Generator (C circuit, λ is ☣️):
						(pk, vk) = G(λ, C)
						Prover (x pub inp, w sec inp):
						π = P(pk, x, w)
						Verifier:
						V(vk, x, π) == (∃ w s.t. C(x,w))
						
							6:53 pm - 18 Nov 2016


 
  
  "The first constructions of SNARK protocols were inspired by the PCP theorem which shows that NP statements have ‘short’ probabilistic checkable proofs. New instantiations were found which allow faster and shorter proofs, when a pre-processing state is permitted" [4.]   
  
zkSNARKs deserve a separate and detailed report on their own as they are widely used in cryptocurrencies like Zcash and Monero. We here focus on a single step in the zkSNARK process, is Rank-1Constraint Systems.  
  

  
 
  ### 5-step process of creating a zk-SNARK for a computation 

In a ZK-SNARK proof, a computation is verified step by step [[8.]](https://electriccoin.co/blog/snark-explain5/). To do so, the computation is first turned into an arithmetic circuit, then each of its wires is assigned a value that results from feeding specific inputs to the circuit. Each computing node of the arithmetic circuit (called a “gate” in analogy to the nomenclature of electronic circuits) is transformed into a constraint, that verifies the output wire has the value it should have for the values assigned to the input wires.

For example, if in the assignment to the arithmetic circuit one has the values  2  and  3  as inputs to a multiplication gate and the value 10 is assigned to the output wire, you would have found a fishy step in the proof. That computation would not be correct, and you, as a verifier, would not be convinced and should reject the proof.

Since it is very expensive in general to verify the constraints for all the gates individually, we encode all of them in 3 polynomial vectors  $A(t)$,  $B(t)$  and  $C(t)$. The constraints can then all be verified at once if a linear combination of these three vectors can be evenly divided by a vanishing polynomial  $Z(t)$ , 

$$ < A(t) \cdot  s > * < B(t) \cdot s> - < C(t) \cdot s >  =  H(t) * Z(t). $$

See [[5.]](http://coders-errand.com/the-vanishing-polynomial-for-qaps/ ) for a discussion on the vanishing polynomial. The above expression shows precisely the test. The vector  $s$  is the computation’s witness, that represents a full assignment of all the circuit’s wires,  and the left-hand side just shows it being mixed with all the constraints at the same time, defining the linear combination of polynomials. The equality to the right hand side then expresses that this is a multiple of  $Z(t)$.  


zk-SNARKs are not applied directly to computational problems. Therefore every computational problem the Verifier challenges the Prover with needs to first be converted into a 'form' called *Quadratic Arithmetic Program* (QAP), and the QAP is in turn transformed into a *Linear Probabilistically Checkable Proof* (PCP). So then, starting with a computational problem, a zk-SNARK is achieved by a 5-step process;  [[12.]](http://www.cs.tau.ac.il/~tromer/istvr1516-files/lecture12-verified-computation.pdf)
  	 
  **Computational Problem —> Arithmetic Circuit —> R1CS —> QAP —> Linear PCP —> zk-SNARK** 

Eran Tromer in  [[12.]](http://www.cs.tau.ac.il/~tromer/istvr1516-files/lecture12-verified-computation.pdf)  adds an extra 'form' between a linear PCP and the zk-SNARKs, and that's *Linear Interactive Proof*;   See Eran Tromer's diagram in  [[2.]](https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649)

  
Our focus in this part of the report is on the R1CS form. 

     
  
  

   
### Example 

  Here's the Verifier's *challenge* to the Prover:  
  Prove that you know the solution to  $$ x^3 + x + 5 = 35 $$ 

  It is easy to find the solution, which is$\ \ $ $ x = 3 $. Here's how a zk-SNARK approach uses an R1CS to verify the solution (focusing on the R1CS part of the 5-step process above). 
  
  
  **Computational Problem**: A program that takes  $x$  as an input and checks whether it satisfies the given equation. It can be written as; 
  
  	def qeval(x): 
	y = x**3  
 	return x + y + 5
 
**Arithmetic Circuit**: The aim is to express everything in this code in a low level language that only handles two operations, *addition* and *multiplication*, and in each equation there's one operation between two terms. This is called 'flattening' as it literally means converting the entire equation into smaller equation in which all variables are to the power 1, i.e., 'linearising'. 
 
	sym_1 = x * x
	y = sym_1 * x 
	sym_2 = y + x 
	~out = sym_2 + 5
 
**Rank-1 Constraint System**:  

In order to set up the solution vector $s$, we note all the variables involved thus far. The *auxiliary* variables are  'sym_1',  'y' , and  'sym_2', while the *primary* variables are the input  $x$ , the output  '~out', and dummy variable  '~one'  representing the number  1. The vector  $s$  is defined as 
  
  s  =  [  ~one , x , ~out , sym_1 , y , sym_2  ].    
  
 In fact, using the Arithmetic Circuit above, one can quickly check that,   s  =  [ 1 ,  3  , 35 , 9 , 27 , 30 ].   
  
 Now, about the vectors  a , b  and  c :   Each triple    ( a , b , c )    corresponds to each equation (actually each gate) in the Arithmetic Circuit. Since each equation has three terms, a  1  in either  a  ,  b  or  c   indicates the appearance of the corresponding variable in the equation. The constraints are therefore, 

		sym_1 = x * x   corresponds to	a = [ 0 , 1 , 0 , 0 , 0 , 0 ] , 
							b = [ 0 , 1 , 0 , 0 , 0 , 0 ] ,  and  
							c = [ 0 , 0 , 0 , 1 , 0 , 0 ]. 
						
		y = sym_1 * x   corresponds to	a = [ 0 , 0 , 0 , 1 , 0 , 0 ] ,  
						 	b = [ 0 , 1 , 0 , 0 , 0 , 0 ] ,  and  
						 	c = [ 0 , 0 , 0 , 0 , 1 , 0 ]. 
						
		sym_2 = y + x  which is best expressed as  sym_2 = 1*(y + x) ,  	corresponds to
							a = [ 0 , 1 , 0 , 0 , 1 , 0 ] ,  
							b = [ 1 , 0 , 0 , 0 , 0 , 0 ] ,  and  
							c = [ 0 , 0 , 0 , 0 , 0 , 1 ].  
							
		~out = sym_2 + 5  which is also best expressed as  ~out = 1*(sym_2 + 5)  corresponds to 
							a = [ 5 , 0 , 0 , 0 , 0 , 1 ] ,  
							b = [ 1 , 0 , 0 , 0 , 0 , 0 ] ,  and  
							c = [ 0 , 0 , 1 , 0 , 0 , 0 ].    

We leave the rest of the computations in the above example to later reports. Vitalik Buterin, also Alex Pinto, continue with the example in [[2.]]( ) and [[5b.]](http://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/) respectively.  
  

### Adding zero-knowledge to the above 5-step process 

In this R1CS representation, the verifier has to check many constraints — one for almost every wire of the circuit. (For technical reasons, it turns out we only have a constraint for wires coming out of multiplication gates.) In their 2012 paper [[6.]](https://eprint.iacr.org/2012/215.pdf) on the topic, Gennaro, Gentry, Parno and Raykova presented a nice way to “bundle all these constraints into one”. This method uses a representation of the circuit called a Quadratic Arithmetic Program (QAP). The single constraint that needs to be checked is now between polynomials rather than between numbers. The polynomials can be quite large, but this is alright because when an identity does not hold between polynomials, it will fail to hold at most points. Therefore, you only have to check that the two polynomials match at one randomly chosen point [[9.]](https://z.cash/blog/snark-explain2) in order to correctly verify the proof with high probability.

If the prover knew in advance which point the verifier would choose to check, they might be able to craft polynomials that are invalid, but still satisfy the identity at that point. With zk-SNARKs, sophisticated mathematical techniques such as _homomorphic encryption_ and _pairings_ of elliptic curves are used to evaluate polynomials “blindly” – i.e. without knowing which point is being evaluated. The public parameters described above are used to determine which point will be checked, but in encrypted form so that neither the prover nor the verifier know what it is.

The description so far has mainly addressed how to get the S and N in “SNARKs” — how to get a short, non-interactive, single message proof — but hasn’t addressed the “zk” (zero-knowledge) part which allows the prover to maintain the confidentiality of their secret inputs. It turns out that at this stage, the “zk” part can be easily added by having the prover use “random shifts” of the original polynomials that still satisfy the required identity.

For a step-by-step, in-depth explanation of key concepts behind zk-SNARKs in Zcash, see their SNARKs Explainer series with posts on:

1. [Homomorphic Hiding](https://z.cash/blog/snark-explain)
2. [Blind Evaluation of Polynomials](https://z.cash/blog/snark-explain2)
3. [The Knowledge of Coefficient Test and Assumption](https://z.cash/blog/snark-explain3)
4. [How to make Blind Evaluation of Polynomials Verifiable](https://z.cash/blog/snark-explain4)
5. [From Computations to Polynomials](https://z.cash/blog/snark-explain5)
6. [The Pinocchio Protocol](https://z.cash/blog/snark-explain6)
7. [Pairings of Elliptic Curves](https://z.cash/blog/snark-explain7)

Zcash uses [bellman](https://github.com/zcash/librustzcash/tree/master/bellman), a Rust-language library for zk-SNARKs. Before the Sapling upgrade, Zcash used a fork of the C++ library, [libsnark](https://github.com/scipr-lab/libsnark). For a deeper dive into the protocols used for Zcash’s zk-SNARKs, refer to the paper on the [Pinocchio protocol](https://eprint.iacr.org/2013/279.pdf), which was used until the Sapling upgrade, and [Jens Groth’s zk-SNARK](https://eprint.iacr.org/2016/260.pdf) which is used currently.



### How zk-SNARKs are applied to create a shielded transaction 

In Bitcoin, transactions are validated by linking the sender address, receiver address, and input and output values on the public blockchain. Zcash uses zk-SNARKs to prove that the conditions for a valid transaction have been satisfied without revealing any crucial information about the addresses or values involved. The sender of a shielded transaction constructs a proof to show that, with high probability:

- the input values sum to the output values for each shielded transfer.
- the sender proves that they have the private spending keys of the input notes, giving them the authority to spend.
- The private spending keys of the input notes are cryptographically linked to a signature over the whole transaction, in such a way that the transaction cannot be modified by a party who did not know these private keys.

In addition, shielded transactions must satisfy some other conditions that are described below.

Bitcoin tracks unspent transaction outputs (UTXOs) to determine what transactions are spendable. In Zcash, the shielded equivalent of a UTXO is called a “commitment”, and spending a commitment involves revealing a “nullifier”. Zcash nodes keep lists of all the commitments that have been created, and all the nullifiers that have been revealed. Commitments and nullifiers are stored as hashes, to avoid disclosing any information about the commitments, or which nullifiers relate to which commitments.

For each new note created by a shielded payment, a commitment is published which consists of a hash of: the address to which the note was sent, the amount being sent, a number “rho” which is unique to this note (later used to derive the nullifier), and a random nonce.

$$ Commitment = HASH(recipient address, amount, \rho, r) $$

When a shielded transaction is spent, the sender uses their spending key to publish a nullifier which is the hash of the secret unique number (“rho”) from an existing commitment that has not been spent, and provides a zero-knowledge proof demonstrating that they are authorized to spend it. This hash must not already be in the set of nullifiers tracking spent transactions kept by every node in the blockchain.

$$ Nullifier = HASH(spending key, \rho) $$ 

The zero-knowledge proof for a shielded transaction verifies that, in addition to the conditions listed above, the following assertions are also true:

- For each input note, a revealed commitment exists.
- The nullifiers and note commitments are computed correctly.
- It is infeasible for the nullifier of an output note to collide with the nullifier of any other note.

In addition to the spending keys used to control addresses, Zcash uses a set of proving and verifying keys to create and check proofs. These keys are generated in the public parameter ceremony discussed above, and shared among all participants in the Zcash network. For each shielded transaction, the sender uses their proving key to generate a proof that their inputs are valid. Miners check that the shielded transaction follows consensus rules by checking the prover’s computation with the verifying key. The way that Zcash’s proof generation is designed requires the prover to do more work up-front, but it simplifies verifying, so that the major computational work is offloaded to the creator of the transaction (this is why creating a shielded Zcash transaction can take several seconds, while verifying that a transaction is valid only takes milliseconds). 
  
  
  
  
  
### Remarks 

Firstly, note that the zk-SNARK end-to-end journey is to create a function or a protocol that takes the proof, given by the Prover, and checks its veracity. Secondly, in using an R1CS, we note that "an R1CS is not a computer program, you are not asking it to produce a value from certain inputs. Instead, an R1CS is a verifier, it shows that an already complete computation is correct" [3.]  

  Also, the main aim is not for the Verifier to pose a difficult computational problem at the Prover. The challenge may be very easy, as seen in the above example. In other words, this is not proof-of-work but validation. 
  
"To do so, the R1CS must receive as input an assignment of the full state of the computation. This is defined by the value of all the variables used in the process, and because each was set only once, its final value is equal to what it was when it was used. Therefore, by getting a snapshot of the values of all variables we can verify they have all been properly computed and guarantee the computation was correct one step at a time." [3.] 
  
   
   
   
 ## References  [to be moved to the end !! and rearranged properly]

  [1.] https://diyhpl.us/wiki/transcripts/cryptoeconomic-systems/2019/zksharks/   
  
  [2.] Vitalik Buterin, "Quadratic Arithmetic Programs: from Zero to Hero", Dec 12, 2016. https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649 
  
  [3.] Pinto, Alex, "Constraint Systems for ZK SNARKs", Mar 06, 2019. http://coders-errand.com/constraint-systems-for-zk-snarks/ 
  
  [4.] Mayer, Hartwig; *zk-SNARKS Explained: Basic Principles*, CoinFabrik, 06 Dec., 2016.
   
  [5.] Alex Pinto, "The Vanishing Polynomial for QAPs,"  Mar 23, 2019,  http://coders-errand.com/the-vanishing-polynomial-for-qaps/, accessed on 10 October 2019.
  
  [5b.] Alex Pinto, "How to Build a Quadratic Arithmetic Program," Mar 14, 2019,  http://coders-errand.com/how-to-build-a-quadratic-arithmetic-program/,  accessed on 10 October 2019.
  
  [6.] Gennaro R., Gentry C., Parno B. and Raykova M., "Quadratic Span Programs and Succinct NIZKs without PCPs," 2012. 
  
  [7.] "What are zk-SNARKs," https://z.cash/technology/zksnarks/, accessed on 27 November 2019. 
  
  [8.] Ariel Gabizon, "Explaining SNARKs Part V: From Computations to Polynomials," https://electriccoin.co/blog/snark-explain5/, accessed on 27 November 2019.
  
  [9.] Ariel Gabizon, "Explaining SNARKs Part II: Blind Evaluation of Polynomials," https://electriccoin.co/blog/snark-explain2/, accessed on 27 November 2019. 
  
  [10.] Christian Lundkvist, "Introduction to zk-SNARKs with Examples," https://media.consensys.net/introduction-to-zksnarks-with-examples-3283b554fc3b 
  
  [11.] Jonathan Bootle, Andrea Cerulli, Pyrros Chaidos, Jens Groth, and Christophe Petit, "Efficient Zero-Knowledge Arguments for Arithmetic Circuits," 2016. 
  
  [12.] Eran Tromer, "Lecture 12: Verified computation and its applications, course conclusion," 2016  http://www.cs.tau.ac.il/~tromer/istvr1516-files/lecture12-verified-computation.pdf  
  
  [13.]  Cathie Yun, Henry de Valence and Oleg Andreev, https://doc-internal.dalek.rs/develop/bulletproofs/notes/r1cs_proof/index.html   
  
  [14.]  Benedikt B¨unz,, Jonathan Bootle, Dan Boneh, et al, "Bulletproofs: Short Proofs for Confidential Transactions and More,"  
  
  [15.]   
  
 
  ​ 




  
# PART II: R1CS in *Bulletproofs*  
  
The aim here is to consider Rank-1 Constraint Systems as they apply to bulletproofs, which have been extensively covered in TLU. 

Bulletproofs were first introduced in a research paper written by Benedikt B¨unz and five other researchers in [11] as "short proofs for confidential transactions" aimed at creating fast and efficient range proofs.   

The bulletproofs paper [14] was an optimisation of earlier range proof techniques in [12] which showed that it was possible to prove arithmetic circuit satisfiability in the discrete log setting. Bootle et al, in [11], introduced the log-sized inner-product argument and a technique to convert an arithmetic circuit into a rank-1 constraint system (R1CS). As seen in the zk-SNARKs case, it has become the norm to first convert computational problems to arithmetic circuits in order to ultimately produce a proof. 

In the bulletproofs paper [14] the authors optimised the inner-product argument by a factor, and shows how to use Pedersen commitments as input to constraint system.  
[Insert here a comparison table of results from the paper [14].]     

We herein focus mainly on the work done by the Interstellar team; Cathie Yun, Henry de Valence and Oleg Andreev; who intended to improve the efficiency of inner-product proofs, hence bulletproofs, by bypass the arithmetic circuit yet utilise constraint systems.  

 
 
 
 ## Constraint Systems 
 
 
 A constraint system is a collection of arithmetic constraints over a set of variables [13.]  

The constraint system used by Cathie Yun *et al* is specifically a rank-1 constraint system or R1CS. 
Such a system consists of two sets of constraints; multiplication gates and linear constraints in terms of the variables [13.] 

A constraint system has two kinds of variables, *high-level* variables which are the input secrets, and *low-level* variables consisting of internal inputs and outputs of multiplication gates. 

A *multiplication gate* typically takes two input variables (say, ${\bf{a}}_L$ and ${\bf{a}}_R$), multiplies them together, and outputs the resulting product (denoted by  $\bf{a}_O$), 

 $$ {\bf{a}}_L  \cdot  {\bf{a}}_R = {\bf{a}}_O $$ 

A *linear constraint* refers to a linear combination of variables  ${\bf{a}}_L$, ${\bf{a}}_R$, ${\bf{a}}_O$ and ${\bf{v}}$ ,  
 $$ {\bf{W}}_L \cdot {\bf{a}}_L +  {\bf{W}}_R \cdot {\bf{a}}_R +  {\bf{W}}_O \cdot {\bf{a}}_O  =  {\bf{W}}_v \cdot {\bf{v}}  +  {\bf{c}} $$   where  ${\bf{W}}_L$, ${\bf{W}}_R$, ${\bf{W}}_O$  and $ {\bf{W}}_v $
are weights applied to the respective inputs and outputs of the multiplication gates and the high-level variable  $\mathbf{v}$  

### Building Constraints 

  Bulletproofs framework allows building constraint systems _on the fly_, without a trusted setup. This allows instantiating constraints from a _family of constraint systems_ parametrized by
public values and commitments to the secret inputs. As a result, the instantiation can be thought of as a _challenge_ generated by a verifier to a prover.

The prover starts out by committing to its secret inputs $\mathbf{v}$
and obtaining $m$ _variables_ representing these inputs.

Then, the prover performs a combination of the following operations to generate the constraints:

1. **Allocate a multiplier:** a new multiplication gate is added, represented by three low-level variables  $a_L, a_R, a_O$, for left input, right input and output value respectively.
2. **Add a constraint:** a linear combination of any number of variables is encoded into appropriate positions in matrices  $\mathbf{W}_L, \mathbf{W}_R, \mathbf{W}_O, \mathbf{W}_V$  and a vector of constants $\mathbf{c}$.
3. **Request a challenge scalar:** a random scalar returned in response to committed high-level variables.



## Gadgets

Gadgets are buildings blocks of a constraint system that map to some functions in a higher-level protocol.
Gadgets receive some variables as inputs, may allocate more variables for internal use,
and produce constrains involving all these variables.

Examples:

- a **shuffle gadget** creates constraints that prove that two sets of variables are equal up to a permutation;
- a **range proof gadget** checks that a given value is composed of a specific number of bits.

## Low-level variables

Often a [gadget](#gadgets) needs a variable to connect with another gadget,
or to implement its internal logic, without requiring a distinct [high-level variable](#variables) commitment $V_i$ for it.
Such **low-level variables** are created from left and right variables $a_L, a_R$ of additional multiplication gates.
Output variables $a_O$ are not used for this purpose because
they are implicitly constrained by a [multiplication gate](#multiplication-gates)
and cannot be used as independent uncommitted variables.

**Important:** uncommitted variables have their name due to lack of the individual commitments $V_i$,
but they are still committed collectively with all [low-level variables](#variables)
using a single vector Pedersen commitment $A_I$ as required by the underlying proof protocol.
The distinction is important when [building constraints](#building-constraints) using [challenges](#gadget-as-a-challenge),
which are bound only to the high-level variables, but not to the low-level variables (hence, “uncommitted”).

## Gadget as a challenge

Intermediate challenge scalars can be used to construct [gadgets](#gadgets) more efficiently.

For example, a shuffle gadgets (“proof of permutation”) can be done by proving equality of
two polynomials sampled at a challenge point, where roots of each polynomial
represent secret values of the corresponding side of a permutation:
$$
  \lbrace a,b \rbrace  =  \lbrace c,d \rbrace   \iff  (a-x)\cdot(b-x) = (c-x)\cdot(d-x),
$$
where  $x$ is a random challenge, sampled after all values $a,b,c,d$ are committed.

  
  
Making a proof of permutation using a static gadget (without challenge values) may require
building a [sorting network][sorting_network] that would use significantly more multiplication gates.

**Important:** challenges are bound to the [high-level variables](#variables) and the
 committed portion of [low-level variables](#low-level-variables).
 The remaining [low-level variables](#low-level-variables) are uncommitted and must be uniquely determined
 by the committed variables and the challenge scalars in order for gadgets to be _locally sound_.
 To facilitate this, the [constraint system API](../../r1cs/index.html) prevents use of challenges
 before all freely chosen variables are committed.

[sorting_network]: https://en.wikipedia.org/wiki/Sorting_network

## Representation of constraints

The matrices $\mathbf{W}_L, \mathbf{W}_R, \mathbf{W}_O, \mathbf{W}_V$ are typically very sparse
because most constraints apply only to a few variables. As a result, constraints are represented as short lists
of pairs $(i, w)$ where $i$ is an index of a variable, and $w$ is its (non-zero) weight.

Multiplication of a matrix by a vector is implemented via multiplication of each weight $w$ by 
a scalar in the vector at a corresponding index $i$. This way, all zero-weight terms are automatically skipped.

## Constraint system proof

Constraint system proof allows a **verifier** to specify the constraints, for which a **prover** is asked to generate a valid proof.
The resulting proof is zero-knowledge: the constraints are known to both the prover and the verifier, but the variables remain secret.

The proving system uses efficient [inner product protocol](../inner_product_proof/index.html)
by expressing all the constraints in terms of a single inner product.
The following notes describe in detail how this is accomplished.

### Constraint system notation

Dimensions of vectors:

- $m$ — number of secret values ${\mathbf{v}}$,
- $n$ — number of variable multiplication gates represented by $\mathbf{a}_L, \mathbf{a}_R, \mathbf{a}_O$,
- $q$ — number of linear constraints represented by $\mathbf{W}_L, \mathbf{W}_R, \mathbf{W}_O, \mathbf{W}_V$.

In the [Bulletproofs paper][bp_website], matrices are labeled as $\mathbf{W}_L, \mathbf{W}_R, \mathbf{W}_O, \mathbf{W}_V$. We will keep this notation, but will note to readers not to confuse the $\mathbf{W}_{L,R,O,V}$ notation for being a vector of points.

We will use the general [notation](#notation) described above and, for consistency, rename two more variables from the paper:
$$
\begin{aligned}
    \beta         &\xrightarrow{} \tilde{o} \\\\
    Q             &\xrightarrow{} q \\\\
\end{aligned}
$$

[bp_website]: https://crypto.stanford.edu/bulletproofs/

## Combining statements using challenge variables

We can rewrite the statement about multiplication gates into an inner product equation, using the challenge variable $y$.
We can do this for a random challenge $y$ because ${\mathbf{b}} = {\mathbf{0}}$ if and only
if[^1] ${\langle {\mathbf{b}}, {\mathbf{y}}^{n} \rangle} = 0$. The equation $\mathbf{a}_L \circ \mathbf{a}_R = \mathbf{a}_O$ becomes:
$$
\langle \mathbf{a}_L \circ \mathbf{a}_R - \mathbf{a}_O ,
\mathbf{y}^n \rangle = 0
$$
We can rewrite the statement about the linear constraints into an inner product equation, using the challenge variable $z$.
We can do this for a random challenge $z$, for the same reason as above. The equation
$
\mathbf{W}_L \cdot \mathbf{a}_L +
\mathbf{W}_R \cdot \mathbf{a}_R +
\mathbf{W}_O \cdot \mathbf{a}_O =
\mathbf{W}_V \cdot \mathbf{v} +
\mathbf{c}
$ 
becomes:
$$
\langle z \mathbf{z}^q,
\mathbf{W}_L \cdot \mathbf{a}_L +
\mathbf{W}_R \cdot \mathbf{a}_R +
\mathbf{W}_O \cdot \mathbf{a}_O -
\mathbf{W}_V \cdot \mathbf{v} -
\mathbf{c}
\rangle = 0
$$
We can combine these two inner product equations, since they are offset by different multiples of challenge variable $z$.
The statement about multiplication gates is multiplied by $z^0$, while the statements about addition and scalar multiplication gates
are multiplied by a powers of $z$ between $z^1$ and $z^q$. Combining the two equations gives us:
$$
\langle \mathbf{a}_L \circ \mathbf{a}_R - \mathbf{a}_O ,
\mathbf{y}^n \rangle +
\langle z \mathbf{z}^q, 
\mathbf{W}_L \cdot \mathbf{a}_L +
\mathbf{W}_R \cdot \mathbf{a}_R +
\mathbf{W}_O \cdot \mathbf{a}_O -
\mathbf{W}_V \cdot \mathbf{v} -
\mathbf{c}
\rangle = 0
$$
Before we proceed, we factor out “flattened constraints” from the terms that involve weight matrices:
$$
\langle \mathbf{a}_L \circ \mathbf{a}_R - \mathbf{a}_O ,
\mathbf{y}^n \rangle +
\langle \mathbf{w}_L, \mathbf{a}_L \rangle +
\langle \mathbf{w}_R, \mathbf{a}_R \rangle +
\langle \mathbf{w}_O, \mathbf{a}_O \rangle -
\langle \mathbf{w}_V, \mathbf{v}    \rangle - w_c = 0
$$

$$
\begin{aligned}
\mathbf{w}_L &= z \mathbf{z}^q \cdot \mathbf{W}_L, \\\\
\mathbf{w}_R &= z \mathbf{z}^q \cdot \mathbf{W}_R, \\\\
\mathbf{w}_O &= z \mathbf{z}^q \cdot \mathbf{W}_O, \\\\
\mathbf{w}_V &= z \mathbf{z}^q \cdot \mathbf{W}_V, \\\\
w_c          &= \langle z \mathbf{z}^q, \mathbf{c} \rangle, \\\\
\end{aligned}
$$

where each of $\mathbf{w}_L, \mathbf{w}_R, \mathbf{w}_O$ has length $n$ and $\mathbf{w}_V$ has length $m$.

[^1]: This is because the polynomial in terms of $y$ is zero at every point if and only if every term of it is zero. The verifier is going to sample
a random $y$ after the prover commits to all the values forming the terms of
that polynomial, making the probability that the prover cheated negligible.
This trick allows implementing logical `AND` with any number of terms.

## Rearranging into a single inner product statement

We want to work towards expressing the constraints in terms of a single inner product,
so that we can use the inner product argument to represent it in a more compact and efficient-to-verify form. 
To do that we will rearrange the above equation so that terms
involving  ${\bf{a}}_L$  and  ${\bf{a}}_O$  appear only on the left-hand side, terms involving $ {\bf{a}}_R $ 
 appear only on the right-hand side, and non-secret terms (which the verifier can compute on its own) are
factored out into a new term  $ \delta(y, z) $


This arrangement will allow us to verify relations between the resulting inner product,
its vectors and the commitments to high-level and low-level variables.

The choice of placing  $ {\bf{a}}_O $
on the same side with 
 ${\bf{a}}_L$  is arbitrary, the proof would still work even if  ${\bf{a}}_O$ 
was rearranged on the right-hand side instead. 

If we reorder terms, we get: 

$$ w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle=\langle \mathbf{a}_L \circ \mathbf{a}_R, \mathbf{y}^n \rangle -\langle \mathbf{a}_O, \mathbf{y}^n \rangle +\langle \mathbf{w}_L, \mathbf{a}_L \rangle +\langle \mathbf{w}_R, \mathbf{a}_R \rangle +\langle \mathbf{w}_O, \mathbf{a}_O \rangle $$ 

Merge the statements containing $\mathbf{a}_O$ 


$$ w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle=\langle \mathbf{a}_L \circ \mathbf{a}_R, \mathbf{y}^n \rangle +\langle \mathbf{a}_L, \mathbf{w}_L                    \rangle +\langle \mathbf{a}_O, -\mathbf{y}^n + \mathbf{w}_O    \rangle +\langle \mathbf{a}_R, \mathbf{w}_R                    \rangle
				$$

Rearrange $\langle \mathbf{a}_L \circ \mathbf{a}_R, \mathbf{y}^n \rangle$ into 

$\langle \mathbf{a}_L, \mathbf{y}^n \circ \mathbf{a}_R \rangle $

$$ w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle=\langle \mathbf{a}_L, \mathbf{y}^n \circ \mathbf{a}_R \rangle + \langle \mathbf{a}_L, \mathbf{w}_L   \rangle +\langle \mathbf{a}_O, -\mathbf{y}^n + \mathbf{w}_O    \rangle +\langle \mathbf{a}_R, \mathbf{w}_R  \rangle $$

 
 Multiply the $ \langle \mathbf{a}_R, 
\mathbf{w}_R \rangle $ term by $\mathbf{y}^n$ one one side of the inner product and by $\mathbf{y}^{-n}$ on the other side:
 
 $$ w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle=\langle \mathbf{a}_L,    \mathbf{y}^n \circ \mathbf{a}_R    \rangle + \langle \mathbf{a}_L,                    \mathbf{w}_L                       \rangle +\langle \mathbf{a}_O,      -\mathbf{y}^n + \mathbf{w}_O        \rangle +\langle \mathbf{y}^n \circ \mathbf{a}_R, \mathbf{y}^{-n} \circ \mathbf{w}_R \rangle $$
 
 Merge the statements containing  $\mathbf{y}^n \circ \mathbf{a}_R$ 
 
 $$ w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle=\langle \mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{y}^n \circ \mathbf{a}_R \rangle + \langle \mathbf{a}_L,                                       \mathbf{w}_L                    \rangle +\langle \mathbf{a}_O,     -\mathbf{y}^n + \mathbf{w}_O   \rangle $$
 
Add $ \delta(y, z) = \langle \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{w}_L \rangle $ to both sides 
 
 
 $$ \begin{aligned}
w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle +
\delta(y, z)  &=
\langle \mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{y}^n \circ \mathbf{a}_R \rangle + 
\langle \mathbf{a}_L,                       \mathbf{w}_L                \rangle \\\\ 
&+
\langle \mathbf{a}_O,                      -\mathbf{y}^n + \mathbf{w}_O \rangle + 
\langle \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{w}_L                \rangle
\end{aligned}
$$
Merge the terms containing $\mathbf{w}_L$:
$$
\begin{aligned}
w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle + \delta(y, z)
&= \langle \mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R, 
\mathbf{y}^n \circ \mathbf{a}_R \rangle + 
\langle \mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R,
\mathbf{w}_L \rangle +
\langle \mathbf{a}_O, 
-\mathbf{y}^n + \mathbf{w}_O \rangle
\end{aligned}
$$
Merge the terms containing $\mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R$:
$$
w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle + \delta(y, z) =
\langle \mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R, 
\mathbf{y}^n \circ \mathbf{a}_R +
\mathbf{w}_L \rangle +
\langle \mathbf{a}_O, 
-\mathbf{y}^n + \mathbf{w}_O \rangle
$$
We want to combine a sum of two inner products $\langle a, b \rangle + \langle c, d \rangle$ into one inner product.
To do that, we will take a term of a polynomial formed by a linear combination of these products with respect to a challenge scalar $x$. Specifically, the 2nd degree term of $\langle a \cdot x + c \cdot x^2, b \cdot x + d \cdot x^0 \rangle$ is equal to
$ \langle a, b \rangle + \langle c, d \rangle$.

To apply this technique to the above equation we assign $a, b, c, d$ the following values:
$$
\begin{aligned}
a &= \mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R \\\\
b &= \mathbf{y}^n \circ \mathbf{a}_R +
\mathbf{w}_L\\\\
c &= \mathbf{a}_O \\\\
d &= -\mathbf{y}^n + \mathbf{w}_O
\end{aligned}
$$
Next, we combine $a, b, c, d$ using the equation $ \langle a \cdot x + c \cdot x^2, b \cdot x + d \cdot x^0 \rangle $. When we take its second degree, we recover a single inner product, which was our original goal:
$$
w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle + \delta(y, z) = 
\text{2nd degree of }
\langle (\mathbf{a}_L + \mathbf{y}^{-n} \circ \mathbf{w}_R) \cdot x + 
\mathbf{a}_O \cdot x^2,
(\mathbf{y}^n \circ \mathbf{a}_R +
\mathbf{w}_L) \cdot x +
(-\mathbf{y}^n + \mathbf{w}_O) \cdot x^0 \rangle 
$$
Distribute the $x$ values 

$$ w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle + \delta(y, z) = 
\text{2nd degree of }
\langle \mathbf{a}_L \cdot x + \mathbf{y}^{-n} \circ \mathbf{w}_R \cdot x + 
\mathbf{a}_O \cdot x^2,
\mathbf{y}^n \circ \mathbf{a}_R \cdot x +
\mathbf{w}_L \cdot x -
\mathbf{y}^n + \mathbf{w}_O \rangle $$ 

This is equivalent to the equation we started with, but has a single
inner product with  $ {\bf{a}}_L$  and  $ {\bf{a}}_O $  on the left, 
$ {\bf{a}}_R $ 
on the right, and non-secret terms factored out.

## Blinding the inner product

In the current form, the vectors in the inner-product reveal information about the values  $ {\bf{a}}_L ,  
 {\bf{a}}_R $  and 
$ {\bf{a}}_O $, which in turn reveal the values
$\bf{v}$.  

And since the inner-product argument is not zero-knowledge, the vectors cannot be used in the inner-product argument either. 

To solve this problem, the prover chooses two vectors of blinding factors

$$ {\bf{s}}_L, {\bf{s}}_R \; {\xleftarrow{\\$}} \; {\mathbb Z_p}^{n}, $$
and uses them to blind
${\bf{a}}_L$
and
${\bf{a}}_R$
within left and right sides of the inner product respectively. 

$$ \begin{aligned} {\bf{a}}_L &\leftarrow {\bf{a}}_L + {\bf{s}}_L  \cdot x^2 \\\\
{\bf{a}}_R &\leftarrow {\bf{a}}_R + {\bf{s}}_R \cdot x^2
\end{aligned} $$ 

The blinding factors are multiplied by $x^2$ so that when the substitution is made into the $\mathbf{l}(x)$ and $\mathbf{r}(x)$ equations,
${\bf{s}}_L$,  will be in the 3rd degree of  $x$  in  $\mathbf{r}(x)$. 
As a result, the blinding factors will not interfere with the value  $t_2$, which is the 2nd degree of 
$\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle$.  


Note: Multiplication outputs  
$ \mathbf{a}_O $  
do not need their own blinding factors, they are automatically blinded by  
$ \mathbf{s}_L $  
since both  
$ \mathbf{a}_L $  
and  
$ \mathbf{a}_O$  
are terms in the same (left) side of the inner product. 


We now construct vector polynomials ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$,
which represent the left and right sides of the input to the inner-product equation, with these new definitions:
$$ \begin{aligned}  {\mathbf{l}}(x) &= (\mathbf{a}_L + \mathbf{s}_L \cdot x^2) \cdot x + \mathbf{y}^{-n} \circ \mathbf{w}_R \cdot x + \mathbf{a}_O \cdot x^2 \\\\                  &= \mathbf{a}_L \cdot x + \mathbf{s}_L \cdot x^3 + \mathbf{y}^{-n} \circ \mathbf{w}_R \cdot x + \mathbf{a}_O \cdot x^2 \\\\  {\mathbf{r}}(x) &= \mathbf{y}^n \circ (\mathbf{a}_R + \mathbf{s}_R \cdot x^2) \cdot x + \mathbf{w}_L \cdot x - \mathbf{y}^n + \mathbf{w}_O \\\\                  &= \mathbf{y}^n \circ \mathbf{a}_R \cdot x + \mathbf{y}^n \circ \mathbf{s}_R \cdot x^3 + \mathbf{w}_L \cdot x - \mathbf{y}^n + \mathbf{w}_O\end{aligned} $$ 

When we take the inner product of ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$, we get:
$$ \begin{aligned}
  t(x) &= {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle} \\\\
       &= t_{1} x + t_{2} x^{2} + t_{3} x^{3} + t_{4} x^{4} + t_{5} x^{5} + t_{6} x^{6} \\\\
       &= \sum_{i=1}^{6} t_i x^i
\end{aligned} $$

Notice that the second degree of $t(x)$ does not include any blinding factors (because the blinding factors end up in the third or greater degrees of $t(x)$) and only contains the inner product forms of the initial arithmetic gate statements that we are trying to prove:

$$ \begin{aligned}
t_2 &= \text{2nd degree of } \langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle
\\\\
&= w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle + \delta(y, z) \\\\
&=
\langle \mathbf{a}_L \circ \mathbf{a}_R, \mathbf{y}^n \rangle -
\langle \mathbf{a}_O, \mathbf{y}^n \rangle + 
\langle \mathbf{w}_L, \mathbf{a}_L \rangle +
\langle \mathbf{w}_R, \mathbf{a}_R \rangle +
\langle \mathbf{w}_O, \mathbf{a}_O \rangle +
\delta(y, z)
\end{aligned}
$$


## Proving that $t_2$ is correct

The prover first forms a commitment to the coefficients of $t(x)$, then convinces the verifier that these commit to the correct $t(x)$ by evaluating the polynomial at a challenge point $x$. This proves that $t(x)$ is correct and follows the following equation: 

$$ \begin{aligned}t(x) &= \sum_{i=1}^{6} x^i t_{i} \\\\t_2 &= w_c + \langle \mathbf{w}_V, \mathbf{v} \rangle + \delta(y, z) \\\\\end{aligned} $$


We define $\mathbf{V}$ as the vector of Pedersen commitments to $\mathbf{v}$, and $T_i$ as the Pedersen commitment to $t_i$ for $i \in [1, 3, 4, 5, 6]$: 

$$ \begin{aligned}V_j &= v_j \cdot B + \tilde{v}_j \cdot \widetilde{B} \quad \forall j \in [1, m] \\\\T_i &= t_i \cdot B +  \tilde{t}_i \cdot \widetilde{B} \quad \forall i \in [1, 3, 4, 5, 6] \\\\\end{aligned} $$



The prover forms these commitments, and sends them to the verifier. These commitments are related to each other and to $t(x)$ by the following diagram: 




$$ 
\begin{aligned}
  t(x) B                         &\quad &= \quad & x^2 \langle \mathbf{w}_V, \mathbf{v} \rangle \cdot B                      & \quad &+ \quad & x^2 \big(w_c + \delta(y,z)\big) B   &\quad &+\quad & \sum_{i = 1,3,4,5,6} &x^i t_{i} B                       \\\\
    +                            &\quad &  \quad &  +                                                                         & \quad &  \quad &  +                                   &\quad & \quad &                                     &         +           \\\\
  {\tilde{t}}(x) {\widetilde{B}} &\quad &= \quad & x^2 \langle \mathbf{w}_V, \tilde{\mathbf{v}} \rangle \cdot \widetilde{B}  & \quad &+ \quad & 0 {\widetilde{B}}                    &\quad &+\quad & \sum_{i = 1,3,4,5,6} &x^i \tilde{t_{i}} {\widetilde{B}} \\\\
  \shortparallel                 &\quad &  \quad & \shortparallel                                                             & \quad &  \quad & \shortparallel                       &\quad & \quad &                                     &    \shortparallel   \\\\
                                 &\quad &= \quad & x^2 \langle \mathbf{w}_V, \mathbf{V} \rangle                              & \quad &+ \quad & x^2 \big(w_c + \delta(y,z)\big) B   &\quad &+\quad & \sum_{i = 1,3,4,5,6} &x^i T_{i}
\end{aligned} 
$$ 


Notice that the sum of each column is a commitment to the variable in the top row using the blinding factor in the second row. The sum of all of the columns is

$t(x) B + {\tilde{t}}(x) {\widetilde{B}}$, 

a commitment to the value of $t$ at the point $x$, using the synthetic blinding factor[^2]:

$$ {\tilde{t}}(x) = x^2 \langle \mathbf{w}_V, \tilde{\mathbf{v}} \rangle + \sum_{i = 1,3,4,5,6} x^i \tilde{t}_{i} $$

To convince the verifier that

$t(x) = \delta(y,z) + \sum_{i=1}^{6} x^i t_{i}$, 

the prover sends the opening $t(x), {\tilde{t}}(x)$ to the verifier, who uses the
bottom row of the diagram to check consistency: 

$$ t(x) B + {\tilde{t}}(x) {\widetilde{B}} \stackrel{?}{=} x^2 \langle \mathbf{w}_V, \mathbf{V} \rangle + x^2 \big(w_c + \delta(y,z)\big) B + \sum_{i = 1,3,4,5,6} x^i T_{i} $$


[^2]: The blinding factor is synthetic in the sense that it is

```
synthesized from the blinding factors of the other commitments.
```

## Proving that $\mathbf{l}(x)$, $\mathbf{r}(x)$ are correct

We want to relate ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$ to commitments
to ${\mathbf{a}}_{L}$, ${\mathbf{a}}_{R}$, ${\mathbf{s}}_{L}$, and
${\mathbf{s}}_{R}$. Since
$$
{\mathbf{r}}(x) = \mathbf{y}^n \circ \mathbf{a}_R \cdot x + \mathbf{y}^n \circ \mathbf{s}_R \cdot x^3 + \mathbf{w}_L \cdot x - \mathbf{y}^n + \mathbf{w}_O
$$
we need commitments to ${\mathbf{y}}^{n} \circ {\mathbf{a}}_{R}$ and
${\mathbf{y}}^{n} \circ {\mathbf{s}}_{R}$. However, the prover
must form commitments before receiving the verifier’s challenge $y$,
so they can only commit to ${\mathbf{a}}_{R}$ and ${\mathbf{s}}_{R}$,
requiring the verifier to transmute the prover’s commitment over
$
({\mathbf{a}}_{L},{\mathbf{a}}_{R}, {\widetilde{a}})
$
into a commitment over
$
({\mathbf{a}}_{L}, {\mathbf{y}}^{n} \circ {\mathbf{a}}_{R}, {\widetilde{a}})
$
(and similarly for ${\mathbf{s}}_{R}$).

To do this, notice that
$$
\begin{aligned}
  \text{commitment over }({\mathbf{a}}_{L}, {\mathbf{a}}_{R}, {\widetilde{a}})
  &=
  {\langle {\mathbf{a}}_{L}, {\mathbf{G}} \rangle} + {\langle {\mathbf{a}}_{R}, {\mathbf{H}} \rangle} + {\widetilde{a}} {\widetilde{B}} \\\\
  &=
  {\langle {\mathbf{a}}_{L}, {\mathbf{G}} \rangle} + {\langle {\mathbf{y}}^{n} \circ {\mathbf{a}}_{R}, {\mathbf{y}}^{-n} \circ {\mathbf{H}} \rangle} + {\widetilde{a}} {\widetilde{B}},
\end{aligned}
$$
so that by changing generators to
$\hat{\mathbf{H}} = {\mathbf{y}}^{-n} \circ {\mathbf{H}}$, the point which
is a commitment to
$({\mathbf{a}}_{L}, {\mathbf{a}}_{R}, {\widetilde{a}})$ with respect to
$({\mathbf{G}}, {\mathbf{H}}, {\widetilde{a}})$ is transmuted into a
commitment to
$({\mathbf{a}}_{L}, {\mathbf{y}}^{n} \circ {\mathbf{a}}_{R}, {\widetilde{a}})$
with respect to $({\mathbf{G}}, \hat{\mathbf{H}}, {\widetilde{a}})$.

We define the following commitments over the components of ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$:
$$
\begin{aligned}
A_I &= \widetilde{B} \cdot \tilde{a} + \langle \mathbf{G} , \mathbf{a}_L \rangle + \langle \mathbf{H}, \mathbf{a}_R \rangle \\\\
A_O &= \widetilde{B} \cdot \tilde{o} + \langle \mathbf{G} , \mathbf{a}_O \rangle \\\\
W_L &= \langle \mathbf{y}^{-n} \circ \mathbf{w}_L, \mathbf{H} \rangle \\\\
W_R &= \langle \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{G} \rangle \\\\
W_O &= \langle \mathbf{y}^{-n} \circ \mathbf{w}_O, \mathbf{H} \rangle \\\\
S   &= \widetilde{B} \cdot \tilde{s} + \langle \mathbf{G} , \mathbf{s}_L \rangle + \langle \mathbf{H}, \mathbf{s}_R \rangle
\end{aligned}
$$
The prover forms these commitments, and sends them to the verifier.

For reference, here are the equations for ${\mathbf{l}}(x)$, and ${\mathbf{r}}(x)$ multiplied by $\mathbf{y}^{-n}$:
$$
\begin{aligned}
                        \mathbf{l}(x)  &= \mathbf{a}_L \cdot x + \mathbf{s}_L \cdot x^3 + \mathbf{y}^{-n} \circ \mathbf{w}_R \cdot x + \mathbf{a}_O \cdot x^2 \\\\
  \mathbf{y}^{-n} \circ \mathbf{r}(x)  &= \mathbf{a}_R \cdot x + \mathbf{s}_R \cdot x^3 + \mathbf{y}^{-n} \circ \mathbf{w}_L \cdot x - \mathbf{1}^n + \mathbf{y}^{-n} \circ \mathbf{w}_O
\end{aligned}
$$
To relate the prover’s commitments to
${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$, we use the following diagram:
$$
\begin{aligned}
  {\langle {\mathbf{l}}(x), {\mathbf{G}} \rangle}      &\quad &= \quad & {\langle {\mathbf{a}}_L \cdot x, {\mathbf{G}} \rangle}      & \quad &+ \quad & {\langle {\mathbf{a}}_O \cdot x^2, {\mathbf{G}} \rangle}  & \quad &+ \quad& \langle \mathbf{y}^{-n} \circ \mathbf{w}_R \cdot x , \mathbf{G} \rangle                     &\quad &+\quad & \langle \mathbf{s}_L \cdot x^3 , \mathbf{G} \rangle \\\\
                                                +      &\quad &  \quad &  +                                                           & \quad &  \quad &  +                                                         & \quad &  \quad& +                                                                                            &\quad & \quad & +   \\\\
  {\langle {\mathbf{r}}(x), \hat{\mathbf{H}} \rangle}  &\quad &= \quad & \langle \mathbf{a}_R \cdot x, {\mathbf{H}} \rangle          & \quad &+ \quad & - \langle \mathbf{1}, \mathbf{H} \rangle                   & \quad &+ \quad& \langle \mathbf{y}^{-n} \circ (\mathbf{w}_L \cdot x + \mathbf{w}_O), \mathbf{H} \rangle    &\quad &+\quad & \langle \mathbf{s}_R \cdot x^3 , \mathbf{H} \rangle \\\\
                                                +      &\quad &  \quad &  +                                                           & \quad &  \quad &  +                                                         & \quad &  \quad& +                                                                                            &\quad & \quad & +   \\\\
  \tilde{e} \cdot \widetilde{B}                        &\quad &= \quad & \tilde{a} \cdot x \cdot \widetilde{B}                        & \quad &+ \quad & \tilde{o} \cdot x^2 \cdot \widetilde{B}                    & \quad &+ \quad& 0                                                                                            &\quad &+\quad & \tilde{s} \cdot x^3 \cdot \widetilde{B} \\\\
                                    \shortparallel     &\quad &  \quad & \shortparallel                                               & \quad &  \quad & \shortparallel                                             & \quad &  \quad& \shortparallel                                                                               &\quad & \quad & \shortparallel   \\\\
                                                       &\quad &= \quad & x \cdot A_I                                                  & \quad &+ \quad & x^2 \cdot A_O - \langle \mathbf{1}, \mathbf{H} \rangle     & \quad &+ \quad& W_L \cdot x + W_R \cdot x + W_O                                                              &\quad &+\quad & x^3 \cdot S
\end{aligned}
$$
We can interpret the rows and columns similarly to the previous diagram:
the sum of each column is a vector Pedersen commitment with left and right halves from the first and second rows respectively
and blinding factor from the third row.
The sum of all of the columns is a vector
Pedersen commitment to ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$ with
synthetic blinding factor ${\widetilde{e}}$.

To convince the verifier that
$t(x) = {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle}$, the prover
sends ${\widetilde{e}}$ to the verifier, who uses the bottom row
to compute
$$
\begin{aligned}
  P &= -{\widetilde{e}} {\widetilde{B}} + x \cdot A_I + x^2 \cdot A_O - \langle \mathbf{1}, \mathbf{H} \rangle + W_L \cdot x + W_R \cdot x + W_O + x^3 \cdot S \\\\
\end{aligned}
$$
if the prover is honest, this is
$P = {\langle {\mathbf{l}}(x), {\mathbf{G}} \rangle} + {\langle {\mathbf{r}}(x), \hat{\mathbf{H}} \rangle}$,
so the verifier uses $P$ and $t(x)$ as inputs to the [inner product protocol](../notes/index.html#inner-product-proof)
to prove that
$t(x) = {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle}$.



## Verifying challenge-based constraints

Some [gadgets](#gadgets) can be made more efficient if they can [sample random challenges](#gadget-as-a-challenge) when building
a constraint system, provided certain variables are committed. For instance, a shuffle gadget
can use just $2(k-1)$ multipliers for $k$ inputs and $k$ outputs to implement the check 
that two polynomials are equal up to a permutation of their roots, but it is only sound as long
as the roots of the polynomials (the inputs/outputs to the shuffle) are fixed before the evaluation point is chosen.

To faciliate this, we split the vectors of multipliers and their blinding factors in two subvectors of lengths $n'$ and $n''$: 
$$
\begin{aligned}
 n                &= n' + n''                              \\\\
 {\mathbf{a}}_L  &= {\mathbf{a}}_L' || {\mathbf{a}}_L'' \\\\
 {\mathbf{a}}_R  &= {\mathbf{a}}_R' || {\mathbf{a}}_R'' \\\\
 {\mathbf{a}}_O  &= {\mathbf{a}}_O' || {\mathbf{a}}_O'' \\\\
 {\mathbf{s}}_L  &= {\mathbf{s}}_L' || {\mathbf{s}}_L'' \\\\
 {\mathbf{s}}_R  &= {\mathbf{s}}_R' || {\mathbf{s}}_R'' \\\\
\end{aligned}
$$
The vectors of flattened weights are split accordingly:
$$
\begin{aligned}
 \mathbf{w}_L  &= {\mathbf{w}}_L' || {\mathbf{w}}_L'' \\\\
 \mathbf{w}_R  &= {\mathbf{w}}_R' || {\mathbf{w}}_R'' \\\\
 \mathbf{w}_O  &= {\mathbf{w}}_O' || {\mathbf{w}}_O'' \\\\
\end{aligned}
$$
The statements of each slice of the vectors $\mathbf{l}(x), \mathbf{r}(x)$ become:
$$
\begin{aligned}
                                 \mathbf{l}'(x) &= \mathbf{a}'_L \cdot x  + \mathbf{s}_L' \cdot x^3  +        \mathbf{y}^{-n'}  \circ \mathbf{w}_R'  \cdot x + \mathbf{a}_O' \cdot x^2\\\\
                                \mathbf{l}''(x) &= \mathbf{a}''_L \cdot x + \mathbf{s}_L'' \cdot x^3 + y^{-n'}\mathbf{y}^{-n''} \circ \mathbf{w}_R'' \cdot x + \mathbf{a}_O'' \cdot x^2\\\\
          \mathbf{y}^{-n'} \circ \mathbf{r}'(x) &= \mathbf{a}'_R \cdot x  + \mathbf{s}_R' \cdot x^3  +        \mathbf{y}^{-n'}  \circ \mathbf{w}_L' \cdot x  - \mathbf{1}^{n'}  +        \mathbf{y}^{-n'}  \circ \mathbf{w}_O'\\\\
y^{-n'} \mathbf{y}^{-n''} \circ \mathbf{r}''(x) &= \mathbf{a}''_R \cdot x + \mathbf{s}_R'' \cdot x^3 + y^{-n'}\mathbf{y}^{-n''} \circ \mathbf{w}_L'' \cdot x - \mathbf{1}^{n''} + y^{-n'}\mathbf{y}^{-n''} \circ \mathbf{w}_O''\\\\
\end{aligned}
$$
Now we need to express the statements above using independent commitments to the subvectors $\mathbf{a}'_{L,R,O}$ and $\mathbf{a}''_{L,R,O}$.
The commitments must be independent because second subvectors are computed with the use of challenges generated _after_ the first subvectors are determined and committed.

To do that, we split vectors of generators and combine the statements in two:
the first one in terms of the commitments to the first subvectors,
and the second one in terms of the commitments to the second subvectors.
$$
\begin{aligned}
\mathbf{G} &= \mathbf{G}' || \mathbf{G}'' \\\\
\mathbf{H} &= \mathbf{H}' || \mathbf{H}'' \\\\
\end{aligned}
$$

$$
\begin{aligned}
{\langle \mathbf{l}'(x), {\mathbf{G}'} \rangle}                                   &\quad &= \quad & x \cdot {\langle \mathbf{a}'_L, \mathbf{G}' \rangle}       & \quad &+ \quad x^2 \cdot {\langle \mathbf{a}'_O, \mathbf{G}' \rangle}    & \quad &+ \quad \langle x \cdot \mathbf{y}^{-n'} \circ \mathbf{w}_R', \mathbf{G}' \rangle                                 &\quad &+\quad  x^3 \cdot \langle \mathbf{s}'_L , \mathbf{G}' \rangle \\\\
                                                                      +           &\quad &  \quad &  +                                                          & \quad &  \quad \quad  +                                                   & \quad &  \quad \quad +                                                                                                    &\quad & \quad  \quad +   \\\\
{\langle  \mathbf{y}^{-n'} \circ \mathbf{r}'(x), {\mathbf{H}'} \rangle}           &\quad &= \quad & x \cdot {\langle \mathbf{a}'_R, \mathbf{H}' \rangle}       & \quad &- \quad \langle \mathbf{1}, \mathbf{H}' \rangle                    & \quad &+ \quad \langle \mathbf{y}^{-n'} \circ (x \cdot \mathbf{w}_L'  + \mathbf{w}_O'), \mathbf{H}' \rangle             &\quad &+\quad  x^3 \cdot \langle \mathbf{s}'_R , \mathbf{H}' \rangle \\\\
                                                                                  &\quad &  \quad &                                                             & \quad &  \quad \quad                                                      & \quad &  \quad \quad                                                                                                      &\quad & \quad  \quad     \\\\
{\langle \mathbf{l}''(x), {\mathbf{G}''} \rangle}                                 &\quad &= \quad & x \cdot {\langle \mathbf{a}''_L, \mathbf{G}'' \rangle}     & \quad &+ \quad x^2 \cdot {\langle \mathbf{a}''_O, \mathbf{G}'' \rangle}  & \quad &+ \quad \langle x \cdot y^{-n'} \mathbf{y}^{-n''} \circ \mathbf{w}_R'', \mathbf{G}'' \rangle                      &\quad &+\quad  x^3 \cdot \langle \mathbf{s}''_L , \mathbf{G}'' \rangle \\\\
                                                                            +     &\quad &  \quad &  +                                                          & \quad &  \quad \quad  +                                                   & \quad &  \quad \quad +                                                                                                    &\quad & \quad  \quad +   \\\\
{\langle  y^{n'} \mathbf{y}^{-n''} \circ \mathbf{r}''(x), {\mathbf{H}''} \rangle} &\quad &= \quad & x \cdot {\langle \mathbf{a}''_R, \mathbf{H}'' \rangle}     & \quad &- \quad \langle \mathbf{1}, \mathbf{H}'' \rangle                   & \quad &+ \quad \langle y^{-n'} \mathbf{y}^{-n''} \circ (x \cdot \mathbf{w}_L''  + \mathbf{w}_O''), \mathbf{H}'' \rangle &\quad &+\quad  x^3 \cdot \langle \mathbf{s}''_R , \mathbf{H}'' \rangle \\\\
\end{aligned}
$$

We need to combine the above statements in one in order to have an expression for the complete vectors $\mathbf{l}(x), \mathbf{r}(x)$.
For that we will multiply the second statement by a random challenge $u \in {\mathbb Z_{p}^{\times}}$, and add it to the first statement.
$$
\begin{aligned}
{\langle \mathbf{l}'(x), {\mathbf{G}'} \rangle}                                           &\quad &= \quad & x \cdot {\langle \mathbf{a}'_L, \mathbf{G}' \rangle}               & \quad &+ \quad x^2 \cdot {\langle \mathbf{a}'_O, \mathbf{G}' \rangle}            & \quad &+ \quad \langle x \cdot \mathbf{y}^{-n'} \circ \mathbf{w}_R', \mathbf{G}' \rangle                                         &\quad &+\quad  x^3 \cdot \langle \mathbf{s}'_L , \mathbf{G}' \rangle \\\\
                                                                      +                   &\quad &  \quad &  +                                                                  & \quad &  \quad \quad  +                                                           & \quad &  \quad \quad +                                                                                                            &\quad & \quad  \quad +   \\\\
{\langle  \mathbf{y}^{-n'} \circ \mathbf{r}'(x), {\mathbf{H}'} \rangle}                   &\quad &= \quad & x \cdot {\langle \mathbf{a}'_R, \mathbf{H}' \rangle}               & \quad &- \quad \langle \mathbf{1}, \mathbf{H}' \rangle                            & \quad &+ \quad \langle \mathbf{y}^{-n'} \circ (x \cdot \mathbf{w}_L'  + \mathbf{w}_O'), \mathbf{H}' \rangle                     &\quad &+\quad  x^3 \cdot \langle \mathbf{s}'_R , \mathbf{H}' \rangle \\\\
                                                                      +                   &\quad &  \quad &  +                                                                  & \quad &  \quad \quad  +                                                           & \quad &  \quad \quad +                                                                                                            &\quad & \quad  \quad     \\\\
{\langle u \cdot \mathbf{l}''(x), {\mathbf{G}''} \rangle}                                 &\quad &= \quad & u \cdot x \cdot {\langle \mathbf{a}''_L, \mathbf{G}'' \rangle}     & \quad &+ \quad u \cdot x^2 \cdot {\langle \mathbf{a}''_O, \mathbf{G}'' \rangle}  & \quad &+ \quad u \cdot \langle x \cdot y^{-n'} \mathbf{y}^{-n''} \circ \mathbf{w}_R'', \mathbf{G}'' \rangle                      &\quad &+\quad  u \cdot x^3 \cdot \langle \mathbf{s}''_L , \mathbf{G}'' \rangle \\\\
                                                                      +                   &\quad &  \quad &  +                                                                  & \quad &  \quad \quad  +                                                           & \quad &  \quad \quad +                                                                                                            &\quad & \quad  \quad +   \\\\
{\langle u \cdot  y^{n'} \mathbf{y}^{-n''} \circ \mathbf{r}''(x), {\mathbf{H}''} \rangle} &\quad &= \quad & u \cdot x \cdot {\langle \mathbf{a}''_R, \mathbf{H}'' \rangle}     & \quad &- \quad u \cdot \langle \mathbf{1}, \mathbf{H}'' \rangle                   & \quad &+ \quad u \cdot \langle y^{-n'} \mathbf{y}^{-n''} \circ (x \cdot \mathbf{w}_L''  + \mathbf{w}_O''), \mathbf{H}'' \rangle &\quad &+\quad  u \cdot x^3 \cdot \langle \mathbf{s}''_R , \mathbf{H}'' \rangle \\\\
\end{aligned}
$$
The commitments to the components of $\mathbf{l}(x)$ and $\mathbf{r}(x)$ are then split up as follows:
$$
\begin{aligned}
  A_I'  &= \langle \mathbf{G}'  , \mathbf{a}_L'  \rangle + \langle \mathbf{H}', \mathbf{a}_R' \rangle + \widetilde{B} \cdot \tilde{a}'  \\\\
  A_I'' &= \langle \mathbf{G}'' , \mathbf{a}_L'' \rangle + \langle \mathbf{H}'', \mathbf{a}_R'' \rangle + \widetilde{B} \cdot \tilde{a}'' \\\\
  A_O'  &= \langle \mathbf{G}'  , \mathbf{a}_O'  \rangle + \widetilde{B} \cdot \tilde{o}'  \\\\
  A_O'' &= \langle \mathbf{G}'' , \mathbf{a}_O'' \rangle + \widetilde{B} \cdot \tilde{o}'' \\\\
  S'    &= \langle \mathbf{G}'  , \mathbf{s}_L'  \rangle + \langle \mathbf{H}', \mathbf{s}_R' \rangle + \widetilde{B} \cdot \tilde{s}'  \\\\
  S''   &= \langle \mathbf{G}'' , \mathbf{s}_L'' \rangle + \langle \mathbf{H}'', \mathbf{s}_R'' \rangle + \widetilde{B} \cdot \tilde{s}'' \\\\
\end{aligned}
$$
We can now relate the above commitments to ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$ using a new diagram:
$$
\begin{aligned}
{\langle {\mathbf{l}}(x), \hat{\mathbf{G}} \rangle}     &\quad &= \quad & x \cdot \big( {\langle \mathbf{a}'_L, \mathbf{G}' \rangle} + u \cdot {\langle \mathbf{a}''_L, \mathbf{G}'' \rangle} \big)      & \quad &+ \quad x^2 \cdot \big( {\langle \mathbf{a}'_O, \mathbf{G}' \rangle}  +  u \cdot {\langle \mathbf{a}''_O, \mathbf{G}'' \rangle} \big)         & \quad &+ \quad \langle \mathbf{y}^{-n} \circ (\mathbf{w}_R' || u \cdot \mathbf{w}_R'') \cdot x , \mathbf{G} \rangle                                                                     &\quad &+\quad  x^3 \cdot \big( \langle \mathbf{s}'_L , \mathbf{G}' \rangle + u \cdot \langle \mathbf{s}''_L , \mathbf{G}'' \rangle \big) \\\\
                                                 +      &\quad &  \quad &  +                                                                                                                               & \quad &  \quad \quad  +                                                                                                                                & \quad &  \quad \quad +                                                                                                                                                                    &\quad & \quad  \quad +   \\\\
{\langle {\mathbf{r}}(x), \hat{\mathbf{H}} \rangle}     &\quad &= \quad & x \cdot \big( {\langle \mathbf{a}'_R, \mathbf{H}' \rangle} + u \cdot {\langle \mathbf{a}''_R, \mathbf{H}'' \rangle} \big)      & \quad &- \quad \langle \mathbf{1}, \mathbf{H}' \rangle -  u \cdot \langle \mathbf{1}, \mathbf{H}'' \rangle                                             & \quad &+ \quad \langle \mathbf{y}^{-n} \circ (\mathbf{w}_L' || u \cdot \mathbf{w}_L'') \cdot x + (\mathbf{w}_O' || u \cdot \mathbf{w}_O''), \mathbf{H} \rangle                        &\quad &+\quad  x^3 \cdot \big( \langle \mathbf{s}'_R , \mathbf{H}' \rangle + u \cdot \langle \mathbf{s}''_R , \mathbf{H}'' \rangle \big) \\\\
                                                 +      &\quad &  \quad &  +                                                                                                                               & \quad &  \quad \quad +                                                                                                                                 & \quad &  \quad \quad +                                                                                                                                                                    &\quad & \quad  \quad +   \\\\
                     \tilde{e} \cdot \widetilde{B}      &\quad &= \quad & x \cdot \big( \tilde{a}' \cdot \widetilde{B} + u \tilde{a}'' \cdot \widetilde{B} \big)                                           & \quad &+ \quad x^2 \cdot \big( \tilde{o}' \cdot \widetilde{B} + u \cdot \tilde{o}'' \cdot \widetilde{B} \big)                                          & \quad &+ \quad 0                                                                                                                                                                          &\quad &+\quad  x^3 \cdot \big( \tilde{s}' \cdot \widetilde{B} + u \tilde{s}'' \cdot \widetilde{B} \big) \\\\
                                     \shortparallel     &\quad &  \quad & \shortparallel                                                                                                                   & \quad &  \quad \quad \shortparallel                                                                                                                    & \quad &  \quad \quad \shortparallel                                                                                                                                                       &\quad & \quad  \quad \shortparallel   \\\\
                                                        &\quad &= \quad & x \cdot \big(A_I' + u \cdot A_I'')                                                                                               & \quad &+ \quad x^2 \cdot \big(A_O' + u \cdot A_O'' \big) - \langle \mathbf{1}, \mathbf{H}' \rangle - u \cdot \langle \mathbf{1}, \mathbf{H}'' \rangle  & \quad &+ \quad x \cdot \langle \mathbf{w}_L,  \hat{\mathbf{H}} \rangle + x \cdot \langle \mathbf{w}_R,  \hat{\mathbf{G}} \rangle + \langle \mathbf{w}_O,  \hat{\mathbf{H}} \rangle     &\quad &+\quad  x^3 \cdot (S' + u \cdot S'')
\end{aligned}
$$
with generators transmuted using challenges $y$ and $u$:
$$
\begin{aligned}
\hat{\mathbf{G}} &= \mathbf{G}' || (u \cdot \mathbf{G}''), \\\\
\hat{\mathbf{H}} &= \mathbf{y}^{-n} \circ \big( \mathbf{H}' || (u \cdot \mathbf{H}'') \big). \\\\
\end{aligned}
$$
The sum of each column is a vector Pedersen commitment with left and right halves from the first and second rows respectively
and blinding factor from the third row.
The sum of all of the columns is a vector
Pedersen commitment to ${\mathbf{l}}(x)$ and ${\mathbf{r}}(x)$ with
synthetic blinding factor $\widetilde{e} = (\tilde{a}' + u \tilde{a}'') \cdot x + (\tilde{o}' + u \tilde{o}'') \cdot x^2 + (\tilde{s}' + u \tilde{s}'') \cdot x^3$.

To convince the verifier that $\mathbf{l}(x)$ and $\mathbf{r}(x)$ are computed correctly,
the prover can send the evaluations $\mathbf{l}(x), \mathbf{r}(x)$ along with $\tilde{e}$ to the verifier,
who uses the bottom row of the diagram to check the following statement:
$$
\begin{aligned}
{\langle {\mathbf{l}}(x), \hat{\mathbf{G}} \rangle} + {\langle {\mathbf{r}}(x), \hat{\mathbf{H}} \rangle} \stackrel{?}{=}
&-{\widetilde{e}} {\widetilde{B}} + x \cdot (A_I' + u \cdot A_I'') + x^2 \cdot (A_O' + u \cdot A_O'')\\\\
&- \langle \mathbf{1}, \mathbf{H}' \rangle - u \cdot \langle \mathbf{1}, \mathbf{H}'' \rangle +
x \cdot \langle \mathbf{w}_L,  \hat{\mathbf{H}} \rangle + x \cdot \langle \mathbf{w}_R,  \hat{\mathbf{G}} \rangle + \langle \mathbf{w}_O,  \hat{\mathbf{H}} \rangle +
x^3 \cdot (S' + u \cdot S'') \\\\
\end{aligned}
$$


## Compressing the proof with an inner product argument

Once the verifier has checked correctness of $t(x)$, $\mathbf{l}(x)$ and $\mathbf{r}(x)$,
they can directly compute the inner product to check the relation $t(x) \stackrel{?}{=} {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle}$.
This, however, would require transmitting $2n$ 32-byte elements representing the vectors $\mathbf{l}(x)$ and $\mathbf{r}(x)$.

To make the proof smaller, the prover will use the [inner product argument](../notes/index.html#inner-product-proof)
to indirectly prove the inner product relation using $t(x)$ and the vectors represented
by a commitment $P = {\langle {\mathbf{l}}(x), \hat{\mathbf{G}} \rangle} + {\langle {\mathbf{r}}(x), \hat{\mathbf{H}} \rangle}$.

The verifier checks the inner product proof with $P$ computed using the bottom row of the diagram,
which proves that the vectors $\mathbf{l}(x), \mathbf{r}(x)$ are computed correctly:
$$
\begin{aligned}
P = &-{\widetilde{e}} {\widetilde{B}} + x \cdot (A_I' + u \cdot A_I'') + x^2 \cdot (A_O' + u \cdot A_O'') \\\\
    &- \langle \mathbf{1}, \mathbf{H}' \rangle - u \cdot \langle \mathbf{1}, \mathbf{H}'' \rangle +
    x \cdot \langle \mathbf{w}_L,  \hat{\mathbf{H}} \rangle + x \cdot \langle \mathbf{w}_R,  \hat{\mathbf{G}} \rangle + \langle \mathbf{w}_O,  \hat{\mathbf{H}} \rangle +
    x^3 \cdot (S' + u \cdot S'') \\\\
\end{aligned}
$$
If the inner product proof with such $P$ is correct, the verifier is convinced of two facts:
that $t(x) = {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle}$, and
$\mathbf{l}(x), \mathbf{r}(x)$ are correct.

## Padding $\mathbf{l}(x)$ and $\mathbf{r}(x)$ for the inner product proof

The above discussion did not have restrictions on the value $n$.
However, the [inner product argument](../notes/index.html#inner-product-proof)
requires the input vectors to have power-of-two elements: $n^{+} = 2^k$.
To resolve this mismatch, we need to specify how to pad vectors $\mathbf{l}(x), \mathbf{r}(x)$
and related commitments before we can use the inner product argument.

Our goal is to translate the _padding of the constraint system_ into the _padding of proof data_,
so we can keep the constraint system small and perform less computations in proving and verification.

We will use the following notation for the padding:
$$
\begin{aligned}                           n^{+} &= 2^{\lceil \log_2 n \rceil} \\\\                      \mathbf{0} &= [0,...,0]\end{aligned}
$$
We start by padding the entire constraint system:
multipliers are padded with all-zero assignments $a_{L,j}, a_{R,j}, a_{O,j}$,
all-zero blinding factors $s_{L,j}, s_{R,j}$,
and all-zero weights $W_{R,i,j}, W_{L,i,j}, W_{O,i,j}$,
for all constraints $i \in [0, q)$ and all additional multipliers $j \in [n,n^{+})$:
$$
\begin{aligned}
\mathbf{a}_L^{+} &= \mathbf{a}_L \hspace{0.1cm} || \hspace{0.1cm} \mathbf{0} \\\\
\mathbf{a}_R^{+} &= \mathbf{a}_R \hspace{0.1cm} || \hspace{0.1cm} \mathbf{0} \\\\
\mathbf{a}_O^{+} &= \mathbf{a}_O \hspace{0.1cm} || \hspace{0.1cm} \mathbf{0} \\\\
\mathbf{s}_L^{+} &= \mathbf{s}_L \hspace{0.1cm} || \hspace{0.1cm} \mathbf{0} \\\\
\mathbf{s}_R^{+} &= \mathbf{s}_R \hspace{0.1cm} || \hspace{0.1cm} \mathbf{0} \\\\
\mathbf{W}_L^{+} &= \mathbf{W}_L \hspace{0.1cm} || \hspace{0.1cm} [\mathbf{0}, ..., \mathbf{0}] \\\\
\mathbf{W}_R^{+} &= \mathbf{W}_R \hspace{0.1cm} || \hspace{0.1cm} [\mathbf{0}, ..., \mathbf{0}] \\\\
\mathbf{W}_O^{+} &= \mathbf{W}_O \hspace{0.1cm} || \hspace{0.1cm} [\mathbf{0}, ..., \mathbf{0}] \\\\
\end{aligned}
$$
As a result, we have to take larger slices of the vectors of generators $\mathbf{G},\mathbf{H}$ and more powers of the challenge $y$:
$$
\begin{aligned}
\mathbf{G}^{+}     &= \mathbf{G}   \hspace{0.1cm} || \hspace{0.1cm} [G_n,...,G_{n^{+}-1}] \\\\
\mathbf{H}^{+}     &= \mathbf{H}   \hspace{0.1cm} || \hspace{0.1cm} [H_n,...,H_{n^{+}-1}] \\\\
\mathbf{y}^{n^{+}} &= \mathbf{y}^n \hspace{0.1cm} || \hspace{0.1cm} [y^n,...,y^{n^{+}-1}] \\\\
\end{aligned}
$$
The low-level variables are padded with zeroes, which is equivalent to padding only the second-phase variables
(since we pad the combined vector of all variables on the right end).
If there are no second-phase variables, we can consider their vector to be of length 0 before padding,
but still apply padding to that vector, instead of the vector of first-phase variables.

Since we never pad first-phase variables, it is obvious that the commitments $A_I', A_O', S'$ remain unchanged.

Commitments to the second-phase low-level variables remain unchanged since the padding scalars are zeroes:
$$
\begin{aligned}
{A_I''}^{+} &= \widetilde{B} \cdot \tilde{a}'' + \langle {\mathbf{G}''}^{+}, {\mathbf{a}''}_L^{+} \rangle + \langle {\mathbf{H}''}^{+}, {\mathbf{a}''}_R^{+} \rangle \\\\
            &= \widetilde{B} \cdot \tilde{a}'' + \langle {\mathbf{G}''}, \mathbf{a}_L'' \rangle + \langle {\mathbf{H}''}, \mathbf{a}_R'' \rangle +
               \langle [G_n, ..., G_{n^{+}-1}], \mathbf{0} \rangle + \langle [H_n, ..., H_{n^{+}-1}], \mathbf{0} \rangle \\\\
            &= \widetilde{B} \cdot \tilde{a}'' + \langle {\mathbf{G}}'', \mathbf{a}_L'' \rangle + \langle {\mathbf{H}}'', \mathbf{a}_R'' \rangle + 0 \\\\
            &= A_I'' \\\\
\end{aligned}
$$
Similarly, $A_O''$ and $S''$ are unchanged:
$$
\begin{aligned}
{A_O''}^{+} &= A_O'' \\\\
{S''}^{+}   &= S''
\end{aligned}
$$
The flattened weight vectors $\mathbf{w}_{L,R,O}$ are padded with zeroes
because the corresponding weights are padded with zeroes:
$$
\begin{aligned}
\mathbf{w}_L^{+} &= z \mathbf{z}^q \cdot \mathbf{W}_L^{+}  &{}={}& (z \mathbf{z}^q \cdot \mathbf{W}_L) || (z \mathbf{z}^q \cdot \mathbf{0}) &{}={}& \mathbf{w}_L || \mathbf{0}, \\\\
\mathbf{w}_R^{+} &= z \mathbf{z}^q \cdot \mathbf{W}_R^{+}  &{}={}& (z \mathbf{z}^q \cdot \mathbf{W}_R) || (z \mathbf{z}^q \cdot \mathbf{0}) &{}={}& \mathbf{w}_R || \mathbf{0}, \\\\
\mathbf{w}_O^{+} &= z \mathbf{z}^q \cdot \mathbf{W}_O^{+}  &{}={}& (z \mathbf{z}^q \cdot \mathbf{W}_O) || (z \mathbf{z}^q \cdot \mathbf{0}) &{}={}& \mathbf{w}_O || \mathbf{0}. \\\\
\end{aligned}
$$
The $\delta(y,z)$ remains unchanged because the padding weights are zeroes:
$$
\begin{aligned}
\delta(y, z)^{+} &= \langle \mathbf{y}^{-n^{+}} \circ \mathbf{w}_R^{+}, \mathbf{w}_L^{+} \rangle \\\\
                 &= \langle \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{w}_L \rangle      +     \langle [y^n,...,y^{n^{+}-1}] \circ \mathbf{0}, \mathbf{0} \rangle \\\\
                 &= \langle \mathbf{y}^{-n} \circ \mathbf{w}_R, \mathbf{w}_L \rangle      +     0 \\\\
                 &= \delta(y, z)
\end{aligned}
$$
Vector polynomial $\mathbf{l}(x)$ is padded with zeroes:
$$
\begin{aligned}
\mathbf{l}(x)^{+} &= \mathbf{a}_L^{+} \cdot x + \mathbf{s}_L^{+} \cdot x^3 + \mathbf{y}^{-n^{+}} \circ \mathbf{w}_R^{+} \cdot x + \mathbf{a}_O^{+} \cdot x^2 \\\\
                  &= \mathbf{a}_L \cdot x + \mathbf{s}_L \cdot x^3 + \mathbf{y}^{-n} \circ \mathbf{w}_R \cdot x + \mathbf{a}_O \cdot x^2 \\\\
                  &  \hspace{0.5cm} || \hspace{0.1cm} \mathbf{0} \cdot x + \mathbf{0} \cdot x^3 + [y^n,...,y^{n^{+}-1}] \circ \mathbf{0} \cdot x + \mathbf{0} \cdot x^2 \\\\
                  &= \mathbf{l}(x) || \mathbf{0} \\\\
\end{aligned}
$$
Vector polynomial $\mathbf{r}(x)$ is padded with additional (negated) powers of $y$:
$$
\begin{aligned}
\mathbf{r}(x)^{+} &= \mathbf{y}^{n^{+}} \circ \mathbf{a}_R^{+} \cdot x + \mathbf{y}^{n^{+}} \circ \mathbf{s}_R^{+} \cdot x^3 + \mathbf{w}_L^{+} \cdot x - \mathbf{y}^{n^{+}} + \mathbf{w}_O^{+} \\\\
                  &= \mathbf{y}^n \circ \mathbf{a}_R \cdot x + \mathbf{y}^n \circ \mathbf{s}_R \cdot x^3 + \mathbf{w}_L \cdot x - \mathbf{y}^n + \mathbf{w}_O \\\\
                  &  \hspace{0.5cm} || \hspace{0.1cm} [y^n,...,y^{n^{+}-1}] \circ \mathbf{0} \cdot x + [y^n,...,y^{n^{+}-1}] \circ \mathbf{0} \cdot x^3 + \mathbf{0} \cdot x - [y^n,...,y^{n^{+}-1}] + \mathbf{0} \\\\
                  &= \mathbf{r}(x) || [-y^n,...,-y^{n^{+}-1}]
\end{aligned}
$$
Transmuted generators are padded as follows:
$$
\begin{aligned}
   \hat{\mathbf{G}}^{+}  &= \mathbf{G}' \hspace{0.1cm} || \hspace{0.1cm} u \cdot \mathbf{G}'' \hspace{0.1cm} || \hspace{0.1cm} u \cdot [G_n,...,G_{n^{+}-1}] \\\\
   \hat{\mathbf{H}}^{+}  &= \mathbf{y}^{-n^{+}} \circ \big( \mathbf{H}' \hspace{0.1cm} || \hspace{0.1cm} u \cdot \mathbf{H}'' \hspace{0.1cm} || \hspace{0.1cm} u \cdot [H_n,...,H_{n^{+}-1}]\big) \\\\
\end{aligned}
$$
The commitment $P$ is also padded:
$$
\begin{aligned}
P^{+} &=& &-{\widetilde{e}} {\widetilde{B}} + x \cdot (A_I' + u \cdot A_I'') + x^2 \cdot (A_O' + u \cdot A_O'') \\\\
      &&  &- \langle \mathbf{1}, \mathbf{H}' \rangle - u \cdot \langle \mathbf{1}, {\mathbf{H}''}^{+} \rangle +
          x \cdot \langle \mathbf{w}_L || \mathbf{0},  \hat{\mathbf{H}}^{+} \rangle + x \cdot \langle \mathbf{w}_R || \mathbf{0},  \hat{\mathbf{G}}^{+} \rangle + \langle \mathbf{w}_O || \mathbf{0},  \hat{\mathbf{H}}^{+} \rangle +
          x^3 \cdot (S' + u \cdot S'') \\\\
      &=& &P - \langle u, [H_n,...,H_{n^{+}-1}] \rangle
\end{aligned}
$$
The inner product $t(x)$ remains unchanged because the non-zero padding in the right vector gets multiplied with the zero padding in the left vector:
$$
\begin{aligned}
t(x)^{+} &= {\langle {\mathbf{l}}(x)^{+}, {\mathbf{r}}(x)^{+} \rangle} \\\\
         &= {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle} + {\langle \mathbf{0}, [y^n,...,y^{n^{+}-1}] \rangle} \\\\
         &= {\langle {\mathbf{l}}(x), {\mathbf{r}}(x) \rangle} + 0 \\\\
         &= t(x)
\end{aligned}
$$
This implies that the terms $t_{0, 1, 2, 3, 4, 5, 6}$ also remain unchanged.

# Aggregated Constraint System Proof

(Under development.)

Range proofs can be naturally aggregated keeping each statement independent.
For constraint systems proofs, two options exist:

1. each party can prove satisfiability of **their own constraint system** (systems can be distinct);
2. parties can collaborate to prove satisfiability of a **single constraint system** without having to reveal secrets to each other.

The aggregation of distinct proofs can be done in a very similar way
to the aggregation of range proofs and is useful purely for the space savings (just like with the range proofs).

The collaborative construction of a proof of a single constraint system requires a different framework,
but is very useful for computations that increase privacy for each party, e.g. by allowing them to mix their inputs,
while not making them share secrets between each other.  
  
  
  
  
  
  
  
  

 
 
    
  
  ## Various Ways to Implement R1CS 
- Cf. [2.] above regarding the “binary file format” versa vis “JSON lines format” (sizes of the constraints) 
- Implementation cases: Zcash, Zcoin,… Stellar (cf. Cathie Yun’s article “Programmable Constraint Systems for Bulletproofs”
- "Libra: Succinct Zero-Knowledge Proof with Optimal Prover Computation" - Tiancheng Xie et al, eprint.iacr.org/2019/317.pdf  


