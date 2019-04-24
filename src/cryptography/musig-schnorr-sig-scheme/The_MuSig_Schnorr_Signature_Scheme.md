# The MuSig Schnorr Signature Scheme 

- - [Introduction](#introduction)
    - [Schnorr Signatures and their Attack Vectors](#schnorr-signatures-and-their-attack-vectors)
    - [MuSig](#musig)
    - [Key Aggregation](#key-aggregation) 

  - [Overview of Multi-signatures](#overview-of-multi-signatures)
    - [Bitcoin $m-of-n$ Multi-signatures](#bitcoin--m-of-n--multi-signatures)
      - [What are $m-of-n$ Transactions?](#what-are-$-m-of-n-$-transactions)
      - [Use Cases for $m-of-n $ Multi-signatures](#use-cases-for--m-of-n--multi-signatures)
    - [Rogue Attacks](#rogue-attacks)
    - [Interactive Aggregate Signatures](#interactive-aggregate-signatures)
      - [Applications of Interactive Aggregate Signatures](#applications-of-interactive-aggregate-signatures)
      - [Native Multi-signature Support](#native-multi-signature-support)
      - [Cross-input Multi-signatures](#cross-input-multi-signatures)
      - [Protection against Rogue-key Attacks](#protection-against-rogue-key-attacks)

  - [Formation of MuSig](#formation-of-musig)
    - [Notation Used](#notation-used)

    - [Recap on Schnorr Signature Scheme](#recap-on-schnorr-signature-scheme)
    - [Design of Schnorr Multi-signature Scheme](#design-of-schnorr-multi-signature-scheme)
    - [Bellare and Neven Signature Scheme](#bellare-and-neven-signature-scheme)   
    - [MuSig Signature Scheme](#musig-signature-scheme)
    - [Revisions](#revisions) 
    - [Turning Bellare and Neven's Scheme into an IAS Scheme](#turning-bellare-and-nevens-scheme-into-an-ias-scheme)

  - [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)

  - [References](#references)

  - [Contributors](#contributors)

## Introduction 

This report investigates Schnorr Multi-Signature Schemes (MuSig), which make use of key aggregation and are provably secure in the *plain public-key model*.

Signature aggregation involves mathematically combining several signatures into a single signature, without having to prove Knowledge of Secret Keys (KOSK). This is known as the *plain public-key model,* where the only requirement is that each potential signer has a public key. The KOSK scheme requires that users prove knowledge (or possession) of the secret key during public key registration with a certification authority. It is one way to generically prevent rogue-key attacks.

Multi-signatures are a form of technology used to add multiple participants to cryptocurrency transactions. A traditional multi-signature protocol allows a group of signers to produce a joint multi-signature on a common message.

### Schnorr Signatures and their Attack Vectors 

Schnorr signatures produce a smaller on-chain size, support faster validation and have better privacy. They natively allow for combining multiple signatures into one through aggregation and they permit more complex spending policies.

Signature aggregation also has its challenges. This includes the rogue-key attack, where a participant steals funds using a specifically constructed key. Although this is easily solved for simple multi-signatures through an enrollment procedure that involves the keys signing themselves, supporting it across multiple inputs of a transaction requires *plain public-key security*, meaning there is no setup.

There is an additional attack, named the Russel attack, after Russel O'Connor, who discovered that for multiparty schemes, a party could claim ownership of someone else's private key and so spend the other outputs.

P. Wuille&nbsp;[[1]] has addressed some of these issues and has provided a solution that refines the Bellare-Neven (BN) scheme. He also discussed the performance improvements that were implemented for the scaler multiplication of the BN scheme and how they enable batch validation on the blockchain&nbsp;[[2]].

### MuSig

Introduced by [[3]], multi-signature protocols allow a group of signers (that individually possess their own private/public key pair) to produce a single signature $ \sigma $ on a message $ m $. Verification of the given signature $ \sigma $ can be publicly performed given the message and the set of public keys of all signers.

A simple way to change a standard signature scheme into a multi-signature scheme is to have each signer produce a stand-alone signature for $ m $ with its private key, and to then concatenate all individual signatures.

The transformation of a standard signature scheme to a multi-signature scheme needs to be useful and practical. The newly calculated multi-signature scheme must therefore produce signatures where the size is independent of the number of signers and similar to that of the original signature scheme&nbsp;[[4]].

A traditional multi-signature scheme is a combination of a signing and verification algorithm, where multiple signers (each with their own private/public key) jointly sign a single message, resulting in a combined signature. This can then be verified by anyone knowing the message and the public keys of the signers, where a trusted setup with KOSK is a requirement.

MuSig is a multi-signature scheme that is novel in combining:

- support for key aggregation; and
- security in the *plain public-key model*.

There are two versions of MuSig that are provably secure, and which differ based on the number of communication rounds:

1. Three-round MuSig, which only relies on the Discrete Logarithm (DL) assumption, on which Elliptic Curve Digital Signature Algorithm (ECDSA) also relies.
2. Two-round MuSig, which instead relies on the slightly stronger One-More Discrete Logarithm (OMDL) assumption.

### Key Aggregation

The term *key aggregation* refers to multi-signatures that look like a single-key signature, but with respect to an aggregated public key that is a function of only the participants' public keys. Thus, verifiers do not require the knowledge of the original participants' public keys. They can just be given the aggregated key. In some use cases, this leads to better privacy and performance. Thus, MuSig is effectively a key aggregation scheme for Schnorr signatures.

To make the traditional approach more effective and without needing a trusted setup, a multi-signature scheme must provide sublinear signature aggregation, along with the following properties:

- It must be provably secure in the *plain public-key model*.
- It must satisfy the normal Schnorr equation, whereby the resulting signature can be written as a function of a combination of the public keys.
- It must allow for Interactive Aggregate Signatures (IAS), where the signers are required to cooperate
- It must allow for Non-interactive Aggregate Signatures (NAS). where the aggregation can be done by anyone.
- It must allow each signer to sign the same message.
- It must allow each signer to sign their own message.

This is different to a normal multi-signature scheme, where one message is signed by all. MuSig potentially provides all of those properties.

Other multi-signature schemes that already exist, provide key aggregation for Schnorr signatures. However, they come with some limitations, such as needing to verify that participants actually have the private key corresponding to the pubic keys that they claim to have. *Security in the plain public-key model* means that no limitations exist. All that is needed from the participants is their public keys&nbsp;[[1]].

## Overview of Multi-signatures 

Recently, the most obvious use case for multi-signatures is with regard to Bitcoin, where it can function as a more efficient replacement of $ n-of-n $ multisig scripts (where the signatures required to spend and the signatures possible are equal in quantity) and other policies that permit a number of possible combinations of keys.

A key aggregation scheme also lets one reduce the number of public keys per input to one, as a user can send coins to the aggregate of all involved keys rather than including them all in the script. This leads to a smaller on-chain footprint, faster validation and better privacy.

Instead of creating restrictions with one signature per input, one signature can be used for the entire transaction. Traditionally, key aggregation cannot be used across multiple inputs, as the public keys are committed to by the outputs, and those can be spent independently. MuSig can be used here, with key aggregation done by the verifier.

No non-interactive aggregation scheme is known that only relies on the DL assumption, but interactive schemes are trivial to construct where a multi-signature scheme has every participant sign the concatenation of all messages. Reference [[4]] focused on key aggregation for Schnorr Signatures, showed that this is not always a desirable construction, and gave an IAS variant of BN with better properties instead&nbsp;[[1]].

### Bitcoin $ m-of-n $ Multi-signatures 

#### What are $m-of-n$ Transactions?

Currently, standard transactions on the Bitcoin network can be referred to as single-signature transactions, as they require only one signature, from the owner of the private key associated with the Bitcoin address. However, the Bitcoin network supports much more complicated transactions, which can require the signatures of multiple people before the funds can be transferred. These are often referred to as $ m-of-n $ transactions, where m represents the number of signatures required to spend, while n represents the number of signatures possible&nbsp;[[5]].

#### Use Cases for $ m-of-n $ Multi-signatures

When $ m=1 $ and $ n>1 $, it is considered a **shared wallet**, which could be used for small group funds that do not require much security. This is the least secure multi-signature option, because it is not multifactor. Any compromised individual would jeopardize the entire group. Examples of use cases include funds for a weekend or evening event, or a shared wallet for some kind of game. Besides being convenient to spend from, the only benefit of this setup is that all but one of the backup/password pairs could be lost and all of the funds would be recoverable.

When $ m=n $, it is considered a **partner wallet**, which brings with it some nervousness as no keys can be lost. As the number of signatures required increases, the risk also increases. This type of multi-signature can be considered to be a hard multifactor authentication.

When $ m<0.5n $, it is considered a **buddy account**, which could be used for spending from corporate group funds. The consequence for the colluding minority needs to greater than possible benefits. It is considered less convenient than a shared wallet, but much more secure.

When $ m>0.5n $, it is referred to as a **consensus account**. The classic multi-signature wallet is a two of three and is a special case of a consensus account. A two of three scheme has the best characteristics for creating new bitcoin addresses, and for secure storing and spending. One compromised signatory does not compromise the funds. A single secret key can be lost and the funds can still be recovered. If done correctly, off-site backups are created during wallet setup. The way to recover funds is known by more than one party. The balance of power with a multi-signature wallet can be shifted by having one party control more keys than the other parties. If one party controls multiple keys, there is a greater risk of those keys not remaining as multiple factors.

When $ m=0.5n $, it is referred to as a **split account**. This is an interesting use case, as there would be three of six, where one person holds three keys and three people hold one key. In this way, one person could control their own money, but the funds could still be recoverable, even if the primary key holder were to disappear with all of their keys. As $ n $ increases, the level of trust in the secondary parties can decrease. A good use case might be a family savings account that would automatically become an inheritance account if the primary account holder were to die&nbsp;[[5]].

### Rogue Attacks 

Refer to [Key cancellation attack](../digital_signatures/schnorr_signatures.md#key-cancellation-attack) demonstration in [Introduction to Schnorr Signatures](../digital_signatures/introduction.md).

Rogue attacks are a significant concern when implementing multi-signature schemes. Here, a subset of corrupted signers manipulate the public keys computed as functions of the public keys of honest users, allowing them to easily produce forgeries for the set of public keys (despite them not knowing the associated secret keys).

Initial proposals from [[10]], [[11]], [[12]], [[13]], [[14]], [[15]] and [[16]] were thus undone before a formal model was put forward along with a provably secure scheme fromnbsp;[[17]]. Unfortunately, despite being provably secure their scheme is costly and an impractical interactive key generation protocol.&nbsp;[[4]]

A means of generically preventing rogue-key attacks is to make it mandatory for users to prove knowledge (or possession) of the secret key during public key registration with a certification authority [[18]]. Certification authority is a setting known as the KOSK assumption. The pairing-based multi-signature schemes by Boldyreva [[19]] and Lu *et al.* [[20]] rely on the KOSK assumption in order to maintain security. However, according to [[21]] and [[18]], the cost of complexity and expense of the scheme and the unrealistic and burdensome assumptions on the Public-key Infrastructure (PKI) have made this solution problematic.

As it stands, the Bellare M. *et al.* [[21]] provides one of the most practical multi-signature schemes, based on the Schnorr signature scheme, which is provably secure that does not contain any assumption on the key setup. Since the only requirement of this scheme is that each potential signer has a public key, this setting is referred to as the *plain-key model.*

The Micali-Ohta-Reyzin multi-signature scheme [[17]] solves the rogue-key attack using a sophisticated interactive key generation protocol.

Bagherzandi *et al.* [[22]] reduced the number of rounds from three to two using an homomorphic commitment scheme. Unfortunately, this increases the signature size and the computational cost of signing and verification.

Ma *et al.* [[23]] proposed a signature scheme that involved the "double hashing" technique, which sees the reduction of the signature size compared to Bagherzandi *et al.* [[22]] while using only two rounds.

However, neither of these two variants allow for key aggregation.

Multi-signature schemes supporting key aggregation are easier to come by in the KOSK model. In particular, Syta *et al.* [[24]] proposed the CoSi scheme which can be seen as the naive Schnorr multi-signature scheme,  where the co-signers are organized in a tree structure for fast signature generation.

### Interactive Aggregate Signatures

In some situations, it may be useful to allow each participant to sign a different message rather than a single common one. An IAS is one where each signer has its own message $ m_{i} $ to sign, and the joint signature proves that the $ i $ -th signer has signed $ m_{i} $. These schemes are considered to be more general than multi-signature schemes, however they are not as flexible as non-interactive aggregate signatures ([[25]], [[26]]) and sequential aggregate signatures [[27]].

According to Bellare M. *et al.* [[21]], a generic way to turn any multi-signature scheme into an IAS scheme, is if the signer running the multi-signature protocol use the tuple of all public keys/message pairs involved in the IAS protocol as message.

For BN's scheme and Schnorr multi-signatures, this does not increase the number of communication rounds as messages can be sent together with shares $ R_{i} $.

#### Applications of Interactive Aggregate Signatures

With regards to digital currency schemes, where all participants have the ability to validate transactions, these transactions consist of outputs (which have a verification key and amount) and inputs (which are references to outputs of earlier transactions). Each input contains a signature of a modified version of the transaction to be validated with its referenced output's key. Some outputs may require multiple signatures to be spent. Transactions spending such an output are referred to as *m*-of-*n* multi-signature transactions [[28]], and the current implementation corresponds to the trivial way of building a multi-signature scheme by concatenating individual signatures. Additionally, a threshold policy can be enforced where only $ m $ valid signatures out of the $ n $ possible ones are needed to redeem the transaction (again this is the most straightforward way to turn a multi-signature scheme into some kind of basic threshold signature scheme).

While several multi-signature schemes could offer an improvement over the currently available method, two properties increase the possible impact:

- The availability of key aggregation removes the need for verifiers to see all the involved keys, improving bandwidth, privacy, and validation cost.
- Security under the *plain public-key model* enables multi-signatures across multiple inputs of a transaction, where the choice of signers cannot be committed to in advance. This greatly increases the number of situations in which mulit-signatures are beneficial.

#### Native Multi-signature Support 

An improvement is to replace the need for implementing $ n-of-n $ multi-signatures with a constant-size multi-signature primitive like BN. While this is on itself an improvement in terms of size, it still needs to contain all of the signers' public keys. Key aggregation improves upon this further, as a single-key predicate can be used instead which is both smaller and has lower computational cost for verification. Predicate encryption is an encryption paradigm which gives a master secret key owner fine-grained control over access to encrypted data [[29]]. It also improves privacy, as the participant keys and their count remain private to the signers.

When generalizing to the $ m-of-n $ scenario, several options exist. One is to forego key aggregation, and still include all potential signer keys in the predicates while still only producing a single signature for the chosen combination of keys. Alternatively, a Merkle tree [[30]] where the leaves are permitted combinations of public keys (in aggregated form), can be employed. The predicate in this case would take as input an aggregated public key, a signature and a proof. Its validity would depend on the signature being valid with the provided key, and the proof establishing that the key is in fact one of the leaves of the Merkle tree, identified by its root hash. This approach is very generic, as it works for any subset of combinations of keys, and as a result has good privacy as the exact policy is not visible from the proof.

Some key aggregation schemes that do not protect against rogue-key attacks can be used instead in the above cases, under the assumption that the sender is given a proof of knowledge/possession for the receivers' private keys. However, these schemes are difficult to prove secure except by using very large proofs of knowledge. As those proofs of knowledge/possession do not need to be seen by verifiers, they are effectively certified by the sender's validation. However, passing them around to senders is inconvenient, and easy to get wrong. Using a scheme that is secure in the *plain public-key model* categorically avoids these concerns.

Another alternative is to use an algorithm whose key generation requires a trusted setup, for example in the KOSK model. While many of these schemes have been proven secure, they rely on mechanisms that are usually not implemented by certification authorities. ([[18]], [[19]], [[20]]) [[21]])

#### Cross-input Multi-signatures 

The previous sections explained how the numbers of signatures per input can generally by reduced to one, but one can go further and replace it with a single signature per transaction. Doing so requires a fundamental change in validation semantics, as the validity of separate inputs is no longer independent. As a result, the outputs can no longer be modeled as predicates, where the secret key owner is given access to encrypted data. Instead, they are modeled as functions that return a boolean (data type with only two possible values) plus a set of zero or more public keys.

Overall validity requires all returned booleans to be `True` and a multi-signature of the transaction with $ L $ the union of all returned keys.

With regards to Bitcoin, this can be implemented by providing an alternative to the signature checking opcode OP\_CHECKSIG and related opcodes in the Script language. Instead of returning the result of an actual ECDSA verification, they always return `True`, but additionally add the public key with which the verification would have taken place to a transaction-wide multi-set of keys. Finally, after all inputs are verified, a multi-signature present in the transaction is verified against that multi-set. In case the transaction spends inputs from multiple owners, they will need to collaborate to produce the multi-signature, or choose to only use the original opcodes. Adding these new opcodes is possible in a backward-compatible way.&nbsp;[[4]]

#### Protection against Rogue-Key Attacks

In Bitcoin, when taking cross-input signatures into account, there is no published commitment to the set of signers, as each transaction input can independently spend an output that requires authorization from distinct participants. This functionality was not restricted as it would then interfere with fungibility improvements such as CoinJoin [[31]]. Due to the lack of certification, security against rogue-key attacks is of great importance.

If it is assumed that transactions used a single multi-signature that was vulnerable to rogue-attacks, an attacker could identify an arbitrary number of outputs he wants to steal, with the public keys $ X_{1},...,X_{n-t} $ and then use the rogue-key attack to determine $ X_{n-t+1},...,X_{n} $ such that he can sign for the aggregated key $ \tilde{X} $. He would then send a small amount of his own money to outputs with predicates corresponding to the keys $ X_{n-t+1},...,X_{n} $. Finally, he can create a transaction that spends all of the victims' coins together with the ones he just created by forging a multi-signature for the whole transaction.

It can be seen that in the case of mulit-signatures across inputs, theft can occur through the ability to forge a signature over a set of keys that includes at least one key which is not controlled by the attacker. According to the *plain public-key model* this is considered a win for the attacker. This is in contrast to the single-input multi-signature case where theft is only possible by forging a signature for the exact (aggregated) keys contained in an existing output. As a result, it is no longer possible to rely on proofs of knowledge/possession that are private to the signers.

## Formation of MuSig

### Preliminaries 

#### Notation Used 

The  general notation of mathematical expressions when specifically referenced are listed here. These notations are important pre-knowledge for the remainder of the report. 

- Let  $ p ​$  a be large prime number.
- Let $ \mathbb{G} ​$ denote cyclic group of the prime order  $ p ​$. 
- Let $ \mathbb Z_p ​$ denote the ring of integer $ modulo \mspace{4mu} p ​$. 
- Let a generator of  $ \mathbb{G} ​$ be denoted by $ g ​$. Thus, there exists a number $ g \in\mathbb{G}  ​$ such that $ \mathbb{G} =  \lbrace 1, \mspace{3mu}g,  \mspace{3mu}g^2,\mspace{3mu}g^3, ..., \mspace{3mu}g^{p-1} \rbrace   ​$. 
- Let $ \textrm{H} $ denote the hash function. 
- Let $ S= \lbrace (X_{1}, m_{1}),..., (X_{n}, m_{n}) \rbrace  ​$ be the multi-set of all public key/message pairs of all participants, where $  X_{1}=g^{x_{1}}  ​$.  
- Let $ \langle S \rangle $ denote a lexicographically encoding of the multiset of public key/message pairs in $ S $. 
- Let $ L= \lbrace X_{1}=g^{x_{1}},...,X_{n}=g^{x_{n}} \rbrace  ​$ be the multi-set of all public keys. 
- Let $ \langle L \rangle ​$ denote a lexicographically encoding of the multiset of public keys $ L= \lbrace X_{1}...X_{n} \rbrace  ​$. 
- Let $ \textrm{H}_{com} $ denote the hash function in the commitment phase.
- Let $ \textrm{H}_{agg} $ denote the hash function used to compute the aggregated key.
- Let $ \textrm{H}_{sig} ​$ denote the hash function used to compute the signature.
- Let $ X_{1} $ and $ x_{1} $ be the public and private key of a specific signer.
- Let $ m $ be the message that will be signed.
- Let $ X_{2},...,X_{n} $ be the public keys of other cosigners.

### Recap on Schnorr Signature Scheme 

The Schnorr signature scheme [[6]] uses group parameters $(\mathbb{G\mathrm{,p,g)}}$ and a hash function $ \textrm{H} $.

A private/public key pair is a pair 

$$
(x,X) \in  \lbrace 0,...,p-1 \rbrace  \mspace{6mu} \mathsf{x} \mspace{6mu} \mathbb{G}
$$

where $  X=g^{x} $ 

To sign a message $ m $, the signer draws a random integer $ r \in Z_{p} $ and computes

$$
\begin{aligned} 
R &= g^{r} \\\\
c &= \textrm{H}(X,R,m) \\\\
s &= r+cx 
\end{aligned}
$$

The signature is the pair $ (R,s) $, and its validity can be checked by verifying whether 
$$
g^{s} = RX^{c}
$$

This scheme is referred to as the "key-prefixed" variant of the scheme, which sees the public key hashed together with $ R ​$ and $ m ​$ [[7]]. This variant was thought to have a better multi-user security bound than the classic variant [[8]], however in [[9]] the key-prefixing was seen as unnecessary to enable good multi-user security for Schnorr signatures.

For the development of the MuSig Schnorr-based multi-signature scheme [[4]], key-prefixing is a requirement for the security proof, despite not knowing the form of an attack. The rationale also follows the process in reality, as messages signed in Bitcoin always indirectly commits to the public key.

### Design of Schnorr Multi-signature Scheme

The naive way to design a Schnorr multi-signature scheme would be as follows:

A group of $ n $ signers want to cosign a message $ m $. 
Each cosigner randomly generates and communicates to others a share 

$$
R_i = g^{r_{i}}
$$

Each of the cosigners then computes:  


$$
R = \prod \_{i=1}^{n} R_{i} \mspace{30mu} \mathrm{and} \mspace{30mu} c = \textrm{H} (\tilde{X},R,m)
$$

where $$ \tilde{X} = \prod_{i=1}^{n}X_{i} $$

is the product of individual public The partial signature is then given by

$$
s_{i} = r_{i}+cx_{i}
$$

All partial signatures are then combined into a single signature $(R,s)​$ where 

$$
s = \displaystyle\sum_{i=1}^{n}s_i \mod p ​
$$

The validity of a signature $ (R,s) $ on message $ m $ for public keys $  \lbrace X_{1},...X_{n} \rbrace  $ is equivalent to 

$$
g^{s} = R\tilde{X}^{c}
$$

where 

$$
\tilde{X} = \prod\_{i=1}^{n} X_{i} \mspace{30mu} \mathrm{and} \mspace{30mu}  c = \textrm{H}(\tilde{X},R,m)
$$

Note that this is exactly the verification equation for a traditional key-prefixed Schnorr signature with respect to public key $ \tilde{X} $, a property termed *key aggregation*. 
However, these protocols are vulnerable to a rogue-key attack ([[12]], [[14]], [[15]] and [[17]]) where a corrupted signer sets its public key to 

$$
X_{1}=g^{x_{1}} (\prod\_{i=2}^{n} X_{i})^{-1} 
$$

allowing the signer to produce signatures for public keys $  \lbrace X_{1},...X_{n} \rbrace  ​$ by themselves. 

### Bellare and Neven Signature Scheme

Bellare M. *et al.* [[21]] proceeded differently in order to avoid any key setup. A group of $ n $ signers want to cosign a message $ m $. Their main idea is to have each cosigner use a distinct "challenge" when computing their partial signature 

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

A preliminary round is also added to the signature protocol, where each signer commits to its share $ R_i $ by sending $ t_i = \textrm{H}^\prime(R_i) $ to other cosigners first. 

This stops any cosigner from setting $ R = \prod_{i=1}^{n}R_{i}  ​$ to some maliciously chosen value and also allows the reduction to simulate the signature oracle in the security proof. 

Bellare M. *et al.* [[21]] showed that this yields a multi-signature scheme provably secure in the *plain public-key* model under the Discrete Logarithm assumptions, modeling $ \textrm{H} ​$ and $ \textrm{H}^\prime ​$ as random oracles. However, this scheme does not allow key aggregation anymore since the entire list of public keys is required for verification.

### MuSig Signature Scheme

MuSig is paramaterised by group parameters $(\mathbb{G\mathrm{,p,g)}}$ and three hash functions $ ( \textrm{H}\_{com}  ,  \textrm{H}\_{agg} ,  \textrm{H}\_{sig} ) $ from $  \lbrace 0,1 \rbrace ^{*} $ to $  \lbrace 0,1 \rbrace ^{l} $ (constructed from a single hash, using proper domain separation).

#### Round 1

A group of $ n $ signers want to cosign a message $ m $. Let $ X_1 $ and $ x_1 $ be the public and private key of a specific signer, let $ X_2 , . . . , X_n $ be the public keys of other cosigners and let $ \langle L \rangle $ be the multiset of all public keys involved in the signing process.

For $ i\in  \lbrace 1,...,n \rbrace  ​$ , the signer computes the following

$$
a_{i} = \textrm{H}\_{agg}(\langle L \rangle,X\_{i})
$$

as well as the "aggregated" public key 


$$
\tilde{X} = \prod_{i=1}^{n}X_{i}^{a_{i}} ​
$$

#### Round 2

The signer generates a random private nonce $ r_{1}\leftarrow\mathbb{Z_{\mathrm{p}}} $, computes $ R_{1} = g^{r_{1}} $ (the public nonce) and commitment $ t_{1} = \textrm{H}\_{com}(R\_{1}) $ and sends $t_{1}​$ to all other cosigners.

When receiving the commitments $t_{2},...,t_{n}$ from the other cosigners, the signer sends $R_{1}$ to all other cosigners. This ensures that the public nonce is not exposed until all commitments have been received. 

Upon receiving $R_{2},...,R_{n}​$ from other cosigners, the signer verifies that $t\_{i}=\textrm{H}\_{com}(R_{i})​$ for all $ i\in  \lbrace 2,...,n \rbrace  ​$

The protocol is aborted if this is not the case. 

#### Round 3

If all commitment and random challenge pairs can be verified with $ \textrm{H}_{agg} $, the following is computed:

$$
\begin{aligned} 
R &= \prod^{n}\_{i=1}R\_{i} \\\\
c &= \textrm{H}\_{sig} (\tilde{X},R,m) \\\\
s\_{1} &= r\_{1} + ca\_{1} x\_{1} \mod p
\end{aligned}
$$

Signature $s_{1}​$ is sent to all other cosigners.
When receiving $ s_{2},...s_{n} ​$ from other cosigners, the signer can compute $ s = \sum_{i=1}^{n}s_{i} \mod p​$. The signature is $ \sigma = (R,s) ​$.

In order to verify the aggregated signature $ \sigma = (R,s) ​$, given a lexicographically encoded multiset of public keys $ \langle L \rangle ​$ and message $ m ​$, the verifier computes:

$$
\begin{aligned} 
a_{i} &= \textrm{H}\_{agg}(\langle L \rangle,X\_{i}) \mspace{9mu} \textrm {for} \mspace{9mu}  i \in  \lbrace 1,...,n \rbrace  \\\\
\tilde{X} &= \prod\_{i=1}^{n}X\_{i}^{a\_{i}} \\\\
c &=  \textrm{H}\_{sig} (\tilde{X},R,m) 
\end{aligned}
$$

then accepts the signature if 

$$
g^{s} = R\prod_{i=1}^{n}X_{i}^{a_{i}c}=R\tilde{X^{c}.}
$$

### Revisions 

In a previous version of the paper by Maxwell *et al.* [[4]] published on 15 January 2018 they proposed a 2-round variant of MuSig, where the initial commitment round is omitted claiming a security proof under the One More Discrete Logarithm (OMDL) assumptions ([[32]], [[33]]). Drijvers *et al.* [[34]] then discovered a flaw in the security proof and showed that through a meta-reduction the initial multi-signature scheme cannot be proved secure using an algebraic black box reduction under the DL or OMDL assumption.

In more details, it was observed that in the 2-round variant of MuSig, an adversary (controlling public keys $ X_{2},...,X_{n} $) can impose the value of $ R=\Pi_{i=1}^{n}R_{i} $ used in signature protocols since he can choose $ R_{2},...,R_{n} $ after having received $ R_{1} $ from the honest signer (controlling public key $ X_{1}=g^{x_{1}} $ ). This prevents one to use the initial method of simulating the honest signer in the Random Oracle model without knowing $ x_{1} $ by randomly drawing $ s_{1} $ and $ c $, computing $ R\_1=g^{s\_1}(X\_1)^{-a\_1c}$, and later programming $ \textrm{H}\_{sig}(\tilde{X}, R, m) \mspace{2mu} : = c\_1 $ since the adversary might have made the random oracle query $ \textrm{H}\_{sig}(\tilde{X}, R, m) $ *before*  engaging the corresponding signature protocol.  

Despite this, there is no attack currently known against the 2-round variant of MuSig and that it might be secure, although this is not provable under standard assumptions from existing techniques.&nbsp;[[4]]

### Turning Bellare and Neven’s Scheme into an IAS Scheme

In order to change the BN multi-signature scheme into an IAS scheme, Wuille *et al.* [[4]] proposed the scheme described below, which includes a fix to make the execution of the signing algorithm dependent on the message index. 

If $ X = g^{x_i} $ is the public key of a specific signer and $ m $ the message he wants to sign, and 

$$
S^\prime =  \lbrace (X^\prime\_{1}, m^\prime\_{1}),..., (X^\prime\_{n-1}, m^\prime\_{n-1}) \rbrace 
$$

is the set of the public key/message pairs of other signers, this specific signer merges $ (X, m) $ and $ S^\prime $ into the ordered set 

$$
\langle S \rangle \mspace{6mu} \mathrm{of} \mspace{6mu}  S =  \lbrace (X_{1}, m_{1}),..., (X_{n}, m_{n}) \rbrace 
$$

and retrieves the resulting message index $ i ​$ such that 

$$
(X_{1}, m_{i}) = (X, m)
$$

Each signer then draws $ r_{1}\leftarrow\mathbb{Z_{\mathrm{p}}} ​$, computes $  R_{i} = g^{r_{i}} ​$ and subsequently sends commitment $  t_{i} = H^\prime(R_{i}) ​$ in a first round and then $ R_{i} ​$ in a second round, and then computes  

$$
R = \prod_{i=1}^{n}R_{i}
$$

The signer with message index $ i ​$ then computes:

$$
c_{i} = H(R,  \langle S \rangle, i) \mspace{30mu} \\\\
s_{i} = r_{i} + c_{i}x_{i} \mod p
$$

and then sends $ s_{i} ​$ to other signers. All signers can compute 

$$
s = \displaystyle\sum_{i=1}^{n}s_{i} \mod p
$$

The signature is $ \sigma = (R, s) ​$. 

Given an ordered set $ \langle S \rangle \mspace{6mu} \mathrm{of} \mspace{6mu} S =  \lbrace (X_{1}, m_{1}),...,(X_{n}, m_{n}) \rbrace  $ and a signature $ \sigma = (R, s) $  then $ \sigma $ is valid for $ S ​$ when 

$$
g^s = R\prod_{i=1}^{n}X_{i} ^{H(R, \langle S \rangle, i)}
$$

It must be noted that there is no need to include $ \langle L \rangle $ in the hash computation nor the public key $ X_i $ of the local signer since they are already "accounted for" through ordered set $ \langle S \rangle $ and the message index $ i $. 

**Note:** As of writing of this report, the secure IAS scheme presented here still needs to undergo a complete security analysis.


## Conclusions, Observations and Recommendations

- MuSig leads to both native and private multi-signature transactions with signature aggregation.
- Signature data for multi-signatures can be large and cumbersome. MuSig will allow users to create more complex transactions without burdening the network and revealing compromising information.
- The IAS case where each signer signs their own message must still be proven by a complete security analysis.

## References 

[[1]] P. Wuille, “Key Aggregation for Schnorr Signatures”, 2018. <br>Available: <https://blockstream.com/2018/01/23/musig-key-aggregation-schnorr-signatures/>. Date accessed: 2019-01-20. 

[1]: https://blockstream.com/2018/01/23/musig-key-aggregation-schnorr-signatures/
"Key Aggregation for Schnorr Signatures" 

[[2]] Blockstream, “Schnorr Signatures for Bitcoin - BPASE ’18”, 2018. <br>Available: <https://blockstream.com/2018/02/15/schnorr-signatures-bpase/>. Date accessed: 2019-01-20. 

[2]: https://blockstream.com/2018/02/15/schnorr-signatures-bpase/
"Schnorr Signatures for Bitcoin- BPASE '18"

[[3]] K. Itakura, “A Public-key Cryptosystem Suitable for Digital Multisignatures”, NEC J. Res. Dev., Vol. 71, 1983. Available: <https://scinapse.io/papers/200023587/>. Date accessed: 2019-01-20. 

[3]: https://scinapse.io/papers/200023587/
"A Public-key Cryptosystem Suitable for Digital Multisignatures"

[[4]] G. Maxwell *et al*., “Simple Schnorr Multi-Signatures with Applications to Bitcoin”, pp. 1–34, 2018. Available: <https://eprint.iacr.org/2018/068.pdf>. Date accessed: 2019-01-20. 

[4]: https://eprint.iacr.org/2018/068.pdf
"Simple Schnorr Multi-Signatures with 
Applications to Bitcoin"

[[5]] B. W. Contributors, “Multisignature”, 2017. Available: <https://wiki.bitcoin.com/w/Multisignature>. Date accessed: 2019-01-20. 

[5]: https://wiki.bitcoin.com/w/Multisignature
"Multisignature"

[[6]] C. P. Schnorr, “Efficient Signature Generation by Smart Cards”, in *Journal of Cryptology*, Vol. 4, No. 3, pp. 161–174, 1991. Available: <https://link.springer.com/article/10.1007/BF00196725>. Date accessed: 2019-01-20.

[6]: https://link.springer.com/article/10.1007/BF00196725
"Efficient Signature Generation by Smart Cards"

[[7]] D. J. Bernstein *et al.*, “High-speed High-security Signatures”, in *Journal of Cryptographic Engineering*, Vol. 2, No. 2, pp. 77–89, 2012. Available: <https://ed25519.cr.yp.to/ed25519-20110705.pdf>. Date accessed: 2019-01-20.

[7]: https://ed25519.cr.yp.to/ed25519-20110705.pdf
"High-speed high-security signatures"

[[8]] D. J. Bernstein, “Multi-user Schnorr Security, Revisited”, IACR Cryptology ePrint Archive, Vol. 2015, p. 996, 2015. Available: <https://eprint.iacr.org/2015/996.pdf>. Date accessed: 2019-01-20.

[8]: https://eprint.iacr.org/2015/996.pdf
"Multi-user Schnorr Security, Revisited" 

[[9]] E. Kiltz *et al.*, “Optimal Security Proofs for Signatures from Identification Schemes”, in A*nnual Cryptology Conference*, pp. 33–61, Springer, 2016. Available: <https://eprint.iacr.org/2016/191.pdf>. Date accessed: 2019-01-20.

[9]: https://eprint.iacr.org/2016/191.pdf
"Optimal Security Proofs for Signatures from
Identification Schemes"

[[10]] C. M. Li *et al.*, “Threshold-multisignature Schemes where Suspected Forgery Implies Traceability of Adversarial Shareholders”, in *Workshop on the Theory and Application of Cryptographic Techniques*, pp. 194–204, Springer, 1994. Available: <https://link.springer.com/content/pdf/10.1007%2FBFb0053435.pdf>. Date accessed: 2019-01-20.

[10]: https://link.springer.com/content/pdf/10.1007%2FBFb0053435.pdf
"Threshold-multisignature Schemes 
where Suspected Forgery Implies 
Traceability of Adversarial Shareholders"

[[11]] L. Harn, “Group-oriented (t, n) Threshold Digital Signature Scheme and Digital Multisignature”, IEE Proceedings-Computers and Digital Techniques, Vol. 141, No. 5, pp. 307–313, 1994. Available: <https://ieeexplore.ieee.org/abstract/document/326780>. Date accessed: 2019-01-20.

[11]: https://ieeexplore.ieee.org/abstract/document/326780
"Group-oriented (t, n) Threshold Digital 
Signature Scheme and Digital Multisignature"

[[12]] P. Horster *et al.*, “Meta-Multisignature Schemes Based on the Discrete Logarithm Problem”, in Information Security-the Next Decade, pp. 128–142, Springer, 1995. Available: <https://link.springer.com/content/pdf/10.1007%2F978-0-387-34873-5_11.pdf>. Date accessed: 2019-01-20.

[12]: https://link.springer.com/content/pdf/10.1007%2F978-0-387-34873-5_11.pdf
"Meta-Multisignature Schemes Based
on the Discrete Logarithm Problem"

[[13]] K. Ohta and T. Okamoto, “A Digital Multisignature Scheme Based on the Fiat-Shamir Scheme,” in *International Conference on the Theory and Application of Cryptology*, pp. 139–148, Springer, 1991. Available: <https://link.springer.com/chapter/10.1007/3-540-57332-1_11>. Date accessed: 2019-01-20.

[13]: https://link.springer.com/chapter/10.1007/3-540-57332-1_11
"A Digital Multisignature Scheme 
Based on the Fiat-Shamir Scheme"

[[14]] S. K. Langford, “Weaknesses in Some Threshold Cryptosystems”, in *Annual International Cryptology Conference*, pp. 74–82, Springer, 1996. Available: <https://link.springer.com/content/pdf/10.1007%2F3-540-68697-5_6.pdf>. Date accessed: 2019-01-20.

[14]: https://link.springer.com/content/pdf/10.1007%2F3-540-68697-5_6.pdf

"Weaknesses in Some Threshold 
Cryptosystem" 

[[15]] M. Michels and P. Horster, “On the Risk of Disruption in Several Multiparty Signature Schemes”, in *International Conference on the Theory and Application of Cryptology and Information Security*, pp. 334–345, Springer, 1996. Available: <https://pdfs.semanticscholar.org/d412/e5ab35fd397931cef0f8202324308f44e545.pdf>. Date accessed: 2019-01-20.

[15]: https://pdfs.semanticscholar.org/d412/e5ab35fd397931cef0f8202324308f44e545.pdf

"On the Risk of Disruption in Several 
Multiparty Signature Schemes" 

[[16]] K. Ohta and T. Okamoto, “Multi-signature Schemes Secure Against Active Insider Attacks”, IEICE Transactions on Fundamentals of Electronics, Communications and Computer Sciences, Vol. 82, No. 1, pp. 21–31, 1999. Available: <http://search.ieice.org/bin/summary.php?id=e82-a_1_21&category=A&year=1999&lang=E&abst=>. Date accessed: 2019-01-20.

[16]: http://search.ieice.org/bin/summary.php?id=e82-a_1_21&category=A&year=1999&lang=E&abst=

"Multi-signature Schemes Secure 
Against Active Insider Attacks" 

[[17]] S. Micali *et al.*, “Accountable-subgroup Multisignatures”, in *Proceedings of the 8th ACM Conference on Computer and Communications Security*, pp. 245–254, ACM, 2001. Available: <https://pdfs.semanticscholar.org/6bf4/f9450e7a8e31c106a8670b961de4735589cf.pdf >. Date accessed: 2019-01-20.

[17]: https://pdfs.semanticscholar.org/6bf4/f9450e7a8e31c106a8670b961de4735589cf.pdf

"Accountable-subgroup Multisignatures" 

[[18]] T. Ristenpart and S. Yilek, “The Power of Proofs-of-possession: Securing Multiparty Signatures Against Rogue-key Attacks”, in *Annual International Conference on the Theory and Applications of Cryptographic Techniques*, pp. 228–245, Springer, 2007. Available: <https://link.springer.com/content/pdf/10.1007%2F978-3-540-72540-4_13.pdf>. Date accessed: 2019-01-20.

[18]: https://link.springer.com/content/pdf/10.1007%2F978-3-540-72540-4_13.pdf

"The Power of Proofs-of-possession: 
Securing Multiparty Signatures Against
Rogue-key Attacks" 

[[19]] A. Boldyreva, “Threshold Signatures, Multisignatures and Blind Signatures Based on the Gap-Diffie-Hellman-group Signature Scheme”, in *International Workshop on Public Key Cryptography*, pp. 31–46, Springer, 2003. Available: <https://www.iacr.org/archive/pkc2003/25670031/25670031.pdf >. Date accessed: 2019-01-20.

[19]: https://www.iacr.org/archive/pkc2003/25670031/25670031.pdf

"Threshold Signatures, Multisignatures
and Blind Signatures Based on the
Gap-Diffie-Hellman-Group Signature 
Scheme"

[[20]] S. Lu *et al.*, “Sequential Aggregate Signatures and Multisignatures without Random Oracles,” in *Annual International Conference on the Theory and Applications of Cryptographic Techniques*, pp. 465–485, Springer, 2006. Available: <https://eprint.iacr.org/2006/096.pdf>. Date accessed: 2019-01-20.

[20]: https://eprint.iacr.org/2006/096.pdf

"Sequential Aggregate Signatures and
Multisignatures without Random Oracles" 

[[21]] M. Bellare and G. Neven, “Multi-Signatures in the Plain Public-Key Model and a General Forking Lemma”, in *Acm Ccs*, pp. 390– 399, 2006. Available: <https://cseweb.ucsd.edu/~mihir/papers/multisignatures-ccs.pdf>. Date accessed: 2019-01-20.

[21]: https://cseweb.ucsd.edu/~mihir/papers/multisignatures-ccs.pdf

"Multi-Signatures in the Plain Public-Key 
Modeland a General Forking Lemma"

[[22]] A. Bagherzandi *et al.*, “Multisignatures Secure under the Discrete Logarithm Assumption and a Generalized Forking Lemma”, *Proceedings of the 15th ACM Conference on Computer and Communications Security - CCS ’08*, p. 449, 2008. Available: <http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.544.2947&rep=rep1&type=pdf>. Date accessed: 2019-01-20.

[22]: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.544.2947&rep=rep1&type=pdf

"Multisignatures Secure under the
Discrete Logarithm Assumption
and a Generalized Forking Lemma"

[[23]] C. Ma *et al.*, “Efficient Discrete Logarithm Based Multi-signature Scheme in the Plain Public Key Model”, Designs, Codes and Cryptography, Vol. 54, No. 2, pp. 121–133, 2010. Available: <https://link.springer.com/article/10.1007/s10623-009-9313-z>. Date accessed: 2019-01-20.

[23]: https://link.springer.com/article/10.1007/s10623-009-9313-z

"Efficient Discrete Logarithm 
Based Multi-signature Scheme 
in the Plain Public Key Model"

[[24]] E. Syta *et al.*, “Keeping Authorities 'Honest or Bust' with Decentralized Witness Cosigning”, in *Security and Privacy (SP), 2016 IEEE Symposium* on pp. 526–545, Ieee, 2016. Available: <4 https://arxiv.org/pdf/1503.08768.pdf >. Date accessed: 2019-01-20.

[24]: https://arxiv.org/pdf/1503.08768.pdf

"Keeping Authorities 'Honest 
or Bust' with Decentralized 
Witness Cosigning"

[[25]] D. Boneh *et al.*, “Aggregate and Verifiably Encrypted Signatures from Bilinear Maps”, in *International Conference on the Theory and Applications of Cryptographic Techniques*, pp. 416–432, Springer, 2003. Available: <http://crypto.stanford.edu/~dabo/papers/aggreg.pdf>. Date accessed: 2019-01-20.

[25]: http://crypto.stanford.edu/~dabo/papers/aggreg.pdf

"Aggregate and Verifiably Encrypted 
Signatures from Bilinear Maps"

[[26]] M. Bellare *et al.*, “Unrestricted Aggregate Signatures”, in I*nternational Colloquium on Automata, Languages, and Programming*, pp. 411–422, Springer, 2007. Available: <https://cseweb.ucsd.edu/~mihir/papers/agg.pdf>. Date accessed: 2019-01-20.

[26]: https://cseweb.ucsd.edu/~mihir/papers/agg.pdf

"Unrestricted Aggregate Signatures"

[[27]] A. Lysyanskaya *et al.*, “Sequential Aggregate Signatures from Trapdoor Permutations”, in *International Conference on the Theory and Applications of Cryptographic Techniques*, pp. 74–90, Springer, 2004. Available: <https://hovav.net/ucsd/dist/rsaagg.pdf>. Date accessed: 2019-01-20.

[27]: https://hovav.net/ucsd/dist/rsaagg.pdf

"Sequential Aggregate Signatures 
from Trapdoor Permutations" 

[[28]] G. Andersen, “M-of-N Standard Transactions”, 2011. Available: <https://bitcoin.org/en/glossary/multisig>. Date accessed: 2019-01-20.

[28]: https://bitcoin.org/en/glossary/multisig

"M-of-N Standard Transactions"

[[29]] E. Shen *et al.*, “Predicate Privacy in Encryption Systems”, in *Theory of Cryptography Conference*, pp. 457–473, Springer, 2009. Available: <https://www.iacr.org/archive/tcc2009/54440456/54440456.pdf>. Date accessed: 2019-01-20.

[29]: https://www.iacr.org/archive/tcc2009/54440456/54440456.pdf

"Predicate Privacy in Encryption 
Systems"

[[30]] R. C. Merkle, “A Digital Signature Based on a Conventional Encryption Function”, in *Conference on the Theory and Application of Cryptographic Techniques*, pp. 369–378, Springer, 1987. Available: <https://people.eecs.berkeley.edu/~raluca/cs261-f15/readings/merkle.pdf>. Date accessed: 2019-01-20.

[30]: https://people.eecs.berkeley.edu/~raluca/cs261-f15/readings/merkle.pdf

"A Digital Signature Based on a 
Conventional Encryption Function" 

[[31]] G. Maxwell, “CoinJoin: Bitcoin Privacy for the Real World”, 2013. Available: <https://bitcointalk.org/index.php?topic=279249.0>. Date accessed: 2019-01-20.

[31]: https://bitcointalk.org/index.php?topic=279249.0

"CoinJoin: Bitcoin Privacy for the 
Real World"

[[32]] M. Bellare and A. Palacio, “GQ and Schnorr Identification Schemes: Proofs of Security against Impersonation under Active and Concurrent Attacks”, in *Annual International Cryptology Conference*, pp. 162–177, Springer, 2002. Available: <https://cseweb.ucsd.edu/~mihir/papers/gq.pdf>. Date accessed: 2019-01-20.

[32]: https://cseweb.ucsd.edu/~mihir/papers/gq.pdf

"GQ and Schnorr Identification Schemes: Proofs of Security Against Impersonation Under Active and Concurrent Attacks"

[[33]] M. Bellare *et al.*, “The One-More-RSA Inversion Problems and the Security of Chaum’s Blind Signature Scheme”, *Journal of Cryptology*, Vol. 16, No. 3, 2003. Date accessed: 2019-01-20.

[33]: https://eprint.iacr.org/2001/002.pdf

"The One-More-RSA-Inversion Problems
and the Security of Chaum’s Blind 
Signature Scheme*"

[[34]] M. Drijvers, K. Edalatnejad, B. Ford, and G. Neven, “Okamoto Beats Schnorr: On the Provable Security of Multi-Signatures”, Tech. Rep., 2018. Available: <https://www.semanticscholar.org/paper/Okamoto-Beats-Schnorr%3A-On-the-Provable-Security-of-Drijvers-Edalatnejad/154938a12885ff30301129597ebe11dd153385bb>. Date accessed: 2019-01-20.

[34]: https://www.semanticscholar.org/paper/Okamoto-Beats-Schnorr%3A-On-the-Provable-Security-of-Drijvers-Edalatnejad/154938a12885ff30301129597ebe11dd153385bb

"Okamoto Beats Schnorr: On the Provable Security of Multi-Signatures".



## Contributors 

- <https://github.com/kevoulee>
- <https://github.com/hansieodendaal>
- <https://github.com/CjS77>
- <https://github.com/anselld>