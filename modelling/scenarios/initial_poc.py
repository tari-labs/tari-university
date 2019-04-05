#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Tue Mar 26 11:46:27 2019

@author: kevoulee
"""

#!/usr/bin/python
import /..../utlis/hyper_dist_prob as hdp

import numpy as np
from IPython import get_ipython
get_ipython().magic('clear')
#get_ipython().magic('reset -sf')

import matplotlib.pyplot as plt

# Declare and create an array with the total nodes (N) 

no_of_nodes = []

## Initialize first array value 


## Initialize the array size 

no_of_nodes_start = 1  
no_of_nodes_max = 100
array_size = 5  

## Initialize increment 

incr = 1 

## Create the array 

no_of_nodes = np.arange(no_of_nodes_start, no_of_nodes_max + 1, incr)

# Create and Initialize scenario variables 

## Percentage of Bad actors 

no_of_bad_actors = []
bad_actors_percentage = 0.6

## Percentage of Committee size

committee_size = []
committee_size_percentage = 0.6

## BFT threshold percentage 

bft_threshold = []
bft_threshold_percentage = 0.5

# Populate input arrays 

## For each index in the array:

### Calculate the number of Bad actors (product of total nodes (N) and Bad actors percentage, round down)

no_of_bad_actors = np.int_(np.floor(no_of_nodes * bad_actors_percentage))

### Calculate the committee size (product of Total nodes (N) and Committee size percentage, round up)

committee_size = np.int_(np.ceil(no_of_nodes * committee_size_percentage))

### Calculate the BFT threshold (product of committee size (n) and BFT threshold percentage, round up)

bft_threshold = np.int_(np.ceil(committee_size * bft_threshold_percentage))

# Calculate the total probability of bad actors controlling the committee, starting from the BFT threshold 
# up to the committee size 

P_tot = []
for j in range(0, len(no_of_nodes)):
    P_tot.append(hdp.probability(bft_threshold, no_of_bad_actors, committee_size[j], no_of_nodes[j]))
    print(' %3s  %3s  %3s  %3s    j=%3s     P_tot= %-20s' % \
        (no_of_nodes[j], no_of_bad_actors[j], committee_size[j], bft_threshold[j], j, P_tot[j]))

x = no_of_nodes
y1 = P_tot
y2 = no_of_bad_actors
y3 = committee_size
y4 = bft_threshold

fig, ax1 = plt.subplots()

ax2 = ax1.twinx()
ax1.plot(x, y1, 'g-')
ax2.plot(x, y2, 'b-')
ax2.plot(x, y3, 'r-')
ax2.plot(x, y4, 'y-')

ax1.legend('Probability')
ax2.legend('No of bad actors')
ax2.legend('Committee size')
ax2.legend('BFT threshold')


ax1.set_xlabel('Total of Network Nodes')
ax1.set_ylabel('Probability of bad acotrs controlling the network')
ax2.set_ylabel('No of nodes')

plt.show()
