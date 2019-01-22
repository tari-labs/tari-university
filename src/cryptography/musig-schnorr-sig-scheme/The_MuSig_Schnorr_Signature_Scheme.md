# The MuSig Schnorr Signature Scheme 

## Abstract

This report investigates Schnorr Multi-Signature Schemes (MuSig), which makes use of key aggregation and is provably secure in the *plain public-key model*.

Signature aggregation involves mathematically combining several signatures into a single signature, without having to prove Knowledge of Secret Keys (KOSK). This is known as the *plain public-key model* where the only requirement is that each potential signer has a public key. The KOSK scheme requires that users prove knowledge (or possession) of the secret key during public key registration with a certification authority, and is one way to generically prevent rogue-key attacks.

Multi-signatures are a form of technology used to add multiple participants to cryptocurrency transactions. A traditional multi-signature protocol allows a group of signers to produce a joint multi-signature on a common message.

## Contents 
- [The MuSig Schnorr Signature Scheme](#the-musig-schnorr-signature-scheme)
	- [Abstract](#abstract)
	- [Contents](#contents)
	- [Introduction](#introduction)
		- [Schnorr signatures and their attack vectors](#schnorr-signatures-and-their-attack-vectors)
		- [MuSig](#musig)
		- [Key aggregation](#key-aggregation) 
	- [Overview of multi-signatures](#overview-of-multi-signatures)
		- [Bitcoin $ m-of-n $ multi-signatures](#bitcoin--m-of-n--multi-signatures)
			- [Use cases for $ m-of-n $ multi-signatures](#use-cases-for--m-of-n--multi-signatures)
		- [Recap on the Schnorr signature scheme](#recap-on-the-schnorr-signature-scheme)
			- [Rogue attacks](#rogue-attacks)
		- [Design of a Schnorr multi-signature scheme](#design-of-a-schnorr-multi-signature-scheme)
			- [Micali-Ohta-Reyzin multi-signature scheme](#micali-ohta-reyzin-multi-signature-scheme)
			- [The Bagherzandi *et al.* scheme](#the-bagherzandi-*et-al.*-scheme)
			- [The Ma *et al.* scheme ](#the-ma-*et-al.*-scheme)
			- [Bellare and Neven signature scheme](#bellare-and-neven-signature-scheme)   
	- [The formation of MuSig](#the-formation-of-musig)
		- [Interactive Aggregate Signatures](#interactive-aggregate-signatures)
        	- [Application of IAS](#application-of-IAS)
            - [Native mulit-signature support](#native-multi-signature-support)
            - [Cross-input multi-signatures](#cross-input-multi-signatures)
            - [Protection against rogue-key attacks](#protection-against-rogue-key-attacks)
        - [Revisions](#revisions) 
	- [Conclusions, observations and recommendations](#conclusions,-observations-and-recommendations)
	- [Contributors](#contributors)


## Introduction 

### Schnorr signatures and their attack vectors 

Schnorr signatures produce a smaller on-chain size, support faster validation and have better privacy. They natively allow for combining multiple signatures into one through aggregation and they permit more complex spending policies.

Signature aggregation also has its challenges. This includes the rogue-key attack, where a participant steals funds using a specifically constructed key. Although this is easily solved for simple multi-signatures through an enrollment procedure which involves the keys signing themselves, supporting it across multiple inputs of a transaction requires *plain public-key security*, meaning there is no setup.

There is an additional attack, termed the Russel attack, after Russel O'Connor, who has discovered that for multi-party schemes a party could claim ownership of someone else's private key and so spend the other outputs.

Wuille P., [[1]] has been able to address some of these issues and has provided a solution which refines the Bellare-Neven (BN) scheme. He also discussed the performance improvements that were implemented for the scaler multiplication of the BN scheme and how they enable batch validation on the blockchain. [[2]]

### MuSig

Introduced by Itakura *et al.* [[3]], multi-signature protocols allow a group of signers (that individually possess their own private/public key pair) to produce a single signature $ \sigma $ on a message $ m $. Verification of the given signature $ \sigma $ can be publicly performed given the message and the set of public keys of all signers.

A simple way to change a standard signature scheme into a multi-signature scheme is to have each signer produce a stand-alone signature for $ m $ with its private key and to then concatenate all individual signatures.

The transformation of a standard signature scheme to a multi-signature scheme needs to useful and practical, thus the newly calculated multi-signature scheme must produce signatures where the size is independent of the number of signers and similar to that of the original signature scheme. [[4]]

A traditional multi-signature scheme is a combination of a signing and verification algorithm, where multiple signers (each with their own private/public key) jointly sign a single message, resulting in a combined signature. This can then be verified by anyone knowing the message and the public keys of the signers, where a trusted setup with KOSK is a requirement.

MuSig is a multi-signature scheme that is novel in combining:

1.  Support for key aggregation;

2.  Security in the *plain public-key model*.

There are two versions of MuSig, that are provably secure, which differ based on the number of communication rounds:

1.  Three-round MuSig only relies on the Discrete Logarithm (DL) assumption, on which ECDSA (Elliptic Curve Digital Signature Algorithm) also relies

2.  Two-round MuSig instead relies on the slightly stronger One-More Discrete Logarithm (OMDL) assumption

### Key aggregation

The term *key aggregation* refers to multi-signatures that look like a single-key signature, but with respect to an aggregated public key that is a function of only the participants' public keys. Thus, verifiers do not require the knowledge of the original participants' public keys- they can just be given the aggregated key. In some use cases, this leads to better privacy and performance. Thus, MuSig is effectively a key aggregation scheme for Schnorr signatures.

To make the traditional approach more effective and without needing a trusted setup, a multi-signature scheme must provide sub-linear signature aggregation along with the following properties:

-   It must be provably secure in the *plain public-key model*

-   It must satisfy the normal Schnorr equation, whereby the resulting signature can be written as a function of a combination of the public keys

-   It must allow for Interactive Aggregate Signatures (IAS) where the signers are required to cooperate

-   It must allow for Non-interactive Aggregate Signatures (NAS) where the aggregation can be done by anyone

-   It must allow each signer to sign the same message

-   It must allow each signer to sign their own message

This is different to a normal multi-signature scheme where one message is signed by all. MuSig provides all of those properties.

There are other multi-signature schemes that already exist that provide key aggregation for Schnorr signatures, however they come with some limitations, such as needing to verify that participants actually have the private key corresponding to the pubic keys that they claim to have. *Security in the plain public-key model* means that no limitations exist. All that is needed from the participants is their public keys. [[1]]

## Overview of multi-signatures 

Recently the most obvious use case for multi-signatures is with regards to Bitcoin, where it can function as a more efficient replacement of $ n-of-n $ multisig scripts (where the signatures required to spend and the signatures possible are equal in quantity) and other policies that permit a number of possible combinations of keys.

A key aggregation scheme also lets one reduce the number of public keys per input to one, as a user can send coins to the aggregate of all involved keys rather than including them all in the script. This leads to a smaller on-chain footprint, faster validation, and better privacy.

Instead of creating restrictions with one signature per input, one signature can be used for the entire transaction. Traditionally key aggregation cannot be used across multiple inputs, as the public keys are committed to by the outputs, and those can be spent independently. MuSig can be used here (with key aggregation done by the verifier).

No non-interactive aggregation scheme is known that only relies on the DL assumption, but interactive schemes are trivial to construct where a multi-signature scheme has every participant sign the concatenation of all messages. Maxwell G., *et al.* [[4]] focused on key aggregation for Schnorr Signatures and showed that this is not always a desirable construction, and gave an IAS variant of BN with better properties instead. [[1]]

### Bitcoin $ m-of-n $ multi-signatures 

Currently, standard transactions on the Bitcoin network can be referred to as single-signature transactions, as they require only one signature, from the owner of the private key associated with the Bitcoin address. However, the Bitcoin network supports much more complicated transactions which can require the signatures of multiple people before the funds can be transferred. These are often referred to as $ m-of-n $ transactions, where m represents the amount of signatures required to spend, while n represents the amount of signatures possible. [[5]]

#### Use cases for $ m-of-n $ multi-signatures

When $ m=1 $ and $ n>1 $ it is considered a shared wallet, which could be used for small group funds that do not require much security. It is the least secure multi-sig option because it is not multi-factor. Any compromised individual would jeopardize the entire group. Examples of use cases include funds for a weekend or evening event, or a shared wallet for some kind of game. Besides being convenient to spend from, the only benefit of this setup is that all but one of the backup/password pairs could be lost and all of the funds would be recoverable.

When $ m=n $, it is considered a partner wallet, which brings with it some nervousness as no keys can be lost. As the number of signatures required increases, the risk also increases. This type of multi-signature can be considered as a hard multi-factor authentication.

When $ m<0.5n $, it is considered a buddy account, which could be used for spending from corporate group funds. The consequence for the colluding minority needs to greater than possible benefits. It is considered less convenient than a shared wallet, but much more secure.

When $ m>0.5n $, a consensus account is termed. The classic multi-signature wallet is a 2 of 3 and is a special case of a consensus account. A 2 of 3 scheme has the best characteristics for creating new bitcoin addresses and for secure storing and spending. One compromised signatory does not compromise the funds. A single secret key can be lost and the funds can still be recovered. If done correctly, off-site backups are created during wallet setup. The way to recover funds is known by more than one party. The balance of power with a multi-signature wallet can be shifted by having one party control more keys than the other parties. If one party controls multiple keys, there is a greater risk of those keys not remaining as multiple factors.

When $ m=0.5n $, it is referred to as a split account, and is an interesting use case, as there would be 3 of 6 where one person holds 3 keys and 3 people hold 1 key. In this way one person could control their own money, but the funds could still be recoverable even if the primary key holder were to disappear with all of his keys. As $ n $ increases, the level of trust in the secondary parties can decrease. A good use case might be a family savings account that would just automatically become an inheritance account if the primary account holder were to die. [[5]]

### Recap on the Schnorr signature scheme 

The Schnorr signature scheme uses:[[6]]

-   A cyclic group $ G ​$ of prime order $ p ​$ 

-   A generator $ g ​$ of $ G ​$ 

-   A hash function $ \textrm{H} ​$ 

-   A private/public key pair is a pair $ (x,X) \in \{0,...,p-1\} \mspace{6mu} \mathsf{x} \mspace{6mu} \mathbb{G} $ where $ X=g^{x} $ 

-   To sign a message $ m $, the signer draws a random integer $ r $ in $ Z_{p}, $ computes $ R=g^{r} $,&nbsp; $ c=\textrm{H}(X,R,m) $ and $ s=r+cx ​$ 

-   The signature is the pair $ (R,s) ​$, and its validity can be checked by verifying whether $ g^{s}=RX^{c} ​$ 

The above described is referred to as the so-called "key-prefixed" variant of the scheme, which sees the public key hashed together with $ R ​$ and $ m ​$ [[7]]. This variant was thought to have a better multi-user security bound than the classic variant [[8]], however in [[9]] the key-prefixing was seen as unnecessary to enable good multi-user security for Schnorr signatures.

For the development of the new Schnorr-based multi-signature scheme [[4]], key-prefixing seemed a requirement for the security proof to go through, despite not knowing the form of an attack. The rationale also follows the process in reality, as messages signed in Bitcoin always indirectly commits to the public key.

#### Rogue attacks 

Please see [Key cancellation attack](../digital_signatures/schnorr_signatures.md#key-cancellation-attack) demonstration in [Introduction to Schnorr Signatures](../digital_signatures/introduction.md).

Rogue attacks are a significant concern when implementing multi-signature schemes. Here a subset of corrupted signers, manipulate the public keys computed as functions of the public keys of honest users, allowing them to easily produce forgeries for the set of public keys (despite them not knowing the associated secret keys).

Initial proposals from [[10]], [[11]], [[12]], [[13]], [[14]], [[15]] and [[16]] were thus undone before a formal model was put forward along with a provably secure scheme from Micali *et al*. [[17]]. Unfortunately, despite being provably secure their scheme is costly and an impractical interactive key generation protocol. [[4]]

A means of generically preventing rogue-key attacks is to make it mandatory for users to prove knowledge (or possession) of the secret key during public key registration with a certification authority [[18]]. Certification authority is a setting known as the KOSK assumption. The pairing-based multi-signature schemes by Boldyreva [[19]] and Lu *et al*. [[20]] rely on the KOSK assumption in order to maintain security. However according to [[21]] and [[18]] the cost of complexity and expense of the scheme and the unrealistic and burdensome assumptions on the public-key infrastructure (PKI) have made this solution problematic.

As it stands, the Bellare and Neven [[21]] provides one of the most practical multi-signature schemes, based on the Schnorr signature scheme, which is provably secure that does not contain any assumption on the key setup. Since the only requirement of this scheme is that each potential signer has a public key, this setting is referred to as the *plain-key model.*

### Design of a Schnorr multi-signature scheme

The naive way to design a Schnorr multi-signature scheme would be as follows:

-   A group of $ n $ signers want to cosign a message $ m $ 

-   Let $ L=\{X_{1}=g^{x_{1}},...,X_{n}=g^{x_{n}}\} $ be the multi-set of all public key[^1]

-   Each cosigner randomly generates and communicates to others a share $ R_i = g^{r_{i}} $

-   Each of the cosigners then computes $ R = \prod {i=1}^{n} R_{i} , c = \textrm{H} (\tilde{X},R,m) $

-   Where $ \tilde{X}=\Pi_{i=1}^{n}X_{i} $ is the product of individual public keys, and a partial signature $ s_{i}=r_{i}+cx_{i} $ 

-   Partial signatures are then combined into a single signature $(R,s)$ where $s=\Sigma_{i=1}^{n}s_i \mod p $

-   The validity of a signature $ (R,s) ​$ on message $ m ​$ for public keys $ \{X_{1},...X_{n}\} ​$ is equivalent to $ g^{s}=R\tilde{X}^{c} ​$ where $ \tilde{X}=\Pi_{i=1}^{n}X_{i} ​$ and $ c=\textrm{H}(\tilde{X},R,m) ​$ 

Note that this is exactly the verification equation for a traditional key-prefixed Schnorr signature with respect to public key $ \tilde{X} $, a property termed *key aggregation*

However, as mentioned above, [[12]], [[14]], [[15]] and [[17]] these protocols are vulnerable to a rogue-key attack where a corrupted signer sets its public key to $ X_{1}=g^{x_{1}}(\Pi_{i=2}^{n}X_{i})^{-1}​ $, allowing the signer to produce signatures for public keys $ \{X_{1},...X_{n}\}​ $ by themself.

#### Micali-Ohta-Reyzin multi-signature scheme 

The Micali-Ohta-Reyzin multi-signature scheme [[17]] solves the rogue-key attack using a sophisticated interactive key generation protocol.

#### The Bagherzandi *et al.* scheme

Bagherzandi *et al.* [[22]] reduced the number of rounds from three to two using an homomorphic commitment scheme. Unfortunately, this increases the signature size and the computational cost of signing and verification.

#### The Ma *et al.* scheme 

Ma *et al.* [[23]] proposed a signature scheme that involved the "double hashing" technique, which sees the reduction of the signature size compared to Bagherzandi *et al.* [[22]] while using only two rounds.

However, neither of these two variants allow for key aggregation.

Multi-signature schemes supporting key aggregation are easier to come by in the KOSK model. In particular, Syta *et al.* [[24]] proposed the CoSi scheme which can be seen as the naive Schnorr multi-signature scheme,  where the co-signers are organized in a tree structure for fast signature generation.

#### Bellare and Neven signature scheme

Bellare-Neven (BN) [[21]] proceeded differently in order to avoid any key setup. Their main idea is to have each cosigner use a distinct "challenge" when computing their partial signature $ s_{i}=r_{i}+c_{i}x_{i} $, defined as $ c_{i}=\textrm{H}(LX_{i},R,m) $, where $ R=\prod_{i=1}^{n}R_{i} $ and $ \langle L 
\rangle $ is a unique encoding of the multiset of public keys $ L=\{X_{1}...X_{n}\} $. It is a more widely known *plain public-key* multi-signature scheme, that does not support key aggregation. It is possible to use BN multi-signatures where the individual keys are MuSig aggregates. BN multi-signature scheme is secure without such assumptions. Below are details:

-   Call $ L=\textrm{H}(X_{1,}X_{2...}) $ 

-   Each signer chooses a random nonce $ r_{i} $ and shares $ R_{i}=r_{i}G $ with the other signers

-   Call R the sum of the $ R_{i} $ points

-   Each signer computes $ s_{i}=r_{i}+\textrm{H}(L,X_{i,}R,m)x_{i} $ 

-   The final signature is $ (R,s) $ where $ s $ is the sum of the $ s_{i} $ values

-   Verification requires $ sG=R+\textrm{H}(L,X_{1,}R,m)X_{2}+... $ 

Technically, BN has a pre-commit round, where the signers initially reveal $ \textrm{H}(R_{i}) $ to each other, prior to revealing the $ R_{i} $ points themselves. This step is a requirement in order to prove security under the DL assumption, but it can be dismissed if instead the OMDL assumption is accepted.

Furthermore, when an IAS is desired (where each signer has their own message), $ L=\textrm{H}((X_{1},m_{1}),(X_{2},m_{2}),...) $ and $ s_{i}=r_{i}+\textrm{H}(L,R,i)x_{i} $ is used for signing (and analogous for verification).

The resulting signature does not satisfy the normal Schnorr equation anymore, nor any other equation that can be written as a function of a combination of the public keys; the key aggregation property is lost in order to gain security in the *plain public-key model*.

Bellare and Neven showed that this yields a multi-signature scheme provably secure in the *plain public-key* model under the Discrete Logarithm assumptions, modeling $ \textrm{H} $ and $ \textrm{H}' $ as random oracles. However, this scheme does not allow key aggregation anymore since the entire list of public keys is required for verification.

## The formation of MuSig

This is where MuSig comes in. It recovers the *key aggregation property without losing security:*

-   Call $ L=\textrm{H}(X_{1},X_{2}...) $ 

-   Call $ X $ the sum of all $ \textrm{H}(L,X_{i})X_{i} $ 

-   Each signer chooses a random nonce $ r_{i}, $ and shares $ R_{i}=r_{i}G $ with the other signers

-   Call $ R $ the sum of the $ R_{i} $ points

-   Each signer computes $ s_{i}=r_{i}+\textrm{H}(X,R,m)\textrm{H}(L,X_{i})x_{i} $ 

-   The final signature is $ (R,s) $ where $ s $ is the sum of the $ s_{i} $ values

-   Verification again satisfies $ sG=R+\textrm{H}(X,R,m)X $ 

So what was needed was to define $ X $ not as a simple sum of the individual public keys $ X_{i} $, but as a sum of multiples of those keys, where the multiplication factor depends on a hash of all participating keys. [[1]]

The new proposed Schnorr-based multi-signature scheme can be seen as a variant of the BN scheme, allowing key aggregation in the *plain public-key model.* This scheme consists of three rounds, the first two being exactly the same as in BN. Challenges $ c_{i} ​$ are changed from 

$$
c_{i}=\textrm{H}(\langle L \rangle, X_{i},R,m)  
$$

to 

$$
c_i = \textrm H_{agg} (\langle L \rangle , X_i) \cdot \textrm H _{sig} (\tilde{X},R,m)
$$

where $ \tilde{X} $ is the so-called aggregated public key corresponding to the multi-set of public keys $ L=\{X_{1},...X_{n}\}, $ defined as 

$$
\tilde{X} = \prod  ^n_{i=1} X_{i}^{a_i}
$$

where 

$$
g^s = R \prod  ^n_{i=1} X_{i}^{a_{i}c} = R \tilde {X} ^ c
$$

where

$$
c=\textrm{H}_{sig}(\tilde{X},R,m)  
$$

Basically , the key aggregation property has been recovered and can now be enjoyed by the naive scheme, which respect to a more complex aggregation key 

$$
\tilde{X} = \prod  ^n_{i=1} X_{i}^{a_i} 
$$

Note that $ c=H_{sig}(\langle L \rangle,R,m) $ also yields a secure scheme, however does not allow key aggregation since verification is impossible without knowing all the individual singer keys.

### Interactive Aggregate Signatures

In some situations, it may be useful to allow each participant to sign a different message rather than a single common one. An IAS is one where each signer has its own message $ m_{i} $ to sign, and the joint signature proves that the $ i $ -th signer has signed $ m_{i} $. These schemes are considered to be more general than multi-signature schemes, however they are not as flexible as non-interactive aggregate signatures ([[25]], [[26]]) and sequential aggregate signatures [[27]].

According to Bellare *et al.* [[21]], a generic way to turn any multi-signature scheme into an IAS scheme by the signer running the multi-signature protocol using as message the tuple of all public keys/message pairs involved in the IAS protocol.

For BN's scheme and Schnorr multi-signatures, this does not increase the number of communication rounds as messages can be sent together with shares $ R_{i} $.

#### Applications of IAS

With regards to digital currency schemes, where all participants have the ability to validate transactions, these transactions consist of outputs (which have a verification key and amount) and inputs (which are references to outputs of earlier transactions). Each input contains a signature of a modified version of the transaction to be validated with its referenced output's key. Some outputs may require multiple signatures to be spent. Transactions spending such an output are referred to as *m*-of-*n* multi-signature transactions [[28]], and the current implementation corresponds to the trivial way of building a multi-signature scheme by concatenating individual signatures. Additionally, a threshold policy can be enforced where only $ m $ valid signatures out of the $ n $ possible ones are needed to redeem the transaction (again this is the most straightforward way to turn a multi-signature scheme into some kind of basic threshold signature scheme).

While several multi-signature schemes could offer an improvement over the currently available method, two properties increase the possible impact:

-   The availability of key aggregation removes the need for verifiers to see all the involved key, improving bandwidth, privacy, and validation cost

-   Security under the *plain public-key model* enables multi-signatures across multiple inputs of a transaction, where the choice of signers cannot be committed to in advance. This greatly increases the number of situations in which mulit-signatures are beneficial.

#### Native multi-signature support 

An improvement is to replace the need for implementing $ n-of-n $ multi-signatures with a constant-size multi-signature primitive like Bellare-Neven. While this is on itself an improvement in terms of size, it still needs to contain all of the signers' public keys. Key aggregation improves upon this further, as a single-key predicate[^2] can be used instead which is both smaller and has lower computational cost for verification. It also improves privacy, as the participant keys and their count remain private to the signers.

When generalizing to the $ m-of-n $ scenario, several options exist. One is to forego key aggregation, and still include all potential signer keys in the predicates while still only producing a single signature for the chosen combination of keys. Alternatively, a Merkle tree [[30]] where the leaves are permitted combinations of public keys (in aggregated form), can be employed. The predicate in this case would take as input an aggregated public key, a signature and a proof. Its validity would depend on the signature being valid with the provided key, and the proof establishing that the key is in fact one of the leaves of the Merkle tree, identified by its root hash. This approach is very generic, as it works for any subset of combinations of keys, and as a result has good privacy as the exact policy is not visible from the proof.

Some key aggregation schemes that do not protect against rogue-key attacks can be used instead in the above cases, under the assumption that the sender is given a proof of knowledge/possession for the receivers' private keys. However, these schemes are difficult to prove secure except by using very large proofs of knowledge. As those proofs of knowledge/possession do not need to be seen by verifiers, they are effectively certified by the senders's validation. However, passing them around to senders is inconvenient, and easy to get wrong. Using a scheme that is secure in the *plain public-key model* categorically avoids these concerns.

Another alternative is to use an algorithm whose key generation requires a trusted setup, for example in the KOSK model. While many of these schemes have been proven secure [[19]][[20]], they rely on mechanisms that are usually not implemented by certification authorities [[21]] [[18]].

#### Cross-input multi-signatures 

The previous sections explained how the numbers of signatures per input can generally by reduced to one, but one can go further and replace it with a single signature per transaction. Doing so requires a fundamental change in validation semantics, as the validity of separate inputs is no longer independent. As a result, the outputs can no longer be modeled as predicates, where the secret key owner is given access to encrypted data. Instead, they are modeled as functions that return a boolean (data type with only two possible values) plus a set of zero or more public keys.

Overall validity requires all returned booleans to be True and a multi-signature of the transaction with $ L $ the union of all returned keys.

With regards to Bitcoin, this can be implemented by providing an alternative to the signature checking opcode OP\_CHECKSIG and related opcodes in the Script language. Instead of returning the result of an actual ECDSA verification, they always return True, but additionally add the public key with which the verification would have taken place to a transaction-wide multi-set of keys. Finally, after all inputs are verified, a multi-signature present in the transaction is verified against that multi-set. In case the transaction spends inputs from multiple owners, they will need to collaborate to produce the multi-signature, or choose to only use the original opcodes. Adding these new opcodes is possible in a backward-compatible way. [[4]]

#### Protection against rogue-key Attacks

In Bitcoin, when taking cross-input signatures into account, there is no published commitment to the set of signers, as each transaction input can independently spend an output that requires authorization from distinct participants. This functionality was not restricted as it would then interfere with fungibility improvements such as CoinJoin [[31]]. Due to the lack of certification, security against rogue-key attacks is of great importance.

If it is assumed that transactions used a single multi-signature that was vulnerable to rogue-attacks, an attacker could identify an arbitrary number of outputs he wants to steal, with the public keys $ X_{1},...,X_{n-t} $ and then use the rogue-key attack to determine $ X_{n-t+1},...,X_{n} $ such that he can sign for the aggregated key $ \tilde{X} $. He would then send a small amount of his own money to outputs with predicates corresponding to the keys $ X_{n-t+1},...,X_{n} $. Finally, he can create a transaction that spends all of the victim coins together with the ones he just created by forging a multi-signature for the whole transaction.

It can be seen that in the case of mulit-signatures across inputs, theft can occur through the ability to forge a signature over a set of keys that includes at least one key which is not controlled by the attacker. According to the *plain public-key model* this is considered a win for the attacker. This is in contrast to the single-input multi-signature case where theft is only possible by forging a signature for the exact (aggregated) keys contained in an existing output. As a result, it is no longer possible to rely on proofs of knowledge/possession that are private to the signers.

### Revisions 

In a previous version of the paper by Maxwell *et al.* [[4]] published on 15 January 2018 they proposed a 2-round variant of MuSig, where the initial commitment round is omitted claiming a security proof under the One More Discrete Logarithm (OMDL) assumptions ([[32]], [[33]]). Drijvers *et al* [[34]] then discovered a flaw in the security proof and showed that through a meta-reduction the initial multi-signature scheme cannot be proved secure using an algebraic black box reduction under the DL or OMDL assumption.

In more details, it was observed that in the 2-round variant of MuSig, an adversary (controlling public keys $ X_{2},...,X_{n}) $ can impose the value of $ R=\Pi_{i=1}^{n}R_{i} $ used in signature protocols since he can choose $ R_{2},...,R_{n} $ after having received $ R_{1} $ from the honest signer (controlling public key $ X_{1}=g^{x_{1}} $ ). This prevents one to use the initial method of simulating the honest signer in the Random Oracle model without knowing $ x_{1} $ by randomly drawing $ s_{1} $ and $ c $, computing

Despite this, there is no attack currently known against the 2-round variant of MuSig and that it might be secure, although this is not provable under standard assumptions from existing techniques. [[4]]

## Conclusions, observations and recommendations

-   MuSig leads to both native and private multi-signature transactions with both signature aggregation.

-   Signature data for multi-signatures can be large and cumbersome. MuSig will allow users to create more complex transactions without burdening the network and revealing compromising information.

-   However, the case of interactive signature aggregation where each signer signs their own message must still be proven by a complete security analysis.

## Contributors 

Kevoulee Sardar, Hansie Odendaal, Cayle Sharrock

## References 

[[1]] P. Wuille, “Key Aggregation for Schnorr Signatures,” 2018. Date accessed: 2019-01-20 

[1]: https://blockstream.com/2018/01/23/musig-key-aggregation-schnorr-signatures/
"Key Aggregation for Schnorr Signatures, 
2018, P. Wuille" 

[[2]] Blockstream, “Schnorr Signatures for Bitcoin - BPASE ’18,” 2018. Date accessed: 2019-01-20 

[2]: https://blockstream.com/2018/02/15/schnorr-signatures-bpase/
"Schnorr Signatures for Bitcoin- BPASE '18, 
2018, Blockstream"

[[3]] K. Itakura, “A public-key cryptosystem suitable for digital multisignatures,” NEC J. Res. Dev., vol. 71, 1983. Date accessed: 2019-01-20 

[3]: https://scinapse.io/papers/200023587/
"A public-key cryptosystem suitable for digital multisignatures,
1983, K. Itakura"

[[4]] G. Maxwell *et al*. , “Simple Schnorr Multi-Signatures with Applications to Bitcoin,” pp. 1–34, 2018. Date accessed: 2019-01-20 

[4]: https://eprint.iacr.org/2018/068.pdf
"Simple Schnorr Multi-Signatures with 
Applications to Bitcoin, 2018, 
G. Maxwell *et al*."

[[5]] B. W. Contributors, “Multisignature,” 2017. Date accessed: 2019-01-20 

[5]: https://wiki.bitcoin.com/w/Multisignature
"Multisignature, 2017"

[[6]] C. P. Schnorr, “Efficient signature generation by smart cards,” Journal of cryptology, vol. 4, no. 3, pp. 161–174, 1991. Date accessed: 2019-01-20

[6]: https://link.springer.com/article/10.1007/BF00196725
"Efficient signature generation by smart cards, 
1991, C. P. Schnorr"

[[7]] D. J. Bernstein *et al.* , “High-speed high-security signatures,” Journal of Cryptographic Engineering, vol. 2, no. 2, pp. 77–89, 2012. Date accessed: 2019-01-20

[7]: https://ed25519.cr.yp.to/ed25519-20110705.pdf
"High-speed high-security signatures, 
2012, D. J. Bernstein *et al.*"

[[8]] D. J. Bernstein, “Multi-user Schnorr security, revisited.,” IACR Cryptology ePrint Archive, vol. 2015, p. 996, 2015. Date accessed: 2019-01-20

[8]: https://eprint.iacr.org/2015/996.pdf
"Multi-user Schnorr security, revisited,
2015, D. J. Bernstein" 

[[9]] E. Kiltz *et al.*, “Optimal security proofs for signatures from identification schemes,” in Annual Cryptology Conference, pp. 33–61, Springer, 2016. Date accessed: 2019-01-20

[9]: https://eprint.iacr.org/2016/191.pdf
"Optimal Security Proofs for Signatures from
Identification Schemes, 2016,
E. Kiltz *et al.*"

[[10]] C. M. Li *et al.*, “Threshold-multisignature schemes where suspected forgery implies traceability of adversarial shareholders,” in Workshop on the Theory and Application of Cryptographic Techniques, pp. 194–204, Springer, 1994. Date accessed: 2019-01-20

[10]: https://link.springer.com/content/pdf/10.1007%2FBFb0053435.pdf
"Threshold-multisignature schemes 
where suspected forgery implies 
traceability of adversarial shareholders,
1994, C. M. Li *et al.*"

[[11]] L. Harn, “Group-oriented (t, n) threshold digital signature scheme and digital multisignature,” IEE Proceedings-Computers and Digital Techniques, vol. 141, no. 5, pp. 307–313, 1994. Date accessed: 2019-01-20

[11]: https://ieeexplore.ieee.org/abstract/document/326780
"Group-oriented (t, n) threshold digital 
signature scheme and digital multisignature
1994, L. Harn"

[[12]] P. Horster *et al.*, “Meta-Multisignature schemes based on the discrete logarithm problem,” in Information Security-the Next Decade, pp. 128–142, Springer, 1995. Date accessed: 2019-01-20

[12]: https://link.springer.com/content/pdf/10.1007%2F978-0-387-34873-5_11.pdf
"Meta-Multisignature schemes based
on the discrete logarithm problem, 
1995, P. Horster *et al.*"

[[13]] K. Ohta and T. Okamoto, “A digital multisignature scheme based on the Fiat-Shamir scheme,” in International Conference on the Theory and Application of Cryptology, pp. 139–148, Springer, 1991. Date accessed: 2019-01-20

[13]: https://link.springer.com/chapter/10.1007/3-540-57332-1_11
"A digital multisignature scheme 
based on the Fiat-Shamir scheme,
1991, K. Ohta and T. Okamoto"

[[14]] S. K. Langford, “Weaknesses in some threshold cryptosystems,” in Annual International Cryptology Conference, pp. 74–82, Springer, 1996. Date accessed: 2019-01-20

[14]: https://link.springer.com/content/pdf/10.1007%2F3-540-68697-5_6.pdf
"Weaknesses in Some Threshold 
Cryptosystem, 1996, 
S. K. Langford" 

[[15]] M. Michels and P. Horster, “On the risk of disruption in several multiparty signature schemes,” in International Conference on the Theory and Application of Cryptology and Information Security, pp. 334–345, Springer, 1996. Date accessed: 2019-01-20

[15]: https://pdfs.semanticscholar.org/d412/e5ab35fd397931cef0f8202324308f44e545.pdf
"On the risk of disruption in several 
multiparty signature schemes, 1996, 
M. Michels and P. Horster" 

[[16]] K. Ohta and T. Okamoto, “Multi-signature schemes secure against active insider attacks,” IEICE Transactions on Fundamentals of Electronics, Communications and Computer Sciences, vol. 82, no. 1, pp. 21–31, 1999. Date accessed: 2019-01-20

[16]: http://search.ieice.org/bin/summary.php?id=e82-a_1_21&category=A&year=1999&lang=E&abst=
"Multi-signature schemes secure 
against active insider attacks, 1999, 
K. Ohta and T. Okamoto" 

[[17]] S. Micali *et al.*, “Accountable-subgroup multisignatures,” in Proceedings of the 8th ACM conference on Computer and Communications Security, pp. 245–254, ACM, 2001. Date accessed: 2019-01-20

[17]: https://pdfs.semanticscholar.org/6bf4/f9450e7a8e31c106a8670b961de4735589cf.pdf
"Accountable-subgroup multisignatures,
2001, S. Micali *et al.*" 

[[18]] T. Ristenpart and S. Yilek, “The power of proofs-of-possession: Securing multiparty signatures against rogue-key attacks,” in Annual International Conference on the Theory and Applications of Cryptographic Techniques, pp. 228–245, Springer, 2007. Date accessed: 2019-01-20

[18]: https://link.springer.com/content/pdf/10.1007%2F978-3-540-72540-4_13.pdf
"The power of proofs-of-possession: 
Securing multiparty signatures against
rogue-key attacks, 2007, 
T. Ristenpart and S. Yilek" 

[[19]] A. Boldyreva, “Threshold signatures, multisignatures and blind signatures based on the gap-Diffie-Hellman-group signature scheme,” in International Workshop on Public Key Cryptography, pp. 31–46, Springer, 2003. Date accessed: 2019-01-20

[19]: https://www.iacr.org/archive/pkc2003/25670031/25670031.pdf
"Threshold Signatures, Multisignatures
and Blind Signatures Based on the
Gap-Diffie-Hellman-Group Signature 
Scheme, 2003, A. Boldyreva"

[[20]] S. Lu *et al.*, “Sequential aggregate signatures and multisignatures without random oracles,” in Annual International Conference on the Theory and Applications of Cryptographic Techniques, pp. 465–485, Springer, 2006. Date accessed: 2019-01-20

[20]: https://eprint.iacr.org/2006/096.pdf
"Sequential Aggregate Signatures and
Multisignatures without Random Oracles, 
2006, S. Lu *et al.*" 

[[21]] M. Bellare and G. Neven, “Multi-Signatures in the Plain Public- Key Model and a General Forking Lemma,” Acm Ccs, pp. 390– 399, 2006. Date accessed: 2019-01-20

[21]: https://cseweb.ucsd.edu/~mihir/papers/multisignatures-ccs.pdf
"Multi-Signatures in the Plain Public-Key 
Modeland a General Forking Lemma,
2006, M. Bellare and G. Neven"

[[22]] A. Bagherzandi *et al.*, “Multisignatures Secure under the Discrete Logarithm Assumption and a Generalized Forking Lemma,” Proceedings of the 15th ACM conference on Computer and communications security - CCS ’08, p. 449, 2008. Date accessed: 2019-01-20

[22]: http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.544.2947&rep=rep1&type=pdf
"Multisignatures Secure under the
Discrete Logarithm Assumption
and a Generalized Forking Lemma,
2008, A. Bagherzandi *et al.*"

[[23]] C. Ma *et al.*, “Efficient discrete logarithm based multi-signature scheme in the plain public key model,” Designs, Codes and Cryptography, vol. 54, no. 2, pp. 121–133, 2010. Date accessed: 2019-01-20

[23]: https://link.springer.com/article/10.1007/s10623-009-9313-z
"Efficient discrete logarithm 
based multi-signature scheme 
in the plain public key model, 
2010, C. Ma *et al.*"

[[24]] E. Syta *et al.*, “Keeping authorities" honest or bust" with decentralized witness cosigning,” in Security and Privacy (SP), 2016 IEEE Symposium on, pp. 526–545, Ieee, 2016. Date accessed: 2019-01-20

[24]: https://arxiv.org/pdf/1503.08768.pdf
"Keeping authorities" honest 
or bust" with decentralized 
witness cosigning, 2016, 
E. Syta *et al.*"

[[25]] D. Boneh *et al.*, “Aggregate and verifiably encrypted signatures from bilinear maps,” in International Conference on the Theory and Applications of Cryptographic Techniques, pp. 416–432, Springer, 2003. Date accessed: 2019-01-20

[25]: http://crypto.stanford.edu/~dabo/papers/aggreg.pdf
"Aggregate and Verifiably Encrypted 
Signatures from Bilinear Maps, 
2003, D. Boneh *et al.*"

[[26]] M. Bellare *et al.*, “Unrestricted aggregate signatures,” in International Colloquium on Automata, Languages, and Programming, pp. 411–422, Springer, 2007. Date accessed: 2019-01-20

[26]: https://cseweb.ucsd.edu/~mihir/papers/agg.pdf
"Unrestricted aggregate signatures, 
2007, M. Bellare *et al.*"

[[27]] A. Lysyanskaya *et al.*, “Sequential aggregate signatures from trapdoor permutations,” in International Conference on the Theory and Applications of Cryptographic Techniques, pp. 74–90, Springer, 2004. Date accessed: 2019-01-20

[27]: https://hovav.net/ucsd/dist/rsaagg.pdf
"Sequential Aggregate Signatures 
from Trapdoor Permutations, 
2004, A. Lysyanskaya *et al.*" 

[[28]] G. Andersen, “M-of-N Standard Transactions,” 2011. Date accessed: 2019-01-20

[28]: https://bitcoin.org/en/glossary/multisig
"M-of-N Standard Transactions, 2011"

[[29]] E. Shen *et al.*, “Predicate privacy in encryption systems,” in Theory of Cryptography Conference, pp. 457–473, Springer, 2009. Date accessed: 2019-01-20

[29]: https://www.iacr.org/archive/tcc2009/54440456/54440456.pdf
"Predicate privacy in encryption 
systems, 2009, E. Shen *et al.*"

[[30]] R. C. Merkle, “A digital signature based on a conventional encryption function,” in Conference on the theory and application of cryptographic techniques, pp. 369–378, Springer, 1987. Date accessed: 2019-01-20

[30]: https://people.eecs.berkeley.edu/~raluca/cs261-f15/readings/merkle.pdf
"A digital signature based on a 
conventional encryption function, 
1987, R. C. Merkle" 

[[31]] G. Maxwell, “CoinJoin: Bitcoin privacy for the real world,” 2013. Date accessed: 2019-01-20

[31]: https://bitcointalk.org/index.php?topic=279249.0
"CoinJoin: Bitcoin privacy for the 
real world, 2013"

[[32]] M. Bellare and A. Palacio, “GQ and Schnorr identification schemes: Proofs of security against impersonation under active and concurrent attacks,” in Annual International Cryptology Conference, pp. 162–177, Springer, 2002. Date accessed: 2019-01-20

[32]: https://cseweb.ucsd.edu/~mihir/papers/gq.pdf
"GQ and Schnorr identification schemes: Proofs of security against impersonation under active and concurrent attacks, 2002, M. Bellare and A. Palacio"

[[33]] M. Bellare *et al.*, “The One-More-RSA Inversion Problems and the Security of Chaum’s Blind Signature Scheme.,” Journal of Cryptology, vol. 16, no. 3, 2003. Date accessed: 2019-01-20

[33]: https://eprint.iacr.org/2001/002.pdf
"The One-More-RSA-Inversion Problems
and the Security of Chaum’s Blind 
Signature Scheme, 2003, M. Bellare *et al.*"

[[34]] M. Drijvers, K. Edalatnejad, B. Ford, and G. Neven, “Okamoto Beats Schnorr: On the Provable Security of Multi-Signatures,” tech. rep., 2018. Date accessed: 2019-01-20

[34]: https://www.semanticscholar.org/paper/Okamoto-Beats-Schnorr%3A-On-the-Provable-Security-of-Drijvers-Edalatnejad/154938a12885ff30301129597ebe11dd153385bb
"Okamoto Beats Schnorr: On the Provable Security of Multi-Signatures

[^1]: No constraints are imposed on the key setup, the adversary thus can choose corrupted public keys at random, hence the same public key can appear more than once in $ L $ 

[^2]: Predicate encryption is an encryption paradigm which gives a master secret key owner fine-grained control over access to encrypted data.[[29]]
