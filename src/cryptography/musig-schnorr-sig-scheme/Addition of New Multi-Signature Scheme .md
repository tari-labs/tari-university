# Formation of New Multi-Signature Scheme 

MuSig is parmaterised by group parameters $(\mathbb{G\mathrm{,p,g)}}$ where,

$p$is the $k$-bit integer

$\mathbb{G}​$ is the cyclic group of order $p​$

$g$is the generator of $G$

hash functions, $H_{com}$, $H_{agg}$, $H_{sig}$ from $\{0,1\}^{*}$to $\{0,1\}^{l}$ (constructed from a single hash, using proper domain separation)

where $H_{com}$ is used in the commitment phase

$H_{agg}$ is used to compute the aggregated key

$H_{sig}$ is used to compute the signature

For the signing :

Let $X_{1}$ and $x_{1}$ be the public and private key of a specific signer

Let $m$ be the message that will be signed

Let $X_{1},...,X_{n}$ be the public keys of other cosigners

Let $L=\{X_{1},...X_{n\}}$ be the multiset of all public keys involved in the signing process

For $i$$\epsilon$$\{1,...,n)$ , the signer computes the following

$a_{i}=H_{agg}(L,X_{i})$

as well as the "aggregated" public key $\tilde{X}=\prod_{i=1}^{n}X_{i}^{a_{i}}$.

The signer generators a random $r_{1}\leftarrow\mathbb{Z_{\mathrm{p}}}$computes $R_{1}=g^{r_{1}},t_{1}=H_{com}(R_{1})$, and sends $t_{1}$to all other cosigners.

When receiving the commitments $t_{2},...,t_{n}$ from the other cosigners, it sends $R_{1}$

When receiving $R_{2},...,R_{n}$ from other cosigners, it checks that $t_{i}=H_{com}(R_{i})$ for all $i$$\epsilon$$\{2,...,n)$

The protocol is aborted if this is not the case. If not the following is computed:

$R=\stackrel[i=1]{n}{\prod}R_{i}$

$c=H_{sig}(\tilde{X,R,m)}$

$s_{1}=r_{1}+ca_{1}x_{1}$ mod$p$

$s_{1}$ is sent to all other cosigners

When receiving $s_{2},...s_{n}$ from other cosigners, the signer can compute $s=\sum_{i=1}^{n}s_{i}$ mod$p$. The signature is $\sigma=(R,s)$.

In order to verify, given a multiset of public keys $L=\{X_{1},...X_{n\}}$ , a message $m$ and a signature $\sigma=(R,s)$, the verifier computes:

$a_{i}=H_{agg}(L,X_{i})​$ for $i​$$\epsilon​$$\{1,...,n)​$, $\tilde{X}=\prod_{i=1}^{n}X_{i}^{a_{i}}​$, $c=H_{sig}(\tilde{X,R,m)}​$ and then accepts the signature if $g^{s}=R\prod_{i=1}^{n}X_{i}^{a_{i}c}=R\tilde{X^{c}.}​$

