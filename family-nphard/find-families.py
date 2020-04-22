#!/usr/bin/python

import networkx as nx

def get_family_graph():
    '''not quite right since not bidi. but might establish clique
       complexity? not really.'''
    seen = set()
    G = nx.Graph()

    with open("../exit-analysis/cached-microdescs.new") as f:
        for line in f:
            if not line.startswith("family "):
                continue
            if line in seen:
                continue
            seen.add(line)
            members = line.split()[1:]
            G.add_nodes_from(members)
            for idx, m1 in enumerate(members):
                for m2 in members[idx+1:]:
                    G.add_edge(m1, m2)

    return G

g = get_family_graph()
for item in nx.algorithms.clique.find_cliques(g):
    print(item)
