# Tor

- [Introduction]()
  - [What is Tor?]()
  - [Installation Instructions]()

- [Using Tor]()
  - [How to use Tor]()
  - [When to use Tor]()
  - [When not to use Tor]()

- [How does Tor Network Work?]()
- [Is Tor Broken?]()

- [Conclusion]()
- [References]()
- [Appendices]()
  - [Appendix A: Further Investigation]()
  - [Appendix B: Links of Interest]()

- [Contributors]()





## Introduction

### What is Tor?

Tor is an anonymity/privacy tool, meant to protect your location and identity, which is Free and Open Source.
The name is derived from an acronym for the original software project name ```The Onion Router``` - 
https://www.torproject.org/ This refers to the way that Tor protects your data by wrapping it in multiple layers of 
encryption like an onion. Tor uses a unique system that was developed by the US Navy to protect government intelligence 
communications. Tor is part of the ```Tor Browser``` package, where the Tor Browser has been configured to proxy request 
via the Tor network.



### Installation Instructions

Visit https://www.torproject.org/download/

* Download the package for your OS and language
* Verify the package
* Install the downloaded and verified package



## How to use Tor

Run the Tor Browser and visit an ```.onion``` site like the DuckDuckGo Search engine https://3g2upl4pq6kufc4m.onion/

Check if you running Tor - https://check.torproject.org/



## When to use Tor

As Tor Browser is an anonymity tool, which blocks tracking and hides some of your details and aims to make all Tor 
users look the same to average users, Tor Browser could be used to anonymize whistle-blowing or journalist reporting on 
sensitive topics where they might be persecuted for their views.



## When not to use Tor

Don’t use the Tor Browser if you want an End to End Encryption too a none .onion site like your banking site, as 
the ```Exit Node``` ends encryption and can be intercepted between the Exit Node and the destination server.



## How does Tor Network work?

Tor data is bundled into layers of same size encrypted packets before it enters the Tor network. This is routed through 
a series of volunteer-operated servers called nodes, relays or bridges. Each time the Tor data passes through one of 
these relays, a layer of encryption is removed to reveal the location of the next relay. When you reach the final relay 
on your path, known as the ```Exit Node```, the last layer of encryption is removed and your data is sent to its final 
destination.
Each relay only decrypts enough data to know the location of the previous and next relays. Since each path is randomly 
generated and none of the relays keep records, it’s nearly impossible for your activity to be traced back to you 
through Tor’s complex network.

Further detailed information can be found at https://2019.www.torproject.org/about/overview.html.en



## Is Tor broken?

Correctly setup Tor services and Tor Browser, used correctly, Tor is not broken, but it's very easy to do something 
that would make Tor seem broken. If you login into a remote services, like FaceBook or GMail, your ```Anonymity``` at 
this site is lost. Thou, recent release of the Tor Browser package would keep both sites isolated from one another.
Older Tor setups needed a user to know how to configure their proxy settings in their OS, to use Tor services. This was 
very easy to get wrong or incomplete, and some of your information or details could be leaked. Newer releases of Tor as 
a single package, preconfigure and shipped with a secure browser resolved many of these problems.
Keeping the Tor Browser package up to date, so as to address any security issues discovered helps with keeping your 
```Anonymity```. Tor by it's self has weaknesses, like if you are the only person using Tor on your  (home/office/school) 
network, you would be discovered - This is called a time/traffic correlation attack.

Interesting talk about some of the Tor attacks - https://www.youtube.com/watch?v=7G1LjQSYM5Q



## Conclusion

The following should be kept in mind:

- Anonymity is not Confidentiality.
- Tor is not a VPN.
- Tor data leaving the Tor network can be intercepted.

Advantages of Tor:

- It is free and open source.
- It supports Linux, OSX and Windows.
- It is easy to install for supported operating systems.
- ```.onion``` sites are securely accessible only using Tor Browser
- It is not controlled by corporation or government agencies.

Disadvantages of Tor:

- It is slow.
- It does not necessarily encrypt data leaving ```Exit Node```. This data can be intercepted.
- It does not stop somebody from de-anonymizing themselves.
- It does not stop interceptors from knowing you using Tor.
- Its network is not user friendly, due to its secure and hidden nature.
- Its nodes (relay/bridge) are run by volunteers.

Would I recommend Tor? Maybe not, as Tor by itself does not guarantee Anonymity. I like the goals that Tor is trying to 
address and think they are making great progress. However, total anonymity has many obstacles, not just the technology related, but also the human component.



## References
https://www.torproject.org/
https://en.wikipedia.org/wiki/Tor_(anonymity_network)
https://www.le-vpn.com/what-is-tor-browser/
https://www.vpnmentor.com/blog/tor-browser-work-relate-using-vpn/



## Appendices

### Appendix A: Further Investigation

Onion Services - Tor services that don’t leave the Tor network:

- <https://2019.www.torproject.org/docs/onion-services.html.en>

### Appendix B: Links of Interest

- Metrics of the Tor Project - https://metrics.torproje<ct.org
- Sharing files using Tor - https://onionshare.org
- Blog post about OnionShare2 and its release - https://blog.torproject.org/new-release-onionshare-2
- List of Tor Projects - https://2019.www.torproject.org/projects/projects.html.en
- Isis Lovecruft's PDF covering Privacy and anonymity - https://github.com/isislovecruft/talks/blob/master/2016-03-21-raboud/slides/2016-03-21-raboud-netsec-anonymity-handout.pdf



## Contributors

- <https://github.com/lee???>
- <https://github.com/anselld>

