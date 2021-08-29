------
theme: default
paginate: true
footer: © Tari Labs, 2018-2021. (License : CC BY-NC-SA 4.0)
_class: lead
backgroundColor: #fff
------
<style>
section {
  font-size: 1.5em;
}
</style>

# Mimblewimble
## A high-level overview

----
# Key features

* No addresses
* Completely private
* Compact blockchain

----

# Transactions

![mw_txs](/images/mimblewimble-1/mw_txs.png)

----

# Transactions explained

## Notice _no public addresses are used_!

The recipient must be available to create a transaction, but he needn't be online at the time of the transaction.

#### Email, instant-messaging, wallet apps, or carrier pigeon are all valid means of the recipient sending his public key to Alice.

-----

## Basic transaction

Here's the basic transaction:

| Inputs | Outputs |||
|----------|------------|--|--|
| 3      | 2 | 1 | f |
| Alice's UTXO | To Bob | Change | fee |

_Assume the fee is zero for now; it doesn't change anything._

-----

Now multiply both sides by the generator _G_ on the EC:

$$ 3.G = 2.G + 1.G + f.G $$

Since _G_ is a constant, the math above still holds, so we can validate that Alice is not
stealing by checking that

$$(3.G) - (2.G) - (1.G) - (f.G) \equiv 0$$

but note that we _only see public keys_ and thus the values are hidden!

Cool!

**Note:** Technically only scalar integer values are valid for elliptic curve multiplication. In practice,
we'll be using MinoTaris (name TBD) so that the amounts are always integers.
The transactions aren't sent as an equation like this, but as a list of inputs, outputs and the fee. The fee
is in cleartext, but you still need to blind it to check the maths.

-----

## But wait...

But if $G$ is constant and known (it is), couldn't someone just pre-calculate $n.G$
for all reasonable values of $n$ and scan the blockchain for those public keys?

Basically, **YES!**, so we're not done yet.

-----

## Blinding factors

We need to add randomness to each in/output to prevent a pre-image attack. The basic idea is to do this by adding a second private key to each output.

_Input were given their private keys when they were outputs in a previous transaction._

So what if we rewrite the inputs and outputs as follows:

$$ C_{ni} = n.G + k_i.H $$

where $H$ is a generator on another (specially selected) EC.

This completely _blinds_ the in- and outputs so that no pre-image attack is possible.
(And is called a _Pedersen commitment_).

-----

Alice now builds a transaction like this:

$$
   \underbrace{(3.G + k_2.H)}_{\text{3T UTXO}} - \underbrace{(2.G + k_1.H)}_{\text{2T to Bob}} - \underbrace{(1.G + k_3.H)}_{\text{1T change}}   = \underbrace{f.G}_{\text{fee}}
$$

Since in an honest transaction`*`

$$3.G - 2.G - 1.G - f.G = E = 0$$

this becomes

$$ k_2.H - k_1.H - k_3.H = E = 0 $$

`*` We'll examine the case where someone is cheating and $E \ne 0$ shortly.

-----

and so we require

$$
  k_2 - k_1 - k_3 = 0 \\
  \therefore k_1 = k_2 - k_3
$$

Alice sends Bob this transaction information (T1) and lets Bob know that his private
key must be $k_1$. To prevent Alice from spending Bob's newly earned Tari, he
can choose a new blinding factor $r_1$ and rewrites the transaction as

$$
     (3.G + k_2.H) - (2.G + (k_1 - r_1).H) - (1.G + k_3.H) - f.G
     = r_1.H + E
$$

Notice that the RHS is $r_1.H + E$ where $E$ is the sum of the transaction values.
The RHS is a valid key on $H$ if and only if $E=0$*`, i.e. Alice has constructed
the transaction without cheating.

More generally, $k_1$ is the sum of all the blinding factors.

`*` This is a consequence of how the curves $H$ and $G$ were selected initially.

-----

## Where we are so far:

* Alice knows all the values in the transaction.
* Bob only knows the shared private key $k_1$, his blinding factor, $r_1$ and subsequently the amount he's receiving.
* Bob can validate that the transaction amounts sum to zero, but doesn't know the other values.
* Alice knows the private key, $k_1$, but not $r_1$, so can't spend Bob's Tari, since she can't derive the
  private key sum from this information.

-----

### Proof of ownership

Bob signs an empty string with $r_1$ which proves that he knows $r_1$.This
signature is submitted for inclusion to the blockchain along with the transaction details.

A validator / node then only has to check that

* $E$ is a valid public key on $H$.
* That the signature Bob provides corresponds to the public key $E$

-----

## One last thing

There's still a flaw in this procedure though.

Alice could have spent a 1 Tari output, giving 2 to Bob, and -1 in change to herself,
thus creating 1 Tari out of thin air.

-----

### Range proofs

To prevent, this, Alice needs to provide a set of _range proofs_ for each amount
she receives, proving that the (masked) values lie between 0 and $2^{64}$ (exclusive).

Similarly, Bob provides range proofs for the value he is receiving.

The details of these proofs are beyond the scope of this presentation. But some comments:

* To complete the range proofs you need the private keys, hence Bob derives the RP for his blinded value.
* Range proofs add considerable overhead to the transaction.

----

# Transaction summary
A MimbleWimble transaction includes the following:

* From Alice, a set of inputs, that reference and spend a set of previous outputs.
* From either / both, a set of new outputs that include:
  * A value and a blinding factor (which is just a new private key)
  * A range proof that shows that the value is non-negative.
* An explicit transaction fee, in clear.
* A signature, using the excess blinding value and using it as a private key.
