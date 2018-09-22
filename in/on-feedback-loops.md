published: 2018-09-05T16:48:00-07:00
hidden:    true
tags:      [programming]
+++
The most important thing that you do, both in first learning to program and in becoming an effective, happy programmer, is to cultivate tight, effective feedback loops.

## The First Loop

As a new learner, your feedback loop should look like this:

1. Pick a programming concept, initially of the  programming language that you're using, and later from programming generally

2. Exercise that concept by writing original code
    - You must create all the source files and write the code yourself, character by character, in your editor. You must not copy paste. You must not use code-completion. You must not reuse code you have already written. Programming is a learned skill, like swinging a hammer.

    - You must run the code by typing the relevant command in your terminal

    - You must read, understand, and fix all errors and warnings

3. Confirm that the code had the intended effect. Consider: If it didn't have the intended effect, how would you know? If it doesn't work, study the concept and modify your code until it does.

Many people misunderstand how tight this feedback loop should be. Reading an entire chapter of a programming book without writing code is madness. Identify the smallest piece of information you can, perhaps the difference between 32-bit and 64-bit integers, or what a `break` statement does. If you spend the vast majority of your time writing and debugging code, and not that much time studying, you are doing it right.

## The Second Loop

Once you want to build something, and you're no longer doing exercises solely to learn, the loop changes a little bit:

1. Decide on a feature or piece of functionality to implement
2. Write code that implements it and tests that make sure it works
3. Make the tests pass, and be sure that they actually test the code
4. Commit it to git with an informative commit message

Note, though, that you are always still seeking feedback.

In the second loop, you get feedback by seeing that the feature works and the tests pass. 


## On Adulthood

Think about how you would teach a child to program. You would sit next to them and show them something new, like an assignment statement. You would explain what it does, why it's useful, and then you would get them to write some assignment statments. You would ask them to write some variations, like assigning `a` to `b` and then `b` to `c`. If they had already learned about `print` statments, you would assign to and then print a variable. Maybe you would show them some assignment statements that didn't work, or were backwards, and get them to fix them.

Once they demonstrated that they understood and could independently apply the concept, you would move on, and you would make sure to have them use assignment statements when exercising the next concept.

Well, adults are just big, silly children, and they learn the same way. Don't labor under the assumption that you should do anything that wouldn't work for someone smol.
