# xmip-core-send

Send Ports, Send Groups and Send Locations, and the chain that resolves them:
a `SendPort` chooses among its `SendLocation`s, a `SendGroup` fans out, and
`SendChain::resolve` says which level of the chain declared the identity Xmip
presents, so an operator asking why Xmip is presenting that certificate gets
the artifact that decided it.

Send owns outbound orchestration, retry over Send Locations and failover
between them; a transport technology moves the bytes. It does not receive, and
it does not infer an identity from what arrived (ADR-0006).

`doc/architecture/runtime-model.md` section 10 governs the Send model and
ADR-0006 the identity a Send Location presents, inherited up the chain where
it is not set; `architecture.toml` carries the maturity.
