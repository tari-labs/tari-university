# Grin Design Choice Criticisms   Truth or Fiction

---
## What is Grin?
- Cryptocurrency implemented in Rust
- Mimblewimble transactions
- Dual Proof-of-Work (PoW) system: Cuckatoo algorithm + Equigrin(?)
- Main design goals:
	- Privacy
	- Transaction scaling
	- Design simplicity 

---
## Grin's Criticisms from Community

- Grin received criticisms on design and implementation decisions
- Investigated if there are truth to these concerns:
	- Emission scheme
	- Dual PoW algorithm
	- Cryptographic curve
	- Key-store library
- Suggestions will be presented to addressed these issues

---
# Monetary Policy Due to Static Emission Scheme

+++
## Bitcoin's Emmision scheme

- Bitcoin - limited and finite supply of coins
	- 21 million coins
	- 10-minute block goal
	- First block reward: 50 BTC
	- Reward halved every 4 years

- What hapens to newly minted coins?
	- New coins and transaction fees are paid to miners
	- Incentive to maintain the block chain
 
+++
## What happens when all coins have been mined?

- Only transaction fees will be paid to miners
- Will transaction fees be sufficient to maintain a large network of miners?
- Possibility 1: Fixed supply
	- Fees will be sufficient
	- Over time transaction fees will increase
	- Mining hardware costs decrease
	- Maintaining the block chain remain lucrative for small mining operations

- Possibility 2: Non-capped supply
	- Fees will be insufficient
	- Only large mining farms remain profitable
	- Centralisation of the network

+++
## Grin's Emmision scheme

- Grin - Not capped to a fixed supply
- Static emission rate: constant 60 Grin is released as a reward for solving every block
- block goal of 60 seconds
- Result in 1 coin being created every second

+++
## Grins motivations for static emission rate

- No upper limit on number of coins
- Inflation will tend to zero over time
- Mitigate the effect of lost coins
- Encourage spending rather than holding

+++
## The future value of Grin?

- Approximately 50 years for the inflation to drop below 2%.
- Fixed emission will limit its use as a SoV, encourage MoE
- Experience constant price pressure
- Difficult for Grin to maintain a high value initially
- The Grin team believes
	- high inflation rate will improve distribution of coins
	- discourage holding of coins
	- produce natural pricing
	- limit price manipulation by large coin holders

+++
## Lost coins reduce high inflation

- Grin team believes the high inflation is much lower
- Many coins are lost and become unusable on a block chain
- Lost coins account for upto 2% per year of the total supply
- lost coins should be deducted inflation rate
 
- Percentage of lost transactional coins are difficult to estimate:
	- More lost coins per year for low value coins
	- High value coins have lower rate of lost coins, users tend to be more careful.

+++
## Deflation vs Inflation in fiat systems

- Most economists agree:
	- Deflation is bad as it increases debt
	- "Some" inflation is good as it stimulates the economy
 
+++
## Effect of inflation in fiat systems

- Inflationary currency: MoE rather than a SoV
- Positive effect of inflation:
	- encourages the purchasing of goods and services
	- People with debt benefit from inflation
		- eroding effect on total debt

- Negative effect of inflation:
	- Discourages saving 
		- purchasing power of savings decrease over time
	- Encourages debt

+++
## Debt benefit does not apply to cryptocurrencies

- Not much debt exists

- Reason: Difficult to maintain successful borrower-lender relationships
	- due to the anonymous nature of cryptocurrencies

+++
## Effect of deflation in fiat systems

- Deflationary currency: SoV rather than a MoE
- Positive effect of deflation:
	- increase of purchasing power 
	- encourages saving 
	- discourages debt

- Negative effect of deflation:
	- Discourages the purchasing of goods and services
	- High deflation can cause a deflationary spiral
		- people with debt will have more debt 
		- people with money will start hoarding  		

Bitcoin can be considered deflationary:
- buy and hold Bitcoins as the price per coin might increase over time
- limiting its use as a MoE

+++
## Who controls deflation and inflation?

- Traditional fiat systems:
	- Deflation:
		- only happen in times of economic crisis and recession
		- managed by introducing inflation 
	- Inflation:
		- fiat systems are government backed
		- control the amount of inflation to help alleviate government debt and finance budget deficits 
			- could result in hyperinflation:

- Cryptocurrency Systems:
	- Algorithmic and transparent monetary inflation/deflation 
	- Potentially not controlled by a central authority or government
	- Limiting its misuse

+++
## Balancing between SoV and MoV

- Finding a good balance between SoV and MoV is an important issue for developing a successful currency:
	- Try to motivate saving
	- Try to motivate spending
	- Worrying about debt -> probably not important

- A low inflationary model where inflation is algorithmically maintained seem like the safest choice

- Only time will tell if the high inflation model proposed by Grin will have the desired effect

---
# From ASIC Resistant to ASIC Friendly

+++
## Initial dual PoW algorithms proposed by Grin

- Two ASIC resistant PoW algorithms:
	- Cuckoo cycles
	- Equigrin

- Encourage mining decentralisation.
- High memory requirements
- Adjust parameters every 6 months to deter stealth ASIC mining 
- Move over to using only Cuckoo cycles as the primary PoW algorithm

+++
## New dual PoW algorithm proposal

- Dual PoW algorithm:
	- 1 ASIC resistant - Cuckatoo cycles
	- 1 ASIC friendly - vague details on second algorithm

- What is Cuckatoo cycles?
	- Variation of Cuckoo 
	- Aims to be more ASIC friendly: 
		- Plain bits for ternary counters
		- Requires large amounts of Static Random-Access Memory (SRAM)
		- SRAM is used to speed up the memory latency bound access of random node bits
		- SRAM is limited on GPU and CPU processors
		- Increasing SRAM on ASIC processors is much easier

+++
## What are ASIC miners?

- Specialised hardware
- very efficient at solving specific PoW algorithms

TODO - Add image of ASIC Miner

- Benefits of being ASIC friendly:
	- higher hash rate on network will make it more difficult to hack
	- Use less electrical power to maintaining blockchain 

+++
## Negative effect of ASIC friendly PoW algorithm

- Network of miners will become more centralised

- General consumers do not have access or a need for this type of hardware
- ASIC miners are primarily reserved for enthusiasts and large corporations
- Localises majority of the networks hash rate in large mining farms. 

- block chain more vulnerable to 51% attacks
- Risk even higher when when hardware is preset to use a specific mining pool
- Mining pool might be controlled by single body.

+++
## ASIC resistant mining

- Use general purpose and multi-use (Gaming) hardware for mining

- CPUs and GPUs that are primarily used for gaming and large workstations
- Network of miners is more widely distributed
- More difficult for single bad player to control more than 50% of the networks computational power
- limiting the potential of double spends.

+++
## ASIC resistant or ASIC friendly?

- Very important decision
- Can affect the security of the block chain. The Grin team's choice to support the ASIC community and trying to balancing an ASIC friendly and an ASIC resistant PoW algorithm will be interesting with many potential pitfalls.

---
# Choice of Cryptographic Elliptic-curve - secp256k1

+++
## Elliptic curve cryptography and Secp256k1

- What is Elliptic curve cryptography?
	- Used to generate Private and Public key pairs:
	- Digital signatures 
	- Authorisation of individuals and transactions
	- More secure than RSA (smaller keys for similar security)

- Secp256k1 is an elliptic curve:
	- Defined in the Standards for Efficient Cryptography
	- Used for digital signatures in a number of cryptocurrencies (Bitcoin, Ethereum, EOS, Litecoin, etc.).

+++
## Security of Secp256k1

- Grin uses Secp256k1
 
- Some security experts recommend not using the secp256k1
	- Some issues have been uncovered, but not necessarily exploited.
	- Complex-multiplication field discriminant is not high enough to be secure.
	- Curves with low complex-multiplication field discriminant tend to be easier to break
	- Could result in  future exploits

+++
## Alternative cryptographic curves

- Starting a project with a potentially compromised curve does not seem like a good idea

- Alternative curves with better security properties do exist: 
	- Curve25519 
	- Edwards-curve Digital Signature Algorithm (EdDSA)  -> Ed25519 public-key signature system
	- Fast signature scheme without sacrificing security

+++
## Additional alternatives using SafeCurves

- SafeCurves Link:  https://safecurves.cr.yp.to/
- Maintained by Daniel J. Bernstein and Tanje Lange
- makes it easier to investigate and select an alternative curve
- evaluate the security properties and potential vulnerabilities of many cryptographic curves

---
# Selection of Key-store Library

+++
## Selecting the "best" key-value store library

- Grin originally made use of RocksDB as an internal key-value store
- Many key-store libraries to choose from with different performance and security characteristics:
	- RocksDB
	- LevelDB
	- HyperLevelDB
	- LMDB (Lightning Memory-Mapped Database)

Selecting the "best" key-value store library is difficult?

+++
## Conflicting online benchmarks

- Many conflicting online benchmarks:
	- Some show that RocksDB or LevelDB are better than LMDB
	- Random records were incorreclty selected 

- Howard Chu wrote the article "Lies, Damn Lies, Statistics, and Benchmarks"
	- Exposing these online benchmarking issues
	- LMDB is the best key-value store library
	- Other benchmarks performed by Symas Corp support this claim

+++
## Grin switched to LMDB

- Grin later replaced RocksDB with LMDB
	- Wallet state stored in LMDB database
	- Block chain storage

- Probably a good idea as LMDB seem to be the best key-value store library for block chain related applications

---
## Conclusions

- Selecting an emission rate to create a sustainable monetary system is important
- Try to find a balance between SoV and/or a MoE
- Being ASIC friendly vs ASIC resistant need to be carefully evaluated.
- SafeCurves can be used to select a secure elliptic curve
- Cryptographic curves with potential security vulnerabilities should rather be ignored.
- Becareful when trusting online benchmarks to select libraries