
## Appendix I: Non-clique topologies with Walking Onions

For future work, we can expand the Walking Onions design to
accommodate network topologies where relays are divided into groups,
and not every group connects to every other.  To do so requires
additional design work, but here I'll provide what I hope will be a
workable sketch.

First, each SNIP needs to contain an ID saying which relay group it
belongs to, and an ID saying which relay group(s) may serve it.

When downloading an ENDIVE, each relay should report its own
identity, and receive an ENDIVE for that identity's group.  It
should contain both the identities of relays in the group, and the
SNIPs that should be served for different indices by members of that
group.

The easy part would be to add an optional group identity field to
SNIPs, defaulting to 0, indicating that the relay belongs to that
group, and an optional served-by field to each SNIP, indicating
groups that may serve the SNIP.  You'd only accept SNIPs if they
were served by a relay in a group that was allowed to serve them.

Would guards work?  Sure: we'd need to have guard SNIPS served by
middle relays.

For hsdirs, we'd need to have either multiple shards of the hsdir
ring (which seems like a bad idea?) or have all middle nodes able to
reach the hsdir ring.

Things would get tricky with making onion services work: if you need
to use an introduction point or a rendezvous point in group X, then
you need to get there from a relay that allows connections to group
X.  Does this imply indices meaning "Can reach group X" or
"two-degrees of group X"?

The question becomes: "how much work on alternative topologies does
it make sense to deploy in advance?"  It seems like there are
unknowns affecting both client and relay operations here, which
suggests that advance deployment for either case is premature: we
can't necessarily make either clients or relays "do the right thing"
in advance given what we now know of the right thing.
