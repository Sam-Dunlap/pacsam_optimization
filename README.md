# PACSAM ROUTE FINDER

Inspired by [Tom7's](tom7.org) [PacTom project](https://www.youtube.com/watch?v=1c8i5SABqwU), this is a tool for finding the shortest route through a given neighborhood that traverses the length of each street at least once. It implements Edmonds' original blossom algorithm to create a graph of the input neighborhood that has an Euler cycle\*, then Hierholzer's algorithm to find that cycle.

\* as of version 0.1.0 the code doesn't actually implement the blossom algorithm. I need to do some serious refactoring to get the graph represented in a way that allows me to write and run the blossom algorithm, so this commit is just to save my progress before I get into the weeds.
