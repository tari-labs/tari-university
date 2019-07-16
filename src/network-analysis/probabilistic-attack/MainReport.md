# Probabilistic Attack Vector Analysis Building Blocks

- [Introduction](#introduction)
	- [Question](#question) 
	- [Aim](#aim)
	- [The Tari Digital Assets Network](#the-tari-digital-assets-network) 
- [Literature Review](#literature-review)
	- [Monte Carlo Simulations](#monte-carlo-simulations)
	- 
	- [Types of Distribution](#types-of-distribution)
		- [Hypergeometric Distribution](#hypergeometric-distribution)
		- [Binomial Distribution](#binomial-distribution)
- [Methodology](#methodology)
	- [Notation Used](#notation-used)
	- [Formulae](#formulae)
		- [Hypergeometric Distribution](#hypergeometric-distribution)
		- [Binomial Distribution](#binomial-distribution)
		- [Summation](#summation)
	- [Explanation of hypergeometric distribution ](#explanation-of-hypergeometric-distribution)
	- [Implemention with Excel](#implementation-with-excel)
- [Implementation](#implementation)
	- [Results](#results)
	- [The Use of Python](#the-use-of-python) 
	- [Probabilistic Attack Graphs](#probabilistic-attack-graphs) 
	- [Demonstration](#demonstration)
	- [Discussion](#discussion) 
		- [Function](#function)
- [Conclusion and Recommendations](#conclusions-and-recommendations)
- [References](#references)
- [Contributions](#contributors) 

# Notes

develops tests, test method, reference data, proof of concept implementations, and technical analysis to advance the development and productive ise of infromation technology. 

Today's information systems face sophisticated attackers who combine multiple vulnerabilitesi to penetrate networks with devastating impact. The overall 


### Malicious Node

A malicious node, or bad actor, is defined as a node seeking to deny service to other nodes in the network.

### Consensus Protocols 

A consensus protocol enables a system of *n* asynchronous processes, some of them faulty, to reach agreement. Both the process and the message system are capable of cooperating to prevent the correct processes from reaching decision. 

### Threshold Cryptography 

There are consensus protocols that use threshold cryptography to encrypt and decrypt sets of transactions. A threshold value *t* is sent so that any group of *t+1* nodes in the netwrok can collaborate to decrypt messages adn construct digitial signatures. The threshold must be met, but is can be met by any combination of active nodes. [[5]] 

## Introduction
(What you researched and why)

### Question

What is the probability of an attacker controlling the majority of nodes in the network?

What is the % chance of me controlling majority of nodes in a random sample of N (N=3, 4, 5, 10 )

Current day network operate on multiple hardware devices assisted by numerous numbers of operating systems. Systems have vulnerablities. These are explored and then exploited. This paper has made an attempt to explore possility of quantifying various probabilities. Cyber system can be modeled in different ways. There are various attack vectors that make cyber network vulnerable. 

### Aim 

This research aims to provide answers to questions posed about the workings of the  Tari DAN environment: Probabilistic attack vector with regards to the total nodes, compromised nodes, committee size and BFT threshold. 

### The Tari Digital Assets Network 
Digital assets (DAs) are managed by committees of special nodes, Validator nodes . 

Validator nodes form committees to manage the digital assets, their state change and ensures that the rules governing asset contracts are enforced. 

Where was the idea borne from? 

### XOR Metric

## Literature Review 
(Other relevant research in this area)

### Monte Carlo Simulations 

Monte Carlo (MC) approch is a computer based analytical method and was developed in the 1940s. It employs statistical sampling techniques for obtaining a probabilistic approximation to the solution of a mathematical equation or model by utilising sequences of random numbers as inputs into a model which yields results that are indicationd of the performance of the developed model. 

It was developed as part of the atomic program, by a scientist at the Los Alamos National Laboratory, who used it to model the random diffusion of neurtrons. The name was given after the city in Monaco and its many casions. Monte Carlo simulstion are used in a wide array of applications, including physics, finance and system reliability. 

Monte Carlo analysis utlizes statical tools to mathematically model a real life system or process and tehn it estimates the probability of obtaining a successful outcome. The statistical distribution of the process to be modeled must be determined first before Monte Carlo simulation can be applied. 

Monte Carlo methods are idely used heuristic techniques which can solce a variety of common problems including optimization and numerical intergration problems. These algorithms work by cleverly sampling from a distribution to simulate the workings of a system. Applications range from solving problems in theoretical physics to predicting trends in financial investments. [[6]] 

#### Monte Carlo Fallacy 

The Monte Carlo Fallacy, or gambler's fallacy is the inaccurate belief that if something happens more frequently than normal during a given period it will happen less fequently in the future. In situations where the outcome being observed is truly random and consists of independent trials of a random process, this belief is false. The fallacy can arise in many situations, but is most stringly associated with gambling, where it is common among players.  

illustrate the flaws of the gamblers falacity 

#### The Crude Monte Carlo

The Monte Carlo technique is built upon this principle: instead of evaluating an indefinite integral, which can sometimes be impossible, the average of the integrand is estimated and that is used to approximate the integral. If one needs to be more precise, the number of samples can be increased. 

#### The Law of Large Numbers 

The law of large numbers (LLN), in probability and statistics, states that as a sample size grows, its mean gets closer to the average of the whole population. In statistical analysis, the law of large numbers can be applied to a variety of subjects. It may not be feasible to poll every individual within a given population to collect the required amount of data, but every additional data point gathered has the potential to increase the likelihood that the outcome is a true measure of the mean. [[7]] 

The LLN is important beacuse it guatantees stable long-term results from the [[8]] 

![law_of_large_numbers](/Users/kevoulee/tari-university/src/network-analysis/probabilistic-attack/assets/law_of_large_numbers.png)

The above figure illustrates the law of large numbers using a particular run of rolls of a single dice. As can be seen in the figure, as the number of rolls in this run increases, the average of the values of all the results approaches 3.5. While different runs would show a different shape over a small number of throws (at the left), over a large number of rolls (to the right) they would be extremely similar. 

By Pred - Own work, CC0, [[9]]


### Types of Distribution 

When considering solving the probability of the of an attacker controlling the majority of nodes in the network, the various types of probability distributions need to be analysed with regards to the specific circumstances and variables of the problem. Types of probability distribution can be split into finite and infinite support [[1]]; where support is defined as a real-valued function *f*, which is the subset of the domain containing those elemets which are not mapped to zero. If the domain of *f* is a topological space, the support of *f* is instead defined as the smallest closed set containing all points not mapped to zero. [[2]] 

#### Hypergeometric Distribution:

Hypergeometric distribution is a dicrete probability distribution that describes the probability *k* successes (random draws for which the object drawn has a specified feature) in *n* draws, *without* replacement, from a finite population of  size *N* that contains exactly *K* objects with that feature, wherein each draw is either a success or a failure. [[3]]

Selecting nodes without replacement, i.e. selecting all 6 nodes at once

Example: You have an urn of 10 marbles -5 red and 5 green, You randomly select 2 marbles without replacement and count the number of red marbles selected. This would be a hypergeometric experiment. 

A hypergeometric random variable is the number of successes that result from a hypergeometri experiment. The probability distribution of a hypergeometric random variable is called a hypergeometric distribution. 

- A sample of size *n* is randomly selected without replacement from a population of N items
- In the population, *k* items can be classified as successes, and *N-k* items can be classified as failures 

(k=m and x=t)

Given $x, N, n$ and $k$ the hypergeometric probability can be computed based on teh following formula 

Example: Suppose a population consists of $N$ items, $k$ of which are successes. And a random sample drawn from that population consists of $n$ items, $x$ of which are successes. Then the hypergeometric probability is:
$$
h(x; N, n, k)= {{k}\choose{x}}{{N-k}\choose{n-x}}{{N}\choose{n}}
$$
The hypergeometric distribution has the following properties:

- The mean of the distribution is equal to $n\cdot\frac{k}{N}$

- The variance is $ n\cdot k \cdot(N-k)\cdot\frac{N-n}{N^2\cdot(N-1)}$

[[10]] https://stattrek.com/probability-distributions/hypergeometric.aspx



#### Binomial Distribution:

The binomial distribution with parameters Selecting nodes with replacement, i.e. selecting each node, noting whether it is malicious or friendly and returning back to the committee. [[4]] a bionomial experiment requires that the probability of success be constant on every trial. 

Example: You have an urn of 10 marbles -5 red and 5 green, You randomly select 2 marbles with replacement, the probability of siccess would not change. It would be 5/10 on every trial. https://stattrek.com/probability-distributions/hypergeometric.aspx

## Methodology 
(What you did and how you found it)

Based on the understanding of the problem, statisical analysis using hypergeometric distribution was conducted.  

### Notation Used  

This section conmtains the general notation of statistical expressions when specifically referenced. This information serves as important pre-knolwedge for the remainder of the report. 

- Let $N$ be the total number of nodes in the network *set size* 
- Let $n$ be the committee size *sample_size*
- Let $m$ be the number of bad actors *no_of_type_in_set*
- Let $T$ be the BFT threshold *type threshold*

There would be a pool with *N* nodes, the pool contains *m* malicious nodes or bad actors,  within the pool a random selection of nodes are drawn *n*, from that selection the probablity of drawning a threshold of bad actors *T* needs to be calculated.  

### Formulae

As a means to derive the formulae, a single probability from the overarching question was used to distill the  formula. 

#####Example 1

What is the probability of selecting a majority of bad nodes from a total of 5 nodes if the committee size is $3$. There are $3$ bad nodes $(B1, B2, B3)$ and $2$ good nodes $(G1, G2)$.

The first step is to calculate the number of combinations where bad and good nodes can be chosen. 

$(B1, B2, B3)$
$(B1, B2, G1)$
$(B1, B2, G2)$
$(B1, B3, G1)$
$(B1, B3, G2)$
$(B1, G1, G2)$
$(B2, B3, G1)$
$(B2, B3, G2)$
$(B2, G1, G2)$
$(B3, G1, G2)$

From this list, the number of combinations where 'B' is the majority can then be tallied. In this case there are $7$ combinations where 'B' is the majority. Thus, from the $10$ combinations, there are $7$ combinations where there is a majority of bad nodes. Therefore, the quoient of $7$ and $10$, is the probability, $0.7$. 

This method is limited in calculating the probability where the variables are large e.g. if the same question was posed, but the one had to now calculate the probability of selecting a majority of bad nodes from a total of 100 nodes, with a committee size is $60$, $60$ bade nodes and $40$ good nodes, the number of combinations where bad and good nodes can be chosen is $1,27E+28$

#### Hypergeometric Distribution

With regards to hypergeometric distribution, a committee of nodes were drawn from the total nodesn without replacement; i.e. nodes are drawn simultatenously, the intention of the node is distinguished and not returned to the total nodes. 

Using the same example from above, 

What is the probability of drawing 3 red cards from a standard deck of 52 cards if you draw 5 cards from the deck? There are 26 red cards. 

- $N$ = 52

- $n$ = 5

- $T$ = 3

- $m$ = 26

Derivation: 

Of the $3$ bad nodes, the threshold is $2$ ${{3}\choose{2}} \therefore {{m}\choose{T}} $

Of the $2$ good nodes, the threshold is $1$ ${{5-3}\choose{3-2}} \therefore {{N-m}\choose{n-T}}$

Of the $5$ nodes in total, the committee size is $3$ ${{5}\choose{3}} \therefore {{N}\choose{n}}$ 

Therefore, 
$$
P = \frac{{{m}\choose{T}}\cdot{{N-m}\choose{n-T}}}{{N}\choose{n}}
$$

#### Binomial Distribution  

With regards to bionomial distribution, a committee of nodes were drawn from the total nodes with replacement; i.e. nodes are drawn, the intention of the node is distinguished and   then the node is returned to the total nodes. 


$$
P = {{n}\choose{T}}\cdot\biggl(\frac{m}{n}\biggr)^{T}\cdot\biggl(\frac{N-m}{n}\biggr)^{n-T}
$$

### Explanation of hypergeometric distribution (combinations)

Once the formula was mapped out, preliminary calculations could be solved using Excel. If the total number of nodes $N$ is fixed.

- The number of bad nodes (m) is set to sixty percent of N
- The committee size (n) increases by a factor of 10 from 0 to $N$
- The BFT threshold (T) is set to sixty-seven percent of $n$

As a recap to the question, What is the probability of selecting a majority of bad nodes from a total of 300 nodes if the committee size is $10$. This problem considers solving for the probability where there is a **majority** of bad nodes in the committee. This entails calculating the probabilities from the BFT threshold to the committee size. Thus, there needs to be a summation of individual probabilities in order to calcuate the probability for selecting the majority of bad nodes. 

In order to understand this, the table below provides some visual insight. (insert hotlink)

| Committee Size | Bad Actors | BFT Threshold | No of Steps |
| -------------- | ---------- | ------------- | ----------- |
| 10             | 180        | 7             | 3           |

The number of steps is counted by considering the BFT thresfold and the committee size. What is actually being said here is that when 10 nodes are selected without replacement from a total of 300 nodes, what is the probability that out the the 10 nodes there will be 7 nodes or more that are bad. Thus, probabilities need to be calculated for when there are 7 bad nodes, 8 bad nodes, 9 bad nodes and 10 bad nodes. 

#### Summation 

$$
P_{tot} = \sum_{i=T}^{n} P(N,m,n,i)
$$

show a hypergeometric distribution graph

### Implementation with Excel 

## Implementation

### Results 

(What you found)

- Certain variables that remain constant 
  - Total nodes, *N* = 500 
  - Bad nodes, *m* = 300 
- Committee size and Threshold increases as the array increases 
- As the commitee size increases, the threshold increases  

| Total Nodes | Bad Nodes | Committee Size | Threshold | Probability        |
| ----------- | --------- | -------------- | --------- | ------------------ |
| 500         | 300       | 10             | 5         | 0.8361286749106096 |
| 500         | 300       | 20             | 10        | 0.8772521971682634 |
| 500         | 300       | 30             | 12        | 0.9097445045868039 |
| 500         | 300       | 40             | 20        | 0.9339389992360666 |
| 500         | 300       | 50             | 25        | 0.9518836452982837 |
| 500         | 300       | 60             | 30        | 0.9651848964241138 |
| 500         | 300       | 70             | 35        | 0.9750201140493069 |
| 500         | 300       | 80             | 40        | 0.98225732828804   |
| 500         | 300       | 90             | 45        | 0.9875451264650159 |
| 500         | 300       | 100            | 50        | 0.9913733032263443 |
| 500         | 300       | 110            | 55        | 0.9941140693110859 |
| 500         | 300       | 120            | 60        | 0.9960508726509922 |
| 500         | 300       | 130            | 65        | 0.9973992086727235 |
| 500         | 300       | 140            | 70        | 0.9983220920579189 |
| 500         | 300       | 150            | 75        | 0.9989418454891916 |

### The Use of Python 

Python is a multi-paradigm programming language. It supports object-oriented programming, structural programming, and functional programming patterns, among others. 

### Demonstration 

Insert hotlink with python link for the static.py 

### Probabilistic Attack Graphs 

From a plot of committee size versus probability of bad actors controlling the network it can be seen that:

- As the commitee size increases, the probability increases 
- Thus, the greater the sample size the greater the chance of there being bad nodes in the set drawn 

![probability-1](/Users/kevoulee/tari-university/src/network-analysis/probabilistic-attack/assets/probability-1.png)

The variables

- Total nodes = N
- Committee size = n 
- Bft threshold = T
- Bad actors = m

**Vary the total nodes**

From a plot of committee size versus probability with a change in $N$, the total number of nodes, it can be seen that:


![probability-2](/Users/kevoulee/tari-university/src/network-analysis/probabilistic-attack/assets/probability-2.png)

**Vary the BFT threshold**

![probability-3](/Users/kevoulee/tari-university/src/network-analysis/probabilistic-attack/assets/probability-3.png)

From a plot of committee size versus probability where the number of nodes remains at 500 with a change in $T$, the BFT threshold, ranging from 50% to 67%, it can be seen that: 

- When the BFT threshold is 50% the probability is lower when the committee size is small, the probabiltiy increases when the committee size increases, and eventually tends to one.

- When the BFT threshold is 55% the probability is lower when the committe size is small, the probability increases when the committee size increases, and eventually tends to one. 

- A similar trend is followed for the cases where the BFT threshold is 50% and 55%

- The probability is higher for the case where the BFT threshold is 50% than when the probability is 55% 

- When the BFT threshold is 60% the probability decreases from 0.63 to approxiamtely 0.53 where it remains constant 

- When the BFT threshold is 65% the probability decreases from 0.38 and tends to zero 

- When the BFT threshold is 67% the probability decreases from 0.38 and tends to zero 

- A similar trend is followed for the cases where the BFT threshold is 65% and 67% 

- It is interesting to note that in the case were there is 


### Monte Carlo Simulation



### Randomness (Histogram of Randomness) 

### Law of Large Numbers 

### Discussion 

(Relevance of your results, how it fits with other research in the area)

#### Function 

## Conclusion and Recommendations
(Summary of results/findings and what needs to be done as a reuslt of your findings)

It has come to light, that in addition to further analysis within the Python environment, research into using the Monte Carlo simulations must be considered. 

## References

[[1]] B. W. Contributors, “List of probability distributions”, 2019. Available: <https://en.wikipedia.org/wiki/List_of_probability_distributions>. 
Date accessed: 2019-05-13. 

[1]: https://en.wikipedia.org/wiki/List_of_probability_distributions
"List of probability distributions"

[[2]] B. W. Contributors, “Support (mathematics)”, 2019. Available: <https://en.wikipedia.org/wiki/Support_(mathematics)>. 
Date accessed: 2019-05-13. 

[2]: https://en.wikipedia.org/wiki/Support_(mathematics)
“Support (mathematics)”

[[3]] B. W. Contributors, “Hypergeometric distribution”, 2019. Available: <https://en.wikipedia.org/wiki/Hypergeometric_distribution>. 
Date accessed: 2019-05-13. 

[3]: https://en.wikipedia.org/wiki/Hypergeometric_distribution
"Hypergeometric distribution"

[[4]] B. W. Contributors, “Binomial distribution", 2019. Available: <https://en.wikipedia.org/wiki/Binomial_distribution>. 
Date accessed: 2019-05-13. 

[4]: https://en.wikipedia.org/wiki/Binomial_distribution
“Binomial distribution"

[[5]] ?? Availiable:<https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87>. Date accessed: 2019-06-28 

[5]: https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87

[[6]]?? Available: <https://towardsdatascience.com/monte-carlo-simulations-with-python-part-1-f5627b7d60b0>. Date accessed: 2019-06-28

[6]:  https://towardsdatascience.com/monte-carlo-simulations-with-python-part-1-f5627b7d60b0

[[7]] ?? Available: <https://www.investopedia.com/terms/l/lawoflargenumbers.asp>. Date accessed: 2019-06-28

[7]: https://www.investopedia.com/terms/l/lawoflargenumbers.asp

[[8]]?? Available: <https://en.wikipedia.org/wiki/Law_of_large_numbers>. Date accessed: 2019-06-28

[8]: https://en.wikipedia.org/wiki/Law_of_large_numbers

[[9]]?? Available: <https://commons.wikimedia.org/w/index.php?curid=58536069>. Date accessed: 2019-06-28 

[9]: https://commons.wikimedia.org/w/index.php?curid=58536069

[[10]]?? Available: <




## Contributions

- <https://github.com/kevoulee>
