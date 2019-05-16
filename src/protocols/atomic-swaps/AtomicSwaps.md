# Atomic Swaps

- [What are Atomic Swaps?](#what-are-atomic-swaps)
- [Hashed Timelock Contracts](#hashed-timelock-contracts)
- [Atomic vs. Etomic](#atomic-vs-etomic)
- [Examples of Current Atomic Swaps and Implementations](#examples-of-current-atomic-swaps-and-implementations)
  - [#1 Manual Method](#a1-manual-method)
  - [#2 Atomic Wallet](#a2-atomic-wallet)
  - [#3 BarterDEX](#a3-barterdex)
- [References](#references)
- [Contributors](#contributors)



## What are Atomic Swaps?

Atomic swaps or cross-chain atomic swaps [[1]] in a nutshell are decentralized 
exchanges, but only for cryptocurrencies. This allows multiple parties to exchange two different crypto currencies in a 
trustless environment. If one party defaults or fails the transaction, neither party can "run off" with the anyone's 
money.  For this to work, we will require two technologies: a payment channel and hashed timelock contracts. An 
implementation of a payment channel is the lightning network. 



## Hashed Timelock Contracts 

Hashed Timelock Contracts (HTLC) [[2]] is one of the most important technologies required for atomic swaps. This is a 
payment class that uses hashlocks and timelocks to require certain public knowledge before doing a payment, otherwise 
the payment is reversed. HTLCs are also 
crucial in the lighting network [[3]]. 

Here is a quick example of how a HTLC works:

![alt](sources/Characters.png)

In this example Alex's wants to pay Carla, but he does not have an open payment channel to Carla. But he does have an 
open channel to Bart who does have an open channel to Carla. 

1. Carla generates a random number and gives the hash of the number to Alex. 
2. Alex pays Bart but adds the condition that if Bart wants to claim the payment he has to provide the random number 
that generated the hash Carlo gave to Alex. 
3. Bart pays Carlo, but he adds the same condition to the payment. 
4. Carla claims the payment by providing the random number, and thus exposing the random number to Bart. 
5. Bart uses the random number to claim the payment from Alex. 

If the payment to Carla does not go through the timelock in the contract will reverse all transactions. 

## Atomic vs. Etomic

For an atomic swap transaction to happen, both cryptocurrencies must use the same hashing function as this is crucial 
for HTLC to function. Etomic swaps was created in an attempt to make atomic swaps happen between Bitcoin tokens and 
Ethereum based tokens. 


## Examples of Current Atomic Swaps and Implementations
### #1 Manual Method 
An article was posted on Hackernoon [[4]] 
showing the exact steps that is required for doing an atomic swap using cli. 

The requirements for this method can be listed as follows:

- Full nodes on both parties.
- Atomic swap package [[5]].
- Use of supported coins (UXTO based protocol coins, eg Bitcoin, Litecoin, Viacoin).
- Power user.



### #2 Atomic Wallet

Atomic wallet [[6]] is an atomic swap exchange. They allow two parties to trade with them as 
a third party.  The process looks as follows:

1. Party A select an order from the BitTorrent order book.
2. Party A enter an amount of coin to swap or coin to receive.
3. Party A confirm the swap.
4. Party B receives notification.
5. Party B confirms the swap.
6. First party and Second party’s Atomic Wallet checks the contracts.
7. Both receive their coins.

### #3 BarterDEX

BarterDEX is a decentralized exchange created by Komodo [[7]] but 
it works with electron servers or native. BarterDEX at its core is more like an auction system then a true decentralized 
exchange. It also uses a security deposit in the form of Zcredits to do swaps without waiting for confirmation.

BarterDEX also supports Etomic swaps. These work by keeping the payments locked in a etomic blockchain which will act 
as a third party. Although swaps have been done, it is stated as not yet production ready [[8]], Currently (July 2018) 
its only possible to use Barterdex out of the cli [[9]]. Barterdex charges a 0.1287% fee for a swap [[10]]. 

## References

[[1]] S. Khatwani (2018), "What is Atomic Swap and Why it Matters?". *Coinsutra*. 
Available: <https://coinsutra.com/atomic-swap/>. Date accessed: 2018&#8209;07&#8209;12.

[1]: https://coinsutra.com/atomic-swap/
"What is Atomic Swap and Why it Matters?" 


[[2]] A. Vohra (2016), "What are Hashed Timelock Contracts (HTLCs)? Application in Lightning Network & Payment Channels", 
*Hackernoon*. Available: <https://hackernoon.com/what-are-hashed-timelock-contracts-htlcs-application-in-lightning-network-payment-channels-14437eeb9345>. 
Date accessed: 2018&#8209;07&#8209;12.


[2]: https://hackernoon.com/what-are-hashed-timelock-contracts-htlcs-application-in-lightning-network-payment-channels-14437eeb9345
"What are Hashed Timelock Contracts (HTLCs)? Application in Lightning Network & Payment Channels" 


[[3]] J. Poon and T. Dryja (2016), "The Bitcoin Lightning Network: Scalable Off-chain Instant Payments v0.5.9.2". 
Available: <https://lightning.network/lightning-network-paper.pdf>. Date accessed: 2018&#8209;07&#8209;13.

[3]: https://lightning.network/lightning-network-paper.pdf
"The Bitcoin Lightning Network: Scalable Off-chain Instant Payments v0.5.9.2" 


[[4]] Hotshot (2018), "So how do I really do an Atomic Swap", *Hackernoon*. 
Available: <https://hackernoon.com/so-how-do-i-really-do-an-atomic-swap-f797852c7639>. Date accessed: 2018&#8209;07&#8209;13.

[4]: https://hackernoon.com/so-how-do-i-really-do-an-atomic-swap-f797852c7639
"So how do I really do an Atomic Swap" 


[[5]] Open Source (ISC) (2018), "viacoin/atomicswap", GitHub [online]. Available: <https://github.com/viacoin/atomicswap>. 
Date accessed: 2018&#8209;07&#8209;13.

[5]: https://github.com/viacoin/atomicswap
"viacoin/atomicswap" 


[[6]] Atomic (2018), "Atomic Wallet" [online]. Available: <https://atomicwallet.io/>. Date accessed: 2018&#8209;07&#8209;13.

[6]: https://atomicwallet.io/
"Atomic Wallet" 


[[7]] Komodo (2018), "BarterDEX" [online]. Available: <https://komodoplatform.com/decentralized-exchange/>. 
Date accessed: 2018&#8209;07&#8209;13.

[7]: https://komodoplatform.com/decentralized-exchange/
"BarterDEX" 



[[8]] Artemii235 (2018), "etomic-swap", GitHub [online]. Available: <https://github.com/artemii235/etomic-swap>.
Date accessed: 2018&#8209;07&#8209;13.

[8]: https://github.com/artemii235/etomic-swap
"etomic-swap" 


[[9]] Komodo (2018), "Barterdex", GitHub [online]. Available:
<https://github.com/KomodoPlatform/KomodoPlatform/wiki/Installing-and-Using-Komodo-Platform-(barterDEX>. 
Date accessed: 2018&#8209;07&#8209;13.

[9]: https://github.com/KomodoPlatform/KomodoPlatform/wiki/Installing-and-Using-Komodo-Platform-(barterDEX
"Barterdex" 


[[10]] Komodo and S. Hossain (2017), "barterDEX Whitepaper v2" [online]. 
Available: <https://github.com/KomodoPlatform/KomodoPlatform/wiki/barterDEX-Whitepaper-v2>. Date accessed: 2018&#8209;07&#8209;13.

[10]: https://github.com/KomodoPlatform/KomodoPlatform/wiki/barterDEX-Whitepaper-v2
"barterDEX Whitepaper v2" 



## Contributors

- <https://github.com/SWvheerden>
- <https://github.com/anselld>
