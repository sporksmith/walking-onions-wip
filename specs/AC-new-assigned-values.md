
## Appendix -- new numbers to assign.

Relay commands:
    We need a new relay command for "FRAGMENT".

CREATE handshake types:
    We need a type for the NIL handshake.

    We need a handshake type for the new ntor handshake variant.

Link specifiers:
    We need a link specifier for extend-by-index.

    We need a link specifier for extend-by-index span.

    We need a link specifier for dirport URL.

Certificate Types and Key Types:
    We need to add the new entries from CertType and KeyUsage to
    cert-spec.txt, and possibly merge the two lists.

Protocol versions:
XXXX

Begin cells:

    We need a flag for Delegated Verifiable Selection.

    We need an extension type for extra data, and a value for indices.

End cells:

    We need an extension type for extra data, a value for indices, and a
    value for IPv4 addresses, and a value for IPv6 addresses.

Network parameters:

    "enforce-endive-dl-delay-after" -- How many seconds after receiving a
    SNIP with some timestamp T does a client wait for rejecting older SNIPs?
    Equivalent to "N" in "limiting ENDIVE variance within the network."
    Min: 0. Max: INT32_MAX. Default: 3600 (1 hour).

    "allow-endive-dl-delay" -- Once a client has received an SNIP with
    timestamp T, it will not accept any SNIP with timestamp earlier than
    "allow-endive-dl-delay" seconds before T.
    Equivalent to "Delta" in "limiting ENDIVE variance within the network."
    Min: 0. Max: 2592000 (30 days). Default: 10800 (3 hours).

    hsv2-index-bytes -- bytes to use when sending an hsv2 index to look up a
    hidden service directory.  Min: 1, Max: 40. Default: 4.

    hsv3-index-bytes -- bytes to use when sending an hsv3 index to look up a
    hidden service directory.  Min: 1, Max: 128. Default: 4.

    hsv3-intro-legacy-fields -- include legacy fields in service descriptors.
    Min: 0. Max: 1. Default: 1.

    hsv3-intro-snip -- include intro point SNIPs in service descriptors.
    Min: 0. Max: 1. Default: 0.

    hsv3-rend-service-snip -- Should services advertise and accept rendezvous
    point SNIPs in INTRODUCE2 cells?    Min: 0. Max: 1. Default: 0.

    hsv3-rend-client-snip -- Should clients place rendezvous point SNIPS in
    INTRODUCE2 cells when the service supports it?
    Min: 0. Max: 1. Default: 0.

    hsv3-tolerate-no-legacy -- Should clients tolerate v3 service descriptors
    that don't have legacy fields? Min: 0. Max: 1. Default: 0.

Extensions for decrypted INTRODUCE2 cells:

    [xx] -- a SNIP for the rendezvous point.

Onion key types for decrypted INTRODUCE2 cells:

    [xx] -- onion key rendezvous point is implicit in SNIP.
