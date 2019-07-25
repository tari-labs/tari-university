# Tor - an Overview

- [Introduction](#introduction)
  - [What is Tor?](#what-is-tor)
  - [Purpose of this Report](#purpose-of-this-report)
  - [Installation Instructions](#installation-instructions)
- [Using Tor](#using-tor)
  - [How to use Tor](#how-to-use-tor)
  - [When to use Tor](#when-to-use-tor)
  - [When not to use Tor](#when-not-to-use-tor)
- [How does Tor Network Work?](#how-does-tor-network-work)
- [Is Tor Broken?](#is-tor-broken)
- [Conclusion](#conclusion)
- [References](#references)
- [Appendices](#appendices)
  - [Appendix A: Further Investigation](#appendix-a-further-investigation)
  - [Appendix B: Links of Interest](#appendix-b-links-of-interest)
- [Contributors](#contributors)


## Introduction

### What is Tor?

Tor is a free and open-source anonymity/privacy tool, meant to protect your location and identity.
The name is derived from the acronym for the original software project name, ```The Onion Router``` ([[1]], [[2]]). This 
refers to the way in which Tor protects your data, by wrapping it in multiple layers 
of encryption, similar to the layers of an onion.

Tor uses a unique system that was developed by the US Navy to protect 
government intelligence communications.  

The Tor network service is the heart of the Tor project. The Tor Browser and other tools, such 
as OnionShare, run on top of or via the Tor network. These tools are meant to make using 
the Tor network as simple and secure as possible.  

Some tools, like the Tor Browser Bundle, come as a single downloadable and installable 
package. Setup and containing everything needed to use the Tor network and be anonymous.  

Almost any network tool or application that can be configured to use a SOCKS proxy can be setup to use the Tor network service.  


### Purpose of this Report

This report gives an overview of Tor, using the Tor Browser. The author reaches a conclusion regarding whether or not to 
recommend Tor, i.e. "to Tor || !2 Tor", which means "to Tor or not to Tor" in programming language.


### Installation Instructions for Tor Browser

With reference to [[1]]:
* Download the package for your operating system and language.
* Verify the package.
* Install the downloaded and verified package.


## Using Tor

### How to use Tor

- Run the Tor Browser and visit an ```.onion``` site such as the DuckDuckGo Search engine [[3]].
- Check if you are running Tor [[4]].

### When to use Tor

The ```Tor Browser``` is an anonymity tool that blocks tracking, hides some user details and aims to make all Tor 
users look the same to average users. It could therefore be used to anonymize whistle-blowers or journalists reporting 
on sensitive topics.


### When not to use Tor

Do not use the Tor Browser if you want end-to-end encryption to a non-```.onion``` site such as a banking site, as the 
```Exit Node``` ends the Tor encryption and your traffic can be intercepted between the Exit Node and the destination server.


## How does Tor Network Work?

Before Tor data enters the Tor network, it is bundled into layers of encrypted packets that are the same size. These 
packets are then routed through a series of volunteer-operated servers called nodes, relays or bridges. Each time the 
Tor data passes through one of these relays, a layer of encryption is removed to reveal the location of the next relay. 
When the data reaches the final relay on its path, known as the ```Exit Node```, the last layer of encryption is removed 
and the data is sent to its final destination.

Each relay only can decrypt enough data to learn the location of the previous and next relay. Since each path is randomly 
generated and the relays do not keep records, it is nearly impossible for your activity to be traced back to you 
through Tor’s complex network. Refer to [[5]] for more detailed information.


## Is Tor broken?

Tor is not broken if Tor services are correctly set up and if the Tor Browser is used correctly. However, it is very 
easy to do something that would make Tor appear to be broken. If you login into a remote service such as Facebook or 
Gmail, your ```anonymity``` at this site is lost. However, the recent release of the Tor Browser package would keep 
both sites isolated from one another.

Older Tor setups needed a user to know how to configure their proxy settings in their operating system, in order to use 
Tor services. This was very easy to get wrong or incomplete, and some user information or details could be leaked. 
Newer releases of Tor as a single package, preconfigured and shipped with a secure browser, resolved many of these 
problems. Keeping the Tor Browser package up to date, thereby addressing any new security issues, as well as helping to keep your 
```anonymity```.

Tor has weaknesses. For example, if you are the only person using Tor on your home, office or school 
network, you will be discovered. This is called a time/traffic correlation attack. For an interesting talk about some 
of the Tor attacks, refer to [[6]].

Two examples of where people using Tor that were discovered:
- 16th December 2013, Havard University received a bomb threat, that was tracked down to Eldo Kim, who 
was one of the few people using Tor on the campus network when the email had been sent. After questioning, 
admitted he had sent the hoax bomb threat, as he wanted to get out of an exam. [[7]]  
- Hector Xavier Monsegur(Sabu) normally used Tor for connecting to IRC but was caught not using it once 
and FBI found his home IP. After 
being caught, he started to collaborate with the FBI.  
While Hector was chatting to Jeremy Hammond(sup_g) on IRC, Jeremy let slip where he had been arrested before 
and other groups he had been involved with. This helped reduce the number of suspects and FBI were able to 
get a court order to monitor internet access and was able to correlate when Jeremy was using Tor.[[8]]


## Conclusion

The following should be kept in mind:
- Anonymity is not confidentiality.
- Tor is not a Virtual Private Network (VPN).
- Tor data leaving the Tor network can be intercepted.

Advantages of Tor:
- It is free and open source.
- It supports Linux, OSX and Windows.
- It is easy to install for supported operating systems.
- It is not controlled by corporate nor government agencies.

Disadvantages of Tor:
- It is slow.
- It does not necessarily encrypt data leaving ```Exit Node```. This data can be intercepted.
- It does not stop somebody from de-anonymizing themselves.
- It does not stop interceptors from knowing you are using Tor.
- Its network is not user-friendly, due to its secure and hidden nature.
- Its nodes (relay/bridge) are run by volunteers, can some times be unreliable.

Would the author of this report recommend Tor? Perhaps not, as Tor by itself does not guarantee anonymity. I like the 
goals that Tor is trying to address and think it is making great progress. However, total anonymity has many obstacles, 
not just technology related, but also the human component.


## References

[[1]] Download for Tor Browser [online]. Available: <https://www.torproject.org/>. Date accessed: 2019&#8209;05&#8209;16.

[1]: https://www.torproject.org/
"Download for Tor Browser"

[[2]] Wikipedia: "Tor (Anonymity Network)" [online]. Available: <https://en.wikipedia.org/wiki/Tor_(anonymity_network)>. 
Date accessed: 2019&#8209;05&#8209;16.

[2]: https://en.wikipedia.org/wiki/Tor_(anonymity_network)
"Wikipedia: Tor (Anonymity Network)"

[[3]] DuckDuckGo Search Engine [online]. Available: <https://3g2upl4pq6kufc4m.onion/>. **Note:** This link will not work unless Tor or the Tor Browser is used. Date accessed: 2019&#8209;05&#8209;16.

[3]: https://3g2upl4pq6kufc4m.onion/
"DuckDuckGo Search Engine - 
link will not work unless 
Tor or Tor Browser is used"

[[4]] Tor Project: Check [online]. Available: <https://check.torproject.org/>. **Note:** This link will help the user identify if Tor or the Tor Browser is been used. Date accessed: 2019&#8209;05&#8209;16

[4]: https://check.torproject.org/
"Tor Project: Check - 
link will help user identify 
Tor or Tor Browser is been used"

[[5]] Tor Project: Overview [online]. Available: <https://2019.www.torproject.org/about/overview.html.en>. 
Date accessed: 2019&#8209;05&#8209;16.

[5]: https://2019.www.torproject.org/about/overview.html.en
"Tor Project: Overview"

[[6]] YouTube: "DEF CON 22 - Adrian Crenshaw - Dropping Docs on Darknets: How People Got Caught" [online]. Available: 
<https://www.youtube.com/watch?v=eQ2OZKitRwc>. Date accessed: 2019&#8209;06&#8209;18.

[6]: https://www.youtube.com/watch?v=eQ2OZKitRwc
"YouTube: DEF CON 22 - Adrian Crenshaw - Dropping Docs on Darknets: 
How People Got Caught"

[[7]] Ars Technica: "Use of Tor helped FBI ID suspect in bomb hoax case" [online]. Available: 
<https://arstechnica.com/security/2013/12/use-of-tor-helped-fbi-finger-bomb-hoax-suspect/>. Date accessed: 2019&#8209;07&#8209;11.

[7]: https://arstechnica.com/security/2013/12/use-of-tor-helped-fbi-finger-bomb-hoax-suspect/
"Ars Technica: Use of Tor helped FBI ID suspect in bomb hoax case"

[[8]] Ars Technica: "Stakeout: how the FBI tracked and busted a Chicago Anon" [online]. Available: 
<http://arstechnica.com/tech-policy/2012/03/stakeout-how-the-fbi-tracked-and-busted-a-chicago-anon/>. Date accessed: 2019&#8209;07&#8209;11.

[8]: http://arstechnica.com/tech-policy/2012/03/stakeout-how-the-fbi-tracked-and-busted-a-chicago-anon/
"Ars Technica: Stakeout: how the FBI tracked and busted a Chicago Anon"


## Appendices

### Appendix A: Further Investigation

Onion Services - Tor services that don’t leave the Tor network: <https://2019.www.torproject.org/docs/onion-services.html.en>.

### Appendix B: Links of Interest

- [What is Tor Browser?](https://www.le-vpn.com/what-is-tor-browser/)
- [The Ultimate Guide to Tor Browser (with Important Tips) 2019](https://www.vpnmentor.com/blog/tor-browser-work-relate-using-vpn/)
- [Metrics of the Tor Project](https://metrics.torproject.org)
- [Sharing files using Tor](https://onionshare.org)
- [Blog post about OnionShare2 and its release](https://blog.torproject.org/new-release-onionshare-2)
- [List of Tor Projects](https://2019.www.torproject.org/projects/projects.html.en)
- [Isis Lovecruft's PDF covering Privacy and Anonymity](https://github.com/isislovecruft/talks/blob/master/2016-03-21-raboud/slides/2016-03-21-raboud-netsec-anonymity-handout.pdf)


## Contributors

- <https://github.com/leet4tari>
- <https://github.com/kevoulee>
- <https://github.com/anselld>

