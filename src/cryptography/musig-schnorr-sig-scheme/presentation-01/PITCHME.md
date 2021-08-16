---
marp: true
theme: default
paginate: true
footer: © Tari Labs, 2018-2021. (License : CC BY-NC-SA 4.0)
---

<style>
section {
  font-size: 1.5em;
}
</style>

## The MuSig Schnorr Signature Scheme

- Introduction
- Terminology Recap
- Recap on Schnorr Signature Scheme
- Design of the Schnorr Multi-Signature Scheme
- Bellare and Neven Signature Scheme
- MuSig Scheme
- Revisions
- Turning BN’s Scheme into a Secure IAS
- Conclusions

[See full report here](https://tlu.tarilabs.com/cryptography/musig-schnorr-sig-scheme/The_MuSig_Schnorr_Signature_Scheme.html).

---

## Introduction

### MuSig

MuSig is a multi-signature scheme that is novel in combining:

- Support for key aggregation;
- Security in the plain public-key model.

There are two versions of MuSig, that are provably secure, which
differ based on the number of communication rounds:

- Three-round MuSig only relies on the Discrete Logarithm (DL) assumption, on which Elliptic Curve Digital Signature Algorithm (ECDSA) also relies
- Two-round MuSig instead relies on the slightly stronger One-More Discrete Logarithm (OMDL) assumption

---

## Terminology Recap

- Let $p​$ be a large prime number

- Let $\mathbb G$ denote a cyclic group of prime order $p​$

- Let $\mathbb Z_p$ denote the ring of integers $modulo \mskip{4mu} p$

- Let a generator of  $\mathbb{G}$ be denoted by $g$. Thus, there exists a number $g \in\mathbb{G}$ such that $\mathbb{G} =  \lbrace 1, \mskip{3mu}g,  \mskip{3mu}g^2,\mskip{3mu}g^3, ..., \mskip{3mu}g^{p-1} \rbrace$.

- Let $\textrm{H}$ denote the hash function.

- Let $S= \lbrace (X_1,m_1),..., (X_n,m_n) \rbrace$ be the multi-set of all public key/message pairs of all participants, where $X_1=g^{x_1}$.

- Let $\langle S \rangle$ denote a lexicographically encoding of the multiset of public key/message pairs in $S$.

- Let $L= \lbrace X_1=g^{x_1},...,X_n=g^{x_n} \rbrace$ be the multi-set of all public keys.


---

- Let $\langle L \rangle$ denote a lexicographically encoding of the multiset of public keys $L= \lbrace X_{1}...X_{n} \rbrace$.

- Let $\textrm{H}_{com}$ denote the hash function in the commitment phase.

- Let $\textrm{H}_{agg}$ denote the hash function used to compute the aggregated key.

- Let $\textrm{H}_{sig}$ denote the hash function used to compute the signature.

- Let $X_{1}$ and $x_{1}$ be the public and private key of a specific signer.

- Let $m$ be the message that will be signed.

- Let $X_{2},...,X_{n}$ be the public keys of other cosigners.

---

## Recap on the Schnorr Signature Scheme

To sign a message $m$, the signer draws a random integer $r \in Z_{p}$ and computes

$$
\begin{aligned}
R &= g^{r} \\\\
c &= \textrm{H}(X,R,m) \\\\
s &= r+cx
\end{aligned}
$$

---

The signature is the pair $(R,s)$, and its validity can be checked by verifying whether

$$
g^{s} = RX^{c}
$$

This scheme is referred to as the "key-prefixed" variant of the scheme, which sees the public key hashed together with $R​$ and $m​$.

---

## Design of the Schnorr Multi-Signature Scheme

A group of $n$ signers want to cosign a message $m$.
Each cosigner randomly generates and communicates to others a share

$$
R_i = g^{r_{i}}
$$

Each of the cosigners then computes:

$$
R = \prod_{i=1}^{n}R_{i} \mskip{30mu} \mathrm{and} \mskip{30mu} c = \textrm{H} (\tilde{X},R,m)
$$

where

$$
\tilde{X} = \prod_{i=1}^{n}X_i
$$

is the product of individual public keys

---

and a partial signature is then given by

$$
s_{i} = r_{i}+cx_{i}
$$

All partial signatures are then combined into a single signature $(R,s)​$ where

$$
s = \displaystyle\sum_{i=1}^{n}s_i \mod p ​
$$

The validity of a signature $(R,s)$ on message $m$ for public keys $\lbrace X_{1},...X_{n} \rbrace$ is equivalent to

$$
g^{s} = R\tilde{X}^{c}
$$

where
$$
\tilde{X} = \prod_{i=1}^{n}X_{i} \mskip{30mu} \mathrm{and} \mskip{30mu}  c = \textrm{H}(\tilde{X},R,m)
$$

---

Note that this is exactly the verification equation for a traditional key-prefixed Schnorr signature with respect to public key $\tilde{X}$, a property termed *key aggregation*.

However, these protocols are vulnerable to a rogue-key attack where a corrupted signer sets its public key to

$$
X_{1}=g^{x_{1}} (\prod_{i=2}^{n}X_{i})^{-1}
$$

allowing the signer to produce signatures for public keys $\lbrace X_{1},...X_{n} \rbrace$ by themselves.


---

### Bellare and Neven Signature Scheme

A group of $n$ signers want to cosign a message $m$. Their main idea is to have each cosigner use a distinct "challenge" when computing their partial signature

$$
s_{i} = r_{i}+c_{i}x_{i}
$$

defined as

$$
c_{i} = \textrm{H}( \langle L \rangle , X_{i},R,m)
$$

where

$$
R = \prod_{i=1}^{n}R_{i}
$$

The equation to verify signature $ (R,s) $ on message $ m $ for the public keys $ L $ is

$$
g^s = R\prod_{i=1}^{n}X_{i}^{c_{i}}
$$

---

A preliminary round is also added to the signature protocol, where each signer commits to its share $R_i$ by sending $t_i = \textrm{H}^\prime(R_i)$ to other cosigners first.

This stops any cosigner from setting $R = \prod_{i=1}^{n}R_{i}$ to some maliciously chosen value.

Bellare and Neven showed that this yields a multi-signature scheme provably secure in the *plain public-key* model under the Discrete Logarithm assumptions. However, this scheme does not allow key aggregation since the entire list of public keys is required for verification.


---

## MuSig Scheme

MuSig is paramaterised by group parameters $(\mathbb{G\mathrm{,p,g)}}$ and three hash functions $( \textrm{H}_{com}  ,  \textrm{H}_{agg} , \textrm{H}_{sig} )$.


### Round 1

A group of $n$ signers want to cosign a message $m$. Let $X_1$ and $x_1$ be the public and private key of a specific signer, let $X_2 , . . . , X_n$ be the public keys of other cosigners and let $\langle L \rangle$ be the multiset of all public keys involved in the signing process.

For $i\in  \lbrace 1,...,n \rbrace​$ , the signer computes the following

$$
a_{i} = \textrm{H}_{agg}(\langle L \rangle,X_{i})
$$

as well as the "aggregated" public key

$$
\tilde{X} = \prod_{i=1}^{n}X_{i}^{a_{i}} ​
$$
---

### Round 2

The signer generates a random $r_{1}\leftarrow\mathbb{Z_{\mathrm{p}}}$, computes $R_{1} = g^{r_{1}}$ and $t_{1} = \textrm{H}_{com}(R_{1})$ and sends commitment $t_{1}$ to all other cosigners.

When receiving the commitments $t_{2},...,t_{n}$ from the other cosigners, the signer sends $R_{1}$ to all other cosigners.

Upon receiving $R_2,...,R_n$ from other cosigners, the signer verifies that $t_{i}=\textrm{H}_{com}(R_{i})$ for all $i\in  \lbrace 2,...,n \rbrace$

The protocol is aborted if this is not the case.

---

### Round 3

If all commitment and random challenge pairs can be verified with $\textrm{H}_{com}$, the following is computed:

$$
\begin{aligned}
R &= \prod_{i=1}^{n}R_i \\\\
c &= \textrm{H}_{sig} (\tilde{X},R,m) \\\\
s_1 &= r_1 + ca_1 x_1 \mod p
\end{aligned}
$$

Signature $s_{1}$ is sent to all other cosigners.
When receiving $s_{2},...s_{n}$ from other cosigners, the signer can compute $s = \sum_{i=1}^{n}s_{i} \mod p$. The signature is $\sigma = (R,s)$.

---

In order to verify the aggregated signature $\sigma = (R,s)$, given a lexicographically encoded multiset of public keys $\langle L \rangle​$ and message $m$, the verifier computes:

$$
\begin{aligned}
a_{i} &= \textrm{H}_{agg}(\langle L \rangle,X_{i}) \mskip{9mu} \textrm {for} \mskip{9mu}  i \in  \lbrace 1,...,n \rbrace  \\\\
\tilde{X} &= \prod_{i=1}^{n}X_{i}^{a_{i}} \\\\
c &=  \textrm{H}_{sig} (\tilde{X},R,m)
\end{aligned}
$$

then accepts the signature if

$$
g^{s} = R\prod_{i=1}^{n}X_{i}^{a_{i}c}=R\tilde{X^{c}.}
$$
---

## Revisions

In a previous version of the paper by Maxwell published on 15 January 2018 they proposed a 2-round variant of MuSig, where the initial commitment round is omitted claiming a security proof under the One More Discrete Logarithm (OMDL) assumptions. Drijvers then discovered a flaw in the security proof and showed that through a meta-reduction the initial multi-signature scheme cannot be proved secure using an algebraic black box reduction under the OMDL assumption.

In more details, it was observed that in the 2-round variant of MuSig, an adversary (controlling public keys $X_{2},...,X_{n}$) can impose the value of $R=\Pi_{i=1}^{n}R_{i}$ used in signature protocols since he can choose $R_{2},...,R_{n}$ after having received $R_{1}$ from the honest signer (controlling public key $X_{1}=g^{x_{1}}$ ). This prevents one to use the initial method of simulating the honest signer in the Random Oracle model without knowing $x_{1}$ by randomly drawing $s_{1}$ and $c$, computing $R_1=g^{s_1}(X_1)^{-a_1c}$, and later programming $\textrm{H}_{sig}(\tilde{X}, R, m) \mskip{2mu} : = c_1$ since the adversary might have made the random oracle query $\textrm{H}_{sig}(\tilde{X}, R, m)$ *before*  engaging the corresponding signature protocol.

---

Despite this, there is no attack currently known against the 2-round variant of MuSig and that it might be secure, although this is not provable under standard assumptions from existing techniques.

---

## Turning BN's Scheme into a Secure IAS

In order to change the BN multi-signature scheme into an IAS scheme, Maxwell proposed the scheme described below, which includes a fix to make the execution of the signing algorithm dependent on the message index.

If $X = g^{x_i}$ is the public key of a specific signer and $m$ the message he wants to sign, and

$$
S^\prime =  \lbrace (X^\prime_1, m^\prime_1),..., (X^\prime_{n-1}, m^\prime_{n-1}) \rbrace
$$

is the set of the public key/message pairs of other signers, this specific signer merges $(X, m)$ and $S^\prime$ into the ordered set

$$
\langle S \rangle \mskip{6mu} \mathrm{of} \mskip{6mu}  S =  \lbrace (X_{1}, m_{1}),..., (X_{n}, m_{n}) \rbrace
$$

and retrieves the resulting message index $i​$ such that

$$
(X_{1}, m_{i}) = (X, m)
$$

---

Each signer then draws $r_{1}\leftarrow\mathbb{Z_{\mathrm{p}}}$, computes $R_{i} = g^{r_{i}}$ and subsequently sends commitment $t_{i} = H^\prime(R_{i})​$ in a first round and then $R_{i}$ in a second round, and then computes

$$
R = \prod_{i=1}^{n}R_{i}
$$

The signer with message index $i​$ then computes:

$$
\begin{aligned}
c_{i} &= H(R, \langle S \rangle, i)\\\\
s_{i} &= r_{i} + c_{i}x_{i} \mod p
\end{aligned}
$$

and then sends $s_i$ to other signers. All signers can compute

$$
s = \displaystyle\sum_{i=1}^{n}s_{i} \mod p
$$

---

The signature is $\sigma = (R, s)​$.

Given an ordered set $\langle S \rangle \mskip{6mu} \mathrm{of} \mskip{6mu} S =  \lbrace (X_{1}, m_{1}),...,(X_{n}, m_{n}) \rbrace$ and a signature $\sigma = (R, s)$  then $\sigma$ is valid for $S​$ when

$$
g^s = R\prod_{i=1}^{n}X_{i} ^{H(R, \langle S \rangle, i)}
$$

<!-- **Note:** As of writing of this, the secure IAS scheme presented here still needs to undergo a complete security analysis. -->
---

## Conclusions, Observations and Recommendations

- MuSig leads to both native and private multi-signature transactions with signature aggregation
- Signature data for multi-signatures can be large and cumbersome. MuSig will allow users to create more complex transactions without burdening the network and revealing compromising information.
- The IAS case where each signer signs their own message must still be proven by a complete security analysis.
