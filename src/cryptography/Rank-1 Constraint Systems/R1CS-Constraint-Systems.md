
# Rank-1 Constraint Systems 

## What is a Rank-1 Constraint System


# PART I: R1CS - as used in *zkSNARKs*  
 


  ## Zero-Knowledge SNARKs (zk-SNARKs)
  
A *zero-knowledge proof* protocol allows two players, the Verifier and the Prover, to carry out a 3-step process,  
   - the Verifier challenges the Prover, say to carry out a certain computational problem, 
   - the Prover solves the problem and sends back proof that they have successfully completed the solution, 
   - the Verifier checks if indeed the proof is correct for the original challenge. 

A lot of work has been done in making these type of protocols practical, making them short and efficient. Research work over the last decade has produced several refinements of these proofs, even mechanisms to transform interactive zero-knowledge proofs to non-interactive zero-knowledge proofs (NIZKs). 

Of interest to us is the so called *zero-knowledge succinct non-interactive arguments of knowledge*, or zk-SNARKs. If we drop the technical requirement for *arguments of knowledge* to have NP completeness and soundness, zk-SNARKs become proofs, and hence a special class of NIZKs (cf. [1.] which contains a brief landscape NIZKs, and includes zk-SNARKs).  

 
 **Definition:** (zk-SNARKs)
 
 A typical zk-SNARK consists of three components  ( **G, P, V** ),  where  **G**  is the Key Generator,  **P**  is the Prover, and  **V**  is the Verifier. 
 
 **G** the Key Generator;
	•  **G** takes as inputs, 
		$$\lambda$$ , a secret parameter, and 
		$$C$$ , a given program. 
	•  The output of **G** is a pair of public keys, 
		$$pk$$ the *proving key* to be used by the Prover, and 
		$$vk$$ the *verification key* to be used by the Verifier. 
	
 **P** the Prover; 
	•  **P** takes 3 inputs,  
		$$pk$$ the prover key, 
		$$x$$ a public input, 
		$$w$$ a private witness 
	•  The output of **P** is a proof, 
		***prf*** = **P**($$pk , x, w$$).
   
 **V** the Verifier;   
	•  **V** takes 3 inputs 
		$$vk$$ the prover key, 
		$$x$$ a public input, 
		***prf*** a private witness. 
	•  **V** computes two values and checks equality, 
		First value  **V**( $$vk, x,$$ ***prf*** ) , 
		Second value  $$C( x, \eta )$$ for some $$\eta$$  in the range.    
 
 Note that  $$pk$$ and $$vk$$ are generated once and only for a given program  $$C$$. i.e. $$(pk, vk) = \bf{G}( \lambda, C )$$. 
 
 
 In order to prove of how short a zk-SNARK is, Christian Lundkvist tweeted this definition @ChrisLundkvist;

						Generator (C circuit, λ is ☣️):
						(pk, vk) = G(λ, C)
						Prover (x pub inp, w sec inp):
						π = P(pk, x, w)
						Verifier:
						V(vk, x, π) == (∃ w s.t. C(x,w))
						
							6:53 pm - 18 Nov 2016


 
  
 
  ###  5-step process of creating a zk-SNARK for a computation 

zk-SNARKs are not applied directly to a computational 
  
  (Computation —> Arithmetic Circuit —> R1CS —> QAP —> Linear PCP —> zk-SNARK). PCP = probabilistically checkable proofs. 


  "The first constructions of SNARK protocols were inspired by the PCP theorem which shows that NP statements have ‘short’ probabilistic checkable proofs. New instantiations were found which allow faster and shorter proofs, when a pre-processing state is permitted" [4.]   
  
 
  ***** *********
 
 
 
 
 
  
  In zk-SNARKs schemes, the statement being proved is reduced to what is called a rank-1 constraint system, or R1CS. In this system, the prover is given a system of arithmetic constraints over a set of variables (elements in a large prime field F_r ), and asked to produce an assignment to the variables that satisfy the constraints.  
 
   

  [1.] https://diyhpl.us/wiki/transcripts/cryptoeconomic-systems/2019/zksharks/   
  [2.]   
  [3.]  
  [4.] Mayer, Hartwig; *zk-SNARKS Explained: Basic Principles*, CoinFabrik, 06 Dec., 2016.
   
 
  ​  
  
# PART I: R1CS - as used in *Bulletproofs*  
  
  
  
  
  ## Various Ways to Implement R1CS 
- Cf. [2.] above regarding the “binary file format” versa vis “JSON lines format” (sizes of the constraints) 
- Implementation cases: Zcash, Zcoin,… Stellar (cf. Cathie Yun’s article “Programmable Constraint Systems for Bulletproofs”
- "Libra: Succinct Zero-Knowledge Proof with Optimal Prover Computation" - Tiancheng Xie et al, eprint.iacr.org/2019/317.pdf  



 \begin{equation}
G^v \cdot G^u = G^{v + u}
\end{equation}
  