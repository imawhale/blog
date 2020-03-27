published: 2020-03-09T20:03:47-0700
tags:      [sharing, programming]
excerpt:   |
  What happened to your personal library?
  ---------------------------------------

  Rows of LPs, shelves of VHS tapes, binders of CDs and DVDs, or maybe hard drives stuffed to overflowing.

  It used to be common to maintain a personal content library, but in the last decade, most of us stopped. We ditched the tedium of tagging and sorting for Netflix, Spotify, and YouTube.

  Whenever I think about this I feel a wistful longing for the past. And I don't think it's just nostalgia.
+++

_Intermodal is not intended to be used for unauthorized sharing. Any discussion of unauthorized sharing is not permitted in any project space, including GitHub and Discord._

---

TL;DR
-----

I've just published the first release of Intermodal, a command-line BitTorrent metainfo utility.

It can create, display, and verify `.torrent` files, as well as generate magnet links.

The binary is called `imdl`.

Get it [here](https://github.com/casey/intermodal), try it out, and let me know what you think!

---

What happened to your personal library?
---------------------------------------

Rows of LPs, shelves of VHS tapes, binders of CDs and DVDs, or maybe hard drives stuffed to overflowing.

It used to be common to maintain a personal content library, but in the last decade, most of us stopped. We ditched the tedium of tagging and sorting for Netflix, Spotify, and YouTube.

Whenever I think about this I feel a wistful longing for the past. And I don't think it's just nostalgia.

Modern content channels are usually designed around a feed and automatic recommendations, and don't make a strong distinction between things that are in _your_ library and everything else. This makes it too easy to consume content compulsively and replaces active search and curation with guzzling down what's on tap, biasing me away from what _I_ like and towards what _everyone else_ likes.

Beyond that, once-accessible items often disappear due to obscure legal and contractual machinations out of one's control.

Worse still, content that is below an invisible popularity threshold is hard to find, not available, or doesn't appear in recommendation streams. One of my favorite tracks from my old music library was a song from a collection of music by people on Usenet called "It's Six AM and Gary's Refrigerator Plays a Raga". Definitely not available on Spotify.

And, if you make something yourself, you can't just put it into your own library, or send it to a friend so they can throw it in theirs.

This is profoundly saddening.

---

How did we get here?
--------------------

I think we stopped curating our own libraries due to three factors: ease of use, features, and incentives. These factors have pushed people away from decentralized content networks, like BitTorrent and store-bought CDs, and towards centralized apps, like Spotify and Netflix.

### Ease of Use

For all their flaws, centralized apps are incredibly easy to use. Just open a browser tab and play.

To get content into your own library and play it, you need to use a number of applications in concert­a web browser to search, another app to download, a file browser to organize, and finally a player to listen or watch. This is doable, but is a huge chore compared to centralized apps.

### Features

Centralized apps maintain large, proprietary databases of metadata, so features like search, recommendation, and preview work flawlessly.

Most desirable features are only as good as the metadata they're built on, so if you're curating a personal library, you have to curate the metadata along with it.

This is the primary reason I gave up on my personal music library: I spent endless amounts of time making sure that metadata was uniform and correct, and although I found some great tools to help, it still felt like a part-time job.

### Incentives

Centralized apps usually have built-in monetization mechanisms. If you like content you find through decentralized channels and want to support the creator or publisher, there is no easy means to do so.

Because of this, first-party releases on decentralized content networks are rare, and resources are instead lavished on centralized apps.

---

Where do we go from here?
-------------------------

The feature-gap between centralized apps and decentralized content networks is vast.

Fortunately, there is a relatively simple way that we can close that gap:

Developing standards for structured, machine-readable metadata in a simple, universal format.

Existing content does contain metadata. However, this metadata is limited to specific content types, for example MP3 tags; or modes of transport, for example BitTorrent.

By developing a standard for metadata manifests that can be accompany content, many desirable features become more reliable and dramatically easier to implement:

- Newly downloaded releases can be integrated into a library without user intervention. If the user has a preferred scheme for naming and tagging, that scheme can be applied automatically.

- Content from a user's library can easily be exported for sharing in a format that's appropriate for the transportation medium, for example as a torrent or as a `.zip` archive.

- By identifying the location and format of content in releases, that content can be extracted and transcoded automatically for mobile devices and web browsers.

- Rich search indices can be built from collections of content, without needing to resort to complex and error-prone heuristics.

- Immutable identifiers, for example ISBN numbers, allow metadata to be automatically updated from external sources.

- Creators and publishers can include a Bitcoin tipping address in their releases, so end users can reward them directly.

- Releases can be signed, allowing users to verify the authenticity of content, and allowing publishers to create new releases that explicitly supersede old ones.

Standardized metadata would be a enormous boon to the decentralized content ecosystem, and the Intermodal project is my attempt to make this possibility a reality.

---

Intermodal
----------

The project for developing these metadata standards, as well as tools and apps to make these standards useful, is called "Intermodal".

Seamless intermodal transportation, enabled by containerization, has led to enormous efficiency gains in the transport of physical goods.

Before the invention of the humble 40' shipping container, and the intermodal transportation network that it enabled, the majority of the world's goods were transported as so-called [bulk break cargo](https://en.wikipedia.org/wiki/Break_bulk_cargo).

Bulk break cargo is packed in bags, barrels, boxes, crates, and drums of varying sizes. Loading such cargo onto a truck, cargo ship, or train took labor and time, and was a major source of friction and cost in the shipping of goods from place to place.

The invention and standardization of intermodal containers, the 20' and 40' shipping containers of today, changed all that. Cargo could now be packed into uniformly sized containers of known strength and weight, of a size suitable for transportation by train, truck, or ship. This standardization made the once back-breaking work of moving cargo through multiple transportation modes easy and fast. What once required teams of stout men could now be done easily with cranes and other equipment.

In many ways, when it comes to decentralized digital content, we are very much living in an era of bulk break cargo. Painstaking effort is required to prepare content for transportation across different modes, e.g. BitTorrent, the web, or Usenet; and it is either impossible or takes complex heuristics to answer simple questions, like what _is_ a piece of content, anyways?

By standardizing metadata, we can make more efficient the conveyance of digital content across different modes of transportation, and allow rich services to be built on top of that content.

Thus, "Intermodal".

---

`imdl`
------

After much hand-wringing and false starts, I decided that a good place to start would be a BitTorrent metainfo (that is to say, `.torrent` file) creator. The first version is full of features and niceties, and is ready for users today.

The binary is called `imdl`, and development is hosted [on GitHub](https://github.com/casey/intermodal).

BitTorrent is widely used, and a torrent creator is much simpler than a torrent client, tracker, or index, making a good place to start.

Although `imdl` does not today have any groundbreaking new features, and no functionality for creating metadata manifests, it is a natural place to start adding such features.

`imdl` is written in Rust. Rust is fast, correct, and makes it easy to distribute self-contained binaries to users. Additionally, Rust can be compiled to [WebAssembly](https://webassembly.org/), so bits of `imdl` might eventually be adapted to run in the browser.

`imdl` will hopefully be extended with all manner of useful features, so torrent-creation functionality lives under the `torrent` subcommand: `imdl torrent create`, `imdl torrent verify`, `imdl torrent show`, and so on.

I've tried to add all the useful features of existing torrent file creators that I could find, so hopefully `imdl` is immediately useful. I owe a huge debt of gratitude to the [many exiting open-source torrent creators](https://imdl.io/book/prior-art.html), which have been a font of inspiration and good ideas.

---

What's next?
------------

First and foremost, I want `imdl` to be a useful torrent creator _today_. If you find any bugs, or have feature requests, don't hesitate to [open an issue](https://github.com/casey/intermodal/issues/new) or hop on [the discord](https://discord.gg/HaaT5Qz)!

Contributions of code, documentation, clean up, tests, ideas, and discussion are all welcome!

Beyond that, there are a few possible next steps.

One possibility is fleshing out the standards for content metadata that I've described above. However, I'm not sure it's the right thing to do just yet. I'm worried that without any consumers of this metadata, like trackers and indices, I'll be designing for abstract users and use-cases, which is always a dangerous way to work.

Another possibility is adding a simple tracker and web index to `imdl`. I could then work on the metadata standard while adding features that consume this metadata to the tracker and index. This would help ensure that the new metadata standard actually enables the intended concrete use-cases.

I think the latter is more likely, but nothing is set in stone.

Thank you so much for reading this rather long-winded blog post!

`imdl`, modest though it may be, is a love letter to the world, to sharing, and to BitTorrent. I hope you find it useful, and it becomes even more useful in the future.
