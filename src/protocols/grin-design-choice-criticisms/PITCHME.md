# Grin Design Choice Criticisms - Truth or Fiction

---
# Introduction

What is Grin?
-  Cryptocurrency implemented in Rust
-  Mimblewimble transactions
-  Proof-of-Work (PoW): Cuckatoo algorithm

Main design goals?
- Privacy
- Transaction scaling
- Design simplicity 

---

# Introduction (cnt'd)

Grin received criticisms on some design and implementation decisions:
- Emission scheme
- PoW algorithm,
- Key-store library
- Cryptographic curve

This report investigated if there are some truth to these concerns
Suggestions will be presented to addressed these issues

---

# Monetary Policy Due to Static Emission Scheme

Bitcoin - limited and finite supply of coins
- 21 million coins
- 10-minute block times
-  First block reward: 50 BTC
-  Reward halved every 4 years

What hapens to newly minted coins?
 - New coins and transaction are paid to miners
 - Incentive to maintain the block chain
 
---

## What happens when all coins have been mined?

Only transaction fees will be paid to miners
Will transaction fees be sufficient to maintain a large network of miners?
Option 1: Non-capped supply
- Fees will be insufficient
- Only large mining farms remain profitable
- Centralisation of the network

Option 2: Fixed supply
- Fees will be sufficient
- Over time transaction fees will increase
- Mining hardware costs decrease
- Maintaining block chain remain lucrative for small mining operations

---

## Grin's Emmision scheme

- Number of coins not capped at a fixed supply.
- Static emission rate: constant 60 Grin is released as a reward for solving every block
- block goal of 60 seconds
- Result in 1 coin being created every second.

Grins motivations for static emission rate: 
- No upper limit on coins amount
- Inflation will tend to zero over time
- Mitigate effect of lost coins
- Encourage spending rather than holding

---

## Topic 3 ???

???

+++

## Topic 3 ??? (cnt'd)

???

---

## Conclusions

???

???

Sample side by side image (set at 250 pixels width, aligned center) and text (aligned left) inside div containers. [HTML tags](https://html.com/tags/) must be used for lists and formatting here.

@div[left-50 s250px text-center]
![My Sample Image](https://raw.githubusercontent.com/tari-labs/tari-university/grin/protocols/grin-design-choice-criticisms/sources/sample.PNG)
@divend

@div[right-50 text-left]

<p>[Unordered list](https://html.com/tags/ul/) of items below:

<ul>
<li>???
<li>???
</ul>
@divend

Sample side by side text inside div containers, with alternating alignment.  [HTML tags](https://html.com/tags/) must be used for lists and formatting here.

@div[left-50]

@div[text-left]

<p>[Paragraph left](https://html.com/tags/p/)
<p>Paragraph left


@divend

@div[text-center]

<ul>
<li>[Unordered list center](https://html.com/tags/ul/)
<li>Unordered list center
</ul>


@divend

@div[text-right]

<ol>
<li>[Ordered list right](https://html.com/tags/ol/)
<li>Ordered list right
</ol>


@divend

@divend



@div[right-50 text-right]

@div[text-left]

<p>Paragraph left
<p>Paragraph left


@divend

@div[text-center]

<ul>
<li>Unordered list center
<li>Unordered list center
</ul>


@divend

@div[text-right]

<ol>
<li>Ordered list right
<li>Ordered list right
</ol>


@divend

@divend

+++

## Conclusions (cnt'd)

???
