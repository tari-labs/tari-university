
# Rank-1 Constraint Systems 

## What is a Rank-1 Constraint System


# PART I: R1CS - as used in *zkSNARKs*  
 


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
 
 
 In order to prove how short a zk-SNARK is, Christian Lundkvist tweeted this definition @ChrisLundkvist;

						Generator (C circuit, λ is ☣️):
						(pk, vk) = G(λ, C)
						Prover (x pub inp, w sec inp):
						π = P(pk, x, w)
						Verifier:
						V(vk, x, π) == (∃ w s.t. C(x,w))
						
							6:53 pm - 18 Nov 2016


 
  
  "The first constructions of SNARK protocols were inspired by the PCP theorem which shows that NP statements have ‘short’ probabilistic checkable proofs. New instantiations were found which allow faster and shorter proofs, when a pre-processing state is permitted" [4.]   
  
 
  ##  5-step process of creating a zk-SNARK for a computation 

zk-SNARKs are not applied directly to computational problems. Therefore every computational problem the Verifier challenges the Prover with needs to first be converted into a 'form' called *Quadratic Arithmetic Program* (QAP), and the QAP is in turn transformed into a *Linear Probabilistically Checkable Proof* (PCP). So then, starting with a computational problem, a zk-SNARK is achieved by a 5-step process; 
  	 
  **Computational Problem —> Arithmetic Circuit —> R1CS —> QAP —> Linear PCP —> zk-SNARK** 

Eran Tromer adds an extra 'form' between a linear PCP and the zk-SNARKs, and that's *Linear Interactive Proof*;  [check "Notes App" for diagram]
  
Our focus in this report is on the R1CS form. 
  
  **Definition**: [2.]  
  An R1CS is a sequence of groups of three vectors  $( a, b, c )$ , and the solution to an R1CS is a vector  $s$  that satisfies the equation 
  $ ( s \cdot a ) * ( s \cdot b ) - ( s \cdot c ) = 0 $ $ \ \ $  
  where  "$\cdot$"  is the dot-product of vectors. 
  
In the context of verification using zero-knowledge proofs, the solution  $s$  in the above definition would be the solution to a computational problem posed by the Verifier. The Prover needs to have  $s$, and prove to the verifier that they are in possession of  $s$.     
  
  
 
  
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

  
### Remarks 

Firstly, note that the zk-SNARK end-to-end journey is to create a function or a protocol that takes the proof, given by the Prover, and checks its veracity. Secondly, in using an R1CS, we note that "an R1CS is not a computer program, you are not asking it to produce a value from certain inputs. Instead, an R1CS is a verifier, it shows that an already complete computation is correct" [3.]  

  Also, the main aim is not for the Verifier to pose a difficult computational problem at the Prover. The challenge may be very easy, as seen in the above example. In other words, this is not proof-of-work but validation. 
  
"To do so, the R1CS must receive as input an assignment of the full state of the computation. This is defined by the value of all the variables used in the process, and because each was set only once, its final value is equal to what it was when it was used. Therefore, by getting a snapshot of the values of all variables we can verify they have all been properly computed and guarantee the computation was correct one step at a time." [3.] 
  

"*" 


****** 
In zk-SNARKs schemes, the statement being proved is reduced to what is called a rank-1 constraint system, or R1CS. In this system, the prover is given a system of arithmetic constraints over a set of variables (elements in a large prime field $\mathcal{ F_r }$), and asked to produce an assignment to the variables that satisfy the constraints.  
 
   
   
   
   
   
   
   
   
   
   
 ## References 

  [1.] https://diyhpl.us/wiki/transcripts/cryptoeconomic-systems/2019/zksharks/   
  
  [2.] Vitalik Buterin, "Quadratic Arithmetic Programs: from Zero to Hero", Dec 12, 2016. https://medium.com/@VitalikButerin/quadratic-arithmetic-programs-from-zero-to-hero-f6d558cea649 
  
  [3.] Pinto, Alex, "Constraint Systems for ZK SNARKs", Mar 06, 2019. http://coders-errand.com/constraint-systems-for-zk-snarks/ 
  
  [4.] Mayer, Hartwig; *zk-SNARKS Explained: Basic Principles*, CoinFabrik, 06 Dec., 2016.
   
 
  ​  
  
# PART II: R1CS - as used in *Bulletproofs*  
  
  
  
  
  ## Various Ways to Implement R1CS 
- Cf. [2.] above regarding the “binary file format” versa vis “JSON lines format” (sizes of the constraints) 
- Implementation cases: Zcash, Zcoin,… Stellar (cf. Cathie Yun’s article “Programmable Constraint Systems for Bulletproofs”
- "Libra: Succinct Zero-Knowledge Proof with Optimal Prover Computation" - Tiancheng Xie et al, eprint.iacr.org/2019/317.pdf  


