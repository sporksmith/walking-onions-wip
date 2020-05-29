#!/usr/bin/python3

import os
import re

class Replacing:
    def __init__(self, fname):
        self.fname = fname
        self.newname = fname + ".tmp"

        self.inp = open(fname,'r')
        self.outp = open(self.newname, 'w')

    def finish(self):
        self.inp.close()
        self.outp.close()
        os.rename(self.newname, self.fname)

    def abort(self):
        self.inp.close()
        self.outp.close()
        os.unlink(self.newname)

    def write(self, s):
        return self.outp.write(s)

    def __iter__(self):
        return iter(self.inp)

class Idx:
    def __init__(self):
        self.p = [0]*6

    def next(self, level):
        level -= 1
        self.p[level] += 1
        self.p[level+1:] = [0]*len(self.p[level+1:])
        return ".".join(str(s) for s in self.p[:level+1])

HEADER_RE = re.compile(r'^(#+) .*')
COMMENT_STR = r'<!-- Section '

def revise(fnames):
    idx = Idx()
    for fname in fnames:
        io = Replacing(fname)
        last_was_blank = False
        try:
            for line in io:
                if line.startswith(COMMENT_STR):
                    continue
                if line.strip() == '':
                    if last_was_blank:
                        continue
                    last_was_blank = True
                else:
                    last_was_blank = False
                m = HEADER_RE.match(line)
                if m != None:
                    if line.startswith('# Appendices'):
                        idx.next(1)
                        pos = "A"
                        idx.p[0] = "A"
                    else:
                        pos = idx.next(len(m.group(1)))
                    print("<!-- Section {0} --> <a id='S{0}'></a>\n".format(pos), file=io)

                io.write(line)

        except:
            io.abort()
            raise

        io.finish()

if __name__ == '__main__':
    import sys
    revise(sys.argv[1:])
