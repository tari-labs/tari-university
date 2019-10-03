<style>
        div.wrap_bad {
            width: 95%; 
            word-wrap: break-word;
            background: #3D85C6;
            font-size: 1.0em;
            padding: 0.5em;
            color: #000000;
            }
        </style>

<style>
        div.wrap_good {
            width: 95%; 
            word-wrap: break-word;
            background: #6AA84F;
            font-size: 1.0em;
            padding: 0.5em;
            color: #000000;
            }
        </style>

<style>
        div.highlight {
            width: 95%; 
            word-wrap: break-word;
            background: #EA9999;
            font-size: 1.0em;
            padding: 0.5em;
            color: #000000;
            }
        </style>



# Probabilistic Attack Vector Analysis Building Blocks

- [Introduction](#introduction)
  - [Aim](#aim)
  - [Question](#question) 
- [Literature Review](#literature-review)
  - [Tari Digital Assets Network](#tari-digital-assets-network) 
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
    - [Crude Monte Carlo](#crude-monte-carlo)
    - [Law of Large Numbers](#law-of-large-numbers)
- [Methodology](#methodology)
   - [Notation Used](#notation-used)
   - [Crude Monte Carlo Simulation](#crude-monte-carlo-simulation)
     - [Programming the Simulation](#programming-the-simulation)
     - [Use of Random Numbers](#use-of-random-numbers)
     - [Computing Statistics](#computing-statistics)
   - [Statistical Calculation](#statistical-calculation)
     - [Formulae](#formulae)
     - [Distribution](#distribution)
       - [Binomial Distribution](#binomial-distribution)
       - [Hypergeometric Distribution](#hypergeometric-distribution)
       - [Summation](#summation)
- [Implementation](#implementation)
  - [Crude Monte Carlo Simulation](#crude-monte-carlo-simulation-1)
    - [Proving the Law of Large Numbers](#proving-the-law-of-large-numbers)
    - [Individual Probabilities](#individual-probabilities)
    - [Histogram and Visualization of Distribution](#histogram-and-visualization-of-distribution)
  - [Statistical Calculation](#statistical-calculation)
     - [Variation of Total Nodes](#variation-of-total-nodes)
     - [Variation of Byzantine Fault Tolerance Threshold](#variation-of-byzantine-fault-tolerance-threshold)
     - [Variation of Total Number of Nodes with Committee Size 10](#variation-of-total-number-of-nodes-with-committee-size-10)
     - [Variation of Total Number of Nodes with Committee Size 100](#variation-of-total-number-of-nodes-with-committee-size-100)
     - [Variation of Bad Nodes with Committee Size 10 and 100](#variation-of-bad-nodes-with-committee-size-10-and-100)
- [Conclusions and Remarks](#conclusions-and-remarks)
- [References](#references)
- [Appendices](#appendices)
   - [Appendix A: Definitions of Terms](#appendix-a-definitions-of-terms)
- [Contributors](#contributors) 

## **Introduction**

### **Aim** 

This research aims to provide answers to questions posed about the workings of the Tari Digital Assets Network (DAN) 
environment: probabilistic attack vector with regard to the total nodes, compromised nodes, committee size and 
Byzantine Fault-tolerance (BFT) threshold. 

### **Question**

This investigation attempts to answer the following question:

*What is the percentage chance of controlling the majority of nodes in a random sample with varying quantities of the total 
number of nodes, committee size, bad nodes and BFT threshold?*

## Literature Review 

### Tari Digital Assets Network 

The Tari Digital Assets Network (DAN) forms part of the Tari second layer, where the management of all digital 
asset interactions takes place. 

These interactions are processed and validated by committees of special nodes, called Validator Nodes (VNs). Management 
of Digital Assets (DAs) involves state changes and ensures enforcement of the rules that govern assets contracts. 
Thus, all actions on this network are due to the interactions of the VNs. 
The registration of VNs occurs on the base layer. To prevent Sybil attacks, they commit collateral. If 
proved that the VN engaged in malicious behavior, the VN will lose its collateral. 

An Asset Issuer (AI) would then issue DAs and draw up a contract. The AI will dictate the size of 
the committee of VNs for a particular DA. The AI will also have the ability to nominate a trusted node to 
form part of the VN committee for the DA [[14]].

#### Kademlia

Kademlia was designed by Petar Maymounkov and David Mazières in 2002 [[16]]. It is a distributed hash table, used for 
decentralized, peer-to-peer computer networks. 

#### Node ID

A node selects an $n$-bit ID, given to nodes on the network. Node IDs have uniformly distributed numbers. 
A node's position is determined by a unique prefix of its ID, which forms a tree structure, with node IDs as leaves. 

The bit length of the node ID should be sufficiently large to make collisions unlikely when using a uniformly 
distributed random number generator [[15]].

##### Bootstrapping a Node

A bootstrap node is a node listed on a predetermined list, and serves as the first point of contact for a new node. The node bootstrapping process is as follows:

- To establish itself on the network without any known contacts, a node needs to contact at least one bootstrap node, 
  requesting an introduction to the network.
- A node ID is generated for the joining node.
- The new node contacts other nodes it is aware of.
- The new node sends a lookup request with its newly generated node ID.
- The contacted nodes return the nodes they know about that are closest.
- The new nodes are added to the routing table, and contacting begins.
- The process continues until the joining node is unable to locate any closer nodes.

This 
*self-lookup* has two effects:

- it allows the node to learn about nodes closer to itself; and
- it populates other nodes' 
  routing tables with the node's ID [[15]].

##### XOR Metric

The Kademlia paper, published in 2002 [[16]], contained the novel idea of using the XOR operator to determine the 
distance and therefore the arrangement of peers within the network. 

Through the XOR metric, a distance is captured. The lookup procedure allows nodes to locate other nodes, 
given a node ID [[15]].


### Types of Distribution 

When considering solving the probability of an attacker controlling the majority of nodes in the network, the various 
types of probability distributions of the specific circumstances and variables of the 
problem need to be analyzed. There are two categories of probability distribution: finite and infinite support [[1]]. Here support is 
defined as a real-valued function *f*, which is the subset of the domain containing those elements that are not mapped 
to zero. If the domain of *f* is a topological space, the support of *f* is instead defined as the smallest closed set 
containing all points not mapped to zero [[2]]. 

#### Continuous Distribution 

A continuous random variable is a random variable with an infinite and uncountable set and range of possible values [[11]].

Probabilities of continuous random variables (*X*) are defined as the area under the curve of its Probability Density 
Function<sup>[def][pdf~]</sup> (PDF). Therefore only ranges of values can have a nonzero probability. The probability that 
a continuous random variable equals some value is always zero [[11]].

<p align="center"><img src="assets/distribution_plot_normal_weight_shade_middle.png" width="700" /></p>

#### Discrete Distribution 

Likewise, to understand the discrete distribution, a discrete random variable requires definition. A discrete random 
variable is a random variable that has countable values, such as a list of non-negative integers. A discrete distribution thus describes the probability of occurrence of each value of a discrete random variable [[11]].

With a discrete probability distribution, each possible value of the discrete random variable can be associated with a 
nonzero probability. Thus, the presentation of a discrete probability distribution is often in tabular form [[11]].

<p align="center"><img src="assets/distribution_plot_poisson_shade_right_tail.png" width="700" /></p>

Examples of discrete distribution with **finite** support include the following: 

| Type of Finite Discrete Distribution | Description   [[1]]                                          |
| ------------------------------------ | ------------------------------------------------------------ |
| Bernoulli Distribution               | Takes value 1 with probability $p$ and value 0 with probability $q=1-p$. |
| Rademacher Distribution              | Takes value 1 with probability $\frac{1}{2}$ and value $-1$ with probability $\frac{1}{2}$. |
| Binomial Distribution                | The number of successes in a series of independent Yes/No experiments, all with the same probability of success. |
| Beta-Binomial Distribution           | The number of successes in a series of independent Yes/No experiments with heterogeneity in the success probability. |
| Degenerate Distribution             | At $X0$, where $X$ is certain to take the value $X0$. This does not look random, but it satisfies the definition of a random variable. This is useful because it puts deterministic variables and random variables in the same formalism. |
| Discrete Uniform Distribution        | Where all elements of a finite set are equally likely. This is the theoretical distribution model for a balanced coin, an unbiased die, a casino roulette, or the first card of a well-shuffled deck. |
| Hypergeometric Distribution          | The number of successes in the first $m$ of a series of $n$ consecutive Yes/No experiments, if the total number of successes is known. This distribution arises where there is no replacement. |
| Poisson Binomial  Distribution       | The number of successes in a series of independent Yes/No experiments with different success probabilities. |

Examples of discrete distribution with **infinite** support include the following: 

| Type of Infinite Discrete Distribution | Description [[1]]                                            |
| -------------------------------------- | ------------------------------------------------------------ |
| Boltzmann Distribution                 | A discrete distribution important in statistical physics, which describes the probabilities of the various discrete energy levels of a system in thermal equilibrium. It has a continuous analog. |
| Geometric Distribution                 | A discrete distribution that describes the number of attempts needed to get the first success in a series of independent Bernoulli trials or, alternatively, only the number of losses before the first success (i.e. one less). |
| Negative Binomial/Pascal Distribution  | A generalization of the geometric distribution of the $nth$ success.             |```
| Poisson Distribution                   | A very large number of individually unlikely events that happen at a specific time interval. Related to this distribution are several other distributions: the displaced Poisson, the hyper-Poisson, the general Poisson binomial and the Poisson-type distributions. |
| Skellam Distribution                   | The distribution of the difference between two independent Poisson-distributed random variables. |
| Zeta Distribution                      | Has uses in applied statistics and statistical mechanics, and perhaps may be of interest to number theorists. It is the Zipf distribution for an infinite number of elements. |
| Zipf's Law                             | A discrete power-law distribution, the most famous example of which is the description of the frequency of words in the English language. |
| Zipf-Mandelbrot Law                    | A discrete power-law distribution, which is a generalization of the Zipf distribution. |

##### Hypergeometric Distribution

A hypergeometric distribution is a discrete probability distribution that describes the probability of $T$ successes (random 
draws for which the object drawn has a specified feature) in $n$ draws, *without* replacement, from a finite population 
of size $N$ that contains exactly $m$ objects with that feature, wherein each draw is either a success or a failure [[3]]:

- A sample of size $n$ is randomly selected without replacement from a population of $N$ items.
- In the population, $T$ items can be classified as successes and $N-T$ items can be classified as failures.

Given $x, N, n$ and $k$, the hypergeometric probability can be computed based on the following example: 

**Example:** Suppose a population consists of $N$ items, $k$ of which are successes; and a random sample drawn from that 
population consists of $n$ items, $x$ of which are successes. Then the hypergeometric probability is [[10]]:
$$
h(m; N, n, T)= {{T}\choose{m}}{{N-T}\choose{n-m}}{{N}\choose{n}}
$$
The hypergeometric distribution has the following properties:

- The mean of the distribution is equal to $n\cdot\frac{T}{N}$
- The variance is $ n\cdot T \cdot(N-T)\cdot\frac{N-n}{N^2\cdot(N-1)}$

##### Binomial Distribution

The binomial distribution with parameters $n$ and $p$ is the discrete probability distribution of the number of 
successes in a sequence of $n$ independent experiments, each asking a yes-no question, and each with its own 
Boolean-valued outcome: success/yes/true/one (with probability $p$) or failure/no/false/zero (with probability $q=1- p$). 
A single success/failure experiment is also called a Bernoulli trial or Bernoulli experiment, and a sequence of outcomes 
is called a Bernoulli process. For a single trial, i.e. $n=1$, the binomial distribution is a Bernoulli distribution. 
The binomial distribution is the basis for the popular binomial test of statistical significance. 

The binomial distribution is frequently used to model the number of successes in a sample of size $n$ drawn with 
replacement from a population of size $N$. If the sampling is carried out without replacement, the draws are not 
independent, and so the resulting distribution is hypergeometric, not binomial. However, for $n$ much larger than 
$n$, the binomial distribution remains a good approximation and is widely used. 

Thus, in a binomial distribution, an object is selected with replacement [[4]].  A binomial experiment requires that the 
probability of success be constant on every trial. 

**Example:** You have an urn containing $10$ marbles - $5$ red and $5$ green. You randomly select $2$ marbles with replacement, and the 
probability of success would not change. It would be $\frac{5}{10}$ on every trial [[10]]. 

### Monte Carlo Simulations 

The Monte Carlo approach is a computer-based analytical method that was developed in the 1940s as part of the atomic program, by a scientist at the Los Alamos National Laboratory, who used it to 
model the random diffusion of neutrons. It was named after the city in Monaco and its many casinos. 

Monte Carlo analysis uses statistical tools to model a real-life system or process mathematically, and then estimates the probability of obtaining a successful outcome. The statistical distribution of the process to be modeled 
must be determined before the Monte Carlo simulation can be applied. 

Monte Carlo methods are widely used heuristic techniques that can solve a variety of common problems, including 
optimization and numerical integration problems. These algorithms work by cleverly sampling from distribution to 
simulate the workings of a system. Applications range from solving problems in theoretical physics to predicting trends 
in financial investments [[6]]. 

#### Monte Carlo Fallacy 

The Monte Carlo Fallacy or gambler's fallacy is the inaccurate belief that if something happens more frequently than 
normal during a given period, it will happen less often in the future. In situations where the outcome being observed is 
truly random and consists of independent trials of a random process, this belief is false. The fallacy can arise in many 
situations but is most strongly associated with gambling, where it is common among players [[13]]. 

The gambler's fallacy can be illustrated by considering the repeated toss of a fair coin. The outcomes in different 
tosses are statistically independent, and the probability of getting heads on a single toss is $\frac{1}{2}$. The 
probability of getting two heads in two tosses is $\frac{1}{4}$, and the probability of getting three heads in three 
tosses is $\frac{1}{8}$. If, after tossing four heads in a row, the next coin toss also came up heads, it would complete 
a run of five successive heads. Since the probability of a sequence of five consecutive heads is $\frac{1}{32}$, a 
person might believe that the next flip would more likely come up tails rather than heads again. This is incorrect 
and is an example of the gambler's fallacy. The events "five heads in a row" and "first four heads, then a tails" 
are equally likely, each having a probability of $\frac{1}{32}$. Since the first four tosses turn up heads, the probability 
that the next toss is a head is $\frac{1}{2}$ . While a run of five heads has a probability of $\frac{1}{32} = 0.03125$, 
the misunderstanding lies in not realizing that this is the case only before the first coin is tossed. After the first 
four tosses, the results are no longer unknown, so the probability at that point is equal to 1. The reasoning that 
it is more likely that a fifth toss is more likely to be tails because the previous four tosses were heads, with a run 
of luck in the past influencing the odds in the future, forms the basis of the fallacy [[13]].

#### Crude Monte Carlo

The Monte Carlo technique is built upon this principle: instead of evaluating an indefinite integral, which can 
sometimes be impossible, the average of the integrand is estimated, and that is used to approximate the integral. If one 
needs to be more precise, the number of samples can be increased. 

It is a widely used heuristic technique that can solve a variety of common problems including optimization and numerical 
integration problems. These algorithms work by cleverly sampling from a distribution to simulate the workings of a system.  
Applications range from solving problems in theoretical physics to predicting trends in financial investments. 

#### Law of Large Numbers 

The Law of Large Numbers (LLN), in probability and statistics, states that as a sample size grows, its mean gets closer 
to the average of the whole population. In statistical analysis, the LLN can be applied to a variety of 
subjects. It may not be feasible to poll every individual within a given population to collect the required amount of 
data. However, every additional data point gathered has the potential to increase the likelihood that the outcome is an 
accurate measure of the mean [[7]]. 

The LLN is crucial because it guarantees stable, long-term results for the averages of some random event [[8]]. 

<p align="center"><img src="assets/law_of_large_numbers.png" width="650" /></p>
The preceding figure illustrates the LLN using a particular run of rolls of a single dice. As can be seen in the figure, as the number of rolls in this run increases, the average of the values of all the results approaches 3.5. While different runs would show a different shape over a small number of throws (at the left), over a large number of rolls (to the right), they would be extremely similar [[9]].

## Methodology 
### Notation Used  


This section gives the general notation of statistical expressions when specifically referenced. This information 
is important knowledge for the remainder of the report. 

- Let $N$ be the total number of nodes in the network. 
- Let $n$ be the committee size.
- Let $m$ be the number of bad actors.
- Let $T$ be the BFT threshold.

There is a pool with *N* nodes. The pool contains *m* malicious nodes or bad actors. From within the pool, a random 
selection of nodes, *n* is drawn. From this selection, the probability of drawing a threshold of bad actors, *T*, needs to 
be calculated.  

### Crude Monte Carlo Simulation

#### Programming the Simulation

It was initially thought that selecting a committee ($n$) from the total nodes ($N$) without replacing the selected nodes 
requires the removal of an element from the pool of total nodes when it is drawn. However, as the program is 
calling for many selections within many experiments, this logic could not be used. 

The experiment is extended to ask the overarching question: *"What is the probability of selecting the threshold worth of* 
*bad nodes or more from a pool of total nodes?"* To this end, we perform a variable number of experiments and 
count how many times the threshold is met, in order to estimate the probability. 

#### Use of Random Numbers  

Some problems in science and technology are described by "exact" mathematics, leading to "precise" results, e.g. 
throwing a ball and oscillating a system. Some problems appear physically uncertain, e.g. rolling a die and molecular motion. 
Random numbers can be used to mimic the uncertainty of the experiment. 

Random numbers make it possible to simulate physical systems with uncertainty, in input data or the process. 

#### Computing Statistics 

<p align="center"><img src="assets/mode_median_mean.png" width="170" /></p>
To describe a set of random numbers $xi$, we are often interested in two things:

- Mean value 

$$
x_{m} = \frac{1}{n}\displaystyle\sum_{j=1}^{n-1}x_j  ​
$$

- "Mean deviation" from the mean value (standard deviation)

$$
x_{s} =\sqrt{\frac{1}{n}\displaystyle\sum_{j=1}^{n-1}(x_j-x_m)^2}
$$

### Statistical Calculation

#### Formulae


As a means to derive the formulae, a single probability from the overarching question was used, as shown in the following example. 

**Example 1**: What is the probability of selecting a majority of bad nodes from a total of $5$ nodes if the committee size is $3$? There 
are $3$ bad nodes $(B1, B2, B3)$ and $2$ good nodes $(G1, G2)$.


The first step is to calculate the number of combinations where bad and good nodes can be chosen: 

|                                  |                                   |                                   | Are bad nodes in the majority? |
| :------------------------------: | :-------------------------------: | :-------------------------------: | :----------------------------: |
| <div class="wrap_bad">$B1$</div> | <div class="wrap_bad">$B2$</div>  | <div class="wrap_bad">$B3$</div>  |              Yes               |
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

From this list, the number of combinations where $B$ is the majority can then be tallied. In this case, there are $7$
combinations where $B$ is the majority. Thus, from the $10$ combinations, there are $7$ combinations where there is a 
majority of bad nodes. Therefore, the quotient of $7$ and $10$ is the probability $0.7$. 

This method is limited in calculating the probability where the variables are large. For example if the same question was posed, 
but one had to calculate the probability of selecting a majority of bad nodes from a total of $100$ nodes, with a 
committee size of $60$, $60$ bad nodes and $40$ good nodes, the number of combinations where bad and good nodes can be 
chosen is $1.27E+28$.

#### Distribution

##### Binomial Distribution  

With regard to binomial distribution, a committee of nodes is drawn from the total nodes with replacement, i.e. nodes 
are drawn, the intention of the node is distinguished and the node is returned to the total nodes. 
$$
P = {{n}\choose{T}}\cdot\biggl(\frac{m}{n}\biggr)^{T}\cdot\biggl(\frac{N-m}{n}\biggr)^{n-T}
$$

##### Hypergeometric Distribution

With regard to hypergeometric distribution, a committee of nodes is drawn from the total nodes without replacement, 
i.e. nodes are drawn simultaneously, the intention of the node is distinguished and not returned to the total nodes. This closely mimics the events that would take place within the network. 
$$
P = \frac{{{m}\choose{T}}\cdot{{N-m}\choose{n-T}}}{{N}\choose{n}}
$$

##### Summation 

Refering to *Example 1*, the **Tally** is the sum of all the combinations where the bad nodes are in a majority.  

As a recap to the question, *what is the probability of selecting a majority of bad nodes from a total of 100 nodes if the committee size is $10$*? This problem considers solving for the probability where there is a **majority** of bad nodes in the committee. This entails calculating the probabilities from the BFT threshold to the committee size. Thus, there needs to be a summation of individual probabilities in order to calculate the probability for selecting the majority of bad nodes. 

$$
P_{tot} = \sum_{i=T}^{n} P(N,m,n,i)
$$

In order to understand this, the table below provides some visual insight. 

| Committee Size | BFT Threshold | No of Steps |
| -------------- | ------------- | ----------- |
| 10             | 7             | 3           |

## Implementation

### Crude Monte Carlo Simulation

#### Proving the Law of Large Numbers

With the Crude Monte Carlo technique, to gain precision, the number of samples can be increased. Thus, before calculating the probability and drawing comparisons, the sample size, number of draws within an experiment, and the number of experiments can be varied to find an optimal amount. 

Below is the input data inserted into the python programme, where the number of draws within an experiment is $10$, and the number of experiments is $10$ :

```Text
What is the total amount of nodes? 100
What is the amount of bad nodes? 60
How many nodes are to be drawn? 3
What is the BFT threshold within the committee? 2
What is the number of draws within an experiment? 10
How many experiments? 10
Do you know the theoretical mean? Y|N: Y
What is the theoretical mean? 0.649474335188621
```
<p align="center"><img src="assets/small_no_exp_convergence.png" width="700" /></p>
Below is the input data inserted into the python programme, where the number of draws within an experiment is $1,000$, and the number of experiments is $1,000$ :

```Text
What is the total amount of nodes? 100
What is the amount of bad nodes? 60
How many nodes are to be drawn? 3
What is the BFT threshold within the committee? 2
What is the number of draws within an experiment? 1,000
How many experiments? 1,000
Do you know the theoretical mean? Y|N: Y
What is the theoretical mean? 0.649474335188621
```

<p align="center"><img src="assets/convergence.png" width="700" /></p>
In each graph, the cumulative probabilities calculated for normal, uniform, Poisson and hypergeometric distribution are plotted against the number of experiments. The bold blue line represents the mean calculated from theoretical data. 

From the graph where the experiments and draws are equal to $10$ there is weak convergence. While the graph where the experiments and draws are equal to $1,000$ the  Law of Large Numbers is proved, as the sample size grows, convergence with the statistical mean is achieved. 

#### Individual Probabilities 

The graph below highlights the varying probabilities of each experiment conducted for the hypergeometric distribution. The mean of which provides us with the average of the probabilities, which can then be compared to the calculated theoretical mean. 

From a comparison of the mean probability of each distribution with the theoretical mean, it can be seen that the distribution type that closely mimics the theoretical result is hypergeometric. 

As discussed in [Section: Types of Distribution](#types-of-distribution) hypergeometric distribution is where there is no replacement, i.e., nodes are drawn simultaneously, distinguished and not returned to the total nodes pool. 

<p align="center"><img src="assets/individual_probability_hypergeometric.png" width="700" /></p>
#### Uniform Distribution

| Statistical Information |                      | Comparison with Theoretical Mean | Difference  Calculated |
| ----------------------- | -------------------- | -------------------------------- | ---------------------- |
| Intercept               | 0.6497887492507493   | 0.649474335188621                | 3.14414E-4             |
| Standard Deviation      | 0.015438728229013219 |                                  |                        |

#### Hypergeometric Distribution

| Statistical Information |                      | Comparison with Theoretical Mean | Difference Calculated |
| ----------------------- | -------------------- | -------------------------------- | --------------------- |
| Intercept               | 0.6495665834165834   | 0.649474335188621                | 9.22482E-5            |
| Standard Deviation      | 0.014812123075035204 |                                  |                       |

#### Poisson Distribution

| Statistical Information |                      | Comparison with Theoretical Mean | Difference Calculated |
| ----------------------- | -------------------- | -------------------------------- | --------------------- |
| Intercept               | 0.6501259280719281   | 0.649474335188621                | 6.51592E-4            |
| Standard Deviation      | 0.015233575444419514 |                                  |                       |

#### Normal Distribution

| Statistical Information |                     | Comparison with Theoretical Mean | Difference Calculated |
| ----------------------- | ------------------- | -------------------------------- | --------------------- |
| Intercept               | 0.6482901778221778  | 0.649474335188621                | 1.18416E-3            |
| Standard Deviation      | 0.01507612979811762 |                                  |                       |

#### Histogram and Visualization of Distribution 

The histogram of randomness highlights the distribution of good and bad nodes selected in each experiment, highlighting the random nature of the experiment. 

<p align="center"><img src="assets/histogram_of_randomness.png" width="700" /></p>

| Statistical Information |                   |
| ----------------------- | ----------------- |
| Mean                    | 120,000.0         |
| Median                  | 119,991.0         |
| Mode                    | -                 |
| Standard Deviation      | 346.4313595341606 |

### Statistical Calculation

#### Variation of Total Nodes

##### Variables

- N = $100, 300, 500, 1000$
- m = $60$% of N
- T = $67$% of N
- n = ranging from $1$ to $1000$ 

Below is a sample of the data where the total nodes are $100$. The highlighted data was previously used in the Crude Monte Carlo Simulation when supplying the theoretical mean.

|           Total Nodes            |            Bad Nodes            |         Committee Size         |         BFT Threshold          | Probability                                    |
| :------------------------------: | :-----------------------------: | :----------------------------: | :----------------------------: | ---------------------------------------------- |
|               100                |               60                |               1                |               1                | 0.6                                            |
|               100                |               60                |               2                |               2                | 0.3575757575757576                             |
| <div class="highlight">100​</div> | <div class="highlight">60</div> | <div class="highlight">3</div> | <div class="highlight">2</div> | <div class="highlight">0.649474335188621</div> |
|               100                |               60                |               4                |               3                | 0.47343240951488375                            |
|               100                |               60                |               5                |               4                | 0.33162085827770661                            |
|               100                |               60                |               6                |               4                | 0.5443381851334722                             |
|               100                |               60                |               7                |               5                | 0.4153500188485931                             |
|               100                |               60                |               8                |               6                | 0.30661160770090995                            |
|               100                |               60                |               9                |               6                | 0.47996269793634677                            |
|               100                |               60                |               10               |               7                | 0.37423758246308586                            |
|               100                |               60                |               11               |               8                | 0.28361605491457653                            |
|               100                |               60                |               12               |               8                | 0.4320215340178938                             |
|               100                |               60                |               13               |               9                | 0.3409545354772218                             |
|               100                |               60                |               14               |               10               | 0.2623321970180976                             |
|               100                |               60                |               15               |               10               | 0.39288184738975973                            |

<p align="center"><img src="assets/variation_of_total_nodes .png" width="700" /></p>
From a plot of committee size versus probability with a change in $N$, the total number of nodes, it can be seen that the probability is lower with respect to the committee size when $N$ is smaller. 

#### Variation of Byzantine Fault-Tolerance Threshold

- ##### Variables

  - N = $100$
  - m = $60$% of N
  - T = $50$%, $55$%, $60$%, $67$% of N
  - n = ranging from $1$ to $100$ 

<p align="center"><img src="assets/variation_of_bft_threshold.png" width="700" /></p>
From a plot of committee size versus probability where the number of nodes remains at 100 with a change in $T$, the BFT 
threshold, ranging from $50$% to $67$%, it can be seen that: When the BFT threshold is $50$% and $55$% the probability is low when the committee size is small; as the committee size increases, the probability increases, and tends to one. The probability is higher for the case where the BFT threshold is $50$% than when the probability is $55$%. 

When the BFT threshold is $60$%, the probability decreases from $0.63$ to approximately $0.59$, where it remains constant. 

When the BFT threshold is $65$% and $67$%, the probability decreases from $0.38$ and tends to zero. Thus, in order to ensure a low probability, the BFT threshold needs to be higher than $60$%.  

#### **Variation of Total Number of Nodes with Committee Size 10**

##### Variables

- N = ranging from $0$ to $350$
- m = $60$% of N
- T = $67$% of N
- n = $10$

<p align="center"><img src="assets/committee_size_10.png" width="700" /></p>
For the graph showing varying probabilities with respect to the total number of network nodes, where the committee size is $10$, the probability dramatically increases when the total nodes is $0$ to$100$. The probability plateaus at 0.35. 

#### Variation of Total Number of Nodes with Committee Size 100

##### Variables

- N = ranging from $0$ to $1300$
- m = $60$% of N
- T = $67$% of N
- n = $100$

<p align="center"><img src="assets/committee_size_100.png" width="700" /></p>
From the two graphs, it can be seen that probabilities are significantly lower when the committee size is 100. There is an increase in probability between $0$ and $700$, albeit, not as steep as the change when the committee size is 10.  The probability plateaus at 0.08.

The larger the committee size, the less dramatic changes there are in the probability. 

#### Variation of Bad Nodes with Committee Size 10 and 100

##### Variables

- N = ranging from $0$ to $50,000$
- m = $10$%, $20$%, $30$%, $40$%, $50$%, $60$%, $70$%, $80$% and $90$% of N
- T = $67$% of N
- n = $10$ and $100$

<p align="center"><img src="assets/bad_actor_grid.png" width="700" /></p>
These graphs show varying probabilities when the number of bad nodes is $20$, $40$, $60$ and $90$. The value when the probability plateaus is used to construct the graph below for both committee sizes $10$ and $100$. 

<p align="center"><img src="assets/bad_actors_varied_committee_size_10_100.png" width="700" /></p>
The graph shows changes in the probability due to changes in % of bad nodes when the committee size is $10$ and $100$.  When the committee size is 10, there is a change in probability when the bad node percentage is between $30$ and $80$.  When the committee size is 100, there is a steep increase in the probability when the bad node percentage is between $50$ and $80$.  When the committee size is $100$, the probability remains lower as the bad node percentage increases and has a steeper gradient when the change in probability occurs. Whereas, when the committee size is $10$, the probability begins to increase at a lower percentage of bad nodes. 

## Conclusions and Remarks 

With regards to the Crude Monte Carlo Simulation, at this building block stage, probabilities were calculated and distributions of nodes within the network illustrated.

With regards to the statisical calculation, comments can be made for each of the varied parameters. 

- Total nodes in the network: the smaller the pool of total nodes in the network, the lower the probability; however, the probability difference is near negligible if the committee size is large. Also, this parameter will be difficult to control, and the network will be ever-increasing 
- BFT threshold: this threshold should be higher than $67$% as per literature.
- Committee size: the larger the committee size, the lower the probability bad nodes controlling the network
- Bad nodes: while this variable cannot be controlled, the probability can remain low as the percentage of bad nodes increase if the committee size approx $100$ or larger. 

## References

[[1]] Wikipedia, “List of Probability Distributions” [online]. Available: <https://en.wikipedia.org/wiki/List_of_probability_distributions>. 
Date accessed: 2019&#8209;05&#8209;13.

[1]: https://en.wikipedia.org/wiki/List_of_probability_distributions
"List of Probability Distributions"

[[2]] Wikipedia, “Support (Mathematics)" [online]. Available: <https://en.wikipedia.org/wiki/Support_(mathematics)>. 
Date accessed: 2019&#8209;05&#8209;13.

[2]: https://en.wikipedia.org/wiki/Support_(mathematics)
"Support (Mathematics)"

[[3]] Wikipedia, “Hypergeometric Distribution” [online]. Available: <https://en.wikipedia.org/wiki/Hypergeometric_distribution>. 
Date accessed: 2019&#8209;05&#8209;13.

[3]: https://en.wikipedia.org/wiki/Hypergeometric_distribution
"Hypergeometric Distribution"

[[4]] Wikipedia, “Binomial Distribution" [online]. Available: <https://en.wikipedia.org/wiki/Binomial_distribution>. 
Date accessed: 2019&#8209;05&#8209;13.

[4]: https://en.wikipedia.org/wiki/Binomial_distribution
"Binomial Distribution"

[[5]] POA Network Team, "POA Network: Honey Badger BFT and Threshold Cryptography" [online]. Available: 
<https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87>. 
Date accessed: 2019&#8209;06&#8209;28.

[5]: https://medium.com/poa-network/poa-network-honey-badger-bft-and-threshold-cryptography-c43e10fadd87
"POA Network: HoneyBadger BFT and Threshold Cryptography"

[[6]] P. Hanbury, "Monte Carlo Simulations with Python (Part 1)" [online]. Available: 
<https://towardsdatascience.com/monte-carlo-simulations-with-python-part-1-f5627b7d60b0>. Date accessed: 2019&#8209;06&#8209;28.

[6]:  https://towardsdatascience.com/monte-carlo-simulations-with-python-part-1-f5627b7d60b0
"Monte Carlo Simulations with Python (Part 1)"

[[7]] W. Kenton, "Law of Large Numbers" [online]. Available: <https://www.investopedia.com/terms/l/lawoflargenumbers.asp>. 
Date accessed: 2019&#8209;06&#8209;28.

[7]: https://www.investopedia.com/terms/l/lawoflargenumbers.asp
"Investopedia: Law of Large Numbers"

[[8]] Wikipedia, "Law of Large Numbers" [online]. Available: <https://en.wikipedia.org/wiki/Law_of_large_numbers>. 
Date accessed: 2019&#8209;06&#8209;28.

[8]: https://en.wikipedia.org/wiki/Law_of_large_numbers
"Law of Large Numbers"

[[9]] Wikipedia, "Law of Large Numbers - Average Dice Roll by Number of Rolls" [online]. 
Available: <https://commons.wikimedia.org/w/index.php?curid=58536069>. Date accessed: 2019&#8209;06&#8209;28.

[9]: https://commons.wikimedia.org/w/index.php?curid=58536069
"Law of Large Numbers - Average Dice Roll by Number of Rolls"

[[10]] Stat Trek, "Hypergeometric Distribution" [online]. Available: <https://stattrek.com/probability-distributions/hypergeometric.aspx>. 
Date accessed: 2019&#8209;06&#8209;28.

[10]: https://stattrek.com/probability-distributions/hypergeometric.aspx
"Hypergeometric Distribution" 

[[11]] Minitab Express Support, "Continuous and Discrete Probability Distributions" [online]. Available: 
<https://support.minitab.com/en-us/minitab-express/1/help-and-how-to/basic-statistics/probability-distributions/supporting-topics/basics/continuous-and-discrete-probability-distributions/>. 
Date accessed: 2019&#8209;07&#8209;18.

[11]: https://support.minitab.com/en-us/minitab-express/1/help-and-how-to/basic-statistics/probability-distributions/supporting-topics/basics/continuous-and-discrete-probability-distributions/
"Continuous and Discrete Probability Distributions" 

[[12]] Wikipedia, "Probability Density Function" [online]. Available: <https://en.wikipedia.org/wiki/Probability_density_function>. 
Date accessed: 2019&#8209;07&#8209;18.

[12]: https://en.wikipedia.org/wiki/Probability_density_function
"Probability Density Function" 

[[13]] Wikipedia, "Gambler's Fallacy" [online]. Available: <https://en.wikipedia.org/wiki/Gambler%27s_fallacy>. Date 
accessed: 2019&#8209;07&#8209;18.

[13]: https://en.wikipedia.org/wiki/Gambler%27s_fallacy
"Gambler's Fallacy" 

[[14]] C. Sharrock and P. Robinson, "Digital Assets Network" [online]. Available: <https://rfc.tari.com/RFC-0300_DAN.html>. 
Date accessed: 2019&#8209;07&#8209;18.

[14]: https://rfc.tari.com/RFC-0300_DAN.html
"Tari RFC - Digital Assets Network" 

[[15]] S. Bondi, "Distributed Hash Tables" [online]. Available: <https://tlu.tarilabs.com/protocols/dht/MainReport.html>. 
Date accessed: 2019&#8209;07&#8209;18.

[15]: https://tlu.tarilabs.com/protocols/dht/MainReport.html
"Distributed Hash Tables" 

[[16]] P. Maymounkov and D. Mazières, "Kademlia: A Peer-to-peer Information System Based on the XOR Metric" [online]. 
Available: <https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf>. Date accessed: 2019‑07‑18.

[16]: "Kademlia'

## Appendices

### Appendix A: Definitions of Terms 

Definitions of terms presented here are high level and general in nature. Full statistical definitions are available 
in the cited references. 

- **Probability Density Function (PDF):**<a name="pdf"> </a> A statistical expression that defines a probability distribution 
for a continuous random variable instead of a discrete random variable ([[11]], [[12]]).

[pdf~]: #pdf
" A statistical expression that 
defines a..." 


## Contributors

- <https://github.com/kevoulee>
- <https://github.com/anselld>
