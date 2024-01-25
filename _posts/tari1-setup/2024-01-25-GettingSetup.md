---
layout: module
title:  Setting Up Your Environment
date:   2024-01-25 12:00:00 +0300
postid: 201
image:  '/images/banner-01.jpg'
category: setup
course: developing-on-tari
time: 20
format: video
level: beginner
modno: 1
tags:   [201]
description: Setting up your development environment for Tari
selftest:
  - title: When was Bitcoin launched?
    content: January 3, 2009, but Satoshi was working on it for some time prior to this.
  - title: How is Bitcoin money different to all the types of money the preceded it?
    content: It’s the first currency that does not have a trusted centralised entity that manages the ledger.
  - title: |
      Select the one that is not true. Bitcoin is <br/>
      (a) borderless,<br/>
      (b) censorship-resistant,<br/>
      (c) programmable,<br/>
      (d) controlled by Satoshi Nakamoto.
    content: (d) controlled by Satoshi Nakamoto.
---

Before you start developing on Tari, you'll need to set up your environment.
Here's some things you'll need
1. A computer running Linux, macOS or Windows
2. A text editor or IDE. We recommend [Visual Studio Code](https://code.visualstudio.com/)
3. Docker (How to set up Docker on [Linux](https://docs.docker.com/engine/install/ubuntu/), [macOS](https://docs.docker.com/docker-for-mac/install/) or [Windows](https://docs.docker.com/docker-for-windows/install/))
4. [Git](https://git-scm.com/downloads)

## Setting up your environment
Begin by pulling the tari testing docker image from Quay.

```bash
docker pull quay.io/tarilabs/dan-testing
```

Start the container by running 

```bash
docker run quay.io/tarilabs/dan-testing
```

This will start up a base node, miner and validator node as well as create some 
accounts and a Tari Asset Wallet.

