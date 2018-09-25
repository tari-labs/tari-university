# <a name="h-Bulletproofs-and-Mimblewimble"> </a>Bulletproofs and Mimblewimble

## <a name="h-Introduction"> </a>Introduction

Bulletproofs form part of the family of distinct Zero-knowledge<sup>[def][zk~]</sup> proof systems, like Zero-Knowledge Succinct Non-Interactive ARguments of Knowledge (zk-SNARK), Succinct Transparent ARgument of Knowledge (STARK) and Zero Knowledge Prover and Verifier for Boolean Circuits (ZKBoo). Zero-knowledge proofs are designed so that a prover is able to indirectly verify that a statement is true without having to provide any information beyond the verification of the statement, for example to prove that a number is found that solves a cryptographic puzzle and fits the hash value without having to reveal the nonce. ([[2]][\[2\]], [[4]][\[4\]])

Bulletproofs is a Non-interactive Zero-knowledge (NIZK) proof protocol for general Arithmetic Circuits<sup>[def][ac~]</sup> with very short proofs (Arguments of Knowledge Systems<sup>[def][afs~]</sup>) and without requiring a Trusted Setup<sup>[def][ts~]</sup>. They rely on the Discrete Logarithmic<sup>[def][dlp~]</sup> assumption and are made non-interactive using the Fiat-Shamir Heuristic<sup>[def][fsh~]</sup>. The name 'Bulletproof' originated from a non-technical summary from one of the original authors of the scheme's properties: "<i>Short like a bullet with bulletproof security assumptions</i>". ([[1]][\[1\]], [[29]][\[29\]])

Bulletproofs also implements a Multi-party Computation (MCP) protocol whereby distributed proofs of multiple provers with secret committed values are aggregated into a single proof before the Fiat-Shamir
challenge is calculated and sent to the verifier. Secret committed values will stay secret.



Add short history here

The essence of Bulletproofs is its inner-product algorithm originally presented by Groth [[13]][\[13\]] and then further refined by Bootle et al. [[12]][\[12\]]. The algorithm provides an argument of knowledge (proof) of two binding vector Pedersen Commitments<sup>[def][pc~]</sup> that satisfy a given inner product relation, which is of independent interest. Bulletproofs builds on these techniques, which yield communication efficient zero-knowledge proofs, but offer a further replacement for the inner product argument that reduces overall communication by a factor of 3. ([[1]][\[1\]], [[29]][\[29\]])



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

[[25]][\[25\]]

[[26]][\[26\]]

[[27]][\[27\]]

[[28]][\[28\]]

[[29]][\[29\]]

[[30]][\[30\]]

### <a name="h-Wallet-Reconstruction-Grin"> </a>Wallet Reconstruction - Grin

See  [[35]][\[35\]]

"{**yeastplume** } Single commit bullet proofs appear to be working, which is all we need. The only think I think we're missing here from being able to use this implementation is the ability to store an amount within the rangeproof (for wallet reconstruction). From conversations with @apoelstra earlier, I believe it's possible to store 64 bytes worth of 'message' (not nearly as much as the current range proofs). We also need to be aware that we can't rely as much on the message hiding properties of range proofs when switching to bullet proofs."

- "{**yeastplume** } @apoelstra the amount, and quite possibly the switch commitment hash as well (or just a hash of the entire output) as per #207..."


"{**apoelstra**} Ok, I can get you 64 bytes without much trouble (xoring them into `tau_1` and `alpha` which are easy to extract from `tau_x` and `mu` if you know the original seed used to produce the randomness). I think it's possible to get another 32 bytes into `t` but that's way more involved since `t` is a big inner product." 

???

## <a name="h-Negatives"> </a>Negatives

- A discrete-log attacker (*e.g. a bad actor employing a quantum computer*) would be able to exploit Bulletproofs to silently inflate any currency that used them. Bulletproofs are perfectly hiding (*i.e. confidential*), but only computationally binding (*i.e. not quantum resistant*). ([[1]][\[1\]], [[5]][\[5\]] and [[10]][\[10\]])
- 

## <a name="h-Conclusions,-Observations,-Recommendations"> </a>Conclusions, Observations, Recommendations

- Bünz B. et al [[1]][\[1\]] proposed that for Bulletproofs all Pedersen commitments be replaced with ElGamal Commitments<sup>[def][pc~]</sup> in order to make the scheme quantum secure. The ElGamal scheme can be improved slightly if the same <code><i>g<sup>r</sup></i></code> is used in multiple range proofs. In order to retain the hiding property, a different <code><i>h</i></code> must be used for every proof.
- 

## <a name="h-Definition-of-Terms"> </a>Definition of Terms

- <u><i>Arithmetic Circuits</i></u>:<a name="ac"> </a>An arithmetic circuit over a field and variables <code><i>(a1, ..., an)</i></code> is a directed acyclic graph whose vertices are called gates. Arithmetic circuits can alternatively be described as a list of multiplication gates with a collection of linear consistency equations relating the inputs and outputs of the gates. [[29]][\[29\]]

[ac~]: #ac
"An arithmetic circuit over a field 
and variables (a1, ..., an) is a 
directed acyclic graph ..."

- <u><i>Argument of Knowledge System</i></u>:<a name="afs"> </a>Proof systems with computational soundness like Bulletproofs are sometimes called argument systems. [[29]][\[29\]]

[afs~]: #afs
"Proof systems with computational 
soundness like Bulletproofs are 
sometimes called argument systems."

- <i><u>Discrete Logarithm/Discrete Logarithm Problem</u></i>:<a name="dlp"> </a>In the mathematics of the real numbers, the logarithm <code>log<i><sub>b</sub>a</i></code> is a number <code><i>x</i></code> such that <code><i>b<sup>x</sup>=a</i></code>, for given numbers <code><i>a</i></code> and <code><i>b</i></code>. Analogously, in any group  <code><i>G</i></code> , powers  <code><i>b<sup>k</sup></i></code> can be defined for all integers <code><i>k</i></code>, and the discrete logarithm <code>log<i><sub>b</sub>a</i></code> is an integer <code><i>k</i></code> such that <code><i>b<sup>k</sup>=a</i></code>. Algorithms in public-key cryptography base their security on the assumption that the discrete logarithm problem over carefully chosen cyclic finite groups and cyclic subgroups of elliptic curves over finite fields has no efficient solution. ([[17]][\[17\]], [[40]][\[40\]])

[dlp~]: #dlp
"In the mathematics of the real 
numbers, the logarithm log_b(a) 
is a number x such that ..."

- <u><i>ElGamal Commitment</i></u>:<a name="egc"> </a>An ElGamal commitment is a Pedersen Commitment<sup>[def][pc~]</sup> with an additional commitment <code><i>g<sup>r</sup></i></code> to the randomness used. [[1]][\[1\]]

[egc~]: #egc
"An ElGamal Commitment ..."

- <u><i>Fiat–Shamir Heuristic/Transformation</i></u>:<a name="fsh"> </a>The Fiat–Shamir heuristic is a technique in cryptography to convert an interactive public-coin protocol (Sigma protocol) between a prover and a verifier into a one-message (non-interactive) protocol using a cryptographic hash function.  ([[18]][\[18\]], [[19]][\[19\]])
  - The prover will use a <code><i>Prove()</i></code> algorithm to calculate a commitment <code><i>A</i></code> with a statement <code><i>Y</i></code> that is shared with the verifier and a secret witness value <code><i>w</i></code> as inputs. The commitment <code><i>A</i></code> is then hashed to obtain the challenge <code><i>c</i></code>, which is further processed with the <code><i>Prove()</i></code> algorithm to calculate the response <code><i>f</i></code>. The single message sent to the verifier then contains the challenge <code><i>c</i></code> and response <code><i>f</i></code>.
  - The verifier is then able to compute the commitment <code><i>A</i></code> from the shared statement <code><i>Y</i></code>, challenge <code><i>c</i></code> and response <code><i>f</i></code>. The verifier will then use a <code><i>Verify()</i></code> algorithm to verify the combination of shared statement <code><i>Y</i></code>, commitment <code><i>A</i></code>, challenge <code><i>c</i></code> and response <code><i>f</i></code>.
  - A weak Fiat–Shamir transformation can be turned into a strong Fiat–Shamir transformation if the hashing function is applied to the commitment <code><i>A</i></code> and shared statement <code><i>Y</i></code> as opposed to only the commitment <code><i>A</i></code>.

[fsh~]: #fsh
"The Fiat–Shamir heuristic is a 
technique in cryptography to 
convert an interactive ..."

- <u><i>Pedersen Commitment</i></u>:<a name="pc"> </a>A commitment scheme is a cryptographic primitive that allows one to commit to a chosen value (or chosen statement) while keeping it hidden to others, with the ability to reveal the committed value later. ([[1]][\[1\]], [[15]][\[15\]], [[22]][\[22\]])
  - A Pedersen Commitment scheme allows a sender to create a commitment to a secret value and consists of three algorithms: 
    - <code><i>Setup()</i></code> to set up the commitment parameters;
    - <code><i>Commit()</i></code> to commit to the message using the commitment parameters;
    - <code><i>Open()</i></code> to open and verify the commitment.
  - A Pedersen Commitment scheme has the following properties:
    - Hiding: A dishonest party cannot discover the honest party's value;
    - Binding: A dishonest party cannot open their commitment in more than one way;
    - Non-correlation: A dishonest party cannot commit to a value that is in some significant way correlated to the honest party's value.
  - An efficient implementation of the Pedersen Commitment will use secure Elliptic Curve Cryptography (ECC), which is based on the algebraic structure of elliptic curves over finite (prime) fields.

[pc~]: #pc
"A commitment scheme is a cryptographic
primitive that allows one to commit to a ...
A Pedersen Commitment scheme allows ..."

- <u><i>Trusted Setup</i></u>:<a name="ts"> </a>???

[ts~]: #ts "???"

- <u><i>Zero-knowledge Proof/Protocol</i></u>:<a name="zk"> </a>In cryptography, a zero-knowledge proof/protocol is a method by which one party (the prover Peggy) can prove to another party (the verifier Victor) that she knows a value `w`, without conveying any information apart from the fact that she knows the value `w`. [[16]][\[16\]]

[zk~]: #zk
"In cryptography, a zero-knowledge 
proof/protocol is a method by which 
one party (the prover Peggy) ..."

## <a name="h-References"> </a>References

[[1]][\[1\]] Bulletproofs: Short Proofs for Confidential Transactions and More, Blockchain Protocol Analysis and Security Engineering 2018, Bünz B. et al., http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf, Date accessed: 2018-09-18.

[\[1\]]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf 
"Bulletproofs: Short Proofs for Confidential Transactions and More, 
Blockchain Protocol Analysis and Security Engineering 2018, 
Bünz B. et al"

[[2]][\[2\]] Bullet Proofs (Transcript), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://diyhpl.us/wiki/transcripts/2018-02-02-andrew-poelstra-bulletproofs, Date accessed: 2018-09-10.

[\[2\]]: https://diyhpl.us/wiki/transcripts/2018-02-02-andrew-poelstra-bulletproofs 
"Bullet Proofs (Transcript), 
Bitcoin Milan Meetup 2018-02-02, 
Andrew Poelstra"

[[3]][\[3\]] Bullet Proofs (Slides), Bitcoin Milan Meetup 2018-02-02, Andrew Poelstra, https://download.wpsoftware.net/bitcoin/2018-02-bp-slides/slides.pdf, Date accessed: 2018-09-10.

[\[3\]]: https://download.wpsoftware.net/bitcoin/2018-02-bp-slides/slides.pdf 
"Bullet Proofs (Slides), 
Bitcoin Milan Meetup 2018-02-02, 
Andrew Poelstra"

[[4]][\[4\]] Decoding zk-SNARKs, https://medium.com/wolverineblockchain/decoding-zk-snarks-85e73886a040, Date accessed: 2018-09-17.

[\[4\]]: https://medium.com/wolverineblockchain/decoding-zk-snarks-85e73886a040 
"Decoding zk-SNARKs" 

[[5]][\[5\]] Bulletproofs: Short Proofs for Confidential Transactions and More (Slides), Bünz B. et al., https://cyber.stanford.edu/sites/default/files/bpase18.pptx, Date accessed: 2018-09-18.

[\[5\]]: https://cyber.stanford.edu/sites/default/files/bpase18.pptx 
"Bulletproofs: Short Proofs for Confidential Transactions and More (Slides), 
Blockchain Protocol Analysis and Security Engineering 2018, 
Bünz B. et al"

[[5]][\[5\]] Bulletproofs: Short Proofs for Confidential Transactions and More (Transcripts), Bünz B. et al., http://diyhpl.us/wiki/transcripts/blockchain-protocol-analysis-security-engineering/2018/bulletproofs, Date accessed: 2018-09-18.

[\[5\]]: http://diyhpl.us/wiki/transcripts/blockchain-protocol-analysis-security-engineering/2018/bulletproofs 
"Bulletproofs: Short Proofs for Confidential Transactions and More (Transcripts), 
Blockchain Protocol Analysis and Security Engineering 2018, 
Bünz B. et al"

[[10]][\[10\]] Bulletproofs presentation at Feb 2 Milan Meetup (Andrew Poelstra), Reddit, https://www.reddit.com/r/Bitcoin/comments/7w72pq/bulletproofs_presentation_at_feb_2_milan_meetup, Date accessed: 2018-09-10.

[\[10\]]: https://www.reddit.com/r/Bitcoin/comments/7w72pq/bulletproofs_presentation_at_feb_2_milan_meetup 
"Bulletproofs presentation at Feb 2 Milan 
Meetup (Andrew Poelstra), Reddit"

[[12]][\[12\]] Efficient zero-knowledge arguments for arithmetic circuits in the discrete log setting, Bootle J et al., Annual International Conference on the Theory and Applications of Cryptographic Techniques, pages 327-357. Springer, 2016., https://eprint.iacr.org/2016/263.pdf, Date accessed: 2018-09-??.

[\[12\]]: https://eprint.iacr.org/2016/263.pdf "Efficient zero-knowledge arguments for arithmetic circuits in the discrete log setting, Bootle J et al."

[[13]][\[13\]] Linear Algebra with Sub-linear Zero-Knowledge Arguments, Groth J., https://link.springer.com/content/pdf/10.1007%2F978-3-642-03356-8_12.pdf, Date accessed: 2018-09-21.

[\[13\]]: https://link.springer.com/content/pdf/10.1007%2F978-3-642-03356-8_12.pdf 
"Linear Algebra with Sub-linear Zero-Knowledge 
Arguments, Groth J."

[[15]][\[15\]] Confidential  Assets, Poelstra A. et al., Blockstream, https://blockstream.com/bitcoin17-final41.pdf, Date accessed: 2018-09-25.

[\[15\]]: https://blockstream.com/bitcoin17-final41.pdf 
"Confidential  Assets,
Poelstra A. et al.,
Blockstream"

[[16]][\[16\]]  Wikipedia: Zero-knowledge Proof,  https://en.wikipedia.org/wiki/Zero-knowledge_proof, Date accessed: 2018-09-18. 

[\[16\]]: https://en.wikipedia.org/wiki/Zero-knowledge_proof	
"Wikipedia - Zero-knowledge Proof"

[[17]][\[17\]] Wikipedia: Discrete logarithm, https://en.wikipedia.org/wiki/Discrete_logarithm, Date accessed: 2018-09-20.

[\[17\]]: https://en.wikipedia.org/wiki/Discrete_logarithm 
"Wikipedia: Discrete logarithm"

[[18]][\[18\]] How to Prove Yourself: Practical Solutions to Identification and Signature Problems, Fiat A. et al., CRYPTO 1986: pp. 186-194, https://link.springer.com/content/pdf/10.1007%2F3-540-47721-7_12.pdf, Date accessed: 2018-09-??.

[\[18\]]: https://link.springer.com/content/pdf/10.1007%2F3-540-47721-7_12.pdf 
"How to Prove Yourself: Practical Solutions to 
Identification and Signature Problems, 
Fiat A. et al."

[[19]][\[19\]] How not to Prove Yourself: Pitfalls of the Fiat-Shamir Heuristic and Applications to Helios, Bernhard D. et al., https://link.springer.com/content/pdf/10.1007%2F978-3-642-34961-4_38.pdf, Date accessed: 2018-09-20.

[\[19\]]: https://link.springer.com/content/pdf/10.1007%2F978-3-642-34961-4_38.pdf 
"How not to Prove Yourself: Pitfalls of the 
Fiat-Shamir Heuristic and Applications to Helios, 
Bernhard D. et al."

[[20]][\[20\]] Mimblewimble Explained, https://www.weusecoins.com/mimble-wimble-andrew-poelstra/, Date accessed: 2018-09-10.

[\[20\]]: https://www.weusecoins.com/mimble-wimble-andrew-poelstra	
"Mimblewimble Explained"

[[21]][\[21\]] Message hiding in Bulletproofs #721, https://github.com/mimblewimble/grin/issues/721, Date accessed: 2018-09-10.

[\[21\]]: https://github.com/mimblewimble/grin/issues/721	
"Message hiding in Bulletproofs #721"

[[22]][\[22\]] pedersen-commitment: An implementation of Pedersen commitment schemes, https://hackage.haskell.org/package/pedersen-commitment, Date accessed: 2018-09-25.

[\[22\]]: https://hackage.haskell.org/package/pedersen-commitment 
"pedersen-commitment: An implementation
of Pedersen commitment schemes"

[[23]][\[23\]] , , Date accessed: 2018-09-??.

[\[23\]]:  
""

[[24]][\[24\]] , , Date accessed: 2018-09-??.

[\[24\]]:  
""

[[25]][\[25\]] GitHub: ElementsProject/secp256k1-zkp, Experimental Fork of libsecp256k1 with Support for Pedersen Commitments and Range Proofs, Date accessed: 2018-09-18.

[\[25\]]: https://github.com/ElementsProject/secp256k1-zkp 
"GitHub: ElementsProject/secp256k1-zkp, Experimental 
Fork of libsecp256k1 with Support for Pedersen 
Commitments and Range Proofs"

[[26]][\[26\]] GitHub: apoelstra/secp256k1-mw, Fork of libsecp-zkp `d78f12b` to Add Support for Mimblewimble Primitives, https://github.com/apoelstra/secp256k1-mw/tree/bulletproofs, Date accessed: 2018-09-18.

[\[26\]]: https://github.com/apoelstra/secp256k1-mw/tree/bulletproofs 
"GitHub: apoelstra/secp256k1-mw, Fork of libsecp-zkp 
`d78f12b` to Add Support for Mimblewimble Primitives"

[[27]][\[27\]] GitHub: bbuenz/BulletProofLib, Library for generating non-interactive zero knowledge proofs without trusted setup (Bulletproofs), https://github.com/bbuenz/BulletProofLib, Date accessed: 2018-09-??.

[\[27\]]: https://github.com/bbuenz/BulletProofLib 
"GitHub: bbuenz/BulletProofLib, Library for generating 
non-interactive zero knowledge proofs without trusted 
setup (Bulletproofs)"

[[28]][\[28\]] GitHub: dalek-cryptography/bulletproofs, A pure-Rust implementation of Bulletproofs using Ristretto, https://github.com/dalek-cryptography/bulletproofs, Date accessed: 2018-09-??.

[\[28\]]: https://github.com/dalek-cryptography/bulletproofs 
"GitHub: dalek-cryptography/bulletproofs, A pure-Rust 
implementation of Bulletproofs using Ristretto"

[[29]][\[29\]] GitHub: adjoint-io/bulletproofs, Bulletproofs are Short Non-interactive Zero-knowledge Proofs that Require no Trusted Setup, https://github.com/adjoint-io/bulletproofs, Date accessed: 2018-09-10.

[\[29\]]: https://github.com/adjoint-io/bulletproofs 
"GitHub: adjoint-io/bulletproofs, Bulletproofs are Short
Non-interactive Zero-knowledge Proofs that Require 
no Trusted Setup"

[[30]][\[30\]] GitHub: mimblewimble/secp256k1-zkp, Fork of secp256k1-zkp for the Grin/MimbleWimble project, https://github.com/mimblewimble/secp256k1-zkp, Date accessed: 2018-09-18.

[\[30\]]: https://github.com/mimblewimble/secp256k1-zkp 
"GitHub: mimblewimble/secp256k1-zkp, Fork of secp256k1-zkp 
for the Grin/MimbleWimble project"

[[35]][\[35\]] GitHub: mimblewimble/grin, Bulletproofs #273, https://github.com/mimblewimble/grin/issues/273, Date  accessed: 2018-09-10.

[\[35\]]: https://github.com/mimblewimble/grin/issues/273	
"GitHub: mimblewimble/grin, Bulletproofs #273"

[[40]][\[40\]] Assumptions Related to Discrete Logarithms: Why Subtleties Make a Real Difference, Sadeghi A et al., http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf, Date accessed: 2018-09-??.

[\[40\]]: http://www.semper.org/sirene/publ/SaSt_01.dh-et-al.long.pdf 
"Assumptions Related to Discrete Logarithms: 
Why Subtleties Make a Real Difference, 
Sadeghi A et al."

[[50]][\[50\]] Elliptic Curve Cryptography: a gentle introduction, http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/, Date accessed: 2018-09-10.

[\[50\]]: http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction 
"Elliptic Curve Cryptography: a gentle introduction"

[[?]][\[?\]] , , Date accessed: 2018-09-??.

[\[?\]]:  
""

## <a name="h-Contributors"> </a>Contributors

- [https://github.com/hansieodendaal](https://github.com/hansieodendaal)
- ???
