# Mimblewimble Multiparty Bulletproof UTXO

- [Introduction](#introduction)
- [Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell](#bitcoin--mtext-of-n--multisig-in-a-nutshell)
- [Conclusions, Observations and Recommendations](#conclusions-observations-and-recommendations)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: ???](#appendix-a-???)
- [Contributors](#contributors)



## Introduction

In [Mimblewimble](../mimblewimble-1/MainReport.md) the concept of a Bitcoin type multi-signature (multisig) applied to an Unspent Transaction Output (UTXO) does not really exist. 

In Bitcoin multisig payments are usually combined with Pay to Script Hash (P2SH) functionality as a means to send funds to a P2SH payment address and then to manage its expenditure from there. The redeem script itself sets the conditions that must be fulfilled in order for the UTXOs linked to the P2SH payment address to be spent [[[1]], [[2]]). 

Unlike Bitcoin, Mimblewimble transactions do not involve payment addresses as all transactions are confidential. The only requirement for a Mimblewimble UTXO to be spent is the ability to open the Pederson Commitment and does not require an "owner" signature.

## Bitcoin $ m\text{-of-}n $ Multisig in a Nutshell

Multiple use cases of $ m\text{-of-}n $ multisig applications exist, for example a $ 1\text{-of-}2 $ petty cash account, or a $ 2\text{-of-}2 $ two-factor authentication wallet, or a $ 2\text{-of-}3 ​$ board of directors account, etc [[3]]. A typical 2-of-3 Bitcoin P2SH multisig redeem script (where any 2 of the 3 predefined public keys must sign the transaction) has the following form:

```Text
redeemScript     = <OP_2> <A pubkey> <B pubkey> <C pubkey> <OP_3> OP_CHECKMULTISIG
```

The P2SH payment address is the result of the redeem script double hashed with SHA-256 and RIPEMD-160 and then Base58Check encoded with a prefix of `0x05`:

```Text
redeemScriptHash = RIPEMD160(SHA256(redeemScript))
P2SHAddress      = base58check.Encode("05", redeemScriptHash)
```

Multiple payments can now be sent to the P2SH payment address. A generic funding transaction's output script for the P2SH payment address has the following form, irrespective of the redeem script's contents,

```Text
pubkeyScript      =     OP_HASH160 <redeemScriptHash> OP_EQUAL
```


with `OP_HASH160` being the combination of SHA-256 and RIPEMD-160. The 2-of-3 multisig redeem transaction's input script would the have the following form

```Text
sigScript         =  OP_0 <A sig> <C sig> <redeemScript>
```

and the combined spending and funding transaction script (validation script) would be

```Text
validationScript    = OP_0 <A sig> <C sig> <redeemScript> OP_HASH160 <redeemScriptHash> OP_EQUAL
```

When `validationScript` is executed all values are added to the execution stack in sequence. When opcode `OP_HASH160` is encountered the preceding value `<redeemScript>` is hashed and added to the stack and when opcode `OP_EQUAL` is encountered the previous two values, hash of the `<redeemScript>` and `<redeemScriptHash>`, are compared and removed from the stack if equal. The top of the stack then contains `<redeemScript>`, which is evaluated with the two entries on top of that, `<A sig>` and `<C sig>`. The last value in the stack, `OP_0`, is needed for a glitch in the `OP_CHECKMULTISIG` opcode implementation which makes it pop one more item than are available on the stack.

*What is signed?*

Partial signatures are created in the same sequence as the public keys are defined in `redeemScript`. A simplified concatenated hexadecimal version of the transaction - consisting of the input transaction ID and UTXO index, amount to be paid, `pubkeyScript` and transaction locktime - is signed. Each consecutive partial signature includes a concatenation of the previous partial signature with the simplified transaction data to be signed, creating multiple cross-references in the signed data. This, combined with the public keys, proves the transaction was created by the real owners of the bitcoins in question ([[4]], [[5]], [[6]]).

*How is change redirected to the multisig UTXO?*

See Bitcoin Stack Exchange question [here](https://bitcoin.stackexchange.com/q/87517/95260).

## ???

???



## Conclusions, Observations and Recommendations

- ???



## References

[[1]] "The Best Step-by-Step Bitcoin Script Guide Part 2" [online]. Available: https://blockgeeks.com/guides/bitcoin-script-guide-part-2. Date accessed: 2019&#8209;05&#8209;02.

[1]: https://blockgeeks.com/guides/bitcoin-script-guide-part-2
"The Best Step-by-Step Bitcoin Script Guide Part 2"

[[2]] "Script" [online]. Available: https://en.bitcoin.it/wiki/Script. Date accessed: 2019&#8209;05&#8209;06.

[2]: https://en.bitcoin.it/wiki/Script
"Script"

[[3]] "Multisignature" [online]. Available: https://en.bitcoin.it/wiki/Multisignature. Date accessed: 2019&#8209;05&#8209;06.

[3]: https://en.bitcoin.it/wiki/Multisignature
"Multisignature"

[[4]] "Transaction" [online]. Available: https://en.bitcoin.it/wiki/Transaction. Date accessed: 2019&#8209;05&#8209;06.

[4]: https://en.bitcoin.it/wiki/Transaction
"Transaction"

[[5]] S. Pour, "Bitcoin multisig the hard way: Understanding raw P2SH multisig transactions" [online]. Available: https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions. Date accessed: 2019&#8209;05&#8209;06.

[5]: https://www.soroushjp.com/2014/12/20/bitcoin-multisig-the-hard-way-understanding-raw-multisignature-bitcoin-transactions
"Bitcoin multisig the hard way: 
Understanding raw P2SH multisig transactions"

[[6]] "GitHub: gavinandresen/TwoOfThree.sh" [online]. Available: https://gist.github.com/gavinandresen/3966071. Date accessed: 2019&#8209;05&#8209;06.

[6]: https://gist.github.com/gavinandresen/3966071
"GitHub: gavinandresen/TwoOfThree.sh"




[[?]] B. Bünz, J. Bootle, D. Boneh, A. Poelstra, P. Wuille and G. Maxwell, "Bulletproofs: Short Proofs for Confidential Transactions and More", Blockchain Protocol Analysis and Security Engineering 2018 [online]. Available: <http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf>. Date accessed: 2018&#8209;09&#8209;18.

[?]: http://web.stanford.edu/~buenz/pubs/bulletproofs.pdf "Bulletproofs: Short Proofs for Confidential Transactions and 
More" 





## Appendices

### Appendix A: ???

??? 



## Contributors

- <https://github.com/hansieodendaal>
- <https://github.com/anselld>
