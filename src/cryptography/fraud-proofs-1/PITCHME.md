# Fraud Proofs and SPV (lightweight) clients - easier said than done?

- Background

- Full node vs SPV client

- What are fraud proofs?

- Fraud proof data structure

- Universal fraud proofs (suggested improvement)

- How SPV clients work

- Security and privacy issues with SPV clients

- Other suggested fraud proof improvements

- Conclusions, Observations, Recommendations

---

# Background
SPV clients will believe anything 

@div[s450px]
![SPV client lied to](https://raw.githubusercontent.com/tari-labs/tari-university/fraudproofs/src/cryptography/fraud-proofs-1/sources/todd-btc-spv.jpg)
@divend

+++

# Full node vs SPV client

<ul>A full Bitcoin node contains the following details:
  <li> every block
  <li> every transaction that has ever been sent
  <li> all the unspent transaction outputs (UTXOs)
</ul>

<ul>An SPV client, however, contains:
  <li> a block header with transaction data relative to the client including other transactiosn required to compute the Merkle root
or 
  <li> just a block header with no transactions.
</ul>


---

# What are fraud proofs?

<ul>
<li> alerts for SPV
<li> improve scalability
<li> improve security of SPV/lightweight clients
</ul>


## Fraud proof data structure
The following fraud proofs would require changes to the Bitcoin protocol itself

## Invalid transaction due to stateless criteria violation (correct syntax, input scripts conditions satisfied,etc)
<ul>For an invalid transaction, the fraud proofs consists of:
<li> the header of invalid block
<li> the invalid transaction
<li> An invalid block's Merkle tree containing the minimum number of nodes needed to proof the existance of the invalid transaction in the tree
</ul>

## Invalid transaction due to incorrect generation output value
<ul>For this case, the fraud proof consists of:
<li> the block itself
</ul>

## Invalid transaction due to input already been spent
<ul>For this case, the fraud proof would consist of the following:
<li> the header of the invalid block
<li> the invalid transaction
<li>  proof that the invalid transaction is within the invalid block
<li>  the header of the block containing original spend transaction
<li>  the original spending transaction
<li>  proof showing that the spend transaction is within the header block of the spend transaction
</ul>

## Invalid transaction due to incorrect generation output value
For this case, the fraud proof consists of:
<li> the block itself

## Invalid transaction if input does not exist
For this case, the fraud proof consists of:
<li> the entire blockchain



## Universal fraud proofs (suggested improvement)



???

---

## Topic 3 ???

???

+++

## Topic 3 ??? (cnt'd)

???

---

## Conclusions

???

+++

## Conclusions (cnt'd)

???
