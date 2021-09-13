---
layout: post
title:  Mimblewimble Implementation
menus:
  learning-paths:
    weight: 2
date:   2020-01-02 15:00:00 +0300
image:  '/images/banner-02.jpg'
category: learning-paths
tags:   [learning-paths, mimblewimble]
featured:
excerpttext: Mimblewimble is a blockchain protocol that focuses on privacy through the implementation of confidential transactions.
---

## Table of Contents

- [Background](#background)
- [Learning Path Matrix](#learning-path-matrix)

## Background

Mimblewimble is a blockchain protocol that focuses on privacy through the implementation of confidential transactions.
It enables a greatly simplified blockchain in which all spent transactions can be pruned, resulting in a much smaller
blockchain footprint and efficient base node validation. The blockchain consists only of block-headers, remaining Unspent
Transaction Outputs (UTXO) with their range proofs and an unprunable transaction kernel per transaction.

## Learning Path Matrix

| Topics                                           |                             Type                             |
| ------------------------------------------------ | :----------------------------------------------------------: |
| Mimblewimble                                     | <span class="wrap_beg">[presentation](/protocols/mimblewimble-1/sources/PITCHME.link.md)</span> |
| Introduction to Schnorr Signatures               | <span class="wrap_int">[report](/cryptography/digital_signatures/introduction_schnorr_signatures.md)</span> |
| Introduction to Scriptless Scripts               | <span class="wrap_int">[report](/cryptography/scriptless-scripts/introduction-to-scriptless-scripts.md)</span> |
| Mimblewimble-Grin Block Chain Protocol Overview  | <span class="wrap_int">[report](/protocols/grin-protocol-overview/MainReport.md)</span> |
| Grin vs. BEAM; a Comparison                      | <span class="wrap_int">[report](/protocols/grin-beam-comparison/MainReport.md)</span> |
| Grin Design Choice Criticisms - Truth or Fiction | <span class="wrap_int">[report](/protocols/grin-design-choice-criticisms/MainReport.md)</span> |
| Mimblewimble Transactions Explained              | <span class="wrap_int">[report](/protocols/mimblewimble-1/MainReport.md)</span> |
| Mimblewimble Multiparty Bulletproof UTXO         | <span class="wrap_adv">[report](/protocols/mimblewimble-mp-bp-utxo/MainReport.md)</span> |
