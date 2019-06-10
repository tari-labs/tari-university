<head>
<style>
div.LineHeight20per {
  line-height: 20%;
}
div.LineHeight100per {
  line-height: 100%;
}
div.LineHeight200per {
  line-height: 200%;
}
div.mywrap {
  width: 95%; 
  word-wrap: break-word;
  background: #f1f1f1;
  font-size: 0.875em;
  font-family: "Source Code Pro", Consolas, "Ubuntu Mono", Menlo, "DejaVu Sans Mono", monospace, monospace;
  padding: 0.5em;
  color: #6e6b5e;
}
</style>
</head>


## Mimblewimble Multiparty Bulletproof UTXO

<div class="LineHeight200per"> <br></div>

- Introduction
- Review of Bitcoin $ m\text{-of-}n $ Multisig
- Security Aspects
- Mimblewimble $ n\text{-of-}n $ Multiparty Bulletproof (BP) UTXO
  - Multiparty Funding Transaction
  - Multiparty BP Range Proof
  - Spending the Multiparty UTXO
- Shamir's Secret Sharing Scheme (SSSS)
- Pedersen Verifiable Secret Sharing
- Mimblewimble $ m\text{-of-}n $ Multiparty BP UTXO
  - Secret Sharing for Multiple Rounds
  - How it Works
  - Spending Protocol
- Conclusions

<div class="LineHeight100per"> <br></div>

See full report [*here*](https://tlu.tarilabs.com/protocols/mimblewimble-mp-bp-utxo/MainReport.html).


---

## Introduction

<div class="LineHeight20per"> <br></div>

@div[text-left]

Mimblewimble does not have a Bitcoin-type multisig applied to a UTXO. In Bitcoin, multisig payments are usually combined 
with P2SH to send funds to a P2SH payment address. The redeem script sets the conditions for the UTXOs linked to the 
P2SH payment address to be spent.

<div class="LineHeight20per"> <br></div>

Mimblewimble transactions do not involve payment addresses. The UTXO can be spent if the Pedersen Commitment can be 
opened/unlocked; does not require an "owner" signature. A typical Mimblewimble UTXO looks like this:

<div class="mywrap">08c15e94ddea81e6a0a31ed558ef5e0574e5369c4fcba92808fe992fbff68884cc</div>

<div class="LineHeight20per"> <br></div>

Also for Mimblewimble all senders receivers must interact to conclude a transaction.

@divend

+++

@div[text-left]

???

@divend

---

## Review of Bitcoin $ m\text{-of-}n $ Multisig

<div class="LineHeight20per"> <br></div>

@div[text-left]

???

@divend

+++

@div[text-left]

???

@divend

---

## Security Aspects

<div class="LineHeight20per"> <br></div>

@div[text-left]

???

@divend

+++

@div[text-left]

???

@divend

---

## Mimblewimble $ n\text{-of-}n $ Multiparty BP UTXO

<div class="LineHeight20per"> <br></div>

@div[text-left]

???

@divend

+++

@div[text-left]

???

@divend

---

## Shamir's Secret Sharing Scheme

<div class="LineHeight20per"> <br></div>

@div[text-left]

???

@divend

+++

@div[text-left]

???

@divend

---

## Pedersen Verifiable Secret Sharing

<div class="LineHeight20per"> <br></div>

@div[text-left]

???

@divend

+++

@div[text-left]

???

@divend

---

## Mimblewimble $ m\text{-of-}n $ Multiparty BP UTXO

<div class="LineHeight20per"> <br></div>

@div[text-left]

???

@divend

+++

@div[text-left]

???

@divend

---

## Conclusions

- ???

+++

- ???
