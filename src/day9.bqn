Diff ← {1↓𝕩-»𝕩}
AllZero ← {(+´𝕩=0)=≠𝕩}
PascalIsh ← {take Repeat ls:
  diff ← Diff ls
  {AllZero diff?
    take ↑ ls
  ;
    (take ↑ ls) ∾ take Repeat diff
  }
}

Ends ← (¯1⊸PascalIsh)
Beginnings ← (1⊸PascalIsh)

Part1 ← (+´ Ends)
Part2 ← (0 -´ Beginnings)