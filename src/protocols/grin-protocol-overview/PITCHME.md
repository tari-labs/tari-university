# Mimblewimble-Grin Block Chain Protocol Overview

- Mimblewimble protocol overview

- Trustless transactions

- Contracts

- Atomic swaps

- Conclusions

+++
## Mimblewimble protocol overview - commitments

- Confiditial transactions

  $ r \cdot G + v \cdot H $	

- An example transaction can be expressed as input = output + change. 


$ (r_i \cdot G + v_i \cdot H) = (r_c \cdot G + v_c \cdot H) + (r_c \cdot G + v_c + \cdot H) $ 



+++
## Mimblewimble protocol overview - Cut-through and pruning 

MimbleWimble removes all spent outputs in the memorial pool and old blocks. 

$ output - inputs = kernel_-excess +(part \mspace{3mu} of)kernel_- offset $ 



```
 I1(x1)    +---> O1
           +---> O2

 I2(x2,O2) +---> O3

 I3(O3)    +---> O4
           +---> O5
```



```
 I1(x1)    +---> O1
 I2(x2)    +---> O4
           +---> O5
```



------
+++
## Mimblewimble protocol overview - Cut-through and pruning (cont'd)

Assuming 10 million transactions with 100&nbsp;000 unspent outputs the ledger will be roughly 130GB,

- 128GB of transaction data (inputs and outputs).
- 1 GB of transaction proof data.
- 250MB of block headers.

The total storage requirements can be reduced if cut-through and pruning is applied, the ledger will shrink to approximately 1.8GB and will result in the following:

- 1 GB of transaction proof data.
- UTXO size of 520MB.
- 250MB of block headers.

---
+++
## Mimblewimble protocol overview - Blocks

1. Transaction outputs, which include for each output:
   1. A Pedersen commitment (33 bytes).
   2. A range proof (over 5KB at this time).
2. Transaction inputs, which are just output references (32 bytes).
3. Transaction fees, in clear text
4. Transaction "proofs", which include for each transaction:
   1. The excess commitment sum for the transaction (33 bytes).
   2. A signature generated with the excess (71 bytes average).
5. A block header that includes Merkle trees and proof of work (about 250 bytes).

+++
## Trustless transactions

1. Alice selects her inputs and her change. The sum of all blinding factors (change output minus inputs) is $ r_s $.

2. Alice picks a random nonce ks and sends her partial transaction, $ k_s\cdot G $ and $ r_s\cdot G $ to Bob.

3. Bob picks his own random nonce $ k_r $ and the blinding factor for his output $ r_r $. Using $ r_r $ Bob adds his output to the transaction.

4. Bob computes the message $ M= fee \Vert lock_-height $, 

   the Schnorr challenge $ e = SHA256(M \Vert K_r \cdot G + K_s\cdot  G \Vert r_r\cdot G + r_s\cdot G) $ 

   and finally his side of the signature $ s_r = k_r + e\cdot G $ 

5. Bob sends: $ s_r $ and $ k_r\cdot G $ and $ r_r\cdot G $  to Alice.

6. Alice computes $ e $ just like Bob did and can check that $ s_r\cdot G = k_r\cdot G + e\cdot r_r \cdot G $ 

7. Alice sends her side of the signature $ s_s = k_s + e\cdot r_s $  to Bob.

8. Bob validates $ s_s\cdot G $  just like Alice did for $ s_r\cdot G $ in step 5 and can produce the final signature $ s = s_s + s_r , k_s\cdot G + k_s\cdot G$ as well as the final transaction kernel including $ s $ and the public key $ r_r\cdot G + r_s\cdot G$

---
+++
## Contracts - time-locked

Absolute - $ M = fee \Vert h $

Relative - $ M = fee \Vert h \Vert c $

+++
## Contracts - Multisig

1. Bob picks a blinding factor $ r_b $ and sends $ r_b\cdot G $ to Alice.
2. Alice picks a blinding factor $ r_a $  and builds the commitment $ C= r_a\cdot G + r_b\cdot G + v\cdot H $, she sends the commitment to Bob.
3. Bob creates a range proof for $ v $  using $ C $  and $ r_b $  and sends it to Alice.
4. Alice generates her own range proof, aggregates it with Bob, finalizing the multiparty output $ O_{ab} $ 
5. The kernel is built following the same procedure as used with Trustless Transactions.

---
+++
## Atomic swaps

1. Alice will send her Grin to a multiparty timelock contract with a refund time $ T_a < T_b $
2. Alice picks a random nonce $ k_s $  and her blinding sum $ r_s $ and sends $ k_s\cdot G $ and $ r_s\cdot G $ to Bob.
3. Bob picks a random blinding factor $ r_r $ and a random nonce $ k_r $. However, this time, instead of simply sending $ s_r = k_r + e\cdot r_r $  with his $ r_r\cdot G $ and $ k_r\cdot G $, Bob sends $ s_r' = k_r + x + e\cdot r_r $ as well as $ x\cdot G $ 
4. Alice can validate that $ s_r'\cdot G = k_r\cdot G + x\cdot G + r_r\cdot G $. She can also check that Bob has money locked with $ x\cdot G $ on the other chain.
5. Alice sends back her $ s_s = k_s + e\cdot x_s $ as she normally would, now that she can also compute $ e = SHA256(M \Vert k_s\cdot G+k_r\cdot G) $
6. To complete the signature, Bob computes $ s_r = k_r + e\cdot r_r $ and the final signature is $ (s_r + s_s, k_r\cdot G + k_s\cdot G) $ 
7. As soon as Bob broadcasts the final transaction to get his Grin, Alice can compute $ s_r' - s_r $ to get $ x $.

+++
## Questions 

?