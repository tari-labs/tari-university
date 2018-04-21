# MimbleWimble
## A high-level overview

---
# Key features

* No addresses
* Completely private
* Compact blockchain

---

# MW transactions (1)
@fa[user green](Alice) @fa[long-arrow-alt-right](2 Tari) @fa[user blue](Bob):

* Bob chooses a secret, private key, `$k_1$`.
* He derives a public key $$P_1 = k.G$$ and sends `$P_1$` to Alice.
* Alice has a UTXO of 3 and knows the private key `$k_2$` that allows her to spend that coin,
  so she can structure a transaction sending 2 Tari to Bob and receiving 1 Tari in change.

+++

# MW transactions (2)

Basic transaction:

$$ 3 = 2 + 1 $$

Masked by EC cryptography on curve H:

$$ 3.H = 2.H + 1.H $$

(UTXO = To Bob + change)

But hang on, _G_ is constant and known, so couldn't someone just pre-calculate _n.G_
for all reasonable values of coin and scan the blockchain for those public keys?

Short answer: YES!

---

# Blinding factors

This is where Bob's private key `$k_1$` comes in. Each in- or output needs a second private key:

1. Bob's 2 Tari output corresponds to `$k_1$`, which only Bob knows (but he's sent the public key to Alice)
1. Alice knows the private key, `$k_2$` corresponding to the 3 Tari she is spending
1. Alice needs to create another private key `$k_3$` for the 1 Tari change she's creating.

So now we can write

$$ (3.H + k_2.G) = (2.H + k_1.G) + (1.H + k_3.G)
