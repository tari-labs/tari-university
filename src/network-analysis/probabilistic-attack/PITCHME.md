
# Probability of a Byzantine Takeover of the Digital Assets Network

## Notation Used  

General notation of statistical expressions.  

- Let $N$ be the total number of nodes in the network. 
- Let $n$ be the committee size.
- Let $m$ be the number of bad actors.
- Let $T$ be the Byzantine Fault Tolerance (BFT) threshold.

There is a pool with $N$ nodes. The pool contains $m$ malicious nodes or bad actors. From within the pool, a random 
selection of nodes, $n$ is drawn. From this selection, the probability of drawing a threshold of bad actors, $T$, needs to 
be calculated.  

## Statistical Calculation

A single probability from the overarching question was used as a means to derive the formulae, as shown in the following 
example. 

**Example**: What is the probability of selecting a majority of bad nodes from a total of $5​$ nodes if the committee 
size is $3​$? There are $3​$ bad nodes $(B1, B2, B3)​$ and $2​$ good nodes $(G1, G2)​$.


The first step is to calculate the number of combinations where bad and good nodes can be chosen: 

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

From this list, the number of combinations where $B$ is the majority can then be tallied. In this case, there are $7$
combinations where $B$ is the majority. Thus, from the $10$ combinations, there are $7$ combinations where there is a 
majority of bad nodes. Therefore, the quotient of $7$ and $10$ is the probability $0.7$. 

This method is limited in calculating the probability where the variables are large. For example, if the same question 
was posed, 
but one had to calculate the probability of selecting a majority of bad nodes from a total of $100$ nodes, with a 
committee size of $60$, $60$ bad nodes and $40$ good nodes, the number of combinations where bad and good nodes can be 
chosen is $1.27E+28$.

# Statistical Calculation 

Literature about [BFT threshold](../../consensus-mechanisms/BFT-consensus-mechanisms-applications/MainReport.md) advises 
the number of good nodes to be at least $\frac{2}{3} \cdot n+1​$, where $n​$ is the number of nodes. In the calculations 
that follow, BFT threshold of, for example, $67​$% of N, is implemented with rounding up to ensure that at least that 
fraction is obtained. In this sense, $67​$% of N simulates $\frac{2}{3} \cdot n+1​$.


