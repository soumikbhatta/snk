# Algorithm

## Concept

**outside**

outside is an infinite place where the snake is free to do any manoeuvre 

**accessible**

for each cell, true if there is two separated path from the cell to the outside

useful because it means the snake can freely go there

## Steps

For each color
- get the list of dot that can be eaten "for free" = the snake can go to the dot and back outside
- eat them all
- for each remaining dot, compute the cost to reach it, and the cost to leave from it
