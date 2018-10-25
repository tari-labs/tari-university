



## Grin vs. BEAM
### a comparison


@snap[west sidebar]
@div[s250px text-center]
![Gringots](https://github.com/tari-labs/tari-university/raw/grin-beam/src/protocols/grin-beam-comparison/sources/gringots.png)
@divend
@snapend


@snap[east sidebar]
@div[s250px text-center]
![BEAM](https://github.com/tari-labs/tari-university/raw/grin-beam/src/protocols/grin-beam-comparison/sources/beam.png)
@divend
@snapend

---
## Introduction

- Mimblewimble is a newly proposed blockchain architecture based on Pederson commitments.
    - Offers built-in privacy
    - Very compact blockchain   
- Grin's stated goal is to produce a minimalistic reference implementation of Mimblewimble.
- BEAM has however made many modifications to the original Mimblewimble approach to achieve new features.
- Some raw implementation differences:
<table>
  <tr>
    <th></th>
    <th>**Grin**</th>
    <th>**Beam**</th>
  </tr>
  <tr>
    <td>**Language**</td>
    <td>Rust</td>
    <td>C++</td>
  </tr>
  <tr>
    <td>**Database**</td>
    <td>LMDB</td>
    <td>SQLite</td>
  </tr>
  <tr>
    <td>**Mempool data structure**</td>
    <td>DAG</td>
    <td>Multiset key-value store</td>
  </tr>
</table>

---
## Dandelion Relay
One piece of functionality that both projects implement outside of Mimblewimble is the Dandelion relay mechanism.

- **Two phases:**
    - Stem phase (Anonymity phase): Randomly forwards the transaction one peer at a time for a random distance
    - Fluff phase (Spreading phase): Broadcast the transactions to the whole network

![Dandelion Relay](https://github.com/tari-labs/tari-university/raw/grin-beam/src/protocols/grin-beam-comparison/sources/dandelion-stem-fluff.png)
---
## Grin unique features
From a functional perspective Grin does not have many features that are not represented in BEAM.

One such feature that Grin has implemented is *Partial History* syncing.

---
## BEAM unique features
- stuff
+++
## things
