---
layout: post
title:  Understanding TariScript
author: Cayle Sharrock <CjS77>
date:   2020-03-01 15:00:00 +0300
image:  '/images/banner-05.jpg'
category: Tari
tags:   [TariScript, Smart contracts, Blockchain]
featured: false
level: Intermediate
excerpttext: TariScript brings Bitcoin's flexibility into Mimblewimble.
---

# How it all started

{: .note .info}
If you're looking for the dry, cut-to-the-bone technical specification for TariScript, then [we've got
your back](https://rfc.tari.com/RFC-0201_TariScript.html).

Tari is based on an implementation of the Mimblewimble protocol. Mimblewimble is much more scalable and private than,
say, Bitcoin.

One doesn't get this scalability and privacy for free, I'm afraid. One of the consequences of how Mimblewimble works is
that transactions are _interactive_. Both parties in a transactions need to be involved in the construction of a valid
Mimblewimble transaction.

This is a bit of a bummer if your counterparty is on holiday in Ibiza and you want to send him a payment before your tax
returns are due. It also means that "standard" things like tip jars are _não fácil_ in standard Mimblewimble.

Some of the other things we want to be able to do in Tari that are hard (though not impossible in some cases, thanks to
[scriptless scripts](/cryptography/introduction-to-scriptless-scripts) include:

* n-of-m multisig
* Payment channels
* Atomic swaps
* Cross-chain atomic swaps
* Side chain peg-in and peg-out transactions
* Digital asset "cold storage"

... and many other things we haven't thought of yet.

Fortunately, we think that TariScript is the tool that let's us have cake and eat it. It brings the power and
flexibility of Bitcoin script to Mimblewimble, while having a minor effect on privacy and scalability.

# TariScript - the basic idea

## A quick recap of Mimblewimble transactions

In standard Mimblewimble, to spend a UTXO, one needs knowledge of

* The spending key of the UTXO commitment,
* The value of the UTXO commitment.

You use this knowledge to construct a valid transaction, which contains

* Range proofs for new UTXOs, proving the values are non-negative AND knowledge of the spend key and value,
* A signature on the kernel, proving knowledge of the transaction excess.

Mimblewimble transactions are _interactive_, because the transaction contains information (the spending keys)
that must remain secret to each party.

So how can we implement something like one-sided payments in Mimblewimble?

This is where TariScript comes in.

## How TariScript works - the 50,000ft view

What if we changed things slightly and added some extra UTXO-spending rules?

Let's say that, in addition to knowing the spending key and commitment value, we also require you to prove possession of
_another_ private key, which we'll call the _script key_.

The twist is that we don't know what the script public key is until a script attached to the UTXO has executed [^1].

[^1]: If this sounds a lot like Bitcoin, you'd be right. This is an idea taken straight from Bitcoin itself.

Think of this as having a door with two locks. Standard Mimblewimble just has one lock. TariScript introduces a second
lock and you need to unlock both to open the door and spend the UTXO.

This lets us do some pretty cool things.

If we lock the first but leave the second lock open, then we have our standard Mimblewimble transaction. In TariScript,
you can leave the lock open by attaching an empty script to a UTXO.

If we leave the first lock open, or if multiple people have keys to the first lock, then we can provide a script that
only unlocks the second lock for a single key. This is Tari in Bitcoin "emulation mode".

And of course, we could also lock both locks to develop some very interesting spend conditions and smart contracts.


## What is all this scripting stuff about?

When you "send" bitcoin to an address, say `1J7mdg5rbQyUHENYdx39WVWK7fsLpEoXZy`. You're not actually sending it anywhere.
This is a convenient metaphor for those of us who grew up in a time when cheques were literally posted to their recipients.

In fact, what you are doing is locking up a UTXO with a _script_ that effectively says, "Anyone who can prove that
the know the private key of the public key that when hashed gives you `1J7mdg5rbQyUHENYdx39WVWK7fsLpEoXZy`, can have this bitcoin".

[Chapter 6 of _Mastering Bitcoin_](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch06.asciidoc#tx_script)
dives into Bitcoin transactions and script in great detail, and I strongly recommend you read it.

It is this ability to assign funds to small pieces of code, rather than just an account number, that opens up
so many potential uses for the world of programmable money.


# The devil in the details

While the idea of TariScript seems pretty simple, getting to place where we could implement it securely took the Tari
community over a year of iterating, refining, breaking, fixing and reviewing before we got to a place where we felt
comfortable implementing the first version in code.

In this section, we'll examine the TariScript scheme in detail. By the end of it, you should have a decent understanding
of how TariScript actually gets the job done and why it was such a long journey to get to this point.

{: .note .warning }
Math ahead. This section also builds on the concepts we discussed in [Mimblewimble - all the bits](??) and
[Mimblewimble transactions](/protocols/mimblewimble-transactions-explained).


## The TariScript language

TariScript is inspired by, and is based on Bitcoin script. Like its older cousin, TariScript is a
[stack-based language](https://en.wikipedia.org/wiki/Stack-oriented_programming). It has very strict limits on size,
and the instruction set is very limited.

This is for a very good reason.

If you've followed this space at all, you're no doubt jaded by the endless stream of headlines marking the various
[hacks](https://therecord.media/hacker-steals-600-million-from-poly-network-in-biggest-cryptocurrency-hack-ever/),
[leaks](https://www.gemini.com/cryptopedia/the-dao-hack-makerdao), and
[oopsies](https://github.com/openethereum/parity-ethereum/issues/6995) stemming largely from the fact that smart contracting
platforms like Ethereum and Polygon offer [Turing-complete](https://en.wikipedia.org/wiki/Turing_completeness)
scripting functionality.

tl;dr The more you let users do with your programmable money, the more bugs and vulnerabilities they will expose.

TariScript on the other hand tries to provide as much rope as we need to enable all of teh thingz, without letting you
hang yourself. Specifically, we want to keep TariScript small enough that it is amenable to
[formal verification](https://en.wikipedia.org/wiki/Formal_verification). At the very least, we're leaving things open
for TariScript scripts to be formally specified using methodology akin to [Miniscript](http://bitcoin.sipa.be/miniscript/).

This also supports Tari's philosophy of keeping the base layer compact, while pushing large-scale contracts and digital
assets platforms onto Layer 2. This is how Tari intends to scale to millions of users and digital assets without having
the entire network come to a grinding halt every time a kitty breeds or an ape gets bored.

### An example

{: .note .info }
For a full list of Opcodes available in TariScript, see [RFC-0202](https://rfc.tari.com/RFC-0202_TariScriptOpcodes.html).

Let's dive in with the one-sided payments example. The approach to one-sided payments is that Alice sends Tari to Bob
using a shared secret in the UTXO commitment and a script locked to Bob's private key. She does this without Bob
getting involved at any point.

This means that both Alice and Bob have a key to unlock the first lock in the door. This situation is not ideal for Bob.

But Bob is mollified by the fact that only he has the key to the second lock, so ultimately, only he can spend these funds.

Let's take a closer look at how the script resolution works.

When Alice creates the transaction, she looks up Bob's public key (`P_b`) and hashes it to `H_b`.
She attaches the following script to the UTXO:

```
  Dup HashBlake256  PushHash(H_b) EqualVerify
```

Alice signs the script along with some other housekeeping chores and broadcasts the transaction.

Later, when Bob comes online, his wallet scans the blockchain for UTXOs that have the script matching his public key
hash.

Now to spend this UTXO, Bob needs the keys for both locks. He gets the key for the first (Mimblewimble) lock using
Diffie-Hellmann key exchange with some public data that Alice included in the transaction.

For the second key, he must make it so that the script executes and returns his public key as the result.

When TariScript is validated, the script is run against an _input stack_ that will be provided by Bob. In this case,
Bob must provide a single element, his public key (`P_b`) as the script input.

Before the first instruction is executed, the stack and script looks like this:

| Step   | 0                                           |
|:-------|:--------------------------------------------|
| Script | Dup HashBlake256  PushHash(H_b) EqualVerify |
| Stack  | `P_b`                                       |

The script then runs, executing the instructions in order. If any instruction fails, the entire script fails. When the
script completes, there must be a single item left on the stack, and that item must be a valid public key.

The first instruction is `Dup`, which takes the top item on the stack and duplicates it:

| Step   | 1 (`Dup`)                               |
|:-------|:----------------------------------------|
| Script | HashBlake256  PushHash(H_b) EqualVerify |
| Stack  | `P_b P_b`                               |

The next instruction, `HashBlake256` hashes the top stack item and pushes the result onto the stack. We'll keep the same
nomenclature that Alice used and note that the hash of `P_b` is `H_b`:

| Step   | 2 (`HashBlake256`)        |
|:-------|:--------------------------|
| Script | PushHash(H_b) EqualVerify |
| Stack  | `H_b P_b`                 |

Next we have `PushHash(H_b)`. As you might guess, the pushes the value `H_b` onto the stack:

| Step   | 3(`PushHash(H_b)`) |
|:-------|:-------------------|
| Script | EqualVerify        |
| Stack  | `H_b H_b P_b`      |

Finally, the last instruction, `EqualVerify` removes and then compares the top two stack items. If they're equal, the
script execution continues. If not, the script fails.

Assuming the hashes match, i.e. Bob provided a public key that matched the hash Alice provided, then the script completes
successfully leaving Bob's public key on the stack.

| Step   | 4 - Complete |
|:-------|:-------------|
| Script |              |
| Stack  | `P_b`        |

The rules of TariScript say that the key for the second lock is the private key corresponding to `P_b`, which only Bob
knows.

In this way, we've achieved one-sided payments in Tari. Incidentally, the script Alice used here is nearly identical to
the most common Bitcoin script called "Pay to public-key hash", or P2PKH. With TariScript, as with Bitcoin, there are
many other ways Alice could have used to lock the UTXO for Bob's exclusive use.

"Pay to public key" (P2PK) is the simplest and would have been `PushPublicKey(P_b)`. Bob would not need to provide any
input at all here, because the stack would already have his public key after the single instruction completes.

[RFC-0201](https://rfc.tari.com/RFC-0201_TariScript.html) has several other more interesting examples that you might
want to go explore.

## Closing loopholes

So far, we've added a script to the UTXO and a consensus rule that requires that the result of the script is a public
key for which the spender holds the private key.

You might not be surprised to know that these requirements aren't enough to prevent shenanigans in Tari transactions.

Here are just some of the vulnerabilities present in the scheme at present:

* Anyone can spend Bob's funds by editing the script.
* A
* In interactive payments, Alice can lock Bob out of his funds by changing the script.
* Miners can edit with other parts of the transaction metadata without detection.
* Miners can take Bob's funds by making the script disappear altogether.
* If Bob later sends funds to Charlie, Alice can collude with Charlie to steal funds from Bob.
* If there's a really, really deep chain re-organisation, some nodes will have no way of knowing whether scripts were
  executed correctly and will have to trust whatever archival nodes tell them.

It took the Tari community almost a year to identify and resolve these vulnaribilities, so let's take some time to have
a look at them in more detail.

The first three issues can be grouped together because they have the same underlying cause: **malleability**.

Put yourself in the shoes of a Tari node for a second. With our current naïve approach to TariScript, there's no way for
the node to know whether the script has been modified by anyone. A malicious intermediate node, or a miner could have
swapped out `PushPublicKey(P_b)` for `PushPublicKey(P_c)` and nobody would be any the wiser.

So the first thing we have to do is have Alice _commit_ to the extra data we're adding to the transaction. For that
matter, we can ask her to commit to some transaction metadata as well, which would close a malleability bug that's been
in Tari for some time. Two birds, one stone. Nice!

We capture Alice's commitment by have her sign a message using a special private key that only she knows.

We call this key the _sender offset_ key. The sender offset key has another important use that we'll share shortly, but for now it provides the all
the proof we need that the data we see attached to the UTXO is the same data that Alice put there:

* the script,
* the output features,
* her sender offset _public_ key
* the UTXO commitment

Alice attaches this signature to the UTXO.

