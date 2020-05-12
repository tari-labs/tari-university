# Amortization of Bulletproofs Inner-Product Proof 



- [Introduction](#introduction) 
- [Notation and Assumptions](#notation-and-assumptions) 
- [Zero-Knowledge Proofs](#zero-knowledge-proofs) 
- [Bulletproofs Range Proofs](#bulletproofs-range-proofs) 
- [What is Recursive Proof Composition?](#what-is-recursive-proof-composition) 
	- [Recursive Functions](#recursive-functions)
	- [Recursion in Bulletproofs Inner-Product Proofs](#recursion-in-bulletproofs-inner-product-proof)
	- [Inductive Proofs](#inductive-proofs) 
- [Verification Amortization Strategies](#verification-amortization-strategies)
	- [Application of Verifiable Computation](#application-of-verifiable-computation)
	- [Incrementally Verifiable Computation](#incrementally-verifiable-computation) 
	- [Nested Armotization](#nested-armotization)
- [Amortized Inner-Product Proof](#amortized-inner-product-proof) 
	- [Bulletproofs Inner Product Proof Verification](#bulletproofs-inner-product-proof-verification) 
	- [Verifiable Computation: Application 1](#verifiable-computation-application-1) 
	- [Verifiable Computation: Application 2](#verifiable-computation-application-2)
	- [Concluding the Amortized Inner Product Proof](#concluding-the-amortized-inner-product-proof) 
- [Application to the Tari Blockchain](#application-to-the-tari-blockchain) 
- [References](#references)
- [Appendix A](#appendix-a) 








## Introduction 

One of the main attractions with blockchains is the idea of trustlessness. 
It is aiming at building a network that allows every participant to run his or her own validation 
should they be in doubt about any given set of transactions. 
Well, this is currently an ideal, especially for blockchains with confidential transactions. 
One of the reasons being the current sizes of zero-knowledge proofs. 
Although much research has focused on reducing the sizes of zero-knowledge proofs, 
seen in the form of zk-SNARKs [[1]] and Bulletproofs [[2]], scalability remains a big challenge. 

Recent efforts in minimizing verification costs have gone the route of recursive proofs. 
Sean Bowe et al lead the pack on recursive proof composition without a trusted setup, 
with the Halo protocol. Other applications of recursive proof composition still rely on some 
kind of a trusted setup,  [[3]] . 
Coda [[4]] and Sonic [[5]], for example, use a common reference string (CRS) and an 
updatable structured reference string, respectively. 

Coindesk previously commented that "In essence, Bowe and Co. discovered a new method 
of proving the validity of transactions, while masked, by compressing computational data to 
the bare minimum," [[6]]. 

For blockchains with confidential transactions such as Mimblewimble, Bulletproofs range proofs 
are the most crucial zero-knowledge proofs involved in validation of blockchain transactions. 
The huge bulk of computations in these range proofs take place in the inner-product proof.  

The aim in this report is to investigate how the innovative amortization strategies of 
the Halo protocol can be used to enhance the Bulletproofs inner-product proof. 





## Notation and Assumptions

The settings and notations used in this report follow the Bulletproofs framework as 
in the Dalek notes [[7]], this includes the underlying field  $\mathbb{F}_p$, elliptic curve, 
elliptic curve group and generators  $G$  and  $H$. The details have been covered in other 
TLU reports, and therefore are left out in this report. 
Note that properties such as completeness, soundness or public coin, as well as 
he discrete log difficulty, are assumed here. 

Since the Ristretto implementation of the curve25519 is used in the Bulletproofs setting, 
unless otherwise stated, all vectors of size  $n$  will be referring to  $n$  32 bytes. 






## Zero-Knowledge Proofs

In a zero-knowledge proof there are two parties, the prover and the verifier. 
The prover seeks to convince the verifier that he has knowledge of a secret value 
$w$  called the *witness* without disclosing any more information about  $w$  to the verifier. 

How does this work? 

The prover receives a challenge  $x$  from the verifier and does two things. He firstly makes 
a commitment  $P$  of the witness which hides the value of  $w$, and secondly creates a 
proof  $\pi$  that attests knowledge of the correct  $w$. He then sends these two to the verifier. 

The verifier then checks correctness of the proof  $\pi$ . This means she tests if some 
particular relation  $\mathcal{R}$  between  $w$  and  $x$  holds true. The proof  $\pi$  is 
deemed correct if  $\mathcal{R}(x,w) = 1$  and incorrect if  $\mathcal{R}(x,w) = 0$. 
Since the verifier does not know  $w$,  she uses some verification algorithm  $\mathcal{V}$  
such that  $\mathcal{V}(x, \pi )  =  \mathcal{R}(x,w)$. 

The whole research on scalability is in pursuit of such an algorithm  $\mathcal{V}$  that is 
most efficient and secure. 





## Bulletproofs Range Proofs 

The Bulletproofs system itself provides a framework for building non-interactive 
zero-knowledge proofs without any need for a trusted setup. And, 
according to Cathie Yun, "it allows for proving a much wider class of statements than just 
range proofs" [[8]]. 

The Bulletproofs framework uses the *Pedersen commitment scheme* which is 
known for its hiding and binding properties. 

A *Pedersen commitment* of a value  $v$  is given by  $Com(v) = v \cdot B  +  \tilde{v} \cdot \tilde{B}$  
where  $B$  and  $\tilde{B}$  are the generators of the elliptic curve group, and  $\tilde{v}$  is 
a blinding factor, [[7]]. 

In a Bulletproofs range proof, 

- a prover, given a challenge  $x$  from the verfier, 
  - makes a commitment to a value  $v$, 
  - creates a proof  $\pi$  that attests to the statement that  $v \in [ 0 , 2^n )$, 
  - sends the proof  $\pi$  to the verifier, without revealing any other information about  $v$.  
- a verifier checks if indeed  $v$  is non-negative and falls within the interval  $v \in [ 0 , 2^n )$. 

The Bulletproofs range proof achieves its goal by first rewriting the statement 
$v \in [ 0 , 2^n )$  in terms of its binary vectors, as well as expressing it as a single inner-product 
$t(x) = \langle \mathbf{l}(x) , \mathbf{r}(x) \rangle$  of specially defined binary polynomial 
vectors  $\mathbf{l}(x)$  and  $\mathbf{r}(x)$.

Thus a so-called *vector Pedersen commitment* is also used in these type of proofs, 
and it is defined as follows. 

A *vector Pedersen commitment* of vectors 
 $\mathbf{a}_L$ and $\mathbf{a}_R$  is given by 
 $ Com(\mathbf{a}_L , \mathbf{a}_R )  =  \langle \mathbf{a}_L ,
  \mathbf{G} \rangle + \langle \mathbf{a}_R , \mathbf{H} \rangle + \tilde{a} \tilde{B} $ 
where  $\mathbf{G}$  and  $\mathbf{H}$  are vectors of generators of the elliptic curve group, [[7]]. 

The major component of a Bulletproofs range proof is no doubt its Inner-product proof (IPP). 
This became even more apparent when Bootle et al introduced an inner-product proof that 
requires only  $2log_2(n) + 2$  proof-elements instead of  $2n$, [[9]]. 
Henry de Valence of Interstellar puts it this way, 

"The inner-product proof allows the prover to convince a verifier that some scalar is 
the inner-product of two length-$n$  vectors using  $\mathcal{O}(log(n))$  steps, and 
it’s the reason that Bulletproofs are compact," [[10]]. 

No wonder the Halo creators also looked at the IPP, particulaly taking advantage of 
its recursive nature, in their amortization strategies. 
Close attention is therefore given to the IPP as described by Bootle et al [[9]]. 





## What is Recursive Proof Composition? 

The aim with Recursive Proof Compositions is to construct "proofs that verify other proofs" 
or "proofs that are capable of verifying other instances of themselves", [[6]]. 
These take advantage of the recursive nature of some components of known 
zero-knowledge proofs. 

Before discussing recursive proofs, or proof recursion, a brief discussion on 
the recursive function concept and the efficiency of recursive algorithms is given. 
The inner-product proof, or IPP, as used in a Bulletproofs range proof is given as 
an example of a recursive algorithm. A diagram that depicts the recursive nature of 
the IPP is given below, and will later be helpful in understanding some of 
Halo's amortization strategies. 



### Recursive Functions 

Recursion is used to define functions or sequences of values that depict a consistent 
pattern. 
And, when written as a formula, it becomes apparent that a  $(j-1)$-th  member of 
such a sequence is needed in computing the  $j$-th  member of the same sequence. 

A function  $F(x)$  that yields a sequence of values  $ F(0) , F(1), ... , F(n)$  for some 
positive integer  $ n $  is a recursive function if  $F(k) = F(k - 1) + g(k)$  for all  $0 < k \leq n$,  
where  $g(x)$  is some function of  $ x $  an  indeterminate. 

A typical recursive function  $F(j)$  for  $j \in \{ 0 , 1 , ... , n \} $  can be represented 
in terms of a chart flow below, depicting how values of the sequence  $F(0) , F(1), ... , F(n)$  
are computed. 


<p align="center"><img src="sources/Basic-recursive-function.png" width="300" /></p>
<div align="center"><b>Figure 1: Recursive Function Flow Chart</b></div> 


In computer programming, algorithms that involve recursive functions are efficiently executed 
by the use of 'for-loops' and 'while-loops'. 
One can say that computers were made and designed to specifically carry out 
repetitive computation without much error. However, although recursive proof composition 
is pointedly applicable to recursive algorithms, there's more to it than just recursiveness. 
That is, proof recursion is not defined by recursiveness but rather takes advantage it. 



### Recursion in Bulletproofs Inner-Product Proof

In Bulletproofs range proofs, a prover commits to a value  $v$  and seeks to construct 
an inner-product proof to the fact that  $v \in [ 0 , 2^n ) $.  
Pedersen commitments are used to keep the value of  $v$  confidential, and are 
expressed as inner-products. 

The main recursive part of a range proof is the Inner-product proof or IPP. 
The inner-product of two vectors  $\mathbf{a}$,  $\mathbf{b}$  and the associated 
Pedersen commitment can be expressed as 

$$ P_k = \langle \mathbf{a} , \mathbf{G} \rangle + \langle \mathbf{b} , \mathbf{H} \rangle + \langle \mathbf{a} , \mathbf{b} \rangle \cdot Q $$

where  $\mathbf{a}$  and  $\mathbf{b}$  are size-$n$  vectors of scalars in 
the field  $\mathbb{F}_p$, while  $\mathbf{G}$  and  $\mathbf{H}$  are vectors of 
points in an elliptic curve  $\mathbb{E} ( \mathbb{F}_p)$  and  $k = log_2(n)$ , see [[11]]. 

Recursion is seen in a  $k-$  round non-interactive IPP-argument where these 
commitments are written in terms of challenges  $u_k$  sent by the verifier, 

$$ P_{k - 1} = P_k + L_k \cdot u_k^{2} + R_k \cdot u_k^{-2} $$ 

where  $ L_k $  and  $ R_k $  are specifically defined as linear combinations of 
inner-products of vectors that are half the size of vectors in the  $k - 1$  round. 

In the IP proof the prover convinces the verifier of the veracity of the commitment  $P_k$  by 
sending only  $k = log(n)$  pairs of values  $L_j$  and  $R_j$  where  $j \in \{ 1, 2, 3, ... , k \}$.  
It is due to this recursion that Bootle et al reduced the previous complexity of 
zero-knowledge proofs from  $O(\sqrt{n})$  to  $O(log(n))$. 

See the diagram below for an overview of the prover's side of the IPP. 

The input to the IP proof is the quadruple of size  $ n = 2^k $  vectors  
$\big( \mathbf{a}^{(j)} , \mathbf{b}^{(j)} , \mathbf{G}^{(j)} , \mathbf{H}^{(j)} \big)$ 
which is initially 
$\big( \mathbf{a} , \mathbf{b} , \mathbf{G} , \mathbf{H} \big) $. 
But when  $j < k$   the input is updated to 
$ \big(\mathbf{a}^{(j-1)} , \mathbf{b}^{(j-1)} , \mathbf{G}^{(j-1)} , \mathbf{H}^{(j-1)} \big)$ 
quadruple vectors each of size  $2^{k-1}$  where  
$ \mathbf{a}^{(j-1)} = \mathbf{a}\_{lo} \cdot u\_j + \mathbf{a}\_{hi} \cdot u\_j^{-1} $, \ \ 
$\mathbf{b}^{(j-1)} = \mathbf{b}\_{lo}  \cdot u\_j^{-1}  + \mathbf{b}\_{hi} \cdot u\_j$, \ \ 
$\mathbf{G}^{(j-1)} = \mathbf{G}\_{lo}  \cdot u\_j^{-1}  + \mathbf{G}\_{hi} \cdot u\_j$,\ \ 
$\mathbf{H}^{(j-1)} = \mathbf{H}\_{lo}  \cdot u\_j  + \mathbf{H}\_{hi} \cdot u\_j^{-1}$, and  $u_k$ 
is the verifier's challenge. 
The vectors\ \ 
$\mathbf{a}\_{lo}, \mathbf{b}\_{lo}, \mathbf{G}\_{lo}, \mathbf{H}\_{lo}$\ \ 
and\ \ 
$\mathbf{a}\_{hi}, \mathbf{b}\_{hi}, \mathbf{G}_{hi}, \mathbf{H}\_{hi} $\ \ 
being the left and the right halves of the vectors\ \ 
$\mathbf{a}, \mathbf{b}, \mathbf{G}, \mathbf{H}$, 
respectively. 



<p align="center"><img src="sources/IPProof-prover-side-0.png" width="600" /></p>
<div align="center"><b>Figure 2: IP Proof - Prover Side </b></div> 



The above diagram is included here not only to display the recursive nature of 
the IP Proof, but it will come handy and pivotal to understanding amortization strategies 
that will be applied to the inner-product proof. 





### Inductive Proofs 

As noted earlier, 'for-loops' and 'while-loops' are powerful in efficiently executing 
repetitive computations but not sufficient to reach the level of scalability needed in 
blockchains. 
The basic reason being that the iterative executions compute each instance of 
the recursive function. 
This is very expensive and clearly undesirable because the aim in 
blockchain validation is not to compute every instance but to prove that 
the current instance was correctly executed. 

The diagram below, depicts how instances of a recursive function are linked, and 
clearly resembles how each block in a blockchain is linked to the previous block 
via hash values. 



<p align="center"><img src="sources/Recursive-funct-resembles-blockchain.png" width="750" /></p>
<div align="center"><b>Figure 3: Recursive Function Resembles Blockchain </b></div> 



The amortization strategy that recursive proof composition, or proof recursion, 
uses 
is based on the ancient but powerful mathematical tool called 
the *Principle of Mathematical Induction*. 
It is so ancient it dates back to the sixteenth century, [[12]]. 

Suppose one has to prove that a given sequence of values 
 $ F(0) , F(1), ... , F(n)$ 
are correct instances of a recursive function 
 $F(n) = F(n - 1) + g(n)$ 
for all positive integer  $ n $  and  $g(n)$  some function. 

According to the **Principle of Mathematical Induction**, 
it is sufficient to prove the above statement in the following 
two steps; 

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
Thus saving the verifier the trouble of checking every instance of 
the function  $F(n)$. 

Michael Straka describes a similar inductive proof which he 
refers to as 'proof recursion', [[13]]. 
Here's his explanation of the simplest case a recursive proof 
will prove a relation 
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
<div align="center"><b>Figure 4: Proof Recursion Diagram [[13]] </b></div>



Straka continues to explain how an arithmetic circuit for the verfier could be build in order 
to carry out the above proof. 
Such a circuit a circuit  $\mathcal{C}$  would either verify 
 $\mathcal{R}( x_0 , w_0 ) = 1$ 
 (for the base case) or verify 
  $\mathcal{R}( x_i , w_i ) = 1$ 
 and then check 
  $\mathcal{V}( x_{i - 1} , π_{i−1} ) = 1$ , [[13]].

Proof systems that utilise this type of inductive proofs for verification solve 
the blockchain's distributed validation problem. 
A participant in the blockchain network need only download the current state of 
the network as well as a single proof that this state is correct.

Recursive proof composition is described in [[14]] as "proofs that can feasibly 
attest to the correctness of other instances of themselves". 

 



## Verification Amortization Strategies 

There are only a few amortization strategies used to achieve proper recursive proof composition 
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
  must be asymptotically smaller and less expensive to check than 
  the delegated computation itself, [[14]].  



### Incrementally Verifiable Computation

The idea here is to attest to validity of a previous proof in addition to 
application of verifiable computation. 
The best thing about this amortization strategy is that 
one proof can be used to inductively demonstrate the correctness of 
many previous proofs. 

So then, any participator in a blockchain network who wishes 
to validate the current state of the chain need only download two things, 

- the current state of the chain network, and 
- a single proof that this state is correct.  

Any further proofs of state changes in a blockchain can be constructed 
with the latest proof alone, allowing active participants to prune old state 
changes, [[14]]. 

Thus, with incrementally verifiable computation, a large and 
virtually unbounded amount of computation can be verified with a single proof, 
and with this proof alone the computation can be extended to further proofs.  

It due to this strategy that Halo achieves better scalability than most known 
SNARKs constructions. That is, by "allowing a single proof to inductively 
demonstrate the correctness of many previous proofs" [[14]]. 



### Nested amortization 

This strategy can be used as part of the above two amortization strategies. 

Whenever the verifier has to compute an expensive fixed operation  $F$  
that is invoked with some input  $x$  the prover is allowed to witness  $y = F(x)$  and 
send the pair  $( x , y )$  to the verifier. The verification circuit takes  $( x , y )$  as 
a public input. 
"The circuit can then proceed under the assumption that  $y$  is correct, delegating 
the responsibility of checking the correctness of  $y$  to the verifier of the proof" [[14]]. 

Now, increasing instances of  $( x , y )$  will accumulate as proofs are continually 
composed, simply because the verification circuit will not check these proofs 
but rather continually delegate the checking to its verifier. 
The problem here is that computational cost will escalate. 

It is here that the amortization strategy of *collapsing computations* is needed: 

- given instances  $( x , y )$  and  $( x′ , y′ )$ , the prover will provide a 
non-interactive proof  $\pi_{y,y'}$  that  $y = F(x)$  and  $y' = F(x')$  is a witness 
to the verification circuit, and 
- the verification circuit will check  $\pi_{y,y'}$  proof.

If the cost of checking correctness of  $\pi_{y,y'}$  is equivalent to invoking 
the operation  $F$  then the verifier will have collapsed the 
two instances  $( x , y )$  and  $( x' , y' )$  into a single fresh instance  $( x'' , y'' )$.  
That's how the cost of invoking  $F$  can be amortized away. 


<p align="center"><img src="sources/Collapsing-computations-00.png" width="400" /></p>
<div align="center"><b>Figure 5: Collapsing Computations </b></div>


Therefore a nested amortization is achieved on the instances of a 
binary tree of accumulated instances, helping to reduce verification costs of 
the final single proof from linear-time to sub-linear marginal verification time. 


As mentioned above, the aim with Recursive Proof Compositions is two-pronged. 
Construction of "proofs that verify other proofs" as well as construction of 
"proofs that are capable of verifying other instances of themselves", [[6]]. 
The discussion in this report focuses on the latter, while the former will be part of forthcoming reports 
as it requires delving into understanding cycles of elliptic curves. 





## Amortized Inner-Product Proof

One application in point, of the armotization strategies discussed thus far, is 
the Bulletproofs Inner-Product Proof. 
The aim here is to investigate how much significance can these strategies bring to 
this powerful and pervasive proof, especially in the blockchain technology where 
efficiency of zero-knowledge proof is pursued.



### Bulletproofs Inner Product Proof Verification   

Consider the Bulletproofs Inner-Product Proof or IP Proof, originally described by 
Bootle et al [[9]], but following the Dalek's Bulletproofs settings [[11]]. 
The IPP is no doubt recursive in the way it is executed. 
[Figure 1](#recursion-in-bulletproofs-inner-product-proof) and 
[Figure 2](#bulletproofs-inner-product-proof-verification)  make this apparent. 
It is therefore the most relevant a case study to amortize verification costs 
especially in the context of recursive proofs. 


See below, a diagram of a naive implementation of the verifier's side of the Bulletproofs IP Proof.  



<p align="center"><img src="sources/IPProof-verifier-side-1.png" width="750" /></p>
<div align="center"><b>Figure 6: Bulletproofs IP Proof - Verifier Side </b></div>







### Verifiable Computation: Application 1



**Delegating Inversion of Verifier's Challenges** 

One of the details omitted from the IPP-verfier-side [diagram](#bulletproofs-inner-product-proof-verification) above 
is the computation of inverses of the verifier's challenges  $u_j$  needed to complete verification. 
The verifier for example needs  $u_j^{-2}$  in order to compute  $- L_j \cdot u_j^2 - R_j \cdot u^{-2}$.  
Verifiable computation strategy is therefore applicable to the Inner-Product Proof, 
where the verifier delegates inversion of challenges to the prover. 

As Sean Bowe et al proposed, in arithmetic circuits where a field inversion of 
a variable  $u$  can be computed (i.e.,  $u^{p−2}$ ), which 
would normally require  $log(p)$  multiplication constraints, 
the prover could witness  $v = u^{− 1}$  instead [[3]]. 
And the verifier could simply check if  $uv = 1$  taking only a single multiplication constraint. 
Thus one trades  $log(p)$  multiplication constraints for only one. 

So, to amortize away some verification costs, the prover in the Bulletproofs IP Proof is 
requested to compute each  $u_j^{- 1}$  and send it to the verifier. That's in addition to 
values  $L_j$  and  $R_j$  for  $j \in \{ 1, 2 , ... , log_2(n) \}$. 

Worth noting is that the prover will have computed these inverses anyway, because 
he needs them in "halving" of the input vectors to the IP Proof. 
The delegation of inversion therefore comes with *no extra cost* to the prover. 

This amortization strategy has huge savings on verification costs in the form of 
the number of multiplications by the verifier. 
Instead of a verification cost of  $(log(p))\*(log_2(n))$  multiplication constraints on 
inversion alone, it now will only cost  $log_2(n)$  multiplication constraints for 
a single Inner-Product Proof. 
The savings are huge considering the fact that  $n$ , which is the size of 
the initial input vectors to the inner-product proof, is typically very small 
compared to  $p$  the prime order of the elliptic curve group. 

 As noted [above](example-application), this amortization strategy reduces the verification costs by factor of  $log(p)$. 





### Verifiable Computation: Application 2



**Delegating Computation of $G_i$'s Coeffiecient**

Consider the vector of group-generators  $\mathbf{G} = ( G_0 , G_1 , G_2 , ... , G_{n-1} )$ , one of the four initial input vectors to the IP Proof. In verifying the prover's inner-product proof, the verifier has to compute the vector  $\mathbf{s} = ( s_0 , s_1 , s_2 , ... , s_{(n-1)} )$  where each  $s_i = \prod\limits_{j = 1}^k u_j^{b(i,j)}$  is the so-called coefficient of  $G_i$  while  $j \in \{ 1 , 2 , 3 , ... , k \}$  with  $k = log_2(n)$. See the IPP-verifier-side [diagram](#bulletproofs-inner-product-proof-verification) above. Note that  

$$ b(i,j) = \begin{cases} {-1} & {\text{if}\ \  (i\ \ mod\ \ 2^j) < 2^{j-1}} \\ {+1} & {\text{if}\ \ (i\ \ mod\ \ 2^j) \geq  2^{j-1}} \end{cases} $$

determines whether the factor multiplied into  $s_i$  is the verifier's challenge  $u_j$  or its inverse.  

Computations of these coefficients can be delegated to the prover or some third party called "helper", hence forth referred to as the prover.  



Since each of these coefficients is a product of field elements, they have strong algebraic properties that can be exploited in two ways; Firstly, the verifier can use these properties to check if the $s_i$'s were correctly computed by the prover. Secondly, they can be used to minimize the prover's computational costs. 



**Verifier's Test of Prover's $s_i$ Values** 

Note that the verifier has to compute the values  $u_j^2$  and  $u_j^{-2}$ for all  $j \in \{ 1, 2, 3, ... , k \}$. The idea here is to use a verifier's test that involves these squares of the challenges and their inverses. The next theorem entails such relationships between these squares and the coefficients   $s_i$ . 





**Theorem 1. [Some properties of the set of coefficients  $\{ s_i \}$ ]** 

Let  $n = 2^k$ and  $s_i = \prod\limits_{j = 1}^k u_j^{b(i,j)}$  for all  $j \in \\{ 1, 2, 3, ... , k \\}$  where each  $G_i$  the $i-$th component of the initial input vector   $\mathbf{G} = ( G_0 , G_1 , G_2 , ... , G_{n-1})$ , then 

(a)	$\ \ s_i \cdot s_{(n-1) - i}  = 1_{\mathbb{F}\_p}$  for all  $i \in  \\{ 0, 1, 2, ... , n-1 \\}$.

(b)	$\ \ s_{2^{(j-1)}} \cdot s_{n-1}  = u\_j^2 $  for all  $j \in \\{ 1, 2, 3, ... , k \\}$.

(c)	$\ \ s_0 \cdot s_{(n-1) - 2^{(j-1)}}  = u\_j^{-2} $  for all  $j \in \\{ 1, 2, 3, ... , k \\}$. 



The proof of part (a) of this theorem follows by induction on the size  $n$  of the initial input vector  $\mathbf{G} = ( G_0 , G_1 , G_2 , ... , G_{n-1} )$  to the IP Proof ,  while parts  (b)  and  (c)  follow by induction on  $k$ .  





**Verifier's Test of the $\{ s_i \}$ values** : The verifier tests the correctness of the coefficients  $s_i$  with the following statement, 

$$\ \ (s_{1} \cdot s_{n-1}  = u_1^2) \and (s_{2} \cdot s_{n-1}  = u_2^2) \and (s_{4} \cdot s_{n-1}  = u_3^2) \and\ \ ...\ \ \and (s_{2^{(k-1)}} \cdot s_{n-1}  = u_k^2)  \and \\ \ \ (s_0 \cdot s_{(n-1) - 2^{(0)}}  = u_1^{-2}) \and (s_0 \cdot s_{(n-1) - 2^{(1)}}  = u_2^{-2}) \and\ \ ...\ \ \and (s_0 \cdot s_{(n-1) - 2^{(k-1)}}  = u_k^{-2})  \and \\ \ \ ( s_{r_1} \cdot s_{(n-1) - {r_1}}  = 1_{\mathbb{F}_p} ) \and (s_{r_2} \cdot s_{(n-1) - {r_2}}  = 1_{\mathbb{F}_p}) \and (s_{r_3} \cdot s_{(n-1) - {r_3}}  = 1_{\mathbb{F}_p}) \and\ \ ...\ \ \and (s_{r_k} \cdot s_{(n-1) - r_k}  = 1_{\mathbb{F}_p})  $$

for randomly sampled values  $ \\{ r_1, r_2, ... , r_k \\} \subset \\{ 0, 1, 2, ... , n-1 \\} $  where  $k = log_2(n)$.





**Optimizing Prover's Computation of the  $\{ s_i \}$  values**

The naively implemented computation of the coefficients  $s_i$  as depicted in [diagram](#bulletproofs-inner-product-proof-verification) is very expensive in terms of number multiplications. 

**Naive Algorithm**: The naive algorithm codes computation of the coefficients  $s_i = \prod\limits_{j = 1}^k u_j^{b(i,j)}$  by cumulatively multiplying the correct factor  $u_j^{b(i,j)}$  in each $j-$th IPP round, running from  $k = log_2(n)$  down to  $1$. 

That is, although recursive according to the nature of the IP Proof, the naive implementation strictly follows the definition without any attempt to optimise.  



`**Naive Algorithm**`

` initialise 	s[n] = [1,1,1, ... ,1]; ` 

` initialise 	k = log_2(n)` 

`... %running inside IPP`

`int main(){`

​	` for (j = k; j > 0; j--){`

​		`for (i = 0; i < n; i++){`

​			`	 s[i] = s[i]*(u_j)^{b(i,j)} `;

​		`}`

​	`}`	  

`return = 0; ` 

`}`



Using the Naive Algorithm, an input vector  $\mathbf{G}$  to the IP Proof, of size  $n = 256$ , costs the verifier at least  $508$  multiplications. If  $n = 1024$, the cost is  $2044$. And this is excluding all the other computations the verifier has to compute. 

 



**Optimized Algorithms** 

There are at least four competing algorithms that optimise computation of the coefficients $\{ s_i \}$ and each one is far much better than the naive algorithm. Some reducing the verification cost by 40%. 

The basic optimisation strategy is based on the observation that every coefficient has at least one common factor with  $2^{k-1}- 1$  other coefficients, two common factors with  $2^{k-2}-1$ ,  three with  $2^{k-3}- 1$,  and so on. It is therefore cost-effective in terms of number of multiplications to aim at computing these common factors and only form the required coefficients later.     



`**Typical Optimised Algorithm**`

` initialise 	s[n] = [1,1,1, ... ,1]; ` 

` initialise 	k = log_2(n)` 

`... %running inside IPP `

`int main(){`

​	` s[0] = u_k^{-1};  s[2^{k-1}] = u_k;`

​	`s[2^{k-2}] = s[0]; s[2^{k-1} + 2^{k-2}] = s[2^{k-1}]; `

​	` 		for (j = k - 1; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if (i mod 2^j == 0) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^(j-1); `

​				` s[l] = s[l]*(u_j^{b(l,j)} `; 

​			`}`

​		`}`

​	`}`	  

`return = 0; ` 

`}`



The value  t  in the first for-loop can be varied in order to form either products of two, or three, or four, and so on. The optimisation of these algorithms depends on whether they only form 'doubles' or 'triples' or 'quadruples', and how they form them. Unlike the Naive Algorithm which keeps spending multiplication disregarding existence of commn factors among the coefficients  $s_i$ , the optimised algorithm uses multiplication only if the new product formed is unique. 



The following table displays the multiplication cost of the Naive Algorithm together with other four algorithms. Full descriptions of these algorithms are given in the [Appendix](#appendix-a). These algorithms are simply referred to as; Algorithm 1 or [A1], Algorithm 2 or [A2], Algorithm 3 or [A3]  and Algorithm 4  or [A4].  





<div align="center"><b>Table 1: Comparison of Multiplication Costs </b></div>  





| Vector Size  $n$ | Naive Algorithm [NA] | Algorithm 1 [A1] | Algorithm 2  [A2] | Algorithm 3 [A3] | Algorithm 4 [A4] |Best Algo & Savings % relative to [NA] |
| :--------------- | -------------------: | ---------------: | ----------------: | ---------------: | ---------------: | --------------------------------------: |
| $n = 4$          |                  $4$ |              $4$ |               $4$ |              $4$ |              $4$ |                               [All]  0% |
| $n = 8$          |                 $12$ |             $12$ |              $12$ |             $12$ |             $12$ |                               [All]  0% |
| $n = 16$         |                 $28$ |             $28$ |              $28$ |             $24$ |             $24$ |                          [A3,A4] 14.29% |
| $n = 32$         |                 $60$ |             $48$ |              $60$ |             $48$ |             $56$ |                             [A1,A3] 20% |
| $n = 64$         |                $124$ |             $88$ |              $96$ |             $92$ |             $92$ |                             [A1] 29.03% |
| $n = 128$        |                $252$ |            $168$ |             $168$ |            $164$ |            $164$ |                         [A3, A4] 34.92% |
| $n = 256$        |                $508$ |            $316$ |             $312$ |            $304$ |            $304$ |                         [A3, A4] 40.16% |
| $n = 512$        |               $1020$ |            $612$ |             $600$ |            $584$ |            $816$ |                             [A3] 42.75% |
| $n = 1024$       |               $2044$ |           $1140$ |            $1144$ |           $1140$ |           $1332$ |                          [A1,A3] 44.23% |
| $n = 2048$       |               $4092$ |           $2184$ |            $2244$ |           $2234$ |           $2364$ |                             [A1] 46.63% | 
| $n = 4096$      |               $8188$ |          $4272$ |           $4436$ |        $4424$ |          $4424$ |                 [A1] 47.83% | 




All four algorithms  [A1], [A2], [A3] and [A4]  are fairly competent in saving multiplication costs, 
showing more than  20\%  savings for vectors of sizes  64  and above. 

The above results indicate that algorithm  [A1] is the best of the four for several reasons; 

(a) It is the second most frequent winner in all the 11 cases investigated, with a score of 7 out of 10, 

(b) In order to account for the other 4 cases; it is 2 times second best, and twice in 3rd place,  

(c) The best savings percentage of  47.83\%  relative to the Naive Algorithm,   

(d) The only case it is in par with the Naive Algorithm is when all other algorithms are in par, and 
that for the smallest cases  $n = 4$  and  $n = 8$. 






### Concluding the Amortized Inner-Product Proof

The amortization strategies herein applied to the Bulletproofs Inner-Product are tangible and significantly enhance the proof.   

With the above amortization of the IP Proof, the prover sends  $3log_2(n) +  n$   inner-product proof elements. That is the set of all  $log_2(n)$  triples  $L_j$, $R_j$ and $u^{-1}$ as well as the  $n$  coefficients  $s_i$'s. 

The given Verifier's Test of the coefficients and the optimisation of their computations solidify the proposed  amortization of the Inner-product proof. Given the Bulletproofs setting, that a vector of size  $n$  refers to  $n$  number of  32-byte values, even seemingly small savings are significant. 










## Application to the Tari Blockchain 



This report not only elucidates the often obscured parts of the Inner-Product Proof mechanism, but also succeeds to bring this powerful proof as close as possible to practicality.  

The Amortized Inner-Product Proof as discussed above does not veer off the Dalek's Bulletproofs setting which is preferred for the Tari Blockchain. 

These amortization strategies though minimal in the changes they bring into the Inner-Product Proof as we are accustomed to have huge savings to the verification costs, cutting down on the number of multiplications by a factor of  $log(p)$  where the prime  $p$  deliberately chosen to be very large. 

The amortized inner-product proof lends itself useful in implementations of any zero-knowledge proof that involves an inner-products especially in the Bulletproofs framework. 











## References 



[[1]] H. Wu, W. Zheng, A. Chiesa, R. A. Popa and I. Stoica, "DIZK: A Distributed Zero Knowledge Proof System", UC Berkeley [online]. 

Available: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf. Date accessed: 2019&#8209;12&#8209;07. 

[1]: https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf	"DIZK: A Distributed Zero Knowledge Proof System"





[[2]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More" [online], Blockchain Protocol Analysis and Security Engineering 2018 . 

Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2020&#8209;04&#8209;21. 

[2]: &lt;http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf&gt;	"Bulletproofs: Short Proofs for Confidential Transactions and More"





[[3]] ECC Posts, "Halo: Recursive Proof Composition without a Trusted Setup" [online] , Electric Coin Co., Sept. 2019

Available: https://electriccoin.co/blog/halo-recursive-proof-composition-without-a-trusted-setup/. Date accessed: 2020‑04‑24. 

[3]: https://electriccoin.co/blog/halo-recursive-proof-composition-without-a-trusted-setup/	"Halo: Recursive Proof Composition without a Trusted Setup"





[[4]]  I. Meckler and E. Shapiro, "Coda: Decentralized cryptocurrency at scale," [online] O(1) Labs May 10, 2018. 

Available: https://cdn.codaprotocol.com/v2/static/coda-whitepaper-05-10-2018-0.pdf. Date accessed: 2020&#8209;04&#8209;27. 

[4]: https://cdn.codaprotocol.com/v2/static/coda-whitepaper-05-10-2018-0.pdf	"Coda: Decentralized cryptocurrency at scale"





[[5]] M. Maller, S. Bowe, M. Kohlweiss and S. Meiklejohn, "Sonic: Zero-Knowledge SNARKs from Linear-Size Universal and Updatable Structured Reference Strings" [online], Proceedings of the 2019 ACM SIGSAC Conference on Computer and Communications, 2019.

Available: https://eprint.iacr.org/2019/099.pdf. Date accessed: 2020&#8209;04&#8209;27. 

[5]: https://eprint.iacr.org/2019/099.pdf	"Sonic: Zero-Knowledge SNARKs from Linear-Size Universal and Updatable Structured Reference Strings"





[[6]] William Foxley, "You Can Now Prove a Whole Blockchain With One Math Problem – Really," [online] Coindesk, 2019. 

Available: https://www.coindesk.com/you-can-now-prove-a-whole-blockchain-with-one-math-problem-really. Date accessed: 2020&#8209;04&#8209;27. 

[6]: https://www.coindesk.com/you-can-now-prove-a-whole-blockchain-with-one-math-problem-really	"You Can Now Prove a Whole Blockchain With One Math Problem – Really"





[[7]]  Dalek's Bulletproofs documents, "Module Bulletproofs::notes ::index" [online]. 

Available: https://doc-internal.dalek.rs/bulletproofs/notes/index.html. Date accessed: 2020‑05‑01.

[7]: https://doc-internal.dalek.rs/bulletproofs/notes/index.html	"Module Bulletproofs::notes ::index"

 



[[8]] C. Yun, "Building on Bulletproofs" [online]. 

Available: <https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8>. Date accessed: 2020‑04‑27.

[8]: https://medium.com/@cathieyun/building-on-bulletproofs-2faa58af0ba8	"Building on Bulletproofs"





[[9]] J. Bootle, A. Cerulli, P. Chaidos, J. Groth and C. Petit, "Efficient Zero-knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting" [online], Annual International Conference on the Theory and Applications of Cryptographic Techniques, pp. 327‑357. Springer, 2016 . 

Available: https://eprint.iacr.org/2016/263.pdf. Date accessed: 2019‑12‑21. 

[9]: https://eprint.iacr.org/2016/263.pdf	"Efficient Zero-knowledge Arguments for Arithmetic Circuits in the Discrete Log Setting"





[[10]] H. de Valence, "Merlin: flexible, composable transcripts for zero-knowledge proofs" [online]. 

Available: https://medium.com/@hdevalence/merlin-flexible-composable-transcripts-for-zero-knowledge-proofs-28d9fda22d9a. Date accessed: 2019‑12‑21. 

[10]: https://medium.com/@hdevalence/merlin-flexible-composable-transcripts-for-zero-knowledge-proofs-28d9fda22d9a	"Merlin: flexible, composable transcripts for zero-knowledge proofs"





[[11]] Dalek's Bulletproofs documents, "Module Bulletproofs:: notes :: index" [online] 

Available: https://doc-internal.dalek.rs/bulletproofs/notes/inner_product_proof/index.html. Date accessed: 2020‑04‑27. 

[11]: https://doc-internal.dalek.rs/bulletproofs/notes/inner_product_proof/index.html	"Module Bulletproofs:: notes :: inner-product-proof"





[[12]] History of Science and Mathematics Beta, "Who introduced the Principle of Mathematical Induction for the first time?" [online]. 

Available: https://hsm.stackexchange.com/questions/524/who-introduced-the-principle-of-mathematical-induction-for-the-first-time. Date accessed: 2020‑04‑30. 

[12]: https://hsm.stackexchange.com/questions/524/who-introduced-the-principle-of-mathematical-induction-for-the-first-time	"Who introduced the Principle of Mathematical Induction for the first time?"





[[13]]  M. Straka, "Recursive Zero-Knowledge Proofs: A Comprehensive Primer" [online]. December 8, 2019.

Available: https://www.michaelstraka.com/posts/recursivesnarks/. Date accessed: 2020‑04‑30. 

[13]: https://www.michaelstraka.com/posts/recursivesnarks/	"Recursive Zero-Knowledge Proofs: A Comprehensive Primer"

   



[[14]] S. Bowe, J. Grigg and D. Hopwood "Recursive Proof Composition without a Trusted Setup" [online].  Electric Coin Company, 2019.

Available: https://eprint.iacr.org/2019/1021.pdf. Date accessed: 2020‑04‑24. 

[14]: https://eprint.iacr.org/2019/1021.pdf	"Recursive Proof Composition without a Trusted Setup"













## Appendix A 



Find here details of the algorithms investigated to optimise computations of the coefficients $\{ s_i \}$  of  $G_i$ the component of the vector input to the IP Proof,  $\mathbf{G} = ( G_0 , G_1 , G_2 , ... , G_{n-1})$. 



**Naive Algorithm**: The naive algorithm codes computation of the coefficients  $s_i = \prod\limits_{j = 1}^k u_j^{b(i,j)}$  by cumulatively multiplying the correct factor  $u_j^{b(i,j)}$  in each $j-$th round of the IP Proof, running from  $k = log_2(n)$  to  $1$. Outlined below;  



**Optimisation Approach**

The basic approach to formulation of efficient and cost-saving algorithms is as follows; 

- each algorithm aims at reducing verification costs in terms of the number of multiplications, 

- each algorithm takes advantage of the fact that each coefficient has common sub-products with other  $2^{k-2}$ coefficients, 

- the intermediate values of the coefficients are not used anywhere in the IP Proof but only the final values made up of all  $k$  factors  $u_j^{b(i,j)}$. Hence their sub-products can be separately computed and only put together at the end of the IP Proof's  $k-$th round, 

-  due to the algebraic structure of the sets of coefficients, they can be systematically computed; note specifically that each 
  $$s_i\ \ =\ \ \prod\limits_{j = 1}^k u_j^{b(i,j)}\ \ =\ \ u_k^{b(i,k)} \cdot u_{k-1}^{b(i,k-1)} \cdot\ \ ...\ \ \cdot u_2^{b(i,2)} \cdot u_1^{b(i,1)} $$ 
  corresponds to a $k-$tuple of field elements 
  $$\Big( u_k^{b(i,k)}, u_{k-1}^{b(i,k-1)}, ... , u_2^{b(i,2)}, u_1^{b(i,1)}\Big)\ \ \in\ \ \mathbb{F}_p^k .$$ 

  Hence sub-products like,  $u_4^{b(i,4)} \cdot u_{3}^{b(i,3)}$  or  $u_{10}^{b(i,10)} \cdot u_{9}^{b(i,9)} \cdot u_{8}^{b(i,8)}$  or  $u_{4}^{b(i,4)} \cdot u_{3}^{b(i,3)} \cdot u_{2}^{b(i,2)} \cdot u_{1}^{b(i,1)}$ are herein referred to as 'doubles' or 'triples' or 'quadruples', respectively.     



**Defining the optimised algorithms**

Since the main strategy with these algorithms is to avoid insisting on immediate updates on the values  $s_i$ , they differ in their scheduling of when 'doubles' or 'triples' or 'quadruples' are computed. 

Note that by "disctinct" sub-products we refer to those with no common factor. 



**Algorithm 1**: This algorithm computes new distinct doubles as soon as it is possible to do so. These new doubles are turned into triples in the next immediate IPP round. This process is repeated until all IPP rounds are completed. Only then are next larger-sized sub-products computed, but consumption of the smallest existing 'tuples' is given priority. 



`**Algorithm 1 or [A1]**`

` initialise 	s[n] = [1,1,1, ... ,1]; ` 

` initialise 	k = log_2(n)` 

`... %running inside IPP`

`int main(){` 

​	` s[0] = u_k^{-1};  s[2^{k-1}] = u_k;`

​	`s[2^{k-2}] = s[0]; s[3*2^{k-2}] = s[2^{k-1}]; `

​	`t = k-3;`

​	` 		for (j = k - 1; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if ( i mod 2^j == 0 ) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^(j-1); `

​				` s[l] = s[l]*(u_j^{b(l,j)} `;

​			`}` `}` `}` 

` %if k-3 > 0 then program proceeds as follows  `

​	` s[1] = u_{k-3}^{-1}; s[1+2^{k-4}] = u_{k-3}; `

​	` s[1+2^{k-1}] = s[1]; s[(1+2^{k-4})+2^{k-1}] = s[1+2^{k-4}]; `

` %if k-4 > 0 then program proceeds as follows  ` 

​	`t = k-6;` 

​	` 		for (j = k-4; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if (i mod (1+2^(k-1)) == 0) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^j; `

​				` s[l] = s[l]*(u_j^{b(l,j)} `;

​			`}` `}` `}` 

`% program continues forming new and distinct triples until k=1 `

`% after which all (2^k) 'legal' k-tuples are formed` 	  

`return = 0; ` 

`}`





**Algorithm 2**: This algorithm starts exactly like Algorithm 1, by forming only new and distinct doubles and turning them into triples in the next immediate IPP round, but goes beyond the triples by immediately forming the next possible quadruples.    



`**Algorithm 2 or [A2]**`

` initialise 	s[n] = [1,1,1, ... ,1]; ` 

` initialise 	k = log_2(n)` 

`... %running inside IPP`

`int main(){` 

​	` s[0] = u_k^{-1};  s[2^{k-1}] = u_k;`

​	`s[2^{k-2}] = s[0]; s[3*2^{k-2}] = s[2^{k-1}]; `

​	`t = k-4;`

​	` 		for (j = k - 1; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if ( i mod 2^j == 0 ) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^(j-1); `

​				` s[l] = s[l]*(u_j^{b(l,j)} `;

​			`}` `}` `}` 

` %if k-4 > 0 then program proceeds as follows  `

​	` s[1] = u_{k-4}^{-1};  s[1+2^{k-4}] = u_{k-4};`

​	`s[1+2^{k-1}] = s[1]; s[(1+2^{k-4})+(2^{k-1})] = s[1+2^{k-4}]; ` 

` %if k-5 > 0 then program proceeds as follows  `

​	`t = k-8;` 

​	` 		for (j = k-5; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if ( i mod (1+2^(k-1) == 0 ) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^j; `

​				` s[l] = s[l]*(u_j^{b(l,j)} `;

​			`}` `}` `}` 

​	`% continues forming new and distinct quadruples until k=1 `

​	`% after which all (2^k) 'legal' k-tuples are formed` 	  

`return = 0; ` 

`}`





**Algorithm 3**: This algorithm computes new distinct doubles as soon as it is possible to do so until the end of the IPP rounds. The program then forms any possible triples. Then next, larger-sized sub-products are computed by firstly consuming the smallest existing 'tuples'. 

 

`**Algorithm 3 or [A3]**`

`initialise 	s[n] = [1,1,1, ... ,1]; ` 

` initialise 	k = log_2(n)` 

`... %running inside IPP`

`int main(){` 

​	` s[0] = u_k^{-1};  s[2^{k-1}] = u_k;`

​	`s[2^{k-2}] = s[0]; s[2^{k-1} + 2^{k-2}] = s[2^{k-1}]; `

​	`t = k-2;`

​	` 		for (j = k - 1; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if ( i mod 2^j == 0 ) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^(j-1); `

​				` s[l] = s[l]*(u_j^{b(l,j)} `;

​			`}` `}` `}` 

` %if k-2 > 0 then program proceeds as follows  `

​	` s[1] = u_{k-2}^{-1};  s[1+2^{k-1}] = u_{k-2};`

​	`s[1+2^{k-2}] = s[1]; s[1+3*(2^{k-2})] = s[1+2^{k-1}]; `

` %if k-3 > 0 then program proceeds as follows  ` 

​	`t = k-3;` 

​	` 		for (j = k-3; j > t; j--){`

​		`		for (i = 0; i < n; i++){`

​			`if ( i mod 1+2^(k-1) == 0 ) { `

​				`	 s[i] = s[i]*(u_j)^{b(i,j)} `; 

​				` l = i + 2^j; `

​				` s[l] = s[l]*(u_j^{b(l,j)} `;

​			`}` `}` `}` 

`% program continues forming new and distinct doubles until k=1 `

`% after which all (2^k) 'legal' k-tuples are formed` 	  

`return = 0; ` 

`}`







**Algorithm 4**: This algorithm is the same as Algorithm 3 throughout the IPP rounds. But at the end of the IPP rounds the program gives preference to the formation all possible distinct quadruples. Then next, larger-sized sub-products are computed by firstly consuming the largest existing 'tuples'. 



`**Algorithm 4 or [A4]**`

`The exact same pseudocode of Algorithm 3 applies to Algorithm 4`







**Example 1 (Algorithm 1 or [A1])** 



Let  $n = 32$  so that  $k = 5$. The coefficients  $s_i$  for  $i \in {0, 1, 2, ... , 15}$  are  $32$  quintets; 

 $$u_5^{b(i,5)} * u_4^{b(i,4)} * u_3^{b(i,3)} * u_2^{b(i,2)} * u_1^{b(i,1)}$$ 

The number of multiplications per IPP-round is given at the bottom of each column. The vector of coefficients is initialised to 
$\mathbf{s} = ( s_0 = 1, s_1 = 1, s_2 = 1, ... , s\_{n-1} = 1 )$. 
The table only displays the updated and computed values  $s_i$  since the initialisation of 
$\mathbf{s}$.  





<div align="center"><b>Table 2: Sub-products and their Multiplication Costs using Algorithm 1  </b></div>  



| [A1] $\text{ for }$ $ n=2^5$ | j = 5      | j = 4                 | j = 3                            | j = 2      | j = 1               |
| ----------------------- | ---------- | --------------------- | -------------------------------- | ---------- | ------------------- |
| $s_0$                    | $u_5^{-1}$ | $u_5^{-1} \* u_4^{-1}$ | $u_5^{-1} \* u_4^{-1} \* u_3^{-1}$ |             |                     |
| $s_1$                    |            |                       |                                  | $u_2^{-1}$ | $u_2^{-1} \* u_1$ |
| $s_2$                    |            |                       |                                  |            |                     |
| $s_3$                    |            |                       |                                  | $u_2$      | $u_2 \* u_1^{-1}$     |
| $s_4$                    |            |                       | $u_5^{-1} \* u_4^{-1} \* u_3$       |            |                     |
| $s_5$                    |            |                       |                                  |            |                     |
| $s_6$                    |            |                       |                                  |            |                     |
| $s_7$                    |            |                       |                                  |            |                     |
| $s_8$                    | $u_5^{-1}$ | $u_5^{-1} \* u_4$ | $u_5^{-1} \* u_4 \* u_3^{-1}$ |             |                     |
| $s_9$                    |            |                       |                                  |            |     |
| $s\_{10}$                 |            |                       |                                  |            |                     |
| $s\_{11}$                 |            |                       |                                  |            |                     |
| $s\_{12}$     |              |                 | $u_5^{-1} \* u_4 \* u_3$ |               |                    |
| $s\_{13}$       |               |           |               |                |                 |
| $s\_{14}$                 |            |                       |                                  |            |                     |
| $s\_{15}$                 |            |                       |                                  |            |                     |
| $s\_{16}$                 | $u_5$      | $u_5 \* u_4^{-1}$        | $u_5 \* u_4^{-1} \* u_3^{-1} $         |            |                     |
| $s \_{17}$                 |            |                       |                                  | $u_2^{-1}$ | $u_2^{-1} \* u_1^{-1}$ |
| $s\_{18}$                 |            |                       |                                  |            |                     |
| $s\_{19}$                 |            |                       |                                  | $u_2$      | $u_2 \* u_1$           |
| $s\_{20}$                 |            |                       | $ u_5 \* u_4^{-1} \* u_3$              |            |                     |
| $s\_{21}$                 |            |                       |                                  |            |                     |
| $s\_{22}$                 |            |                       |                                  |            |                     |
| $s\_{23}$                 |            |                       |                                  |            |                     |
| $s\_{24}$ | $u_5$ | $u_5 \* u_4 $ | $u_5 * u_4 * u_3^{-1} $ |  |  |
| $s\_{25}$                 |            |                       |                                  |            |                     |
| $s\_{26}$                 |            |                       |                                  |            |                     |
| $s\_{27}$ |     |     |      |     |       |
| $ s\_{28} $ |          |        | $u_5 * u_4 * u_3 $ |        |        |
| $s\_{29}$                 |            |                       |                                  |            |                     |
| $s\_{30}$                 |            |                       |                                  |            |                     |
| $s\_{31}$                 |            |                       |                                  |            |                     |
| mult. cost               | $0$        | $4$                   | $8$                              | $0$        | $4$                 |







At the end of the  $k$  IPP rounds, there are  $8$  distinct triples and 4 distinct doubles. These are sufficient to form all the required  $32$  quintets, and it takes exactly  $32$  multiplications form them all. 

The **total cost** of computing the coefficients for  $n = 32$  using  *Algorithm 1*  is  $\mathbf{4 + 8 + 4 + 32 = 48}$.  




























