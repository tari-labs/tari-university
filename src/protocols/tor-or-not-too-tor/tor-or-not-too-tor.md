# Tor - an Overview

- [Introduction](#introduction)
  - [What is Tor?](#what-is-tor)
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

Tor is a free and open source anonymity/privacy tool, meant to protect your location and identity.
The name is derived from the acronym for the original software project name, ```The Onion Router``` ([[1]], [[2]]). This 
refers to the way in which Tor protects your data, by wrapping it in multiple layers 
of encryption, similar to the layers of an onion. Tor uses a unique system that was developed by the US Navy to protect 
government intelligence communications. It is part of the ```Tor Browser``` package, where the Tor Browser has been 
configured to proxy request via the Tor network.


### Installation Instructions
With reference to [[1]]:
* Download the package for your operating system and language.
* Verify the package.
* Install the downloaded and verified package.


## Using Tor

### How to use Tor

- Run the Tor Browser and visit an ```.onion``` site such as the DuckDuckGo Search engine [[3]].
- Check if you are running Tor [[4]].



### When to use Tor

The Tor Browser is an anonymity tool that blocks tracking, hides some user details and aims to make all Tor 
users look the same to average users. It could therefore be used to anonymize whistle-blowers or journalists reporting 
on sensitive topics.


### When not to use Tor

Do not use the Tor Browser if you want end-to-end encryption to a non-```.onion``` site such as your banking site, as 
the ```Exit Node``` ends encryption and your data can be intercepted between the Exit Node and the destination server.


## How does Tor Network Work?

Before Tor data enters the Tor network, it is bundled into layers of encrypted packets that are the same size. These 
packets are then routed through a series of volunteer-operated servers called nodes, relays or bridges. Each time the Tor 
data passes through one of these relays, a layer of encryption is removed to reveal the location of the next relay. 
When the data reaches the final relay on its path, known as the ```Exit Node```, the last layer of encryption is removed 
and the data is sent to its final destination.

Each relay only decrypts enough data to learn the location of the previous and next relays. Since each path is randomly 
generated and the relays do not keep records, it is nearly impossible for your activity to be traced back to you 
through Tor’s complex network. Refer to [[5]] for more detailed information.


## Is Tor broken?

Tor is not broken if Tor services are correctly set up and if the Tor Browser is used correctly. However, it is very 
easy to do something that would make Tor appear to be broken. If you login into a remote service such as Facebook or 
Gmail, your ```anonymity``` at this site is lost. However, the recent release of the Tor Browser package would keep 
both sites isolated from one another.
Older Tor setups needed a user to know how to configure their proxy settings in their operating system, in order to use 
Tor services. This was very easy to get wrong or incomplete, and some user information or details could be leaked. 
Newer releases of Tor as a single package, preconfigured and shipped with a secure browser, resolved many of these problems.
Keeping the Tor Browser package up to date, thereby addressing any new security issues, helps to keep your 
```anonymity```.

Tor has weaknesses. For example, if you are the only person using Tor on your home, office or school 
network, you will be discovered. This is called a time/traffic correlation attack.

For an interesting talk about some of the Tor attacks, refer to [[6]].


## Conclusion

The following should be kept in mind:
- Anonymity is not confidentiality.
- Tor is not a Virtual Private Network (VPN).
- Tor data leaving the Tor network can be intercepted.

Advantages of Tor:
- It is free and open source.
- It supports Linux, OSX and Windows.
- It is easy to install for supported operating systems.
- Its browser is the only browser that provides secure accessibility to ```.onion``` sites.
- It is not controlled by corporation or government agencies.

Disadvantages of Tor:
- It is slow.
- It does not necessarily encrypt data leaving ```Exit Node```. This data can be intercepted.
- It does not stop somebody from de-anonymizing themselves.
- It does not stop interceptors from knowing you are using Tor.
- Its network is not user-friendly, due to its secure and hidden nature.
- Its nodes (relay/bridge) are run by volunteers.

Would I recommend Tor? Perhaps not, as Tor by itself does not guarantee anonymity. I like the goals that Tor is trying 
to address and think it is making great progress. However, total anonymity has many obstacles, not just technology 
related, but also the human component.


## References
[[1]] Download for Tor Browser [online]. Available: <https://www.torproject.org/>. Date accessed: 2019&#8209;05&#8209;16.

[1]: https://www.torproject.org/
"Download for Tor Browser"

[[2]] Wikipedia: "Tor (Anonymity Network)" [online]. Available: <https://en.wikipedia.org/wiki/Tor_(anonymity_network)>. 
Date accessed: 2019&#8209;05&#8209;16.

[2]: https://en.wikipedia.org/wiki/Tor_(anonymity_network)
"Wikipedia: Tor (Anonymity Network)"

[[3]] DuckDuckGo Search Engine [online]. Available: <https://3g2upl4pq6kufc4m.onion/>. Date accessed: 2019&#8209;05&#8209;16.

[3]: https://3g2upl4pq6kufc4m.onion/
"DuckDuckGo Search Engine"

[[4]] Tor Project: Check [online]. Available: <https://check.torproject.org/>. Date accessed: 2019&#8209;05&#8209;16

[4]: https://check.torproject.org/
"Tor Project: Check"

[[5]] Tor Project: Overview [online]. Available: <https://2019.www.torproject.org/about/overview.html.en>. 
Date accessed: 2019&#8209;05&#8209;16.

[5]: https://2019.www.torproject.org/about/overview.html.en
"Tor Project: Overview"

[[6]] YouTube: "Dropping Docs on Darknets: How People Got Caught" [online]. Available: 
<https://www.youtube.com/watch?v=7G1LjQSYM5Q>. Date accessed: 2019&#8209;05&#8209;16.

[6]: https://www.youtube.com/watch?v=7G1LjQSYM5Q
"YouTube: Dropping Docs on Darknets: How People Got Caught"

[[7]] What is Tor Browser [online]. Available: <https://www.le-vpn.com/what-is-tor-browser/>. 
Date accessed: 2019&#8209;05&#8209;16.

[7]: https://www.le-vpn.com/what-is-tor-browser/
"What is Tor Browser"

[[8]] The Ultimate Guide to Tor Browser (with Important Tips) 201 [online]. 
Available: <https://www.vpnmentor.com/blog/tor-browser-work-relate-using-vpn/>. Date accessed: 2019&#8209;05&#8209;16.

[8]: https://www.vpnmentor.com/blog/tor-browser-work-relate-using-vpn/
"The Ultimate Guide to Tor Browser"


## Appendices

### Appendix A: Further Investigation

Onion Services - Tor services that don’t leave the Tor network: <https://2019.www.torproject.org/docs/onion-services.html.en>

### Appendix B: Links of Interest

- [Metrics of the Tor Project](https://metrics.torproject.org)
- [Sharing files using Tor](https://onionshare.org)
- [Blog post about OnionShare2 and its release](https://blog.torproject.org/new-release-onionshare-2)
- [List of Tor Projects](https://2019.www.torproject.org/projects/projects.html.en)
- [Isis Lovecruft's PDF covering Privacy and Anonymity](https://github.com/isislovecruft/talks/blob/master/2016-03-21-raboud/slides/2016-03-21-raboud-netsec-anonymity-handout.pdf)

## Contributors

- <https://github.com/leet4tari>
- <https://github.com/kevoulee>
- <https://github.com/anselld>

