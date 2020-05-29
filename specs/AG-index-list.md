
<!-- Section A.7 --> <a id='SA.7'></a>
## Appendix G: A list of routing indices

Middle -- general purpose index for use when picking middle hops in
circuits.  Bandwidth-weighted for use as middle relays.  May exclude
guards and/or exits depending on overall balance of resources on the
network.

Formula:
      type: 'weighted',
      source: {
          type:'bw', require_flags: ['Valid'], 'bwfield' : ["RM", "mbw"]
      },
      weight: {
          [ "!Exit", "!Guard" ] => "Wmm",
          [ "Exit", "Guard" ] => "Wbm",
          [ "Exit", "!Guard" ] => "Wem",
          [ "!Exit", "Guard" ] => "Wgm",
      }

Guard -- index for choosing guard relays. This index is not used
directly when extending, but instead only for _picking_ guard relays
that the client will later connect to directly.  Bandwidth-weighted
for use as guard relays. May exclude guard+exit relays depending on
resource balance.

      type: 'weighted',
      source: {
           type:'bw',
           require_flags: ['Valid', "Guard"],
           bwfield : ["RM", "mbw"]
      },
      weight: {
          [ "Exit", ] => "Weg",
      }

HSDirV2 -- index for finding spots on the hsv2 directory ring.

Formula:
      type: 'rsa-id',

HSDirV3-early -- index for finding spots on the hsv3 directory ring
for the earlier of the two "active" days. (The active days are
today, and whichever other day is closest to the time at which the
ENDIVE becomes active.)

Formula:
      type: 'ed-id'
      alg: SHA3-256,
      prefix: b"node-idx",
      suffix: (depends on shared-random and time period)

HSDirV3-late -- index for finding spots on the hsv3 directory ring
for the later of the two "active" days.

Formula: as HSDirV3-early, but with a different suffix.

Self -- A virtual index that never appears in an ENDIVE.  SNIPs with
this index are unsigned, and occupy the entire index range.  This
index is used with bridges to represent each bridge's uniqueness.

Formula: none.

Exit0..ExitNNN -- Exits that can connect to all ports within a given
PortClass 0 through NNN.

Formula:


      type: 'weighted',
      source: {
           type:'bw',
           ; The second flag here depends on which portclass this is.
           require_flags: [ 'Valid', "P@3" ],
           bwfield : ["RM", "mbw"]
       },
      weight: {
          [ "Guard", ] => "Wge",
      }
