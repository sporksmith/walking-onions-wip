
## Appendix: Example voting rules

Here we give a set of voting rules for the fields described in our initial
VoteDocuments.

    {
      meta: {
         voting-delay: { op: "Mode", tie_low:false,
                           type:["tuple","uint","uint"] },
         voting-interval: { op: "Median", type:"uint" },
         snip-lifespan: {op: "Mode", type:["tuple","uint","uint","uint"] },
         c-root-lifetime: {op: "Mode", type:["tuple","uint","uint","uint"] },
         s-root-lifetime: {op: "Mode", type:["tuple","uint","uint","uint"] },
         cur-shared-rand: {op: "Mode", min_count: "qfield",
                             type:["tuple","uint","bstr"]},
         prev-shared-rand: {op: "Mode", min_count: "qfield",
                             type:["tuple","uint","bstr"]},
      root: {
         versions: {op:"SetJoin", min_count:"qfield",type:"tstr"},
         require-protos: {op:"BitThreshold", min_count:"sqauth"},
         recommend-protos: {op:"BitThreshold", min_count:"qauth"},
         params: {op:"MapJoin",key_min_count:"qauth",
                     keytype:"tstr",
                     item_op:{op:"Median",min_vote:"qauth",type:"uint"},
                 },
      },
      relay: {
         meta: {
            desc: {op:"Mode", min_count:"qauth",tie_low:false,
                   type:["uint","bstr"] },
            ; XXXX is "1" correcct?
            flags: {op:"MapJoin", key_type:"tstr",
                    item_op:{op:"Mode",type:"bool"}},
            bw : {op:"Median", type:"uint" },
            mbw :{op:"Median", type:"uint" },
            rsaid : {op:"Mode", type:bstr}, // ????XXXX does this go here?
        },
        snip: {
           ; 0: Ed25519 key is handled specially. XXX we'll need to specify it.

           ; ntor onion key.
           1 : { op:"DerivedFrom", fields:[["RM","desc"]],
                 rule:{op:"Mode",type="bstr"} },

           ; link specifiers. this could be a setjoin or a derivedfrom.XXXX
           ; 2 : {},

           ; software description.
           3 : { op:"DerivedFrom", fields:[["RM","desc"]],
                 rule:{op:"Mode",type=["tuple", "tstr", "tstr"] } },

           ; protovers.
           ; XXXX use mapjoin here? Derived doesn't work on maps.
           4 : { op:"DerivedFrom", fields:[["RM","desc"]],
                 rule:{op:"Mode",type=XXX } },

           ; families.
           5 : { op:"SetJoin", min_count:"qfield", type:"bstr" },

           ; countrycode
           6 : { op:"Mode", type="tstr" } ,

           ; 7: exitpolicy.  Return to this after more exit consideration.
           ; 7 : exitpol
        },
        legacy : {
          "sha1-desc" : { op:"DerivedFrom",
                          fields:[["RM","desc"]],
                          rule:{op:"Mode",type="bstr"} },
          "mds" : { op:"DerivedFrom",
                    fields:[["RM":"desc"]],
                    rule : { op:"ThresholdOp", min_count: "qauth",
                             multi_low:false,
                             type:["tuple", "uint", "uint",
                                   "bstr", "bstr" ] }},
        }
      }
      indices: {
        ; XXXXX specify this once index generation is better specified.
      }
    }
