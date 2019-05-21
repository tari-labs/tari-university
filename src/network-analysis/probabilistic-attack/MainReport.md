# Probabilistic Attack Vector Analysis Building Blocks

- [Introduction](#introduction)
	- [Question](#question) 
	- [Aim](#aim)
	- [The Tari Digital Assets Network](#the-tari-digital-assets-network) 
- [Literature Review](#literature-review)
	- [The Use of Python](#the-use-of-python)
	- [Monte Carlo Simulations](#monte-carlo-simulations)
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
	- [The Use of Python](#the-use-of-python)
- [Results](#results)
	- [Probabilistic Attack Graphs](#probabilistic-attack-graphs) 
	- [Demonstration](#demonstration)
	- [Discussion](#discussion) 
- [Conclusion and Recommendations](#conclusions-and-recommendations)
- [References](#references)
- [Contributions](#contributors) 

# Notes

develops tests, test method, reference data, proof of concept implementations, and technical analysis to advance the development and productive ise of infromation technology. 

Today's information systems face sophisticated attackers who combine multiple vulnerabilitesi to penetrate networks with devastating impact. The overall 

## Introduction
(What you researched and why)

### Question 1

What is the probability of an attacker controlling the majority of nodes in the network?

### Question 2

WHat is the % chance of me controlling majority of nodes in a random sample of N (N=3, 4, 5, 10 )

Current day network operate on multiple hardware devices assisted by numerous numbers of operating systems. Systems have vulnerablities. These are explored and then exploited. This paper has made an attempt to explore possility of quantifying various probabilities. Cyber system can be modeled in different ways. There are various attack vectors that make cyber network vulnerable. 

### Aim 

This research aims to provide answers to questions posed about the workings of the  Tari DAN environment: Probabilistic attack vector with regards to the total nodes, compromised nodes, committee size and BFT threshold. 

### The Tari Digital Assets Network 
Digital assets (DAs) are managed by committees of special nodes, Validator nodes . 

Validator nodes form committees to manage the digital assets, their state change and ensures that the rules governing asset contracts are enforced. 

Where was the idea borne from? 

## Literature Review 
(Other relevant research in this area)

### The Use of Python 

### Monte Carlo Simulations 

Monte Carlo (MC) approch is a computer based analytical method and was developed in the 1940s. It employs statistical sampling techniques for obtaining a probabilistic approximation to the solution of a mathematical equation or model by utilising sequences of random numbers as inputs into a model which yields results that are indicationd of the performance of the developed model. 

It was developed as part of the atomic program, by a scientist at the Los Alamos National Laboratory, who used it to model the random diffusion of neurtrons. The name was given after the city in Monaco and its many casions. Monte Carlo simulstion are used in a wide array of applications, including physics, finance and system reliability. 

Monte Carlo analysis utlizes statical tools to mathematically model a real life system or process and tehn it estimates the probability of obtaining a successful outcome. The statistical distribution of the process to be modeled must be determined first before Monte Carlo simulation can be applied. 

### Malicious Node

A malicious node, or bad actor, is defined as a node seeking to deny service to other nodes in the network.

### Consensus Protocols 

A consensus protocol enables a system of *n* asynchronous processes, some of them faulty, to reach agreement. Both the process and the message system are capable of cooperating to prevent the correct processes from reaching decision. 

### Threshold Cryptography 

There are consensus protocols that use threshold cryptography to encrypt and decrypt sets of transactions. A threshold value *t* is sent so that any group of *t+1* nodes in the netwrok can collaborate to decrypt messages adn construct digitial signatures. The threshold must be met, but is can be met by any combination of active nodes. https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87


### Types of Distribution 

When considering solving the probability of the of an attacker controlling the majority of nodes in the network, the various types of probability distributions need to be analysed with regards to the specific circumstances and variables of the problem. Types of probability distribution can be split into finite and infinite support [[1]]; where support is defined as a real-valued function *f*, which is the subset of the domain containing those elemets which are not mapped to zero. If the domain of *f* is a topological space, teh support of *f* is instead defined as the smallest closed set containing all points not mapped to zero. [[2]] 

#### Hypergeometric Distribution:

Hypergeometric distribution is a dicrete probability distribution that describes the probability *k* successes (random draws for which the object drawn has a specified feature) in *n* draws, *without* replacement, from a finite population of  size *N* that contains exactly *K* objects with that feature, wherein each draw is either a success or a failure. [[3]]

Selecting nodes without replacement, i.e. selecting all 6 nodes at once

#### Binomial Distribution:

The binomial distribution with parameters Selecting nodes with replacement, i.e. selecting each node, noting whether it is malicious or friendly and returning back to the committee. [[4]]

## Methodolgy 
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

#### Hypergeometric Distribution

$$
P = \frac{{{m}\choose{T}}\cdot{{N-m}\choose{n-T}}}{{N}\choose{n}}
$$

#### Binomial Distribution  

$$
P = {{n}\choose{T}}\cdot\biggl(\frac{m}{n}\biggr)^{T}\cdot\biggl(\frac{N-m}{n}\biggr)^{n-T}
$$

#### Summation 

$$
P_{tot} = \sum_{i=T}^{n} P(N,m,n,i)
$$

### Explanation of hypergeometric distribution (combinations)

Once the formula was mapped out, preliminary calculations could be solved using Excel. 

## Results 
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

### Probabilistic Attack Graphs 

![image-20190514194933056](/Users/kevoulee/Library/Application Support/typora-user-images/image-20190514194933056.png)

From a plot of committee size versus probability of bad actors controlling the network it can be seen that:

- As the commitee size increases, the probability increases 
- Thus, the greater the sample size the greater the chance of there being bad nodes in the set drawn 

### Demonstration 

### Discussion 
(Relevance of your results, how it fits with other research in the area)

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





## Contributions

- <https://github.com/kevoulee>
