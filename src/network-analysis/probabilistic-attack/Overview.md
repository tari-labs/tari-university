### Question 

What is the probability of an attacker controlling the majority of nodes in the network? 

### Types of Distribution 

##### Hypergeometric Distribution:

Selecting nodes without replacement, i.e. selecting all 6 nodes at once

##### Binomial Distribution:

Selecting nodes with replacement, i.e. selecting each node, noting whether it is malicious or friendly and returning back to the committee.

### Variables 

- Let $N$ be the total number of nodes in the network 
- Let $n$ be the committee size
- Let $m$ be the number of bad actors 
- Let $T$ be the BFT threshold (at least two thirds, however in this case it may vary) 

### Formulae

##### Hypergeometric Distribution 

$$
P=\frac{mCT . (N-m)C(n-T)}{NCn}
$$

#####  Binomial Distribution  

$$
P=nCT.\biggl(\frac{m}{n}\biggr)^{T}.\biggl(\frac{N-m}{n}\biggr)^{n-T}
$$

##### Summation 

$$
P_{tot} = \sum_{i=T}^{n} P(N,m,n,i)
$$


### Explanation of hypergeometric distribution (combinations)



### Simple plot with verification 

### Demonstration 
