---
layout: module
title:  A non-scary introduction to Elliptic Curves
date:   2021-03-01 15:00:00 +0300
postid: cr01
format: video
level: beginner
redirect_from: /cryptography/crypto-1/sources/PITCHME.link.html
image:  '/images/banner-01.jpg'
course: cryptography-101
category: cryptography-101
time: 15
tags:   [cryptography]
featured:
description: This is a very brief overview of how elliptic curves work.
selftest:
  - title: What is an elliptic curve?
    content: An elliptic curve is a relatively simple mathematical graph of the form y^2 = x^3 + ax + b.
  - title: How are discrete elliptic curves that are used in cryptography different to the continuous curves that we 
      learnt about in high school mathematics?
    content: The elliptic curves used in cryptography are different to continuous curves in two key respects. 
      1. They are defined over a finite field, meaning that the curve is only defined on discrete (integer) values 
      on the x-axis. 2. They are defined modulo a prime number, meaning that the y-values "wrap around" some maximum 
      (prime) value.
  - title: How can such a simple curve secure billions of dollars?
    content: The security of elliptic curve cryptography is based on the fact that it is very difficult to find the 
      discrete logarithm of a point on the curve. This is a fancy way of saying that it is very difficult to find 
      the value of x if you know the value of y. This is known as the "elliptic curve discrete logarithm problem" 
      (ECDLP).
  - title: Why is the "elliptic curve discrete logarithm problem" considered hard?
    content: Because the values "wrap around" modulo some large prime, and because the fields (ranges of values) are 
      very, very large, the distribution of values starts to look very random. So although it is very easy to 
      calculate a y-value given an x-value, by using the curve's formula, it is difficult to do the reverse. 
      Furthermore, the derivation of a secret key from a public key involves repeating this process (typically) 
      many many trillions of times, it becomes infeasible to brute-force the solution in a reasonable amount of time.
    

---

<iframe width="560" height="315"
src="https://www.youtube.com/embed/69bl6dfM6jI/"
title="YouTube video player"
frameborder="0"
allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
allowfullscreen>
</iframe>
