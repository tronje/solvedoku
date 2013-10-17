rows = []
squares = [[],[],[],[],[],[],[],[],[]]
cols = []

def loadrows():
  with open('testsudoku.txt', 'r') as f:
    read = f.readlines()
    for s in read:
      rows.append(list(s))
    for e in rows:
      if '\n' in e:
        e.pop()
  return rows

def loadsquares():
  for i in range(0,2):
    for j in range(0,2):
#left off here
      for k in range():
       squares[k].append = rows[j][i]


  for e in rows:
    thissquare = []
    for i in range(1,3):
      thissquare.append(rows[i])

def initialize():
  for e1 in rows:
    for i in range(1,3):
      cols.append([e1])


def prepare():
  helper = [1,2,3,4,5,6,7,8,9]
  for e1 in rows:
    for e2 in e1:
      if e2 != 0:
        rem(helper, e2)
        e2 = [e2]
  # for e1 in l:
  #   for e2 in e1:
  #     if not isinstance(e2, list):
  #       e2 = helper

def solve():
  


def rem(l, val):
  return [value for value in l if value != val]