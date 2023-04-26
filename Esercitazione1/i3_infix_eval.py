#!/usr/bin/env python3
'''
@author  Michele Tomaiuolo - http://www.ce.unipr.it/people/tomamic
@license This software is free - http://www.gnu.org/licenses/gpl.html
'''

# expr = term {( '+' | '-' ) term}
# term = factor {( '*' | '/' ) factor}
# factor = '-' factor | '(' expr ')' | var | num
# var = 'w' | 'x' | 'y' | 'z'

import re


def expr(tok, act):
    x = term(tok, act)
    nxt = tok.peek()
    while nxt in ('+', '-'):
        tok.consume(nxt)
        y = term(tok, act)
        if nxt == '+': x = act.add(x, y)
        else: x = act.sub(x, y)
        nxt = tok.peek()
    return x

# term = factor {( '*' | '/' ) factor}
def term(tok, act):
    x = factor(tok, act)
    nxt = tok.peek()
    while nxt in ('*', '/'):
        tok.consume(nxt)
        y = factor(tok, act)
        if nxt == '*': x = act.mul(x, y)
        else: x = act.div(x, y)
        nxt = tok.peek()
    return x

# factor = '-' factor | '(' expr ')' | var | num
def factor(tok, act):
    nxt = tok.peek()
    if nxt == '-':
        tok.consume('-')
        x = factor(tok, act)
        return act.opp(x)
    elif nxt == '(':
        tok.consume('(')
        x = expr(tok, act)
        tok.consume(')')
        return x
    elif nxt.isalpha():
        tok.consume(nxt)
        x = act.var(nxt)
        return x
    else:
        tok.consume(nxt)
        x = act.num(nxt)
        return x


class Actions:
    def __init__(self, values):
        self._values = values
    def add(self, x, y): return x + y
    def sub(self, x, y): return x - y
    def mul(self, x, y): return x * y
    def div(self, x, y): return x / y
    def opp(self, x): return -x
    def num(self, x): return float(x)
    def var(self, x): return self._values[x]


class Tokenizer:
    def __init__(self, text, regex):
        self._text = text.rstrip()
        self._point = 0
        self._token_re = re.compile(regex)

    def peek(self):
        a= self._token_re.match(self._text, self._point).group(1)
        return a

    def consume(self, x):
        m = self._token_re.match(self._text, self._point)
        if m.group(1) != x:
            raise SyntaxError("expected " + x)
        self._point = m.end()

    def end(self):
        if self._point < len(self._text):
            raise SyntaxError("Extra stuff after expression")


regex = r'\s*([A-Za-z0-9\.]+|.?)'


# Wrapper function
def parse_expr(text, act):
    tok = Tokenizer(text, regex)
    result = expr(tok, act)
    tok.end()
    return result


# Tests
values = {'w': 0.0, 'x': 1.0, 'y': 1.5, 'z': 0.5}
act = Actions(values)

if __name__ == '__main__':
    assert parse_expr('(((1.5)))', act) == 1.5
    assert parse_expr('w *    -z', act) == 0
    assert parse_expr('x / z * -y', act) == -3
    assert parse_expr('x / 0.5 * --y', act) == 3
    assert parse_expr('w', act) == 0
    assert parse_expr('(x + w) * (x + y) * (y - z)', act) == 2.5
