#!/usr/bin/env python3
'''
@author  Michele Tomaiuolo - http://www.ce.unipr.it/people/tomamic
@license This software is free - http://www.gnu.org/licenses/gpl.html
'''

from i3_infix_eval import parse_expr

class Actions:
    def add(self, x, y): return f"+ {x} {y}"
    def sub(self, x, y): return f"- {x} {y}"
    def mul(self, x, y): return f"* {x} {y}"
    def div(self, x, y): return f"/ {x} {y}"
    def opp(self, x): return f"~{x}"
    def num(self, x): return x
    def var(self, x): return x


# Tests.
act = Actions()

if __name__ == '__main__':
    assert parse_expr('(((1.5)))', act) == "1.5"
    assert parse_expr('w * -z', act) == "* w ~z"
    assert parse_expr('x / z * -y', act) == "* / x z ~y"
    assert parse_expr('x / 0.5 * --y', act) == "* / x 0.5 ~~y"
    assert parse_expr('w', act) == "w"
    assert parse_expr('(x + w) * (x + y) * (y - z)', act) == "* * + x w + x y - y z"
