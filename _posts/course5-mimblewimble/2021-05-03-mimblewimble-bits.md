---
layout: module
title:  Mimblewimble - what all the bits do
date:   2021-05-01 15:00:00 +0300
format: article
level: intermediate
image:  '/images/banner-08.jpg'
category: mimblewimble
time: 25
course: mimblewimble-basics
tags:   [protocols, mimblewimble, kernel]
featured:
description: A more detailed look at the various pieces of data in the Mimblewimble 
  blockchain, what they do and why they are needed.
---

The Mimblewimble white paper provided the foundations of the protocol. During the development of working 
implementations of the protocol, the devil that was in the detail necessitated a few deviations and finessing of the 
original design. Some of these changes are introduced and discussed in this article. 

This article also focuses on vanilla Mimblewimble. Tari has introduced several other additional features to the 
Mimblewimble protocol that are not covered here, including TariScript, output burning revealed values.  

# Blocks

The anatomy of a Mimblewimble block is as follows:

![Mimblewimble block](/images/mw-bits/mw-block.png)