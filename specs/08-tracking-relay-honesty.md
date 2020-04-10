
# Tracking Relay honesty

> XXXX explain the problem here: relays might close circuits rather than
> answer a request, or might keep multiple ENDIVEs around and pick from one.

## Defense: tracking close rate

> XXXX basic idea is similar to circuit bias detection today.

## Defense: index stability

> XXXX basic idea is that authorities should try to build indices so they
> don't change any more over time than is necessary.  if the client asks for
> the relay at position P, and the relay has 20 ENDIVEs to choose from, then
> there should be maybe 2-4 choices, not 20.

## Defense: enforced monotonicity

> XXXX basic idea is that once an honest relay has an ENDIVE, it will throw
> away every older ENDIVE.  Clients and immediate-predecessor relays can
> detect and enforce this.

