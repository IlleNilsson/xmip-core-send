# xmip-core-send

Send Ports and Send Port Groups, and the chain that resolves whose identity
a Send Location presents: `SendChain::resolve` walks Send Location, Send
Port, Send Port Group and the sending Xmip Process, innermost first, and says
which level declared the identity, so an operator asking why Xmip is
presenting that certificate gets the artifact that decided it (ADR-0006).

A Send Location is configured once, as `xmip-core-configure`'s
`ConfiguredLocation`; the runtime builds its transport from that through
`xmip-core-transport`'s one trait, which every protocol implements in both
directions (ADR-0010), and a transport that could not send answers with its
own retryable failure, so whatever decides to try again reads the judgement
the transport made (ADR-0037, amendment 2026-09-27). Send does not receive,
and it does not infer an identity from what arrived (ADR-0006).

`doc/architecture/runtime-model.md` section 10 governs the Send model and
ADR-0006 the identity a Send Location presents, inherited up the chain where
it is not set; `architecture.toml` carries the maturity.
