# Atomic swaps
## What is Atomic swaps

Atomic swops or cross-chain atomic swops [[1]](https://coinsutra.com/atomic-swap/) in a nutshell are decentralised exchange's, but only for cryoto currencies. This allows two parties to exchange two different crypto currencies in a trustless environment. If one party defaults or fails the transaction the tranaction is not completed and neither party can "run off" with the other party's money.  From this we can deduce that we require two technologies: a payment channel and hashed timelock contracts. An implementation of a pyament channel is the lightning network. 



## Hashed Timelock Contracts 

Hashed Timelock Contracts (HTLC) [[2]](https://hackernoon.com/what-are-hashed-timelock-contracts-htlcs-application-in-lightning-network-payment-channels-14437eeb9345) is one of the most important technologies required for atomic swaps. This is a payment class that uses haslocks and timelocks to require certain public knowledge before doing a payment, otherwise the payment is reversed. HTLCs are also crucial in the lighting network [[3]](https://lightning.network/lightning-network-paper.pdf). 

Here is a quick example of how a HTLC works:

![alt](sources/Characters.png)

In this example Alex's wants to pay Carla, but he does not have an open payment channel to Carla. But he does have an open channel to Bart who does have an open channel to Carla. 

1. Carla generates a random number and gives the hash of the number to Alex. 
2. Alex pays Bart but adds the condition that if Bart wants to claim the payment he has to provide the random number that generated the hash Carlo gave to Alex. 
3. Bart pays Carlo, but he adds the same condition to the payment. 
4. Carla claims the payment by providing the random number, and thus exposing the random number to Bart. 
5. Bart uses the random number to claim the payment from Alex. 

If the payment to Carla does not go through the timelock in the contract will reverse all transactions. 

## Atomic vs Etomic

For an atomic swap transaction to happen, both currencies must use the same hashing function as this is crucial for HTLC to function. Etomic swaps was created in an attemp to make atomic swaps happend between bitcoin coins and ethereum based tokens. 


## Examples of current atomic swaps and implementations there of 
### #1 Manual method 
An article was posted on Hackernoon [[3]](https://hackernoon.com/so-how-do-i-really-do-an-atomic-swap-f797852c7639) showing the exact steps that is required for doing an atomic swap using cli. 

The requirements for this metod can be listed as follows:

- Full nodes on both party's.
- Atomic swop package [[4].
- Use of supported coins (UXTO based protocol coins, eg Bitcoin, Litecoin, Viacoin).
- Power user.



### #2 Atomic Wallet

Atomic wallet [[5]](https://atomicwallet.io/) is an atomic swop exchange. They allow two parties to trade with them as a third party.  The process looks as follows:

1. Party A select an order from the BitTorrent order book.
2. Party A enter an amount of coin to swap or coin to receive.
3. Party A confirm the swap.
4. Party B receives notification.
5. Party B confirms the swap.
6. First party and Second party’s Atomic Wallet checks the contracts.
7. Both receive their coins.

### #3 BarterDEX

BarterDEX is a decentrilized exhange created by Komodo [[6]](https://komodoplatform.com/decentralized-exchange/) but it works with electron servers or native. BarterDEX at its core is more like an auction system then a true decentrilized exchange. It also uses a security deposit in the form of zcredits to do swaps without waiting for confirmation.

BarterDEX also suports Etomic swaps. These work by keeping the payments locked in a etomic blockchain which will act as a third party. Although swaps have been done, it is stated as not yet production ready [[7]](https://github.com/artemii235/etomic-swap). Currently (July 2018) its only possible to use Barterdex out of the  cli [[8]]("https://github.com/KomodoPlatform/KomodoPlatform/wiki/Installing-and-Using-Komodo-Platform-(barterDEX)").  Barterdex charges a 0.1287% fee for a swop [[9]](https://github.com/KomodoPlatform/KomodoPlatform/wiki/barterDEX-Whitepaper-v2). 

## References

[1] Sudhir Khatwani (2018) *What Is Atomic Swap and Why It Matters?*, *Coinsutra*. Available at: https://coinsutra.com/atomic-swap/ (Accessed: 12 July 2018).

[2] Vohra, A. (2016) *What Are Hashed Timelock Contracts (HTLCs)? Application In Lightning Network & Payment Channels*, *Hackernoon*. Available at: https://hackernoon.com/what-are-hashed-timelock-contracts-htlcs-application-in-lightning-network-payment-channels-14437eeb9345 (Accessed: 12 July 2018).

[3] Poon, J. and Dryja, T. (2016) *The Bitcoin Lightning Network: Scalable Off-Chain Instant Payments v0.5.9.2*. Available at: https://lightning.network/lightning-network-paper.pdf.

[3] Hotshot (2018) *So how do I really do an atomic swap*, *Hackernoon*. Available at: https://hackernoon.com/so-how-do-i-really-do-an-atomic-swap-f797852c7639 (Accessed: 13 July 2018).

[4] open source (ISC) (2018) ‘viacoin/atomicswap’. github. Available at: https://github.com/viacoin/atomicswap.

[5] Atomic (2018) *Atomic wallet*. Available at: https://atomicwallet.io/ (Accessed: 13 July 2018).

[6] Komodo (2018) *BarterDEX*. Available at: https://komodoplatform.com/decentralized-exchange/ (Accessed: 13 July 2018).

[7] Artemii235 (2018) ‘etomic-swap’. github. Available at: https://github.com/artemii235/etomic-swap.

[8] Komodo (2018) ‘Barterdex’. github. Available at: https://github.com/KomodoPlatform/KomodoPlatform/wiki/Installing-and-Using-Komodo-Platform-(barterDEX).

[9] Komodo and Hossain, S. (2017) *barterDEX Whitepaper v2*. Available at: https://github.com/KomodoPlatform/KomodoPlatform/wiki/barterDEX-Whitepaper-v2.

## Contributors

- https://github.com/SWvheerden