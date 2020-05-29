#!/bin/sh

pandoc --toc --standalone \
       --number-sections \
       --metadata pagetitle="Walking Onions Specification" \
       --from markdown specs/*.md > rendered.html

