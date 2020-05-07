
## Better performance and usability for the MyFamily option (v2)

## Problem statement.

The current family interface allows well-behaved relays to
identify that they all belong to the same 'family', and should
not be used in the same circuits.

Right now, this interface works by having every family member
list every other family member in its server descriptor.  This
winds up using O(n^2) space in microdescriptors, server
descriptors, and RAM.  Adding or removing a server from the
family requires all the other servers to change their torrc
settings.

The is growth in size is not just a theoretical problem. Family
declarations currently make up a little over 55% of the
microdescriptors in the directory (around 24% after compression).
The largest family has around 270 members.  270 members times a
160-bit hashed identifier leads to over 5 kilobytes per SNIP, which
is much greater than we'd want to use.

This is an updated version of proposal 242.  It differs by clarifying
requirements and providing a more detailed migration plan.

## Design overview.

In this design, every family has a master "family ed25519" key.  A node
is in the family iff its server descriptor includes a certificate of its
ed25519 identity key with the master ed25519 key.  The certificate
format is as in the tor-certs.txt spec; we would allocate a new
certificate type for this usage.  These certificates would need to
include the signing key in the appropriate extension.

Note that because server descriptors are signed with the node's
ed25519 signing key, this creates a bidirectional relationship
where nodes can't be put in families without their consent.

## Changes to router descriptors

We add a new entry to server descriptors:

    "family-cert" NL
    "-----BEGIN FAMILY CERT-----" NL
    cert
    "-----END FAMILY CERT-----".

This entry contains a base64-encoded certificate as described
above.  It may appear any number of times; authorities MAY reject
descriptors that include it more than three times.

## Changes to microdescriptors

We add a new entry to microdescriptors:
    "family-keys"

This line contains one or more space-separated strings describing
families to which the node belongs.  These strings MUST be sorted in
lexical order.  Clients MUST NOT depend on any particular property
of these strings.

## Changes to voting algorithm

We allocate a new consensus method number for voting on these keys.

When generating microdescriptors using a suitable consensus method,
the authorities include a "family-keys" line if the underlying
server descriptor contains any valid family-cert lines.  For each
valid family-cert in the server descriptor, they add a
base-64-encoded string of that family-cert's signing key.

Additionally, authorities include a "family-keys" line in each
router section in their votes corresponding to such a relay.  When
generating final microdescriptors using this method, the authorities
use these lines to add entries to the microdescriptors' family lines:

1. For every relay appearing in a routerstatus's family-keys, the
   relays calculate a consensus family-keys value by listing including
   all those keys that are listed by a majority of those voters listing
   the same router with the same descriptor.  (This is the algorithm we
   use for voting on other values derived from the descriptor.)

2. The authorities then compute a set of "expanded families": one
   for each family key.  Each "expanded family" is a set containing
   every router in the consensus associated with that key in its consensus
   family-keys value.

3. The authorities discard all "expanded families" of size 1 or
   smaller.

4. Every router listed for the "expanded family" has every other
   router added to the "family" line in its microdescriptor.  (The
   "family" line is then re-canonicalized according to the rules of
   proposal 298 to remove its )

5. Note that the final microdescriptor consensus will include the
   digest of the derived microdescriptor in step 4, rather than the
   digest of the microdescriptor listed in the original votes.  (This
   calculation is deterministic.)

> XXXX This requires authorities to fetch microdescriptors they do
> not have in order to replace their family lines.  Currently,
> voting never requires an authority to fetch a microdescriptor from
> another authority.  If we implement vote compression and diffs as in
> the Walking Onions proposal, however, votes can start to include
> microdescriptors more or less "for free".

## Client behavior

Clients should treat node A and node B as belonging to the same
family if ANY of these is true:

* The client has server descriptors or microdescriptors for A
and B, and A's descriptor lists B in its family line, and
B's descriptor lists A in its family line.

* The client has a server descriptor for A and one for B, and
they both contain valid family-cert lines whose certs are
signed by the family key.

* The client has microdescriptors for A and B, and they both
contain some string in common on their family-cert line.

## Migration

For some time, existing relays and clients will not support family
certificates.  Because of this, we ensure that every family encoded
with certificates is also encoded in microdescriptor family lines.
Note that once authorities support the algorithm above, relays using
family certificates no longer need to include old-style family
lists.

Once enough relays and clients have migrated to using family
certificates, authorities SHOULD add a new consensus method that
stops including old fashioned family lines.

