
# Limit protover values to 0-63.

I propose that we no longer accept protover values higher than 63,
so that they can all fit nicely in a 64-bit bitarray.

## Motivation

Doing this will simplify our implementations and our protocols.

I believe that we will lose nothing by making this
change. Currently, after nearly two decades of Tor development
and 3.5 years of experiences with protovers in production, we have
no protocol version high than 5.

Even if we were someday to need to implement higher protocol
versions, we could simply add a new subprotocol name instead.  For
example, instead of "HSIntro=64", we could say "HSIntroRedux=1".

## Migration

Immediately, authorities should begin rejecting relays with protocol
versions above 63.  (There are no such relays in the consensus right
now.)

Once this change is deployed to a majority of authorities, we can
remove support in other Tor environments for protocol versions
above 63.


