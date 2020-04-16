# Approximate the minimum-k-cut using iterated Stoer-Wagner min-cut algorithm.
#
# This is an approximation only; real algorithms for this are really complex and costly.

import networkx as nx
import sys

vals={}
exec(open(sys.argv[1]).read(), vals)
PORTS = vals['PORTS']
GRAPH = vals['GRAPH']

def load_graph():

    g = nx.Graph()

    assert len(GRAPH) == len(PORTS)
    assert len(GRAPH[0]) == len(PORTS)
    for i in range(len(PORTS)):
        for j in range(i+1,len(PORTS)):
            # Whoops.  In the source we defined our edges so that
            # we are paying for the ones we _don't_ cut.
            weight = 1.0/(GRAPH[i][j] + GRAPH[j][i])
            g.add_edge(i,j,weight=weight)

    return g

INF = 0xffffffffffffffffffffffffff

class GraphComponent:
    def __init__(self, g, skip_evaluation=False):
        self.g = g
        if len(g) == 1:
            self.cutval = INF
        else:
            self.cutval, self.partition = nx.stoer_wagner(g)

    def split(self, skip_evaluation=False):
        g1 = self.g.subgraph(self.partition[0])
        g2 = self.g.subgraph(self.partition[1])
        return GraphComponent(g1,skip_evaluation), GraphComponent(g2,skip_evaluation)

    def show(self):
        print (",".join(repr(PORTS[x]) for x in self.g.nodes()))

def partition(graph, k):
    components = [ GraphComponent(graph) ]
    total_cost = 0
    while len(components) < k:
        # Which component has the best split?
        bestpos,bestgc = min(enumerate(components),
                             key=lambda idx_gc: idx_gc[1].cutval)

        # Apply that split.
        assert bestgc == components[bestpos]
        del components[bestpos]
        total_cost += bestgc.cutval
        skip_eval = len(components) == k-1
        new1,new2 = bestgc.split(skip_eval)
        components.append(new1)
        components.append(new2)

    return components

GRAPH = load_graph()
for p in partition(GRAPH,16):
    p.show()
