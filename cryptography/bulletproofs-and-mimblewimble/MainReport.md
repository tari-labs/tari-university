# <a name="h-Bulletproofs-and-Mimblewimble"> </a>Bulletproofs and Mimblewimble

## <a name="h-Introduction"> </a>Introduction

Bulletproofs form part of the family of distinct Zero-knowledge<sup>[zk_]</sup> proof systems, like Zero-Knowledge Succinct Non-Interactive ARguments of Knowledge (zk-SNARK), Succinct Transparent ARgument of Knowledge (STARK) and Zero Knowledge Prover and Verifier for Boolean Circuits (ZKBoo). Zero-knowledge proofs are designed so that a prover is able to indirectly verify that a statement is true without having to provide any information beyond the verification of the statement, for example to prove that a number is found that solves a cryptographic puzzle and fits the hash value without having to reveal the nonce. ([[2]][\[2\]], [[4]][\[4\]])

Bulletproofs is a non-interactive zero-knowledge proof protocol for general Arithmetic Circuits<sup>[def](#ac)</sup> with very short proofs (Arguments of Knowledge Systems<sup>[def](#afs)</sup>) and without requiring a Trusted Setup<sup>[def](#ts)</sup>. They rely on the discrete logarithmic assumption and are made non-interactive using the Fiat-Shamir Heuristic<sup>[def](#fsh)</sup>. The name 'Bulletproofs' originated from a non-technical summary of the scheme's properties: "<i>short like a bullet with bulletproof security assumption</i>". [[1]][\[1\]]



In essence they are inner product arguments that provide general ways to prove knowledge about multiplying things to some other things. [[2]][\[2\]]



Bulletproofs have wide application [[3]][\[3\]] and can be used for :

- Rangeproofs
  - ???
- Merkle proofs
  - ???
- Proof of solvency
  - ???
- Multisig with deterministic nonces
  - ???
- Scriptless Scripts (with ECDSA in some cases)
  - ???
- Assets / smart contracts / crypto-derivatives
  - ???



## <a name="h-Contents"> </a>Contents

- [Bulletproofs and Mimblewimble](#h-Bulletproofs-and-Mimblewimble)
  - [Introduction](#h-Introduction)
  - [Contents](#h-Contents)
  - [Interesting Bulletproof Implementation Snippets](#h-Interesting-Bulletproof-Implementation-Snippets)
    - [Wallet Reconstruction - Grin](#h-Wallet-Reconstruction-Grin)
    - [Current & Past Efforts](#h-Current-Past-Efforts)
  - [Negatives](#h-Negatives)
  - [Conclusions, Observations, Recommendations](#h-Conclusions,-Observations,-Recommendations)
  - [Definition of Terms](#h-Definition-of-Terms)
  - [References](#h-References)
  - [Contributors](#h-Contributors)



## <a name="h-Interesting-Bulletproof-Implementation-Snippets"> </a>Interesting Bulletproof Implementation Snippets

### <a name="h-Current-Past-Efforts"> </a>Current & Past Efforts



### <a name="h-Wallet-Reconstruction-Grin"> </a>Wallet Reconstruction - Grin

See  [[11]][\[11\]]

"{**yeastplume** } Single commit bullet proofs appear to be working, which is all we need. The only think I think we're missing here from being able to use this implementation is the ability to store an amount within the rangeproof (for wallet reconstruction). From conversations with @apoelstra earlier, I believe it's possible to store 64 bytes worth of 'message' (not nearly as much as the current range proofs). We also need to be aware that we can't rely as much on the message hiding properties of range proofs when switching to bullet proofs."

"{**yeastplume** } @apoelstra the amount, and quite possibly the switch commitment hash as well (or just a hash of the entire output) as per #207..."

"{**apoelstra**} Ok, I can get you 64 bytes without much trouble (xoring them into `tau_1` and `alpha` which are easy to extract from `tau_x` and `mu` if you know the original seed used to produce the randomness). I think it's possible to get another 32 bytes into `t` but that's way more involved since `t` is a big inner product." 

???

## <a name="h-Negatives"> </a>Negatives

- A discrete-log attacker (e.g. a quantum computer) would be able to exploit Bulletproofs to silently inflate any currency that used them [[10]][\[10\]]

## <a name="h-Conclusions,-Observations,-Recommendations"> </a>Conclusions, Observations, Recommendations

- ???

## <a name="h-Definition-of-Terms"> </a>Definition of Terms

- <u><i>Arithmetic Circuits</i></u>:<a name="ac"> </a>An arithmetic circuit over a field and variables <code>(a1, ..., an)</code> is a directed acyclic graph whose vertices are called gates. Arithmetic circuits can alternatively be described as a list of multiplication gates with a collection of linear consistency equations relating the inputs and outputs of the gates. [[12]][\[12\]]
- <u><i>Argument of Knowledge System</i></u>:<a name="afs"> </a>Proof systems with computational soundness like Bulletproofs are sometimes called argument systems. [[12]][\[12\]]
- <u><i>Trusted Setup</i></u>:<a name="ts"> </a>???
- <u><i>Zero-knowledge Proof/Protocol</i></u>:<a name="zk"> </a>In cryptography, a zero-knowledge proof/protocol is a method by which one party (the prover Peggy) can prove to another party (the verifier Victor) that she knows a value `w`, without conveying any information apart from the fact that she knows the value `w`. [[16]][\[16\]]

[zk_]: #zk 'In cryptography, a zero-knowledge proof/protocol is a method by which one party (the prover Peggy) can prove to another party (the verifier Victor) that she knows a value `w`, without conveying any information apart from the fact that she knows the value `w`.'

- <u><i>Fiat–Shamir Heuristic/Transformation</i></u>:<a name="fsh"> </a>The Fiat–Shamir heuristic is a technique in cryptography to convert an interactive public-coin protocol (Sigma protocol) between a prover and a verifier into a one-message (non-interactive) protocol using a cryptographic hash function. A weak Fiat–Shamir transformation can be turned into a strong Fiat–Shamir transformation if the hashing function is applied to the commitment and shared statement as opposed to only the commitment. ([[18]][\[18\]], [[19]][\[19\]])
- <i><u>Discrete Logarithm/Discrete Logarithm Problem</u></i>:<a name="dlp"> </a>In the mathematics of the real numbers, the logarithm <code>log<i><sub>b</sub>a</i></code> is a number <code><i>x</i></code> such that <code><i>b<sup>x</sup>=a</i></code>, for given numbers <code><i>a</i></code> and <code><i>b</i></code>. Analogously, in any group  <code><i>G</i></code> , powers  <code><i>b<sup>k</sup></i></code> can be defined for all integers <code><i>k</i></code>, and the discrete logarithm <code>log<i><sub>b</sub>a</i></code> is an integer <code><i>k</i></code> such that <code><i>b<sup>k</sup>=a</i></code>. Algorithms in public-key cryptography base their security on the assumption that the discrete logarithm problem over carefully chosen cyclic finite groups and cyclic subgroups of elliptic curves over finite fields has no efficient solution. ([[17]][\[17\]], [[29]][\[29\]])

## <a name="h-References"> </a>References

[[1]][\[1\]] Bulletproofs: Short Proofs for Confidential Transactions and More, Bünz B. et al., http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf, Date accessed: 2018-09-18.

[\[1\]]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf "Bulletproofs: Short Proofs for Confidential Transactions and More, Bünz B. et al"

[[2]][\[2\]] Bullet Proofs (Transcript), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://diyhpl.us/wiki/transcripts/2018-02-02-andrew-poelstra-bulletproofs, Date accessed: 2018-09-10.

[\[2\]]: https://diyhpl.us/wiki/transcripts/2018-02-02-andrew-poelstra-bulletproofs "Bullet Proofs (Transcript), Bitcoin Milan Meetup 2018-02-02"

[[3]][\[3\]] Bullet Proofs (Slides), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://download.wpsoftware.net/bitcoin/2018-02-bp-slides/slides.pdf, Date accessed: 2018-09-10.

[\[3\]]: https://download.wpsoftware.net/bitcoin/2018-02-bp-slides/slides.pdf "Bullet Proofs (Slides), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra"

[[4]][\[4\]] Decoding zk-SNARKs, https://medium.com/wolverineblockchain/decoding-zk-snarks-85e73886a040, Date accessed: 2018-09-17.

[\[4\]]: https://medium.com/wolverineblockchain/decoding-zk-snarks-85e73886a040 "Decoding zk-SNARKs" 

[[5]][\[5\]] GitHub: ElementsProject/secp256k1-zkp, Experimental Fork of libsecp256k1 with Support for Pedersen Commitments and Range Proofs, Date accessed: 2018-09-18.

[\[5\]]: https://github.com/ElementsProject/secp256k1-zkp "GitHub: ElementsProject/secp256k1-zkp, Experimental Fork of libsecp256k1 with Support for Pedersen Commitments and Range Proofs"





[[10]][\[10\]] Bulletproofs presentation at Feb 2 Milan Meetup (Andrew Poelstra), Reddit, https://www.reddit.com/r/Bitcoin/comments/7w72pq/bulletproofs_presentation_at_feb_2_milan_meetup, Date accessed: 2018-09-10.

[\[10\]]: https://www.reddit.com/r/Bitcoin/comments/7w72pq/bulletproofs_presentation_at_feb_2_milan_meetup "Bulletproofs presentation at Feb 2 Milan Meetup (Andrew Poelstra), Reddit"

[[11]][\[11\]] Bulletproofs #273, https://github.com/mimblewimble/grin/issues/273, Date  accessed: 2018-09-10.

[\[11\]]: https://github.com/mimblewimble/grin/issues/273	"Bulletproofs #273"

[[12]][\[12\]] GitHub: adjoint-io/bulletproofs, Bulletproofs are Short Non-interactive Zero-knowledge Proofs that Require no Trusted Setup, https://github.com/adjoint-io/bulletproofs, Date accessed: 2018-09-10.

[\[12\]]: https://github.com/adjoint-io/bulletproofs "GitHub: adjoint-io/bulletproofs, Bulletproofs are Short Non-interactive Zero-knowledge Proofs that Require no Trusted Setup"

[[13]][\[13\]] GitHub: apoelstra/secp256k1-mw, Fork of libsecp-zkp `d78f12b` to Add Support for Mimblewimble Primitives, https://github.com/apoelstra/secp256k1-mw/tree/bulletproofs, Date accessed: 2018-09-18.

[\[13\]]: https://github.com/apoelstra/secp256k1-mw/tree/bulletproofs "GitHub: apoelstra/secp256k1-mw, Fork of libsecp-zkp `d78f12b` to Add Support for Mimblewimble Primitives"



[[15]][\[15\]] GitHub: mimblewimble/secp256k1-zkp, Fork of secp256k1-zkp for the Grin/MimbleWimble project, https://github.com/mimblewimble/secp256k1-zkp, Date accessed: 2018-09-18.

[\[15\]]: https://github.com/mimblewimble/secp256k1-zkp ""

[[16]][\[16\]]  Wikipedia: Zero-knowledge Proof,  https://en.wikipedia.org/wiki/Zero-knowledge_proof, Date accessed: 2018-09-18. 

[\[16\]]: https://en.wikipedia.org/wiki/Zero-knowledge_proof	"Wikipedia - Zero-knowledge Proof"

[[17]][\[17\]] Wikipedia: Discrete logarithm, https://en.wikipedia.org/wiki/Discrete_logarithm, Date accessed: 2018-09-20.

[\[17\]]: https://en.wikipedia.org/wiki/Discrete_logarithm "Wikipedia: Discrete logarithm"

[[18]][\[18\]] How to Prove Yourself: Practical Solutions to Identification and Signature Problems, Fiat A. et al., CRYPTO 1986: pp. 186-194, https://link.springer.com/content/pdf/10.1007%2F3-540-47721-7_12.pdf, Date accessed: 2018-09-??.

[\[18\]]: https://link.springer.com/content/pdf/10.1007%2F3-540-47721-7_12.pdf "How to Prove Yourself: Practical Solutions to Identification and Signature Problems, Fiat A. et al."

[[19]][\[19\]] How not to Prove Yourself: Pitfalls of the Fiat-Shamir Heuristic and Applications to Helios, https://link.springer.com/content/pdf/10.1007%2F978-3-642-34961-4_38.pdf, Date accessed: 2018-09-20.

[\[19\]]: https://link.springer.com/content/pdf/10.1007%2F978-3-642-34961-4_38.pdf "How not to Prove Yourself: Pitfalls of the Fiat-Shamir Heuristic and Applications to Helios"

[[20]][\[20\]] Mimblewimble Explained, https://www.weusecoins.com/mimble-wimble-andrew-poelstra/, Date accessed: 2018-09-10.

[\[20\]]: https://www.weusecoins.com/mimble-wimble-andrew-poelstra	"Mimblewimble Explained"

[[21]][\[21\]] Message hiding in Bulletproofs #721, https://github.com/mimblewimble/grin/issues/721, Date accessed: 2018-09-10.

[\[21\]]: https://github.com/mimblewimble/grin/issues/721	"Message hiding in Bulletproofs #721"

[[22]][\[22\]] , , Date accessed: 2018-09-??.

[\[22\]]:  ""

[[23]][\[23\]] , , Date accessed: 2018-09-??.

[\[23\]]:  ""

[[24]][\[24\]] , , Date accessed: 2018-09-??.

[\[24\]]:  ""

[[25]][\[25\]] , , Date accessed: 2018-09-??.

[\[25\]]:  ""

[[26]][\[26\]] , , Date accessed: 2018-09-??.

[\[26\]]:  ""

[[27]][\[27\]] , , Date accessed: 2018-09-??.

[\[27\]]:  ""

[[28]][\[28\]] , , Date accessed: 2018-09-??.

[\[28\]]:  ""

[[29]][\[29\]] Assumptions Related to Discrete Logarithms: Why Subtleties Make a Real Difference, Sadeghi A et al., http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf, Date accessed: 2018-09-??.

[\[29\]]: http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf "Assumptions Related to Discrete Logarithms: Why Subtleties Make a Real Difference, Sadeghi A et al."

[[30]][\[30\]] Elliptic Curve Cryptography: a gentle introduction, http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/, Date accessed: 2018-09-10.

[\[30\]]: http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction "Elliptic Curve Cryptography: a gentle introduction"

## <a name="h-Contributors"> </a>Contributors

- [https://github.com/???](https://github.com/??)
- 
