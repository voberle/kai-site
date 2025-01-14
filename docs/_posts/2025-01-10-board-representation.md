## Board representation

One of the first decision I have to take is how to represent the board. This is a key decision, which impacts all the initial weeks or more of development. When developing my [Nine men's morris game](https://github.com/voberle/morris), I changed several times the board representation, resulting in big refactoring each time.

Reading on chess programming shows that bitboards is what most modern engine use. It's not easy, but does sound like an interesting challenge, so this is what I will be using.

The Chess Programming Wiki section on bitboards is rather complex for beginners. The two most beginner friendly document on them I found is [this overview by Peter Keller](https://pages.cs.wisc.edu/~psilord/blog/data/chess-pages/index.html). It's not complete unfortunately, but what is there is great. And there are the [videos of Code Monkey Key](https://www.youtube.com/channel/UCB9-prLkPwgvlKKqDgXhsMQ/playlists). There are many and they are long, but they seem to always start from the bases.

I started by creating a basic BitBoard and Board struct, with print methods for them, some tests as well. I also created a method to get the king moves.

Some of the other initial decisions I had to make:

Should I use a type for the bitboard value, or even wrap in a struct? I decided to wrap it in a struct, as this allows for a more object-oriented approach, with more encapsulation of the functionalities. I'm trusting the compiler to optimize it to the same level as if there would be no struct.

```rust
pub struct BitBoard(u64);
```

How to define the board? As individual fields, or as an array? This is [discussed on CPW](https://www.chessprogramming.org/Bitboard_Board-Definition), and for now I went with an array, but it's likely to still evolve.