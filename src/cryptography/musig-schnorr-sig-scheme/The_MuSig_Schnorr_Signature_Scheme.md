This report investigates Schnorr Multi-Signature Schemes (MuSig), which makes use of key aggregation and is provably secure in the *plain public-key model*.

Signature aggregation involves mathematically combining several signatures into a single signature, without having to prove Knowledge of Secret Keys (KOSK). This is known as the *plain public-key model* where the only requirement is that each potential signer has a public key. The KOSK scheme requires that users prove knowledge (or possession) of the secret key during public key registration with a certification authority, and is one way to generically prevent rogue-key attacks. 

Multi-signatures are a form of technology used to add multiple participants to cryptocurrency transactions. A traditional multi-signature protocol allows a group of signers to produce a joint multi-signature on a common message.

# Introduction 

## Schnorr signatures and their attack vectors 

Schnorr signatures produce a smaller on-chain size, support faster validation and have better privacy. They natively allow for combining multiple signatures into one through aggregation and they permit more complex spending policies. 

Signature aggregation also has its challenges. This includes the rogue-key attack, where a participant steals funds using a specifically constructed key. Although this is easily solved for simple multi-signatures through an enrollment procedure which involves the keys signing themselves, supporting it across multiple inputs of a transaction requires *plain public-key security*, meaning there is no setup. 

There is an additional attack, termed the Russel attack, after Russel O’Connor, who has discovered that for multi-party schemes a party could claim ownership of someone else’s private key and so spend the other outputs.

Wuille P.,  has been able to address some of these issues and has provided a solution which refines the Bellare-Neven (BN) scheme. He also discussed the performance improvements that were implemented for the scaler multiplication of the BN scheme and how they enable batch validation on the blockchain.

## MuSig

Introduced by Itakura *et al.* , multi-signature protocols allow a group of signers (that individually possess their own private/public key pair) to produce a single signature \(\sigma\) on a message \(m\). Verification of the given signature \(\sigma\) can be publicly performed given the message and the set of public keys of all signers.

A simple way to change a standard signature scheme into a multi-signature scheme is to have each signer produce a stand-alone signature for \(m\) with its private key and to then concatenate all individual signatures.

The transformation of a standard signature scheme to a multi-signature scheme needs to useful and practical, thus the newly calculated multi-signature scheme must produce signatures where the size is independent of the number of signers and similar to that of the original signature scheme. 

A traditional multi-signature scheme is a combination of a signing and verification algorithm, where multiple signers (each with their own private/public key) jointly sign a single message, resulting in a combined signature. This can then be verified by anyone knowing the message and the public keys of the signers, where a trusted setup with KOSK is a requirement.

MuSig is a multi-signature scheme is novel in combining:

1.  Support for key aggregation;

2.  Security in the *plain public-key model*.

There are two versions of MuSig, that are provably secure, which differ based on the number of communication rounds:

1.  Three-round MuSig only relies on the Discrete Logarithm (DL) assumption, on which ECDSA (Elliptic Curve Digital Signature Algorithm) also relies

2.  Two-round MuSig instead relies on the slightly stronger One-More Discrete Logarithm (OMDL) assumption

## Key aggregation

The term *key aggregation* refers to multi-signatures that look like a single-key signature, but with respect to an aggregated public key that is a function of only the participants’ public keys. Thus, verifiers do not require the knowledge of the original participants’ public keys- they can just be given the aggregated key. In some use cases, this leads to better privacy and performance. Thus, MuSig is effectively a key aggregation scheme for Schnorr signatures.

To make the traditional approach more effective and without needing a trusted setup, a multi-signature scheme must provide sub-linear signature aggregation along with the following properties:

  - It must be provably secure in the *plain public-key model*

  - It must satisfy the normal Schnorr equation, whereby the resulting signature can be written as a function of a combination of the public keys

  - It must allow for Interactive Aggregate Signatures (IAS) where the signers are required to cooperate

  - It must allow for Non-interactive Aggregate Signatures (NAS) where the aggregation can be done by anyone

  - It must allow each signer to sign the same message

  - It must allow each signer to sign their own message

This is different to a normal multi-signature scheme where one message is signed by all. MuSig provides all of those properties.

There are other multi-signature schemes that already exist that provide key aggregation for Schnorr signatures, however they come with some limitations, such as needing to verify that participants actually have the private key corresponding to the pubic keys that they claim to have. *Security in the plain public-key model* means that no limitations exist. All that is needed from the participants is their public keys. 

# Overview of multi-signatures 

Recently the most obvious use case for multi-signatures is with regards to Bitcoin, where it can function as a more efficient replacement of *(n-of-n\)* multisig scripts (where the signatures required to spend and the signatures possible are equal in quantity) and other policies that permit a number of possible combinations of keys.

A key aggregation scheme also lets one reduce the number of public keys per input to one, as a user can send coins to the aggregate of all involved keys rather than including them all in the script. This leads to smaller on-chain footprint, faster validation, and better privacy.

Instead of creating restrictions with one signature per input, one signature can be used for the entire transaction. Traditionally key aggregation cannot be used across multiple inputs, as the public keys are committed to by the outputs, and those can be spent independently. MuSig can be used here (with key aggregation done by the verifier).

No non-interactive aggregation scheme is known that only relies on the DL assumption, but interactive schemes are trivial to construct where a multi-signature scheme has every participant sign the concatenation of all messages. Maxwell G., *et al.*  focusing on key aggregation for Schnorr Signatures and shows that this is not always a desirable construction, and gives an IAS variant of BN with better properties
instead.

## Bitcoin \(m-of-n\) multi-signatures 

Currently, standard transactions on the Bitcoin network can be referred to as single-signature transactions, as they require only one signature, from the owner of the private key associated with the Bitcoin address. However, the Bitcoin network supports much more complicated transactions which can require the signatures of multiple people before the funds can be transferred. These are often referred to as \(m-of-n\) transactions, where m represents the amount of signatures required to spend, while n represents the amount of signatures possible. 

### Use cases for \(m-of-n\) multi-signatures

When \(m=1\) and \(n>1\) it is considered a shared wallet, which could be used for small group funds that do not require much security. It is the least secure multi-sig option because it is not multi-factor. Any compromised individual would jeopardize the entire group. Examples of use cases include funds for a weekend or evening event, or a shared wallet for some kind of game. Besides being convenient to spend from, the only benefit of this setup is that all but one of the backup/password pairs could be lost and all of the funds would be recoverable.

When \(m=n\), it is considered a partner wallet, which brings with it some nervousness as no keys can be lost. As the number of signatures required increases, the risk also increases. This type of multi-signature can be considered as a hard multi-factor authentication.

When \(m<0.5n\), it is considered a buddy account, which could be used for spending from corporate group funds. Consequence for the colluding minority need to greater than possible benefits. It is considered less convenient than a shared wallet, but much more secure.

When \(m>0.5n\), a consensus account is termed. The classic multi-signature wallet is a 2 of 3 and is a special case of a consensus account. A 2 of 3 scheme has the best characteristics for creating new bitcoin addresses and for secure storing and spending. One compromised signatory does not compromise the funds. A single secret key can be lost and the funds can still be recovered. If done correctly, off-site backups are created during wallet setup. The way to recover funds is known by more than one party. The balance of power with a multi-signature wallet can be shifted by having one party control more keys than the other parties. If one party controls multiple keys, there is a greater risk of those keys not remaining as multiple factors.

When \(m=0.5n\), it is referred to as a split account, and is an interesting use case, as there would be 3 of 6 where one person holds 3 keys and 3 people hold 1 key. In this way one person could control their own money, but the funds could still be recoverable even if the primary key holder were to disappear with all of his key. As \(n\) increases, the level of trust in the secondary parties can decrease. A good use case might be a family savings account that would just automatically become an inheritance account if the primary account holder were to die.

The above described is referred to as the so-called “key-prefixed” variant of the scheme, which sees the public key hashed together with \(R\) and \(m\). This variant was thought to have a better multi-user security bound than the classic variant , however in  the key-prefixing was seen as unnecessary to enable good multi-user security for Schnorr signatures.

For the development of the new Schnorr-based multi-signature scheme, key-prefixing seemed a requirement for the security proof to go through, despite not knowing the form of an attack. The rationale also follows the process in reality, as messages signed in Bitcoin always indirectly commits to the public key.

### Rogue attacks 

Rogue attacks are a significant concern when implementing multi-signature schemes. Here a subset of corrupted singers, manipulate the public keys computed as functions of the public keys of honest users, allowing them to easily produce forgeries for the set of public keys (despite them not knowing the associated secret keys).

Initial proposals from , , , , ,  and  were thus undone before a formal model was put forward along with a provably secure scheme from Micali *et al*. . Unfortunately, despite being provably secure their scheme is costly and an impractical interactive key generation protocol. 

A means of generically preventing rogue-key attacks is to make it mandatory for users to prove knowledge (or possession) of the secret key during public key registration with a certification authority. Certification authority is a setting known as the KOSK assumption. The pairing-based multi-signature schemes by Boldyreva  and Lu *et al*. rely on the KOSK assumption in order to maintain security. However according to and the cost of complexity and expense of the scheme and the unrealistic and burdensome assumptions on the public-key infrastructure (PKI) have made this solution problematic.

As it stands, the Bellare and Neven  provides one of the most practical multi-signature schemes, based on the Schnorr signature scheme, which is provably secure that does not contain any assumption on the key setup. Since the only requirement of this scheme is that each potential signer has a public key, this setting is referred to as the *plain-key model.*

### The Ma *et al.* scheme 

Ma *et al.*  proposed a signature scheme that involved the “double hashing” technique, which sees the reduction of the signature size compared to Bagherzandi *et al.*  while using only two rounds.

However, neither of these two variants allow for key aggregation.

Multi-signature schemes supporting key aggregation are easier to come by in the KOSK model. In particular, Syta *et al.*  proposed ** the CoSi scheme which can be seen as the naive Schnorr multi-signature scheme described earlier where the co-signers are organized in a tree structure for fast signature generation.

The resulting signature does not satisfy the normal Schnorr equation anymore, nor any other equation that can be written as a function of a combination of the public keys; the key aggregation property is lost in order to gain security in the *plain public-key model*.

Bellare and Neven showed that this yields a multi-signature scheme provably secure in the *plain public-key* model under the Discrete Logarithm assumptions, modeling \(\textrm{H}\) and \(\textrm{H}'\ as random oracles. However, this scheme does not allow key aggregation anymore since the entire list of public keys is required for verification.

Basically , the key aggregation property has been recovered and can now be enjoyed by the naive scheme, which respect to a more complex aggregation key \(\tilde{X}=\stackrel[i=1]{n}{\Pi}X_{i}^{a_{i}c}\).

\(c=H_{sig}(<L>,R,m)\) yields a secure scheme, however does not allow key aggregation since verification is impossible without knowing all the individual singer keys.

## Interactive Aggregate Signatures

In some situations, it may be useful to allow each participant to sign a different message rather than a single common one. An IAS is one where each signer has its own message *\(m_{i}\)* to sign, and the joint signature proves that the \(i\)-th signer has signed \(m_{i}\) . These schemes are considered to be more general than multi-signature schemes, however they are not as flexible as non-interactive aggregate signatures (, ) and sequential aggregate signatures .

According to Bellare *et al.* , a generic way to turn any multi-signature scheme into an IAS scheme by the signer running the multi-signature protocol using as message the tuple of all public keys/message pairs involved in the IAS protocol.

For BN’s scheme and Schnorr multi-signatures, this does not increase the number of communication rounds as messages can be sent together with shares \(R_{i}\).

### Applications of IAS

With regards to digital currency schemes, where all participants have the ability to validate transactions, these transactions consist of outputs (which have a verification key and amount) and inputs (which are references to outputs of earlier transactions). Each input contains a signature of a modified version of the transaction to be validated with its referenced output’s key. Some outputs may require multiple signatures to be spent. Transactions spending such an output are referred to as *m*-of-*n* multi-signature transactions , and the current implementation corresponds to the trivial way of building a multi-signature scheme by concatenating individual signatures.

Additionally, a threshold policy can be enforced where only \(m\) valid signatures out of the \(n\) possible ones are needed to redeem the transaction (again this is the most straightforward way to turn a multi-signature scheme into some kind of basic threshold signature scheme).

While several multi-signature schemes could offer an improvement over the currently available method, two properties increase the possible impact:

  - The availability of key aggregation removes the need for verifiers to see all the involved key, improving bandwidth, privacy, and validation cost

  - Security under the *plain public-key model* enables multi-signatures across multiple inputs of a transaction, where the choice of signers cannot be committed to in advance. This greatly increases the number of situations in which mulit-signatures are beneficial.

### Native multi-signature support 

An improvement is to replace the need for implementing *\(n-of-n\)* multi-signatures with a constant-size multi-signature primitive like Bellare-Neven. While this is on itself an improvement in terms of size, it still needs to contain all of the signers’ public keys. Key aggregation improves upon this further, as a single-key predicate\[1\] can be used instead which is both smaller and has lower computational cost for verification. It also improves privacy, as the participant keys and their count remain private to the signers.

When generalizing to the \(m-of-n\) scenario, several options exist. One is to forego key aggregation, and still include all potential signer keys in the predicates while still only producing a single signature for the chosen combination of keys. Alternatively, a Merkle tree  where the leaves are permitted combinations of public keys (in aggregated form), can be employed. The predicate in this case would take as input an aggregated public key, a signature and a proof. Its validity would depend on the signature being valid with the provided key, and the proof establishing that the key is in fact one of the leaves of the Merkle tree, identified by its root hash. This approach is very generic, as it works for any subset of combinations of keys, and as a result has good privacy as the exact policy is not visible from the proof.

1.  Predicate encryption is an encryption paradigm which gives a master secret key owner fine-grained control over access to encrypted data.


Some key aggregation schemes that do not protect against rogue-key attacks can be used instead in the above cases, under the assumption that the sender is given a proof of knowledge/possession for the receivers’ private keys. However, these schemes are difficult to prove secure except by using very large proofs of knowledge. As those proofs of knowledge/possession do not need to be seen by verifiers, they are effectively certified by the senders’s validation. However, passing them around to senders is inconvenient, and easy to get wrong. Using a scheme that is secure in the *plain public-key model* categorically avoids these concerns.

Another alternative is to use an algorithm whose key generation requires a trusted setup, for example in the KOSK model. While many of these schemes have been proven secure , they rely on mechanisms that are usually not implemented by certification authorities  .

### Cross-input multi-signatures 

The previous sections explained how the numbers of signatures per input can generally by reduced to one, but one can go further and replace it with a single signature per transaction. Doing so requires a fundamental change in validation semantics, as the validity of separate inputs is no longer independent. As a result, the outputs can no longer be modeled as predicates, where the secret key owner is given access to encrypted data. Instead, they are modeled as functions that return a boolean (data type with only two possible values) plus a set of zero or more public keys.

Overall validity requires all returned booleans to be True and a multi-signature of the transaction with \(L\) the union of all returned keys.

With regards to Bitcoin, this can be implemented by providing an alternative to the signature checking opcode OP\_CHECKSIG and related opcodes in the Script language. Instead of returning the result of an actual ECDSA verification, they always return True, but additionally add the public key with which the verification would have taken place to a transaction-wide multi-set of keys. Finally, after all inputs are verified, a multi-signature present in the transaction is verified against that multi-set. In case the transaction spends inputs from multiple owners, they will need to collaborate to produce the multi signature, or choose to only use the original opcodes. Adding these new opcodes is possible in a backward-compatible way.

### Protection against rogue-key Attacks

In Bitcoin, when taking cross-input signatures into account, there is no published commitment to the set of signers, as each transaction input can independently spend an output that requires authorization from distinct participants. This functionality was not restricted as it would then interfere with fungibility improvements such as CoinJoin . Due to the lack of certification, security against rogue-key attacks is of
great importance.

It can be seen that in the case of mulit-signatures across inputs, theft can occur through the ability to forge a signature over a set of keys that includes at least one key which is not controlled by the attacker. According to the *plain public-key model* this is considered a win for the attacker. This is in contrast to the single-input multi-signature case where theft is only possible by forging a signature for the exact (aggregated) keys contained in an existing output. As a result, it is no longer possible to rely on proofs of knowledge/possession that are private to the signers.

## Revisions 

In a previous version of the paper by Maxwell *et al.*  ** published on 15 January 2018 they proposed a 2 round variant of MuSig, where the initial commitment round is omitted claiming a security proof under the One More Discrete Logarithm (OMDL) assumptions (, ). Drijvers *et al*  then discovered a flaw in the security proof and showed that through a meta-reduction the initial multi-signature scheme cannot be proved
secure using an algebraic black box reduction under the DL or OMDL assumption.

Despite this, there is no attack currently known against the 2-round variant of MuSig and that it might be secure, although this is not provable under standard assumptions from existing techniques. 

# Conclusions, Observations and Recommendations

  - MuSig leads to both native and private multi-signature transactions with both signature aggregation.

  - Signature data for multi-signatures can be large and cumbersome. MuSig will allow users to create more complex transactions without burdening the network and revealing compromising information.

  - However, the case of interactive signature aggregation where each signer signs their own message must still be proven by a complete security analysis.

# Contributors 

Kevoulee Sardar, Hansie Odendaal



