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
- [What is an .onion site?](#what-is-an-onion-site)
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

Tor uses a unique system that was originally developed by the US Navy to protect 
government intelligence communications. Naval Research Laboratory released the Tor code 
under a free licence and in 2006 the Tor Project was founded. With the help of 
the Electronic Frontier Foundation (EFF), further research and development has continued 
as a Free and Open Source Project. Run by a worldwide community of volunteers, means 
the Tor network services are not controlled by any corporate nor government agencies.  

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

The ```Tor Browser``` is an anonymity tool, that in addition to using the ```Tor network``` encryption service, 
also blocks tracking, hides user details and aims to make all Tor users look the same. It could 
therefore be used to anonymize whistle-blowers or journalists reporting on sensitive topics.


### When not to use Tor

Do not use only the Tor Browser if you expect end-to-end encryption to a non-```.onion``` site such as a banking site, as the 
```Exit Relay``` ends the Tor encryption and your traffic can be intercepted between the Exit Node and the destination server.


## How does Tor Network Work?

Before Tor data enters the Tor network, it is bundled into nested layers of encrypted packets of the same size. These 
packets are then routed through a random series of volunteer-operated servers called relays( also known as nodes). Each time the 
Tor data passes through one of these relays, a layer of encryption is removed to reveal the location of the next relay. 
When the data reaches the final relay on its path, known as the ```Exit Relays```, the last layer of encryption is removed 
and the data is sent to its final destination.

There are a few different kinds of relays. Guard relays, which deal with traffic entering the Tor network. Middle relays deal with traffic inside the Tor network. Both Guard and Middle relays are often referred too as none-exist relays.

Exit relays, which are internet public-facing relays, where the last layer of Tor encryption is removed as traffic leaves the Tor network as normal traffic into the internet on it's way too its destination server. Exit relays often have to deal with complaints, legal notices, takedown notices as Exit relays are seen as the source address of the traffic.

Also, Exit relays are registered as part of the Tor relay setup process, some adversary agencies have blocked or attacked these Tor relays. In response to blocking tactics, a special kind of relay called a Bridge should be used. Bridge selection is done when installing the Tor Browser bundle. Mostly if Tor does not seem to work, Bridge relays are unlisted relays and less likely to deal with complaints or have internet traffic blocked. 

Each relay can only decrypt enough data to learn the location of the previous and next relay. Since each path is randomly 
generated and the relays do not keep records, it is nearly impossible for your activity to be traced back to you 
through Tor’s complex network. Refer to [[5]] for more detailed information.

More information on what a Tor Relay is or how to setup and volunteer some resources can be found at [[6]]


## What is an ```.onion``` site?

An ```.onion``` address points to some resource on the Tor network called a hidden service or an ```onion``` service. 
```Onion``` services are generally only accessible by using the Tor network. As an example,  visiting the DuckDuckGo 
Search engine onion address (https://3g2upl4pq6kufc4m.onion/) [[3]]], routes the request through the Tor network without 
the client knowing the host IP address of the server. The onion address is practically meaningless without it being routed 
through and resolved by the Tor network. Traffic between a Tor client and an ```.onion``` site should never leave the 
Tor network, keeping the network traffic safer and more anonymous than publicly hosted sites. Though more difficult to 
find, been there is not directory services or easy to remember names
as the ```.onion``` address is not an easy to remember domain name, 
but generally opaque, non-mnemonic, 16- or 56-character alpha-semi-numerical 
strings which are generated on a cryptographically hashed public key. The TLD .onion is not a true domain and can't be found or queried 
on the internet, but only inside of the Tor network.

More information on what an ```.onion``` site and/or address is, can be found at [[7]] and [[8]]


## Is Tor broken?

Tor encrypted network service has not been broken, but due to the complex nature of networks, internet services, 
internet browsers and operating systems, what might seem like a simple request to a URL, could de-anonymize somebody.

Older Tor setups needed a user to know how to configure their proxy settings in their operating system and/or browser, 
in order to use Tor services. This was very easy to get wrong and/or incomplete, and some users information or details 
could be leaked. An example is DNS requests meant for the Tor network, ie .onion address, might be sent directly to the 
public DNS server, if the ```network.proxy.socks_remote_dns``` was not set to true in FireFox. Theses DNS request could 
be used to track where a user might be surfing and thus, de-anonymize the user.

Tor is not broken if Tor services are correctly setup or if the Tor Browser is used correctly. It is very easy to do 
something that would de-anonymize a user, like using an older browser or tool that is not configured to proxy all 
traffic via Tor network services.

Another example is if the user logs into a remote service such as Facebook or Gmail, your anonymity at this site is 
lost. What many people don't know, is that other sites use tracking techniques, like cookies, could de-anonymize 
the user on other sites too. Further information about online tracking can be found at [[9]]

However, recent releases of the Tor Browser notify users of updates and also work toward keeping each site isolated 
in their own Tab or session, addressing old and possible insecure releases and user tracking via cookies.

Tor some has weaknesses, for example, if you are the only person using Tor on your home, office or school network, 
you could be de-anonymized.  Another is that a site knows when it's been accessed using Tor.  These shortcomings 
might not be directly an issue with Tor nor it's encryption, but an expectation of a novice user, using Tor or 
one of the Tor tools and services.

For an interesting talk about some of the Tor attacks, refer to [[10]].

Two real world examples of where people using Tor that were discovered:
- 16th December 2013, Havard University received a bomb threat, that was tracked down to Eldo Kim, who 
was one of the few people using Tor on the campus network when the email had been sent. After questioning, 
admitted he had sent the hoax bomb threat, as he wanted to get out of an exam. [[11]]  
- Hector Xavier Monsegur(Sabu) normally used Tor for connecting to IRC but was caught not using it once 
and FBI found his home IP. After 
being caught, he started to collaborate with the FBI.  
While Hector was chatting to Jeremy Hammond(sup_g) on IRC, Jeremy let slip where he had been arrested before 
and other groups he had been involved with. This helped reduce the number of suspects and FBI were able to 
get a court order to monitor internet access and was able to correlate when Jeremy was using Tor.[[12]]


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
- It can be slow.
- It does not necessarily encrypt data leaving ```Exit Node```. This data can be intercepted.
- It does not stop somebody from de-anonymizing themselves.
- It does not stop interceptors from knowing you are using Tor.
- Its network is not user-friendly, due to its secure and hidden nature.
- Its nodes (relay/bridge) are run by volunteers, can some times be unreliable.

Would the author of this report recommend Tor? Perhaps not, as Tor by itself does not guarantee anonymity. I like the 
goals that Tor is trying to address and think it is making great progress. However, total anonymity has many obstacles, 
not just technology related, but also the human component.


## References

[[1]] Tor Project: "Download for Tor Browser" [online]. Available: <https://www.torproject.org/>. Date accessed: 2019&#8209;05&#8209;16.

[1]: https://www.torproject.org/
"Tor Project: Download for Tor Browser"

[[2]] Wikipedia: "Tor (Anonymity Network)" [online]. Available: <https://en.wikipedia.org/wiki/Tor_(anonymity_network)>. 
Date accessed: 2019&#8209;05&#8209;16.

[2]: https://en.wikipedia.org/wiki/Tor_(anonymity_network)
"Wikipedia: Tor (Anonymity Network)"

[[3]] DuckDuckGo Search Engine inside Tor [online]. Available: <https://3g2upl4pq6kufc4m.onion/>. **Note:** This link will not work unless Tor or the Tor Browser is used. Date accessed: 2019&#8209;05&#8209;16.

[3]: https://3g2upl4pq6kufc4m.onion/
"DuckDuckGo Search Engine - 
link will not work unless 
Tor or Tor Browser is used"

[[4]] Tor Project: "Check" [online]. Available: <https://check.torproject.org/>. **Note:** This link will help the user identify if Tor or the Tor Browser is been used. Date accessed: 2019&#8209;05&#8209;16

[4]: https://check.torproject.org/
"Tor Project: Check - 
link will help user identify 
Tor or Tor Browser is been used"

[[5]] Tor Project: "Overview" [online]. Available: <https://2019.www.torproject.org/about/overview.html.en>. 
Date accessed: 2019&#8209;05&#8209;16.

[5]: https://2019.www.torproject.org/about/overview.html.en
"Tor Project: Overview"

[[6]] Tor Project: "The Tor Relay Guide" [online]. Available: 
<https://trac.torproject.org/projects/tor/wiki/TorRelayGuide>. Date accessed: 2019&#8209;08&#8209;14.

[6]: https://trac.torproject.org/projects/tor/wiki/TorRelayGuide
"Tor Project: The Tor Relay Guide"

[[7]] Wikipedia: ".onion" [online]. Available: 
<https://en.wikipedia.org/wiki/.onion>. Date accessed: 2019&#8209;08&#8209;22.

[7]: https://en.wikipedia.org/wiki/.onion
"Wikipedia: .onion"

[[8]] Tor Project: "How do I access onion services?" [online]. Available: 
<https://2019.www.torproject.org/docs/faq#AccessOnionServices>. Date accessed: 2019&#8209;08&#8209;22.

[8]: https://2019.www.torproject.org/docs/faq#AccessOnionServices
"Tor Project: How do I access onion services?"

[[9]] Robert Heaton: "How does online tracking actually work?" [online]. Available: 
<https://robertheaton.com/2017/11/20/how-does-online-tracking-actually-work/>. Date accessed: 2019&#8209;07&#8209;25.

[9]: https://robertheaton.com/2017/11/20/how-does-online-tracking-actually-work/
"Robert Heaton: How does online 
tracking actually work?"

[[10]] YouTube: "DEF CON 22 - Adrian Crenshaw - Dropping Docs on Darknets: How People Got Caught" [online]. Available: 
<https://www.youtube.com/watch?v=eQ2OZKitRwc>. Date accessed: 2019&#8209;06&#8209;18.

[10]: https://www.youtube.com/watch?v=eQ2OZKitRwc
"YouTube: DEF CON 22 - Adrian Crenshaw - 
Dropping Docs on Darknets: 
How People Got Caught"

[[11]] Ars Technica: "Use of Tor helped FBI ID suspect in bomb hoax case" [online]. Available: 
<https://arstechnica.com/security/2013/12/use-of-tor-helped-fbi-finger-bomb-hoax-suspect/>. Date accessed: 2019&#8209;07&#8209;11.

[11]: https://arstechnica.com/security/2013/12/use-of-tor-helped-fbi-finger-bomb-hoax-suspect/
"Ars Technica: Use of Tor helped 
FBI ID suspect in bomb hoax case"

[[12]] Ars Technica: "Stakeout: How the FBI tracked and busted a Chicago Anon" [online]. Available: 
<https://arstechnica.com/tech-policy/2012/03/stakeout-how-the-fbi-tracked-and-busted-a-chicago-anon/>. Date accessed: 2019&#8209;07&#8209;11.

[12]: https://arstechnica.com/tech-policy/2012/03/stakeout-how-the-fbi-tracked-and-busted-a-chicago-anon/
"Ars Technica: Stakeout: How the 
FBI tracked and busted a Chicago Anon"


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
- <https://github.com/sdbondi>
