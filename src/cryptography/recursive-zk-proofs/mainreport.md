# Trustless Recursive Zero-knowledge Proofs

## Part I: Amortization of Bulletproofs Range Proofs 




- [Introduction](#introduction) 
- [Notation and Assumptions](#notation-and-assumptions) 
- [Zero-Knowledge Proofs](#zero-knowledge-proofs) 
- [Bulletproofs Range Proofs](#bulletproofs-range-proofs) 
- [What is Recursive Proof Composition?](#what-is-recursive-proof-composition-?) 
	- [Recursive Functions](#recursive-functions)
	- [Recursion in Bulletproofs Inner-Product Proofs](#recursion-in-bulletproofs-inner-product-proof)
	- [Inductive Proofs](#inductive-proofs) 
- [Verification Amortization Strategies](#verification-amortization-strategies)
	- [Application of Verifiable Computation](#application-of-verifiable-computation)
	- [Incrementally Verifiable Computation](#incrementally-verifiable-computation) 
	- [Nested Armotization](#nested-armotization)
	- [Example Application](#example-application) 
- [Halo Protocol Amortization Strategies](#halo-protocol-amortization-strategies)
	- [Polynomial commitment scheme](#polynomial-commitment-scheme)
	- [Schnorr Protocol](#schnorr-protocol)
	- [Modified Inner-Product](#modified-inner-product)
	- [Amortized Strategy](#amortized-strategy) 
- [Amortized Range Proofs](#amortized-range-proofs) 
	- [Bulletproofs Inner Product Proof Verification](#bulletproofs-inner-product-proof-verification) 
	- [Amortized Inner Product Proof](#amortized-inner-product-proof) 
	- [Amortized Bulletproof Range Proofs](#amortized-bulletproof-range-proofs)
	- [Halo-type Range Proofs](#halo-type-range-proofs)
- [Application to the Tari Blockchain](#application-to-the-tari-blockchain) 
- [References](#references) 
- [Appendix A: Mathematical Induction](#appendix-a-mathematical-induction)








## Introduction 

One of the main attractions with blockchains is the idea of trustlessness. 
It is aiming at building a network that allows every participant to run his or her own validation 
should they be in doubt about any given set of transactions. 
Well, this is currently an ideal, especially for blockchains with confidential transactions. 
One of the reasons being the current sizes of zero-knowledge proofs. 
Although much research has focused on reducing the sizes of zero-knowledge proofs, 
seen in the form of zk-SNARKs [[1]] and Bulletproofs [[2]], scalability remains a big challenge. 

Recent efforts in minimizing verification costs have gone the route of recursive proofs. 
Sean Bowe et al  [[3]] lead the pack on recursive proof composition without a trusted setup, 
with the Halo protocol. Other applications of recursive proof composition still rely on some 
kind of a trusted setup. 
Coda [[4]] and Sonic [[5]], for example, use a common reference string (CRS) and an 
updatable structured reference string, respectively. 

Coindesk previously commented that "In essence, Bowe and Co. discovered a new method 
of proving the validity of transactions, while masked, by compressing computational data to 
the bare minimum," [[6]]. 

For blockchains with confidential transactions such as Mimblewimble, Bulletproofs range proofs 
are the most crucial zero-knowledge proofs involved in validation of blockchain transactions.  

The aim in this report is to investigate how the innovative amortization strategies of 
the Halo protocol can be used to enhance the Bulletproofs range proofs. 





## Notation and Assumptions

The settings and notations used in this report follow the Bulletproofs framework as 
in the Dalek notes [[7]], this includes the underlying field  $\mathbb{F}_p$, elliptic curve, 
elliptic curve group and generators  $G$  and  $H$. The details are left out in this report because they have been covered in other TLU reports. 
Note that properties such as completeness, soundness or public coin, as well as the discrete log difficulty, are assumed here. 






## Zero-Knowledge Proofs

In a zero-knowledge proof there are two parties, the prover and the verifier. 
The prover seeks to convince the verifier that he has knowledge of a secret value 
$w$  called the *witness* without disclosing any more information about  $w$  to the verifier. 

How does this work? 

The prover receives a challenge  $x$  from the verifier and does two things. He firstly makes a commitment  $P$  of the witness which hides the value of  $w$, and secondly creates a proof  $\pi$  that attests knowledge of the correct  $w$. He then sends these two to the verifier. 

The verifier then checks correctness of the proof  $\pi$ . This means she tests if some particular relation  $\mathcal{R}$  between  $w$  and  $x$  holds true. The proof  $\pi$  is deemed correct if  $\mathcal{R}(x,w) = 1$  and incorrect if  $\mathcal{R}(x,w) = 0$. 
Since the verifier does not know  $w$,  she uses some verification algorithm  $\mathcal{V}$  such that  $\mathcal{V}(x, \pi )  =  \mathcal{R}(x,w)$. 

The whole research on scalability is in pursuit of such an algorithm  $\mathcal{V}$  
that is most efficient and secure. 





##Bulletproofs Range Proofs 

The Bulletproofs system itself provides a framework for building non-interactive 
zero-knowledge proofs without any need for a trusted setup. And, 
according to Cathie Yun, "it allows for proving a much wider class of statements than just range proofs" [[8]].

The Bulletproofs framework uses the *Pedersen commitment scheme* which is 
known for its hiding and binding properties. 

A *Pedersen commitment* of a value  $v$  is given by  $Com(v) = v \cdot B  +  \tilde{v} \cdot \tilde{B}$  where  $B$  and  $\tilde{B}$  are the generators of the elliptic curve group, and  $\tilde{v}$  is a blinding factor, [[7]]. 

In a Bulletproofs range proof, 

- a prover, given a challenge  $x$  from the verfier, 
  - makes a commitment to a value  $v$, 
  - creates a proof  $\pi$  that attests to the statement that  $v \in [ 0 , 2^n )$, 
  - sends the proof  $\pi$  to the verifier, without revealing any other information about  $v$.  
- a verifier checks if indeed  $v$  is non-negative and falls within the interval  $v \in [ 0 , 2^n )$. 

The Bulletproofs range proof achieves its goal by first rewriting the statement  $v \in [ 0 , 2^n )$  in terms of its binary vectors, as well as expressing it as a single inner-product 
$t(x) = \langle \mathbf{l}(x) , \mathbf{r}(x) \rangle$  
of specially defined binary polynomial vectors  $\mathbf{l}(x)$  and  $\mathbf{r}(x)$.

Thus a so-called *vector Pedersen commitment* is also used in these type of proofs, 
and it is defined as follows.   

A *vector Pedersen commitment* of vectors 
 $\mathbf{a}_L$ and $\mathbf{a}_R$ 
is given by 
 $ Com(\mathbf{a}_L , \mathbf{a}_R )  =  \langle \mathbf{a}_L , \mathbf{G} \rangle + \langle \mathbf{a}_R , \mathbf{H} \rangle + \tilde{a} \tilde{B} $ 
where 
 $\mathbf{G}$  and  $\mathbf{H}$  are vectors of generators of the elliptic curve group, [[7]]. 

The major component of a Bulletproofs range proof is no doubt its Inner-product proof (IPP). 
This became even more apparent when Bootle et al introduced an inner-product proof that 
requires only  $2log_2(n) + 2$  proof-elements instead of  $2n$, [[9]]. Henry de Valence of Interstellar puts it this way, 

"The inner-product proof allows the prover to convince a verifier that some scalar is the inner-product of two length-$n$  vectors using  $\mathcal{O}(log(n))$  steps, and it’s the reason that Bulletproofs are compact." 

No wonder the Halo creators also looked at the IPP, particulaly taking advantage of 
its recursive nature, in their amortization strategies. 
Close attention is therefore given to the IPP as described by Bootle et al [[9]]. 

 



## What is Recursive Proof Composition? 

The aim with Recursive Proof Compositions is to construct "proofs that verify other proofs" 
or "proofs that are capable of verifying other instances of themselves", [[6]]. 
These take advantage of the recursive nature of some components of known zero-knowledge proofs. 

Before discussing recursive proofs, or proof recursion, a brief discussion on 
the recursive function concept and the efficiency of recursive algorithms is given. 
The inner-product proof, or IPP, as used in a Bulletproofs range proof is given as 
an example of a recursive algorithm. A diagram that depicts the recursive nature of 
the IPP is given below, and will later be helpful in understanding some of Halo's amortization strategies. 



### Recursive Functions 

Recursion is used to define functions or sequences of values that depict a consistent pattern. 
And, when written as a formula, it becomes apparent that a  $(j-1)$-th  member of such a sequence is needed in computing the  $j$-th  member of the same sequence. 

A function  $F(x)$  that yields a sequence of values  $ F(0) , F(1), ... , F(n)$  for some positive integer  $ n $  is a recursive function if  $F(k) = F(k - 1) + g(k)$  for all  $0 < k \leq n$,  where  $g(x)$  is some function of  $ x $  an  indeterminate. 

A typical recursive function  $F(j)$  for  $j \in \{ 0 , 1 , ... , n \} $  can be represented in terms of a chart flow below, depicting how values of the sequence  $F(0) , F(1), ... , F(n)$  are computed. 


<p align="center"><img src="sources/Basic-recursive-function.png" width="300" /></p>
<div align="center"><b>Figure 1: Recursive Function Flow Chart</b></div> 


In computer programming, algorithms that involve recursive functions are efficiently executed 
by the use of 'for-loops' and 'while-loops'. 
One can say that computers were made and designed to specifically carry out 
repetitive computation without much error. However, although recursive proof composition is pointedly applicable to recursive algorithms, there's more to it than just recursiveness. 
That is, proof recursion is not defined by recursiveness but rather takes advantage it. 



### Recursion in Bulletproofs Inner-Product Proof

In Bulletproofs range proofs, a prover commits to a value  $v$  and seeks to construct an inner-product proof to the fact that  $v \in [ 0 , 2^n ) $.  Pedersen commitments are used to keep the value of  $v$  confidential, and are expressed as inner-products. 

The main recursive part of a range proof is the Inner-product proof or IPP. 
The inner-product of two vectors  $\mathbf{a}$,  $\mathbf{b}$  and the associated Pedersen commitment can be expressed as 

$$ P_k = \langle \mathbf{a} , \mathbf{G} \rangle + \langle \mathbf{b} , \mathbf{H} \rangle + \langle \mathbf{a} , \mathbf{b} \rangle \cdot Q $$

where  $\mathbf{a}$  and  $\mathbf{b}$  are size-$n$  vectors of scalars in the field  $\mathbb{F}_p$, while  $\mathbf{G}$  and  $\mathbf{H}$  are vectors of points in an elliptic curve  $\mathbb{E} ( \mathbb{F}_p)$  and  $k = log_2(n)$ , see [[9]]. 

Recursion is seen in a  $k-$  round non-interactive IPP-argument where these commitments are written in terms of challenges  $u_k$  sent by the verifier, 

$$ P_{k - 1} = P_k + L_k \cdot u_k^{2} + R_k \cdot u_k^{-2} $$ 

where  $ L_k $  and  $ R_k $  are specifically defined as linear combinations of inner-products of vectors that are half the size of vectors in the  $k - 1$  round. 

In the IP proof the prover convinces the verifier of the veracity of the commitment  $P_k$  by sending only  $k = log(n)$  pairs of values  $L_j$  and  $R_j$  where  $j \in \{ 1, 2, 3, ... , k \}$.  
It is due to this recursion that Bootle et al reduced the previous complexity of zero-knowledge proofs from  $O(\sqrt{n})$  to  $O(log(n))$. 

See the diagram below for an overview of the prover's side of the IPP. 

The input to the IP proof is the quadruple 

$\big( \mathbf{a}^{(j)} , \mathbf{b}^{(j)} , \mathbf{G}^{(j)} , \mathbf{H}^{(j)} \big)$ 


which is initially 

$\big( \mathbf{a} , \mathbf{b} , \mathbf{G} , \mathbf{H} \big) $. 

But when  $j < k$   the input is updated to 

$ \big(\mathbf{a}^{(j-1)} , \mathbf{b}^{(j-1)} , \mathbf{G}^{(j-1)} , \mathbf{H}^{(j-1)} \big)$ 

where 

$\mathbf{a}^{(j-1)} = \mathbf{a}_{lo}  \cdot u_j  + \mathbf{a}_{hi} \cdot u_j^{-1}$ 


with  

$\mathbf{a}_{lo}$  

and

  $\mathbf{a}_{hi}$ 

being the left and the right halves of the vector  $\mathbf{a}$ ,  respectively. 






<p align="center"><img src="sources/IPProof-prover-side-0.png" width="600" /></p>
<div align="center"><b>Figure 2: IP Proof - Prover Side </b></div> 



The above diagram is included here not only to display the recursive nature of the IP Proof, but it will come handy and pivotal to understanding amortization strategies that will be applied to the range proofs. 





### Inductive Proofs 

As noted earlier, 'for-loops' and 'while-loops' are powerful in efficiently executing 
repetitive computations but not sufficient to reach the level of scalability needed in blockchains. 
The basic reason being that the iterative executions compute each instance of the recursive function. 
This is very expensive and clearly undesirable because the aim in 
blockchain validation is not to compute every instance but to prove that 
the current instance was correctly executed. 

The diagram below, depicts how instances of a recursive function are linked, and 
clearly resembles how each block in a blockchain is linked to the previous block via hash values. 



<p align="center"><img src="sources/Recursive-funct-resembles-blockchain.png" width="750" /></p>
<div align="center"><b>Figure 3: Recursive Function Resembles Blockchain </b></div> 



The amortization strategy that recursive proof composition, or proof recursion, uses 
is based on the ancient but powerful mathematical tool called 
the *Principle of Mathematical Induction*. 
It is so ancient it dates back to the sixteenth century, [[10]]. 

Suppose one has to prove that a given sequence of values 
 $ F(0) , F(1), ... , F(n)$ 
are 
correct instances of a recursive function 
 $F(n) = F(n - 1) + g(n)$ 
for all positive integer  $ n $  and  $g(n)$  some function. 

According to the **Principle of Mathematical Induction**, 
it is sufficient to prove the above statement in the following two steps; 

- (Base step): Prove that the first possible instance 
 $F(0)$ 
 is correct, and 

- (Inductive step): For any integer 
 $ k > 0 $ , 
prove that 'if the previous instance 
 $F(k - 1)$ 
is correct then the current instance 
 $F(k)$ 
is also correct'. (i.e.,  '$F(k - 1)$ 
is correct'  implies  '$F(k)$ 
is also correct'.) 

These two steps together are sufficient to form a complete proof for 
the verifier to be convinced. And such a proof is valid even if 
the current instance is the zillionth. 
Thus saving the verifier the trouble of checking every instance of the function 
 $F(n)$. 

Michael Straka describes a similar inductive proof which he refers to as 'proof recursion', [[11]]. 
Here's his explanation of the simplest case a recursive proof will prove a relation 
$\mathcal{R}$ 
inductively: 

The verifier has; 

- a “base” proof 
 $\pi_0$  
which attests to the prover knowing some input 
 $( x_0 , w_0 )$ 
 such that 
 $\mathcal{R}( x_0 ,w_0 ) = 1$. 
- the proof 
 $\pi_n$ 
 for any 
  $n>0$ 
will then prove that the prover knows 
$( x_n , w_n )$ 
such that 
$\mathcal{R}(x_n , w_n ) = 1$ 
and that a proof 
$\pi_{n-1}$  
was produced attesting to the knowledge of 
$(x_{n−1} , w_{n−1})$. 

The following diagram illustrates the above proof. 



<p align="center"><img src="sources/proof-recursion-michaelStraka1.png" width="500" /></p>
<div align="center"><b>Figure 4: Proof Recursion Diagram [[11]] </b></div>



Straka continues to explain how an arithmetic circuit for the verfier could be build in order the above proof. 
Such a circuit a circuit  $\mathcal{C}$  would either verify 
 $\mathcal{R}( x_0 , w_0 ) = 1$ 
 (for the base case) or verify 
  $\mathcal{R}( x_i , w_i ) = 1$  
 and then check 
  $\mathcal{V}( x_{i - 1} , π_{i−1} ) = 1$ , [[11]].

Proof systems that utilise this type of inductive proofs for verification solve the blockchain's distributed validation problem. 
A participant in the blockchain network need only download the current state of the network as well as a single proof that this state is correct.

Recursive proof composition is described in [[3]] as "proofs that can feasibly attest to the correctness of other instances of themselves". 

 



## Verification Amortization Strategies 

There only a few amortization strategies used to achieve proper recursive proof composition 
in the literature. The focus here is on those Sean Bowe et al utilized in the Halo protocol. 



### Application of Verifiable Computation

Delegation of computations to an untrusted third party (or at times, the prover) who returns 

- the result 
 $z$ 
 of the computation, and 
- a cryptographic proof 
 $\pi_z$ 
 that the result is correct. 

The ideal property of such a proof 
 $\pi_z$ 
 is *succinctness*, which means the proof 
  $\pi_z$ 
  must be asymptotically smaller and less expensive to check than the delegated computation itself, [[3]].  



### Incrementally Verifiable Computation

The idea here is to attest to validity of a previous proof in addition to application of 
verifiable computation. The best thing about this amortization strategy is that 
one proof can be used to inductively demonstrate the correctness of many previous proofs. 

So then, any participator in a blockchain network who wishes to validate 
the current state of the chain need only download two things, 

- the current state of the chain network, and 
- a single proof that this state is correct.  

Any further proofs of state changes in a blockchain can be constructed with 
the latest proof alone, allowing active participants to prune old state changes, [[3]]. 

Thus, with incrementally verifiable computation, a large and 
virtually unbounded amount of computation can be verified with a single proof, 
and with this proof alone the computation can be extended to further proofs.  

It due to this strategy that Halo achieves better scalability than most known 
SNARKs constructions. That is, by "allowing a single proof to inductively demonstrate 
the correctness of many previous proofs" [[3]]. 



### Nested amortization 

This strategy can be used as part of the above two amortization strategies. 

Whenever the verifier has to compute an expensive fixed operation  $F$  that is invoked with some input  $x$  the prover is allowed to witness  $y = F(x)$  and send the pair  $( x , y )$  to the verifier. The verification circuit takes  $( x , y )$  as a public input. "The circuit can then proceed under the assumption that  $y$  is correct, delegating the responsibility of checking the correctness of  $y$  to the verifier of the proof" [[3]]. 

Now, increasing instances of  $( x , y )$  will accumulate as proofs are continually composed, simply because the verification circuit will not check these proofs but rather continually delegate the checking to its verifier. The problem here is that computational cost will escalate. 

It is here that the amortization strategy of *collapsing computations* is needed: 

- given instances  $( x , y )$  and  $( x′ , y′ )$ , the prover will provide a non-interactive proof  $\pi_{y,y'}$  that  $y = F(x)$  and  $y' = F(x')$  as a witness to the verification circuit, and 
- the verification circuit will check  $\pi_{y,y'}$  proof.

If the cost of checking correctness of  $\pi_{y,y'}$  is equivalent to invoking the operation  $F$  then the verifier will have collapsed the two instances  $( x , y )$  and  $( x' , y' )$  into a single fresh instance  $( x'' , y'' )$.  That's how the cost of invoking  $F$  can be amortized away. 


<p align="center"><img src="sources/Collapsing-computations-00.png" width="400" /></p>
<div align="center"><b>Figure 5: Collapsing Computations </b></div>


Therefore a nested amortization is achieved on the instances of a 
binary tree of accumulated instances, helping to reduce verification costs of 
the final single proof from linear-time to sub-linear marginal verification time. 



### Example Application

Computing of inverses is one example of computations that 
the verifier could delegate to the prover. In arithmetic circuits where 
a field inversion of a variable  $u$  can be computed (i.e.,  $u^{p−2}$ ), which would normally require  $log(p)$  multiplication constraints, the prover could witness  $v = u^{− 1}$  instead. And the verifier could simply check if  $uv = 1$  taking only a single multiplication constraint, [[3]]. 
Thus one trades  $log(p)$  multiplication constraints for only one.  

One application in point is computation of inverses of verifier's challenges in 
the Bulletproofs Inner-Product Proof. 
The prover could be requested to compute  $u_j^{- 1}$  and send it to the verifier together with  $L_j$  and  $R_j$  for each  $j \in \{ 1, 2 , ... , log_2(n) \}$. The prover computes these value anyway, so this comes with no extra cost to the him (see IP Proof-prover-side diagram above). On the other hand, this will cost the verifier only  $log_2(n)$  multiplication constraints in a single Inner-Product Proof for checking all inverses instead of a cost of  $(log(p))\*(log_2(n))$  constraints. 
Note also that  $n$  the size of the initial input vectors to the inner-product proof is typically very small compared to the prime  $p$ .       





## Halo Protocol Amortization Strategies

[Move this section to second report ... focusing the Halo protocol and its type of Range Proofs] 



### Polynomial commitment scheme 



### Schnorr protocol 



### Modified Inner-Product 



### Amortized Strategy 

... polynomial commitment scheme ... (cf., Sec 3.2, [[3]], Page 9)







## Amortized Range Proofs



### Bulletproofs Inner Product Proof Verification   





See below, a diagram of a naive implementation of the verification side of the Bulletproofs IP Proof.  



<p align="center"><img src="sources/IPProof-verifier-side-01.png" width="750" /></p>
<div align="center"><b>Figure 6: Bulletproofs IP Proof - Verifier Side </b></div>







### Amortized Inner-Product Proof





**Delegating Inversion of Verifier's Challenges** 

One of the details ommited from the IPP-verfier-side [diagram](#bulletproofs-inner-product-proof-verification) is the computation of inverses of the verifier's challenges  $u_j$  needed to complete verification. 

Verifiable computation strategy is therefore applicable to the Inner-Product Proof, where the verifier delegates inversion of challenges to the prover. It so happens that the prover needs these inverses to carry out his part of the proof. So this delegation will not cost any extra computation for the verifier except just to send each of the inverses he has already computed to the verifier. 

 As noted [above](example-application), this amortization strategy reduces the verification costs by factor of  $log(p)$. 



**Delegating Computation of $G_i$'s Coeffiecient**

Consider the vector of group-generators  $\mathbf{G} = ( G_1 , G_2 , G_3 , ... , G_n )$ , one of the four initial input vectors to the IP Proof. 
In verifying the prover's inner-product proof, the verifier has to compute the vector  $\mathbf{s} = ( s_1 , s_2 , s_3 , ... , s_n )$  where each  $s_i = \prod\limits_{j = 1}^k u_j^{b(i,j)}$  is the so-called coefficient of  $G_i$ , and  $j \in \{ 1 , 2 , 3 , ... , k \}$  with  $k = log_2(n)$.

See the IPP-verifier-side [diagram](#bulletproofs-inner-product-proof-verification) above. 

[I need to find properties of the set of  $s_i$'s  that can be exploited in two ways ... ; 
one way: the verifier can use to check their veracity, and 
second way: minimizes prover's computational costs   ] 



- **Optimizing the Computation of $G_i$'s Coeffiecient** 

Let the size of the initial input vectors to the IP Proof be  $n = 256$. 





### Amortized Bulletproof Range Proofs



With the above amortization of the IP Proof, the prover sends  $3log_2(n) +  n$   inner-product proof elements, including the  $log_2(n)$  inverses as well as the  $n$  coefficients (that's assuming the verifier does not need any proof of the  $s_i$'s from the prover) 









### Halo-type Range Proofs 

- using the polynomial commitment scheme 
- modified inner-product 







## Application to the Tari Blockchain 







## References 



[[1]] H. Wu, W. Zheng, A. Chiesa, R. A. Popa and I. Stoica, "DIZK: A Distributed Zero Knowledge Proof System", UC Berkeley [online]. 

Available: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf. Date accessed: 2019&#8209;12&#8209;07.



[[2]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More" [online], Blockchain Protocol Analysis and Security Engineering 2018 . 

Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2020&#8209;04&#8209;21.





[[3]] ECC Posts, "Halo: Recursive Proof Composition without a Trusted Setup" [online] , Electric Coin Co., Sept. 2019

Available: https://electriccoin.co/blog/halo-recursive-proof-composition-without-a-trusted-setup/. Date accessed: 2020‑04‑24.



[[3*]] S. Bowe, J. Grigg and D. Hopwood "Recursive Proof Composition without a Trusted Setup" [online].  Electric Coin Company, 2019.

Available: https://eprint.iacr.org/2019/1021.pdf. Date accessed: 2020‑04‑24. 



[[4]]  I. Meckler and E. Shapiro, "Coda: Decentralized cryptocurrency at scale," [online] O(1) Labs May 10, 2018. 

Available: https://cdn.codaprotocol.com/v2/static/coda-whitepaper-05-10-2018-0.pdf. Date accessed: 2020&#8209;04&#8209;27. 



[[5]] M. Maller, S. Bowe, M. Kohlweiss and S. Meiklejohn, "Sonic: Zero-Knowledge SNARKs from Linear-Size Universal and Updatable Structured Reference Strings" [online], Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications, 2019.

Available: https://eprint.iacr.org/2019/099.pdf. Date accessed: 2020&#8209;04&#8209;27. 



[[6]] William Foxley, "You Can Now Prove a Whole Blockchain With One Math Problem – Really," [online] Coindesk, 2019. 

Available: https://www.coindesk.com/you-can-now-prove-a-whole-blockchain-with-one-math-problem-really. Date accessed: 2020&#8209;04&#8209;27. 



[[7]]  Dalek's Bulletproofs documents, "Module Bulletproofs::notes ::index" [online]. 

Available: https://doc-internal.dalek.rs/bulletproofs/notes/index.html. Date accessed: 2020‑05‑01.

 

[[8]] J. Bootle, A. Cerulli, P. Chaidos, J. Groth and C. Petit, "Efficient Zero-knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting" [online], Annual International Conference on the Theory and Applications of Cryptographic Techniques, pp. 327‑357. Springer, 2016 . 

Available: https://eprint.iacr.org/2016/263.pdf. Date accessed: 2019‑12‑21.   



[[8a]] C. Yun, "Building on Bulletproofs" [online]. 

Available: <https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8>. Date accessed: 2020‑04‑27.



[[9]] Dalek's Bulletproofs documents, "Module Bulletproofs:: notes :: index" [online] 

Available: https://doc-internal.dalek.rs/bulletproofs/notes/inner_product_proof/index.html. Date accessed: 2020‑04‑27.



[[10]] History of Science and Mathematics Beta, "Who introduced the Principle of Mathematical Induction for the first time?" [online]. 

Available: https://hsm.stackexchange.com/questions/524/who-introduced-the-principle-of-mathematical-induction-for-the-first-time. Date accessed: 2020‑04‑30.  



[[11]]  M. Straka, "Recursive Zero-Knowledge Proofs: A Comprehensive Primer" [online]. December 8, 2019.

Available: https://www.michaelstraka.com/posts/recursivesnarks/. Date accessed: 2020‑04‑30.   







###Appendix A: Mathematical Induction 

ref: 

"How to use strong induction to prove correctness of recursive algorithms" https://cseweb.ucsd.edu/classes/sp16/cse21-bd/howtorec.pdf 





