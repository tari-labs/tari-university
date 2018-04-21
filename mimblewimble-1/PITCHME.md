# MimbleWimble
## A high-level overview

---
# Key features

* No addresses
* Completely private
* Compact blockchain

---

# Transactions
@fa[user green](Alice) sends @fa[user blue](Bob) 2 Tari:

* Bob chooses a secret, private key, `$k_1$` on the MW EC corresponding to a generator _H_.
* He derives the public key `$P_1 = k_1.H$` and sends `$P_1$` to Alice.
* Alice has a UTXO for 3T and knows the private key `$k_2$` that allows her to spend that coin.
* She creates another private key `$k_3` and then
* She structures a transaction using Bob's _public_ key `$P_1$`, sending 2 Tari to Bob and receiving 1 Tari in change (using `$k_3$`).
* She can the post the transaction to the blockchain where it will be verified and added to the blockchain, assuming everything checks out.
* When he's ready, Bob can spend his Tari using `$k_1` using this same general procedure

---

# Transactions explained

## Notice _no public addresses are used_!

The recipient must be available to create a transaction, but he needn't be online at the time of the transaction.

#### Email, instant-messaging, wallet apps, or carrier pigeon are all valid means of the recipient sending his public key to Alice.

+++

## Basic transaction

Here's the basic transaction:

`\begin{align}
  3 &= 2 + 1 + f\\
  \text(UTXO) &= \text(To Bob) + \text(change) + \text{tx fee}
  \end{align}
`

Assume the fee is zero for now; it doesn't change anything.

+++

Now multiply both sides by the generator _G_ on the EC:

$$ 3.G = 2.G + 1.G + f.G $$

Since _G_ is a constant, the math above still holds, so we can validate that Alice is not
stealing by checking that

$$(3.G) - (2.G) - (1.G) - (f.G) === 0$$

but note that we _only see public keys_ and thus the values are hidden!

Cool!

<small>@fa[comment] You might note that only scalar integer values are valid for
elliptic curve multiplication. In practice, we'll be using MinoTaris so that the
amounts are always integers.</small>

+++

## But wait...

But if _G_ is constant and known (it is), couldn't someone just pre-calculate `$n.G$`
for all reasonable values of _n_ and scan the blockchain for those public keys?

Basically, **YES!**, so we're not done yet.

+++

## Blinding factors

This is where Bob's private key `$k_1$` comes in. Each in- or output needs a second private key:

1. Bob's 2 Tari output corresponds to `$k_1$`, which only Bob knows (since he's sent the public key to Alice)
1. Alice knows the private key, `$k_2$` corresponding to the 3 Tari she is spending
1. Alice knows the private key `$k_3$` for the 1 Tari change she's sending herself.

+++

So what if we rewrite the inputs and outputs as follows:

$$ C_{ni} = n.G + k_i.H $$

this completely _blinds_ the in- and outputs so that no pre-image attack is possible.
This is called a _Pederson commitment_.

+++

But now the equation doesn't necessarily balance:


`\begin{align}
  (3.G + k_2.H) - (2.G + k_1.H) - (1.G + k_3.H) - f.G &= E \\
  \text{but}  \\
  3.G - 2.G - 1.G - f.G &= 0, \text{so} \\
  k_2.H - k_1.H - k_3.H &= E \\
\end{align}`

Given that
