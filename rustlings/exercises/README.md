# Exercise to Book Chapter mapping

| Exercise               | Book Chapter        |
| ---------------------- | ------------------- |
| variables              | §3.1                |
| functions              | §3.3                |
| if                     | §3.5                |
| primitive_types        | §3.2, §4.3          |
| vecs                   | §8.1                |
| move_semantics         | §4.1-2              |
| structs                | §5.1, §5.3          |
| enums                  | §6, §18.3           |
| strings                | §8.2                |
| modules                | §7                  |
| hashmaps               | §8.3                |
| options                | §10.1               |
| error_handling         | §9                  |
| generics               | §10                 |
| traits                 | §10.2               |
| tests                  | §11.1               |
| lifetimes              | §10.3               |
| iterators              | §13.2-4             |
| threads                | §16.1-3             |
| smart_pointers         | §15, §16.3          |
| macros                 | §19.6               |
| clippy                 | §21.4               |
| conversions            | n/a                 |


# Doing exercises
The exercises are sorted by topic and can be found in the subdirectory rustlings/exercises/<topic>. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keeps them from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

rustlings watch
This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the exercises/ directory. If you want to only run it once, you can use:

`rustlings verify`
This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

`rustlings run myExercise1`
Or simply use the following command to run the next unsolved exercise in the course:

`rustlings run next`
In case you get stuck, you can run the following command to get a hint for your exercise:

`rustlings hint myExercise1`
You can also get the hint for the next unsolved exercise with the following command:

`rustlings hint next`
To check your progress, you can run the following command:

`rustlings list`

# Testing yourself
After every couple of sections, there will be a quiz that'll test your knowledge on a bunch of sections at once. These quizzes are found in exercises/quizN.rs.

# Enabling rust-analyzer
Run the command rustlings lsp which will generate a rust-project.json at the root of the project, this allows rust-analyzer to parse each exercise.
