# Mimblewimble Implementation 

- [Background](#background)
- [Learning Path Matrix](#learning-path-matrix)

## Background

Mimblewimble is a blockchain protocol that focuses on privacy through the implementation of confidential transactions. 
It enables a greatly simplified blockchain in which all spent transactions can be pruned, resulting in a much smaller 
blockchain footprint and efficient base node validation. The blockchain consists only of block-headers, remaining Unspent 
Transaction Outputs (UTXO) with their range proofs and an unprunable transaction kernel per transaction.

## Learning Path Matrix 

| Topics                                           |                             Type                             |
| ------------------------------------------------ | :----------------------------------------------------------: |
| Mimblewimble                                     | <div class="wrap_beg">[presentation](/protocols/mimblewimble-1/sources/PITCHME.link.md)</div> |
| Introduction to Schnorr Signatures               | <div class="wrap_int">[report](/cryptography/digital_signatures/introduction_schnorr_signatures.md)</div> |
| Introduction to Scriptless Scripts               | <div class="wrap_int">[report](/cryptography/scriptless-scripts/introduction-to-scriptless-scripts.md)</div> |
| Mimblewimble-Grin Block Chain Protocol Overview  | <div class="wrap_int">[report](/protocols/grin-protocol-overview/MainReport.md)</div> |
| Grin vs. BEAM; a Comparison                      | <div class="wrap_int">[report](/protocols/grin-beam-comparison/MainReport.md)</div> |
| Grin Design Choice Criticisms - Truth or Fiction | <div class="wrap_int">[report](/protocols/grin-design-choice-criticisms/MainReport.md)</div> |
| Mimblewimble Transactions Explained              | <div class="wrap_int">[report](/protocols/mimblewimble-1/MainReport.md)</div> |
| Mimblewimble Multiparty Bulletproof UTXO         | <div class="wrap_adv">[report](/protocols/mimblewimble-mp-bp-utxo/MainReport.md)</div> |
