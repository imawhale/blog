published: 2019-03-12T22:30:51-0700
tags:      [programming]
title:     Unix Utilities in Rust for Great Success
+++
I've often been asked for suggestions for an appropriate first project in Rust, and I think that writing a version of a unix utility is a great choice, for a bunch of reasons!

* There is a diverse and colorful cast of characters to choose from that all provide an appropriate scope and difficulty level, such as:
 * `tree`: Print a graphical representation tree in visual form
 * `strings`: Extract plaintext strings from binary files
 * `wc`: Count the lines, characters, and bytes in a file
 * `ls`: List the contents of a directory
 * `nc`: Read and write bytes to network sockets
 * `cal`: Print a cute text calendar
 * `cat`: Copy streams to stdout
 * `cut`: Extract delimited fields from linewise text records
 * `sort`: Sort lines
 * `uniq`: Print only unique lines

* The existing implementation provided by your system serves as a specification, giving you an idea of how the tool works and whether or not your implementation has the same behavior.

* The core functionality of these utilities is very simple, allowing a learner to quickly build something useful. And, many have additional features, allowing a learner to add and build if they wish. `ls` is simple, but `ls -l` is quite the project!

* Many creative additions are possible, like colorful output, expressive configuration, and fun and useful new features.

* IO and error handling are often front-and-center when writing these utilities, which provides a great chance to get used to explicit error handling.

* [structopt](https://github.com/TeXitoi/structopt) makes argument parsing a breeze. And, by leveraging the type system and custom-derive, it provides a nice example of a situation where Rust has enormous advantages over other languages, allowing you to do more with less code.

* Rust binaries are fast to load and run, so performance is on par with native C implementations, and often much better than implementations in slower languages.

* Rust binaries are self-contained, so packaging and distribution is manageable, so you can share your work with the world.

* It's fun to use utilities that you wrote in your day-to-day workflow!

* There are lots of fabulous examples of utilities in the rust ecosystem, like [ripgrep](https://github.com/BurntSushi/ripgrep), [fd](https://github.com/sharkdp/fd), [bat](https://github.com/sharkdp/bat), [exa](https://github.com/ogham/exa), and [hexyl](https://github.com/sharkdp/hexyl). (Damn, David Peter is a beast.)

* If you're teaching others, a simple utility like `strings` makes for a great demonstration of the basics of the language.

I think whether you start with the book or a project like this depends on the learner.

I much prefer to jump in and struggle mightily, so I started with a project like this (what eventually became [just](https://github.com/casey/just/)), but I think a lot of people might prefer to start with [the book](https://doc.rust-lang.org/book/), or at least parts of the book.
