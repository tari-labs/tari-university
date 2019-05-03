# Mimblewimble Multiparty Bulletproof UTXO

- [Introduction](#introduction)
- [???](#???)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: ???](#appendix-a-???)
- [Contributors](#contributors)



## Introduction

In [Mimblewimble](../mimblewimble-1/MainReport.md) the concept of a Bitcoin type multi-signature (multisig) applied to an Unspent Transaction Output (UTXO) does not really exist. Bitcoin also uses Pay to Script Hash (P2SH) functionality as a means to send transactions to a P2SH payment address, which is comprised of a SHA-256 RIPEMD-160 hashed and Base58Check encoded redeem script. The redeem script sets the conditions that must be fulfilled in order for the UTXO to be spent. A typical Bitcoin P2SH multisig redeem transaction has the following form:

```Text
Pubkey script:     OP_HASH160 <Hash160(redeemScript)> OP_EQUAL
Redeem script:     <OP_2> <A pubkey> <B pubkey> <C pubkey> <OP_3> OP_CHECKMULTISIG
Signature script:  OP_0 <A sig> <C sig> <redeemScript>
```

The 1st line proves that a redeem script can be produced that when hashed with SHA-256 and then with RIPEMD-160 is equal to the hash linked to the multisig UTXO.



## ???

???



## Conclusions, Observations and Recommendations

- ???



## References

[[1]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More", Blockchain Protocol Analysis and Security Engineering 2018 [online]. Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2018&#8209;09&#8209;18.

[1]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf "Bulletproofs: Short Proofs for Confidential Transactions and 
More" 

[[2]] ??? "???" [online]. Available: https://blockgeeks.com/guides/bitcoin-script-guide-part-2/. Date accessed: 2019&#8209;05&#8209;02.




## Appendices

### Appendix A: ???

??? 



## Contributors

- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
