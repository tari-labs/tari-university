# Probabilistic Attack Vector Analysis Building Blocks

- [Introduction](#introduction)
	- [Aim](#aim)
	- [Question](#question) 
- [Literature Review](#literature-review)
  - [The Tari Digital Assets Network](#the-tari-digital-assets-network) 
    - [Kademlia](#kademlia)
    - [Node ID](#node-id)
      - [Bootstrapping a Node](#bootstrapping-a-node)
      - [XOR Metric](#xor-metric)
  - [Types of Distribution](#types-of-distribution)
    - [Continuous Distribution](#continuous-distribution)
    - [Discrete Distribution](#discrete-distribution)
      - [Hypergeometric Distribution](#hypergeometric-distribution)
      - [Binomial Distribution](#binomial-distribution)
  - [Monte Carlo Simulations](#monte-carlo-simulations)
    - [Monte Carlo Fallacy](#monte-carlo-fallacy) 
      - [Illustration of the Gambler's Falacity](#illustration-of-the-gamblers-falacity)
    - [The Crude Monte Carlo](#the-crude-monte-carlo)
    - [The Law of Large Numbers](#the-law-of-large-numbers)
- [Methodology](#methodology)
	
	- [Notation Used](#notation-used)
	- [Crude Monte Carlo Simulation](curde-monte-carlo-simulation)
	  - [Programming the Simulation](#programming-the-simulation)
	  - [Use of Random Numbers](#use-of-random-numbers)
	  - [Computing Statistics](#computing-statistics)
	- [Statistical Calculation](#statistical-calculation)
	  - [Formulae](#formulae)
	    - [Hypergeometric Distribution](#hypergeometric-distribution)
	    - [Binomial Distribution](#binomial-distribution)
	    - [Summation](#summation)
	  - [Explanation of Hypergeometric Distribution ](#explanation-of-hypergeometric-distribution)
- [Implementation](#implementation)
	- [Crude Monte Carlo Simulation](#crude-monte-carlo-simulation)
	  - [Individual Probabilities](#individual-probabilities)
	  - [A Histogram and Visualization of Distribution](#a-histogram-and-visualization-of-distribution)
	  - [Proving the Law of Large Numbers](#proving-the-law-of-large-numbers)
	  - [Summary of Distribution Data](#summary-of-distribution-data)
	- [Statistical Calculation](#statistical-calculation)
	  - [Probabilistic Attack Graphs](#probabilistic-attack-graphs) 
- [Conclusion](#conclusion)
- [References](#references)
- [Appendices](#appendices)
- [Contributions](#contributors) 

## Introduction

### Aim 

This research aims to provide answers to questions posed about the workings of the  Tari Digital Assets Network environment: Probabilistic attack vector with regards to the total nodes, compromised nodes, committee size and byzsntine fault tolerance threshold. 

### Question

This investigation attempts to answer the following question: *What is the % chance of controlling majority of nodes in a random sample of with varying quantities of the total number of nodes, committee size, bad nodes and byzantine fault tolerance threshold?*

This research paper has made an attempt to explore the possibility of quantifying various probabilities through variations in the total number of nodes, the committee selected from the total, and the threshold which governs whether a committee can be deemed corrupt. 

## Literature Review 

### The Tari Digital Assets Network 

The Tari Digital Assets Network (DAN) forms part of the Tari second layer, and is where all digital asset interactions are managed. 

These interactions are processed and validated by committees of special nodes, termed validator nodes (VNs). Management of digital assets involves state changes and ensures that the rules that govern assets contractes are enforced. Thus, all actions on this network are due to the interactions of the VNs. 

The validator nodes are registered on the base layer and in order to prevent Sybil attacks they commit collateral. This collateral can be taken from the VN if it is proven that the VN engaged in malicious behaviour. 

The Digital Asset (DA) would then be issued by an Asset Issuer (AI) and a contract drawn. It is the AI that will dictate the size of committee of validator nodes for a particular DA. The AI will also have the ability to nominate a trusted node to form part of the valiator node committee for the DA. If there are any remaining vacancies in the committee they would be filled with permissionless VNs. [[14]]

#### Kademlia

Kademlia was designed by Petar Maymounkov and David Mazières in 2002 and is  a distributed hash table, used for decentralised peer-to-peer computer networks. 

#### Node ID

With regards to node ID, a node selects an $n$-bit ID, which is given to nodes on the network. Node IDs are  **uniformly** distributed The position of a node is determined by a unique prefix of its ID, which forms a tree structure with node IDs as leaves. This ID should be reused when the node rejoins the network.

The bit length of the node ID should be sufficiently large to make collisions unlikely when using a uniformly distributed random number generator. [[15]]

##### Bootstrapping a Node

In order for a node to establish itself on the network without any know contacts, it needs to contact one or more bootstrap node. A bootstrap node is a node listed on a predetermined list, and serves as the first point of contact for new node requesting to be introduced to the network. First, a node ID is generated for the joining node, then the node contacts other nodes it is aware of, next it sends a lookup request with its newly generated node ID. The contacted nodes return the closest nodes they know about. The newly discovered nodes are added to the joining node's routing table. Thereafter, the joining node contacts some of the new nodes it knows about. The process then continues iteratively until the joining node is unable to locate any closer nodes. This _self-lookup_ has two effects: it allows the node to learn about nodes closer to itself; and it populates other nodes' routing tables with the node's ID [[15]].

##### XOR Metric

The Kademlia paper published in 2002 and provided a offered the novel idea of using the XOR ($\oplus$) operator to determine the distance and therefore the arrangement of peers within the network. 

Through the XOR metric a notion of distance is captured. The lookup procedure allows nodes to locate other nodes, given a node ID. [[15]]

### Types of Distribution 

When considering solving the probability of an attacker controlling the majority of nodes in the network, the various types of probability distributions need to be analysed with regards to the specific circumstances and variables of the problem. Types of probability distribution can be split into finite and infinite support [[1]]; where support is defined as a real-valued function *f*, which is the subset of the domain containing those elemets which are not mapped to zero. If the domain of *f* is a topological space, the support of *f* is instead defined as the smallest closed set containing all points not mapped to zero. [[2]] 

#### Continuous Distribution 

In order to understand continuous distribution, a continuous random variable needs to be defined. A continuous random variaible is a random variable with a set of possible values, a range, that is inifinite and as such, uncountable. [[11]]

Probabilities of continuous random variables (X) are defined as the area under the curve of its PDF, probability density function. <sup>[def][zk~]</sup> Therefore the only ranges of values can have a nonzero probability. The probability that a continuous random variable equals some value is always zero. [[11]]

<p align="center"><img src="assets/distribution_plot_normal_weight_shade_middle.png" width="700" /></p>
The continuous normal distribution can describe the distribution of weight of adult males. For example, you can calculate the probability that a man weighs between 160 and 170 pounds.

#### Discrete Distributions 

Likewise, in order to understand discrete distribution, a discrete random variable requires definition. A discrete random variable is a random variable that has countable values, such as a list of non-negative integers. A discrete distribution, thus describes the probability of occurrence of each value of a discrete random variable. [[11]]

With a discrete probability distribution, each possible value of the discrete random variable can be associated with a non-zero probability. Thus, a discrete probability distribution is often presented in tabular form. [[11]]

<p align="center"><img src="assets/distribution_plot_poisson_shade_right_tail.png" width="700" /></p>
With a discrete distribution, unlike with a continuous distribution, you can calculate the probability that X is exactly equal to some value. For example, you can use the discrete Poisson distribution to describe the number of customer complaints within a day. Suppose the average number of complaints per day is 10 and you want to know the probability of receiving 5, 10, and 15 customer complaints in a day.

| x    | P(X=x)   |
| ---- | -------- |
| 5    | 0.037833 |
| 10   | 0.125110 |
| 15   | 0.034718 |

The shaded bars in this example represents the number of occurrences when the daily customer complaints is 15 or more. The height of the bars sums to 0.08346; therefore, the probability that the number of calls per day is 15 or more is 8.35%.

Examples of discrete distribution with **finite** support include the following: 

| Type of finite discrete distribution              | Description                                                  |
| ------------------------------------------------- | ------------------------------------------------------------ |
| Bernouli Distribution                             | Takes value 1 with probability $p$ and value 0 with probability $q=1-p$ |
| Rademacher Distribution                           | Takes value 1 with probability $\frac{1}{2}$ and value $-1$ with probability $\frac{1}{2}$ |
| Binomial Distribution                             | The number of successes in a series of independent Yes/No experiments all with the same probability of success |
| Beta-Binomial Distribution                        | The number of successes in a series of independent Yes/No experiments with heterogeneity in the success probability |
| Degenate Distribution                             | at x0, where $X$ is certain to take the value x0. This does not look random, but it satisfies the definition of random variable. This is useful because it puts deterministic variables and random variables in the same formalism. |
| Discrete Uniform Distribution                     | where all elements of a finite set are equally likely. This is the theoretical distribution model for a balances coin, an unbiased die, a casino roulette, or the first card of a well-shuffled deck |
| Hypergeometric Distribution                       | The number of successes in the first $m$ of a series of $n$ consective Yes/No experiments, if the total numebr of successes is known. This distribution arises where there is no replacement |
| Poisson Biomial  Distribution                     | The number of successes in a series of independent Yes/No experiments with different success probabilities |

Examples of discrete distribution with **infinite** support include the following: 

| Type of infinite discrete distribution | Description                                                  |
| -------------------------------------- | ------------------------------------------------------------ |
| Boltzmann Distribution                 | A discrete distribution important in statistical physics which describes the probabilities of the various discrete energy levels of a system in thermal equilibrium. It has a continuous analogue. |
| Geometric Distribution                 | A discrete disctribution which describes the number of attempts needed to get the first success in a series of independent Bernoulli trials, or alternatively only the number of losses before the first success (i.e. one less) |
| Negative Binomial Distribution         | Pascal distribution, a generalization of the geometric distribution of the nth success |
| Poisson Distribution                   | A very large number of individually unlikely events that happen in a certain time interval. Relateed to this distribution are a number of other distributions: the displaced Poisson, the hyper-Poisson, teh general Poissin binomial and the Poissan type distributions. |
| Skellam Distribution                   | The distribution of the difference between two independent Poisson-distributed random variables. |
| Zeta Distribution                      | Has uses in applied statistics and statistical mechanics, and perhaps may be of interst to number theorists. It is the Zipf distribution for an infinite number of elements. |
| Zipf's Law                             | A discrete power-law distribution, the most famous example of which is the description of the frequency of words in the English language. |
| Zipf-Mandelbrot Law                    | A discrete power law distribution whcih is a generalization of the Zipf distribution. |

##### Hypergeometric Distribution

Hypergeometric distribution is a discrete probability distribution that describes the probability $T$ successes (random draws for which the object drawn has a specified feature) in $n$ draws, *without* replacement, from a finite population of size $N$ that contains exactly $m$ objects with that feature, wherein each draw is either a success or a failure. [[3]]

- A sample of size $n$ is randomly selected without replacement from a population of $N$ items
- In the population, $T$ items can be classified as successes, and $N-T$ items can be classified as failures 

Given $x, N, n$ and $k$ the hypergeometric probability can be computed based on the following formula 

Example: Suppose a population consists of $N$ items, $k$ of which are successes. And a random sample drawn from that population consists of $n$ items, $x$ of which are successes. Then the hypergeometric probability is:[[10]]
$$
h(m; N, n, T)= {{T}\choose{m}}{{N-T}\choose{n-m}}{{N}\choose{n}}
$$
The hypergeometric distribution has the following properties:

- The mean of the distribution is equal to $n\cdot\frac{T}{N}$
- The variance is $ n\cdot T \cdot(N-T)\cdot\frac{N-n}{N^2\cdot(N-1)}$

##### Binomial Distribution

The binomial distribution with parameters $n$ and $p$ is the discrete probability distribution of the number of successes in a sequence of $n$ independent experiments, each asking a yes-no question, and each with its own boolean-valued outcome: success/yes/true/one (with probability $p$) or failure/no/false/zero (with probability $q=1- p$). A single success/failure experiment is also called a Bernoulli trial or Bernoullu experiment and a sequence of outcomes is called a Bernoulli process; for a single trial, 1.e., $n=1$, the binomial distribution is a Bernoulli distribution. The binomial distribution is the basis for the popular binomial test of statistical significance. 

The binomial distribution is frequently used to model the number of successes in a sample of size $n$ drawn with replacement from a population of size $N$. If the sampling is carried out without replacement, the draws are not independent and so the resulting distribution is hypergeometric, not a binomial one. However, for $n$ much larger than $n$, the binomial distribution remainds a good approximation, and is widely used. 

Thus, in bionial distribution an object is selected with replacement [[4]].  A bionomial experiment requires that the probability of success be constant on every trial. 

Example: You have an urn of $10$ marbles $-5$ red and $5$ green, You randomly select $2$ marbles with replacement, the probability of siccess would not change. It would be $\frac{5}{10}$ on every trial.[[10]] 

### Monte Carlo Simulations 

Monte Carlo (MC) approch is a computer based analytical method and was developed in the 1940s. It employs statistical sampling techniques for obtaining a probabilistic approximation to the solution of a mathematical equation or model by utilising sequences of random numbers as inputs into a model which yields results that are indicationd of the performance of the developed model. 

It was developed as part of the atomic program, by a scientist at the Los Alamos National Laboratory, who used it to model the random diffusion of neurtrons. The name was given after the city in Monaco and its many casions. Monte Carlo simulstion are used in a wide array of applications, including physics, finance and system reliability. 

Monte Carlo analysis utlizes statistical tools to mathematically model a real life system or process and then it estimates the probability of obtaining a successful outcome. The statistical distribution of the process to be modeled must be determined first before Monte Carlo simulation can be applied. 

Monte Carlo methods are ideally used heuristic techniques which can solve a variety of common problems including optimization and numerical intergration problems. These algorithms work by cleverly sampling from a distribution to simulate the workings of a system. Applications range from solving problems in theoretical physics to predicting trends in financial investments. [[6]] 

#### Monte Carlo Fallacy 

The Monte Carlo Fallacy, or gambler's fallacy is the inaccurate belief that if something happens more frequently than normal during a given period it will happen less fequently in the future. In situations where the outcome being observed is truly random and consists of independent trials of a random process, this belief is false. The fallacy can arise in many situations, but is most stringly associated with gambling, where it is common among players. [[13]] 

##### Illustratration of the Gambler's Falacity 

The gambler's fallacy can be illustrated by considering the repeated toss of a fair coin. The outcomes in different tosses are statistically independent and the probability of getting heads on a single toss is $\frac{1}{2}$. The probaility of getting two heads in two tosses is $\frac{1}{4}$ abd tge probability of getthing three heads in three tosses is $\frac{1}{8}$. If after tossing four heads in a row, teh next coin toss also came up heads, it would complete a run of five successive heads. Since the probability of a run of five successive heads is $\frac{1}{32}$, a person might believe that the next flip would be more likely to come up tails rather than heads again. Thsi si incorrect and is an example of the gambler's fallacy. The event 'five heads in a row and the event 'first 4 heads, then a tails' are equally likely, each having probability $\frac{1}{32}$. Since the first four tosses turn up heads, the probability that the next toss is a head is $\frac{1}{2}$ . While a run of five heads has a probability of $\frac{1}{32} = 0.03125$, the misunderstanding lies in not realising that htis is the case only before the first coin is tossed. After the first four tosses, the results are no longer unknown, so therei probabilityes are at that point equal to 1. The reasoning that it is more likely that a fifth toss is more likely to be tails because the previous four tosses were heads, with a run of luck in the past influencing the odds in the future, forms the basis of the fallacy. [[13]]

#### The Crude Monte Carlo

The Monte Carlo technique is built upon this principle: instead of evaluating an indefinite integral, which can sometimes be impossible, the average of the integrand is estimated and that is used to approximate the integral. If one needs to be more precise, the number of samples can be increased. 

It is widely used heursitic technique which can solve a vaiety of common problems including optimization and numerical integration problems. These algorithms work by cleverly sampling from a distribution oto simulate the workings of a system.  Applciations range from solving problems in theoretical physics to predicting trends in financial investments. 

#### The Law of Large Numbers 

The law of large numbers (LLN), in probability and statistics, states that as a sample size grows, its mean gets closer to the average of the whole population. In statistical analysis, the law of large numbers can be applied to a variety of subjects. It may not be feasible to poll every individual within a given population to collect the required amount of data, but every additional data point gathered has the potential to increase the likelihood that the outcome is a true measure of the mean. [[7]] 

The LLN is important beacuse it guatantees stable long-term results from the [[8]] 

<p align="center"><img src="assets/law_of_large_numbers.png" width="700" /></p>
The above figure illustrates the law of large numbers using a particular run of rolls of a single dice. As can be seen in the figure, as the number of rolls in this run increases, the average of the values of all the results approaches 3.5. While different runs would show a different shape over a small number of throws (at the left), over a large number of rolls (to the right) they would be extremely similar. [[9]]

## Methodology 
Based on the understanding of the problem, statisical analysis using hypergeometric distribution was conducted.  

### Notation Used  

This section contains the general notation of statistical expressions when specifically referenced. This information serves as important pre-knolwedge for the remainder of the report. 

- Let $N$ be the total number of nodes in the network *set size* 
- Let $n$ be the committee size *sample_size*
- Let $m$ be the number of bad actors *no_of_type_in_set*
- Let $T$ be the BFT threshold *type threshold*

There would be a pool with *N* nodes, the pool contains *m* malicious nodes or bad actors,  within the pool a random selection of nodes are drawn *n*, from that selection the probablity of drawning a threshold of bad actors *T* needs to be calculated.  

### Crude Monte Carlo Simulation

#### Programming the simulation

It was intially thought that selecting a committee (n) from the total nodes (N) without replacing the selected nodes requires the removal of an element from the pool of total nodes when it is drawn. However, as this the programme is  calling for many selections within many experiments this logic could not be used. 

To begin with a random index is drawn with `randint` from the set of legal indices in the... 

The experiment is extended to ask the overarching question: what is the probability of selecting the threshold worth of bad nodes or more from a pool of total nodes? To this end, we perform a variable amount of experiments (no_of_draws), count how many times the threshold is met in order to estimate the   probability as M/n_of_draws. 

#### Use of Random Numbers  

Some problems in science and technology are described by 'exact' mathematics, leading to 'precise' results, i.e. throwing a ball or oscillating system. Some problems appear physcially uncertain, i.e. rolling a die, molecular motion. Random numbers can be used to mimic the uncertainity of the experiment. 

Random numbers make it possible to simulate physical systems with uncertainity, in input data or the process. 

#### Computing statistics 

<p align="center"><img src="assets/mode_median_mean.png" width="200" /></p>
To describe a set of random numebrs $xi$ we are often interested in two things:

- the mean value 

$$
x_{m} = \frac{1}{n}\displaystyle\sum_{j=1}^{n-1}x_j  ​
$$

- the 'mean deviation' from the mean value (standard deviation)

$$
x_{s} =\sqrt{\frac{1}{n}\displaystyle\sum_{j=1}^{n-1}(x_j-x_m)^2}
$$

### Statistical Calculation

#### Formulae

As a means to derive the formulae, a single probability from the overarching question was used to distill the  formula. 

**Example 1**

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

##### Hypergeometric Distribution

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

##### Binomial Distribution  

With regards to bionomial distribution, a committee of nodes were drawn from the total nodes with replacement; i.e. nodes are drawn, the intention of the node is distinguished and   then the node is returned to the total nodes. 
$$
P = {{n}\choose{T}}\cdot\biggl(\frac{m}{n}\biggr)^{T}\cdot\biggl(\frac{N-m}{n}\biggr)^{n-T}
$$

##### Summation 

$$
P_{tot} = \sum_{i=T}^{n} P(N,m,n,i)
$$

show a hypergeometric distribution graph

#### Explanation of hypergeometric distribution

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



## Implementation

#### Crude Monte Carlo Simulation

<u>Example data</u> 

Total nodes= 100       

Bad nodes= 60

Committee Size= 3

Threshold= 2

P_tot= 0.649474335188621  

Therefore y= 0.649474335188621  

```Text
What is the total amount of nodes? 100

What is the amount of bad nodes? 60

How many nodes are to be drawn? 3

What is the BFT threshold within the committee? 2

What is the no of draws within an experiment? 1000

How many experiments? 1000

Do you know theoratical mean? Y|N: Y

What is the theoretical mean?0.649474335188621
```

##### Individual Probabilities 

<p align="center"><img src="assets/individual_probability_hypergeometric.png" width="700" /></p>
##### A histogram and visualization of distribution 

Histogram: divide `[0,1)` into $n_{i}$ small subintervals, generate $N$ numbers, count how many numbers that fall in each subinterval (and divide the counts  by $N$) - plot the count variation and see if the curve is flat  

<p align="center"><img src="assets/histogram_of_randomness.png" width="700" /></p>
| **Statistical Information ** |                   |
| ---------------------------- | ----------------- |
| Mean                         | 120000.0          |
| Median                       | 119991.0          |
| Mode                         | -                 |
| Standard Deviation           | 346.4313595341606 |

##### Proving the Law of Large Numbers

<p align="center"><img src="assets/convergence.png" width="700" /></p>

Show two graphs one with small number of experiments and one with a large number of experiments. 

##### Summary of Distribution Data 

**Uniform Distribution** 

| Statistical Information |                       | Comparison with theoretical mean |
| ----------------------- | --------------------- | -------------------------------- |
| Slope                   | -1.87686187686188e-07 |                                  |
| Intercept               | 0.6497887492507493    | 0.649474335188621                |
| Standard Deviation      | 0.015438728229013219  |                                  |

**Hypergeometric Distribution** 

| Statisical Information |                      | Comparison with theoretical mean |
| ---------------------- | -------------------- | -------------------------------- |
| Slope                  | -8.279948279948292e-07 |                                  |
| Intercept              | 0.6495665834165834 | 0.649474335188621                |
| Standard Deviation     | 0.014812123075035204 |                                  |

**Poisson**

| Statisical Information |                      | Comparison with theoretical mean |
| ---------------------- | -------------------- | -------------------------------- |
| Slope                  | -1.411267411267412e-06 |                                  |
| Intercept              | 0.6501259280719281 | 0.649474335188621                |
| Standard Deviation     | 0.015233575444419514 |                                  |

**Normal**

| Statisical Information |                      | Comparison with theoretical mean |
| ---------------------- | -------------------- | -------------------------------- |
| Slope                  | 1.5411855411855414e-06 |                                  |
| Intercept              | 0.6482901778221778 | 0.649474335188621                |
| Standard Deviation     | 0.01507612979811762 |                                  |

#### Statistical Calculation

- Certain variables that remain constant 
  - Total nodes, *N* = 100 
  - Bad nodes, *m* = 60
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

##### Probabilistic Attack Graphs 

Graph Variables:

N = 500

m = 50% of N

T = 50% of N

n = increases from 10-500 

From a plot of committee size versus probability of bad actors controlling the network it can be seen that:

- As the commitee size increases, the probability increases 
- Thus, the greater the sample size the greater the chance of there being bad nodes in the set drawn 

<p align="center"><img src="assets/probability-1.png" width="700" /></p>
**Vary the total nodes**

From a plot of committee size versus probability with a change in $N$, the total number of nodes, it can be seen that:

<p align="center"><img src="assets/probability-2.png" width="700" /></p>
**Vary the BFT threshold**

<p align="center"><img src="assets/probability-3.png" width="700" /></p>
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

**Vary the total number of nodes, with committee size of 10**

<p align="center"><img src="assets/committee_size_10.png" width="700" /></p>
**Vary the total number of nodes, with committee size of 100**

<p align="center"><img src="assets/committee_size_100.png" width="700" /></p>

**Vary the bad nodes, with committee size of 100**

<p align="center"><img src="assets/bad_actor_grid.png" width="700" /></p>



**Variation of percentage of  the bad nodes, when committee size is 10 and 100**

<p align="center"><img src="assets/bad_actors_varied_committee_size_10_100.png" width="700" /></p>

## 

## Conclusion



## References

[[1]] Wikipedia:, “List of probability distributions” [online]. Available: <https://en.wikipedia.org/wiki/List_of_probability_distributions>. 
Date accessed: 2019&#8209;05&#8209;13.

[1]: https://en.wikipedia.org/wiki/List_of_probability_distributions
"List of probability distributions"

[[2]] Wikipedia, “Support (mathematics)" [online]. Available: <https://en.wikipedia.org/wiki/Support_(mathematics)>. 
Date accessed: 2019&#8209;05&#8209;13.

[2]: https://en.wikipedia.org/wiki/Support_(mathematics)
"Support (mathematics)"

[[3]] Wikipedia, “Hypergeometric distribution” [online]. Available: <https://en.wikipedia.org/wiki/Hypergeometric_distribution>. 
Date accessed: 2019&#8209;05&#8209;13.

[3]: https://en.wikipedia.org/wiki/Hypergeometric_distribution
"Hypergeometric distribution"

[[4]] Wikipedia, “Binomial distribution" [online]. Available: <https://en.wikipedia.org/wiki/Binomial_distribution>. 
Date accessed: 2019&#8209;05&#8209;13.

[4]: https://en.wikipedia.org/wiki/Binomial_distribution
"Binomial Distribution"

[[5]] POA Network Team, "POA Network: HoneyBadger BFT and Threshold Cryptography" [online]. Availiable:<https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87>. Date accessed: 2019&#8209;06&#8209;28.

[5]: https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87
"POA Network: HoneyBadger BFT and Threshold Cryptography"

[[6]]P. Hanbury, "Monte Carlo Simulations with Python (Part 1)" [online]. Available: <https://towardsdatascience.com/monte-carlo-simulations-with-python-part-1-f5627b7d60b0>. Date accessed: 2019&#8209;06&#8209;28.

[6]:  https://towardsdatascience.com/monte-carlo-simulations-with-python-part-1-f5627b7d60b0
"Monte Carlo Simulations with Python (Part 1)"

[[7]] W. Kenton, "Law of Large Numbers" [online]. Available: <https://www.investopedia.com/terms/l/lawoflargenumbers.asp>. Date accessed: 2019&#8209;06&#8209;28.

[7]: https://www.investopedia.com/terms/l/lawoflargenumbers.asp
"Investopia: Law of Large Numbers"

[[8]]Wikipedia, "Law of Large Numbers" [online]. Available: <https://en.wikipedia.org/wiki/Law_of_large_numbers>. Date accessed: 2019&#8209;06&#8209;28.

[8]: https://en.wikipedia.org/wiki/Law_of_large_numbers
"Law of Large Numbers"

[[9]]Wikipedia, "Law of Large Numbers- Average dice roll by number of rolls" [online]. Available: <https://commons.wikimedia.org/w/index.php?curid=58536069>. Date accessed: 2019&#8209;06&#8209;28.

[9]: https://commons.wikimedia.org/w/index.php?curid=58536069
"Law of Large Numbers- Average dice roll by number of rolls"

[[10]]"Hypergeometric Distribution" [online]. Available: <https://stattrek.com/probability-distributions/hypergeometric.aspx>. Date accessed: 2019&#8209;06&#8209;28.

[10]: https://stattrek.com/probability-distributions/hypergeometric.aspx
"Hypergeometric Distribution" 

[[11]]"Continuous and Discrete Probability Distributions" [online]. Available:<https://support.minitab.com/en-us/minitab-express/1/help-and-how-to/basic-statistics/probability-distributions/supporting-topics/basics/continuous-and-discrete-probability-distributions/>. Date accessed: 2019&#8209;07&#8209;18.

[11]: https://support.minitab.com/en-us/minitab-express/1/help-and-how-to/basic-statistics/probability-distributions/supporting-topics/basics/continuous-and-discrete-probability-distributions/
"Continuous and Discrete Probability Distributions" 

[[12]]Wikipedia, "Probability Density Function" [online]. Available: <https://en.wikipedia.org/wiki/Probability_density_function>. Date accessed: 2019&#8209;07&#8209;18.

[12]: https://en.wikipedia.org/wiki/Probability_density_function
"Probability Density Function" 

[[13]]Wikipedia, "Gambler's Fallacy" [online]. Available: <https://en.wikipedia.org/wiki/Gambler%27s_fallacy>. Date accessed: 2019&#8209;07&#8209;18.

[13]: https://en.wikipedia.org/wiki/Gambler%27s_fallacy
"Gambler's Fallacy" 

[[14]]C. Sharrock and P. Robinson, "Digital Assets Network" [online]. Available: <https://rfc.tari.com/RFC-0300_DAN.html>. Date accessed: 2019&#8209;07&#8209;18.

[14]: https://rfc.tari.com/RFC-0300_DAN.html

"Tari RFC- Digital Assets Network" 

[[15]]S. Bondi, "Distributed Hash Table" [online]. Available: <https://tlu.tarilabs.com/protocols/dht/MainReport.html>. Date accessed: 2019&#8209;07&#8209;18.

[15]: https://tlu.tarilabs.com/protocols/dht/MainReport.html

"Distributed Hash Table" 

## Appendices

Appendix A: Definitions of Terms 

Definitions of terms peresentated here are high level and general in nature. Fill statistical definitions are available in the cited references. 

- **Probability Density Function:**<a name="pdf"> </a> A statistical expression that defines a probability distribution for a continuous random variable instead of a. discrete random variable. [[11]][[12]]

[pdf~]: #pdf
" A statistical expression that 
defines a..." 


## Contributions

- <https://github.com/kevoulee>
