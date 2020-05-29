#!/bin/sh

pandoc --toc --standalone \
       --metadata pagetitle="Walking Onions Specification" \
       --from markdown specs/*.md > rendered.html

