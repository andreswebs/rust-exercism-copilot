This is where things get interesting and I get to write a full Rust program
without knowing any Rust.

The Poker exercise is one of the “hard”-labeled exercises in the Rust track in
Exercism: <https://exercism.org/tracks/rust/exercises/poker>

The instructions are simple: “Pick the best hand(s) from a list of poker hands.”
The implementation took various hours to figure out, in a long sequence of
prompts (see
[raw log](https://github.com/andreswebs/rust-exercism-copilot/blob/main/poker/ai.log.md)
of the 3rd iteraction).

Before that, I had made a few attempts to make Copilot give me the full
implementation, but soon understood the scope of it’s capacity. It turns out you
have to really get good at splitting the problem into smaller tasks, and ask for
help with the implementation of those small pieces. Over a couple of days I had
two botched attempts before I figured out how to ask the proper questions to
build a program. Those helped me get an idea of the algorithm, but I got stuck
and didn’t know how to proceed. I didn’t save a record of those sessions. But on
the third one I was able to solve the exercise. The resulting source file
(“poker v1”) had ~1000 lines of code.

Copilot was also useful on helping me organize the steps and summarize the
tasks, create documentation, and get a grasp on language concepts.

Here’s the full solution:
<https://github.com/andreswebs/rust-exercism-copilot/blob/main/poker/src/lib.rs>

And the raw log of AI interactions that led to this solution:
<https://github.com/andreswebs/rust-exercism-copilot/blob/main/poker/ai.log.md>
