
# Removing TAP from v2 onion services

Though v2 onion services are obsolescent and their cryptographic
parameters are disturbing, we do not want to drop support for them
as part of the Walking Onions migration.  If we did so, then we
would force some users to choose between Walking Onions and v2 onion
services, which we do not want to do.

Instead, we describe here a phased plan to replace TAP in v2 onion
services with ntor.  This change improves the forward secrecy of
some of the session keys used with v2 onion services, but does not
improve their authentication, which is strongly tied to truncated
SHA1 hashes of RSA1024 keys.x

>XXX  (Also see the note at the end of this proposal draft: We don't
>get all the session keys.)

Implementing this change is more complex than similar changes
elsewhere in the Tor protocol, since we do not want clients or
services to leak whether they have support for this proposal, until
support is widespread enough that revealing it is no longer a
privacy risk.

## Ntor keys, link specifiers, and SNIPs in v2 descriptors.

We define these entries that may appear in v2 onion service
descriptors, once per introduction point.

    "identity-ed25519"
    "ntor-onion-key"
    "ntor-onion-key-crosscert"

       [at most once each]

       These values are in the same format as and follow the same
       rules as

    "link-specifiers"

       [at most once]

       This value is the same as the link specifiers in a v3 onion
       service descriptor, and follows the same rules.


Services should not include any of these fields unless a new network
parameter, "hsv2-intro-updated" is set to 1. Clients should not parse
these fields or use them unless "hsv2-use-intro-updated" is set to 1.

We define a new field that can be used for hsv2 descriptors with
walking onions:

   "snip"
       [at most once]

       This value is the same as the snip field introduced to a v3
       onion service descriptor by proposal XXX, and follows the
       same rules.

Services should not include this field unless a new network
parameter, "hsv2-intro-snip" is set to 1. Clients should not parse
this field or use it unless "hsv2-use-intro-snip" is set to 1.

Additionally, relays SHOULD omit the following legacy intro point
parameters when a new network parameter, "hsv2-intro-legacy" is set
to 0: "ip-address", "onion-port", and "onion-key". Clients should
treat them as optional when "hsv2-tolerate-no-legacy" is set to 1.

> XXXX "service-key" is a tricky case. It's used to encrypt intro2
> cells.

## INTRODUCE cells, RENDEZVOUS cells, and ntor.

We allow clients to specify the rendezvous point's ntor key in the
INTRODUCE2 cell instead of the TAP key.  To do this, the client
simply sets KLEN to 32, and includes the ntor key for the relay.

Clients should only use ntor keys in this way if the network
parameter "hsv2-client-rend-ntor" is set to 1. Services should only
accept ntor keys sent in this way of the network parameter
"hsv2-service-rend-ntor" is set to 1.


> XXXX We might also want to remove the use of a TAP-like protocol
> for establishing the end-to-end keys used by v2 onion services.
> We could either sandwich the ntor into the old REND/INTRO
> cells, or allow the use of new style INTRO cells with old v2
> services. We'd need to have this be controlled with network
> parameters as well: one for services to accept the new values and
> one for clients to send them.

