# Lightning Network for Dummies 

![Lightning](lightning.png)

---
## Setting the scene 

In order for Bitcoin, or any other cryptocurrency to hold its value- **It needs value proposition**

However, currently Bitcoin  is too slow and too expensive to be a viable method of payment for everyday use. 

Note: It needs value proposition. In saying that, as a cryptocurrency, it still possess the most important feature: decentralisation. For bitcoin to succeed, the benefits of using it to buy a cup of coffee needs to outweigh those fusing status-quo payments, like cash or credit. At this point in time, bitcoin’s benefits do not outweigh cash or credit. It’s slow and more importantly, too expensive. Although decentralisation is imperative- nobody wants to wait a minimum of 10 minutes to receive 2 confirmation and pay multi dollar fees just to buy a cup of coffee. That’s why solutions to this problem have been debated for the las several years. 

---

Solutions can be boiled down to two primary components: 
1. bigger blocks 
2. Off-chain scaling 

Note: On its face- increasing the block size seems like the logical solution. It’s been done before, so why arbitrarily limit it at 1MB. Satoshi Nakamoto- Bitcoin’s creator- stated that blocks should grow as big as they need to be; and implied that blocks should increase as they approach max capacity 

---

However, the argument is: 
* Big blocks increase the cost of running full-node 
* Big blocks will lead to a centralisation in mining 

Note: If scaling takes place now, Bitcoin risks technical issues and centralization

---

## Introducing Lightning
![Lightningnetworknodes](lightningnetworknodes.png)

---

From the whitepaper, in order to use the LN, a common-user needs to : 
1. Create the parent (funding transaction)
2. Create the children (commitment transitions and all spends from the commitment transactions)
3. Sign the children 
4. Exchange the signatures for the children 
5. Sign the parent 
6. Exchange the signatures for the parent 
7. Broadcast the parent on the blockchain 

Note: Essentially, you fund the network with a transaction on the Bitcoin Mainnet and commitment transactions re-shift the original balances. To sign the funding transaction, they need to exchange their parent signatures and broadcast them back on the mainnet. 

---


---

## Problematic Implications of the Lightning Network 

There are issues with the implentation of this type of network:

* It may not be particularly user friendly 

---





