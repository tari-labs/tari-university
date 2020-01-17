
# Probability of a Byzantine Takeover of the Digital Assets Network

---

## Introduction 

The investigation attempts to answer the following question:

*What is the percentage chance of controlling the majority of nodes in a random sample with varying quantities of the total 
number of nodes, committee size, bad nodes and BFT threshold?*

Note:

This investigation aims to provide answers to questions posed about the workings of the Tari Digital Assets Network (DAN) 
environment. It covers probabilistic attack vector with regard to the total nodes, compromised nodes, committee size and 
Byzantine Fault-tolerance (BFT) threshold. 

---

## Notation Used  

General notation of statistical expressions.  

- Let $N$ be the total number of nodes in the network. 
- Let $n$ be the committee size.
- Let $m$ be the number of bad actors.
- Let $T$ be the Byzantine Fault Tolerance (BFT) threshold.

There is a pool with $N$ nodes. The pool contains $m$ malicious nodes or bad actors. From within the pool, a random 
selection of nodes, $n$ is drawn. From this selection, the probability of drawing a threshold of bad actors, $T$, needs to 
be calculated.  

---

## Statistical Calculation

A single probability from the overarching question was used as a means to derive the formulae, as shown in the following 
example. 

**Example**: What is the probability of selecting a majority of bad nodes from a total of $5​$ nodes if the committee 
size is $3​$? There are $3​$ bad nodes $(B1, B2, B3)​$ and $2​$ good nodes $(G1, G2)​$.


The first step is to calculate the number of combinations where bad and good nodes can be chosen: 

---
Table 1: 
| &nbsp;&nbsp;Draw 1st node&nbsp;&nbsp; | &nbsp;&nbsp;Draw 2nd node&nbsp;&nbsp; | &nbsp;&nbsp;Draw 3rd node&nbsp;&nbsp; | Are bad nodes <br /> in the majority? |
| :------------------------------: | :-------------------------------: | :-------------------------------: | :----------------------------: |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_bad">$B2$</div>  | <div class="wrap_bad">$B3$</div> |              Yes               |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_bad">$B2$</div>  | <div class="wrap_good">$G1$</div> |              Yes               |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_bad">$B2$</div>  | <div class="wrap_good">$G2$</div> |              Yes               |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_bad">$B3$</div>  | <div class="wrap_good">$G1$</div> |              Yes               |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_bad">$B3$</div>  | <div class="wrap_good">$G2$</div> |              Yes               |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_good">$G1$</div> | <div class="wrap_good">$G2$</div> |               No               |
| <div class="wrap_bad">$B2$</div> | <div class="wrap_bad">$B3$</div>  | <div class="wrap_good">$G1$</div> |              Yes ​              |
| <div class="wrap_bad">$B2$</div> | <div class="wrap_bad">$B3$</div>  | <div class="wrap_good">$G2$</div> |              Yes ​              |
| <div class="wrap_bad">$B2$</div> | <div class="wrap_good">$G1$</div> | <div class="wrap_good">$G2$</div> |               No               |
| <div class="wrap_bad">$B3$</div> | <div class="wrap_good">$G1$</div> | <div class="wrap_good">$G2$</div> |               No               |
|                                  |                                   |    **Tally of Yes responses**     |              $7$               |

---

From this list:
- The number of combinations where $B$ is the majority can then be tallied. 
- There are $7$ combinations where $B$ is the majority. 
- Thus, from the $10$ combinations, there are $7$ combinations where there is a 
majority of bad nodes. 
- Therefore, the quotient of $7$ and $10$ is the probability $0.7$. 

This method is limited in calculating the probability where the variables are large. For example, if the same question 
was posed, but one had to calculate the probability of selecting a majority of bad nodes from a total of $100$ nodes, with a 
committee size of $60$, $60$ bad nodes and $40$ good nodes, the number of combinations where bad and good nodes can be 
chosen is $1.27E+28$. 

---

# Statistical Calculation 

Literature about BFT threshold advises the number of good nodes to be at least $\frac{2}{3} \cdot n+1​$, where $n​$ is the number of nodes. In the calculations that follow, BFT threshold of, for example, $67​$% of N, is implemented with rounding up to ensure that at least that fraction is obtained. In this sense, $67​$% of N simulates $\frac{2}{3} \cdot n+1​$.

---

#### Variation of Total Nodes

The variables and results are below: 

- N (total number of nodes in the network) = $100, 300, 500, 1000$
- m (number of bad actors) = $60$% of N
- T (BFT threshold) = $67$% of N
- n (committee size) = ranging from $1$ to $1,000$ 

---

| &nbsp;&nbsp;Total Nodes&nbsp;&nbsp; | &nbsp;&nbsp;Bad Nodes&nbsp;&nbsp; | &nbsp;&nbsp;Committee Size&nbsp;&nbsp; | &nbsp;&nbsp;BFT Threshold&nbsp;&nbsp; | &nbsp;&nbsp;Probability&nbsp;&nbsp;            |
| :---------------------------------: | :-------------------------------: | :------------------------------------: | :-----------------------------------: | ---------------------------------------------- |
|                 100                 |                60                 |                   2                    |                   2                   | 0.3575757575757576                             |
|                 100                 |                60                 |                   4                    |                   3                   | 0.47343240951488375                            |
|                 100                 |                60                 |                   6                    |                   4                   | 0.5443381851334722                             |
|                 100                 |                60                 |                   8                    |                   6                   | 0.30661160770090995                            |
|                 100                 |                60                 |                   10                   |                   7                   | 0.37423758246308586                            |
|                 100                 |                60                 |                   12                   |                   8                   | 0.4320215340178938                             |
|                 100                 |                60                 |                   14                   |                  10                   | 0.2623321970180976                             |

---

@div[s250px]
![graph](https://raw.githubusercontent.com/tari-labs/tari-university/src/network-analysis/probabilistic-attack/assets/variation_of_total_nodes .png)
@divend 

From a plot of committee size versus probability with a change in $N$, the total number of nodes, it can be seen that 
the probability is lower with respect to the committee size when $N$ is smaller. 

---

#### Variation of Byzantine Fault-tolerance Threshold

The variables and results are below: 

  - N (total number of nodes in the network) = $100$
  - m (number of bad actors) = $60$% of N
  - T (BFT threshold) = $50$%, $55$%, $60$%, $67$% of N
  - n (committee size) = ranging from $1$ to $100$ 
  
---
 
@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/variation_of_bft_threshold.png)
@divend 
  
---

The above graph was calculated using Python. From a plot 
of committee size versus probability where the number of nodes remains at 100 with a change in T, the BFT 
threshold, ranging from 50% to 67%, it can be seen that When the BFT threshold is 50% and 55%, the probability 
is low when the committee size is small; as the committee size increases, the probability increases, and tends to 1. 
The probability is higher for the case where the BFT threshold is $50% than when the probability is 55%. 

When the BFT threshold is $60$%, the probability decreases from $0.63$ to approximately $0.59$, where it remains constant. 

When the BFT threshold is $65$% and $67$%, the probability decreases from $0.38$ and tends to zero. This confirms the 
BFT threshold of $\frac{2}{3} \cdot n+1$ as per literature.

---

#### Variation of Total Number of Nodes with Committee Size 10

The variables and results are below: 

- N (total number of nodes in the network) = ranging from $10$ to $350$
- m (number of bad actors) = $60$% of N
- T (BFT threshold) = $67$% of N
- n (committee size) = $10$

---

@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/committee_size_10.png)
@divend

Note: 

The above graph was calculated using Excel. For the 
graph showing varying probabilities with respect to the total number of network nodes, where the committee size is $10$, 
the probability dramatically increases when the total nodes is $3$ times more than the committee size and onwards. The 
probability plateaus at $0.35$. 

---

#### Variation of Total Number of Nodes with Committee Size 100

The variables and results are below: 

- N (total number of nodes in the network) = ranging from $100$ to $1,300$
- m (number of bad actors) = $60$% of N
- T (BFT threshold) = $67​$% of N
- n (committee size) = $100$

---

@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/committee_size_100.png)
@divend

Note: 

The above graph was calculated using Excel. From this and the previous graph, it can be seen that probabilities are significantly lower when the committee size is 
$100$ compared to when it is $10$. There is an increase in probability up to a network size of $700$, albeit, not as steep as the 
change when the committee size is $10$. The probability plateaus at $0.08$. The larger the committee size, the fewer dramatic changes there are in the probability. 

---

#### Variation of Bad Nodes with Committee Size 10 and 100

The variables and results are below: 

- N (total number of nodes in the network) = ranging from $10$ and $100$ to $50,000$
- m (number of bad actors) = $10$%, $20$%, $30$%, $40$%, $50$%, $60$%, $70$%, $80$% and $90$% of N
- T (BFT threshold) = $67$% of N
- n (committee size) = $10$ and $100$

---

@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/probability_when_committee_100_20.png)
@divend

---

@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/probability_when_committee_100_40.png)
@divend

---

@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/probability_when_committee_100_60.png)
@divend

---
@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/probability_when_committee_100_90.png)
@divend

Note: 

The above graphs were calculated using Excel. These 
graphs show varying probabilities when the percentage of bad nodes is $20$, $40$, $60$ and $90$. The value when the 
probability plateaus is used to construct the following graph for both committee sizes $10$ and $100$. 

---

@div[s250px]
![graph](https://github.com/tari-labs/tari-university/blob/ks-network-probability-presentation/src/network-analysis/probabilistic-attack/assets/probability_bad_nodes_10_100.png)
@divend


Note:

The above graph was calculated using Excel. The 
graph shows changes in the probability due to changes in percentage of bad nodes when the committee size is $10$ and $100$.  When 
the committee size is $10$, there is a change in probability when the bad node percentage is between $30$ and $80$.  
When the committee size is $100$, there is a steep increase in the probability when the bad node percentage is between 
$50$ and $80$. When the committee size is $100$, the probability remains lower as the bad node percentage increases and 
has a steeper gradient when the change in probability occurs. Whereas, when the committee size is $10$, the probability 
begins to increase at a lower percentage of bad nodes. 

## Conclusions and Remarks 

- Total nodes in the network: The smaller the pool of total nodes in the network, the lower the probability of bad 
actors controlling the network. However, the probability difference is near negligible if the committee size is large. 
This parameter will also be difficult to control, and the network will be ever-increasing. This can be seen in the 
graph in [Variation of Total Nodes](#variation-of-total-nodes). 
- BFT threshold: This threshold should be at least  $\frac{2}{3} \cdot n+1$ as per literature. This can be seen in the 
graph in [Variation of Byzantine Fault-tolerance Threshold](#variation-of-byzantine-fault-tolerance-threshold).
- Committee size: The larger the committee size, the lower the probability of bad actors controlling the network. This can 
be seen in the graph in 
[Variation of Total Number of Nodes with Committee Size 10](#variation-of-total-number-of-nodes-with-committee-size-10) 
and [Variation of Total Number of Nodes with Committee Size 100](#variation-of-total-number-of-nodes-with-committee-size-100).
- Bad nodes: While this variable cannot be controlled, the probability of bad actors controlling the network can remain 
low, as the percentage of bad nodes increases if the committee size is approximately $100$ or larger. This can be seen in the 
graphs in [Variation of Bad Nodes with Committee Size 10 and 100](#variation-of-bad-nodes-with-committee-size-10-and-100)

Note: 

With regard to the statistical calculation, comments can be made for each of the varied parameters. 