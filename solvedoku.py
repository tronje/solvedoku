l = []

def load():
  with open('testsudoku.txt', 'r') as f:
    read = f.readlines()
    for s in read:
      l.append(list(s))
    for e in l:
      if '\n' in e:
        e.pop()
  return l

def prepare():
  helper = [1,2,3,4,5,6,7,8,9]
  for e1 in l:
    for e2 in e1:
      if e2 != 0:
        rem(helper, e2)
        e2 = [e2]
  for e1 in l:
    for e2 in e1:
      if not isinstance(e2, list):
        e2 = helper

def solve():
  


def rem(l, val):
   return [value for value in l if value != val]