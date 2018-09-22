published: 2019-03-12T13:04:05-0700
tags:      [cryptocurrency]
+++
A [Kademlia](https://en.wikipedia.org/wiki/Kademlia)-inspired modification of [Dandelion](https://arxiv.org/abs/1701.04439) for use in [Grin](https://grin-tech.org/).

### _What_

On boot and restart, Grin nodes pick a random 256-bit _node id_ which is broadcast to their peers.

Upon receiving a block, a new _epoch_ begins if `block_height % 10 == 0`. The _epoch id_ is the 256-bit blake2b hash of the block hash of the block that started the epoch. (I.E. the double blake2b hash of the block header.)

At the start of each epoch, every node:

- Calculates the _relay weight_ of itself and all peers using the formula: `relay_weight = peer_id xor epoch_id`. Relay weight is interpreted as a 256 bit unsigned integer.

- Selects the node among itself and all peers with the highest relay weight as the _relay target_.

- Determines that it is in _stem mode_ if the relay target is a peer, and in _fluff mode_ if the relay target is itself.

Then, for every stem transaction received:

- If it is in stem mode, it immediately relays it to its relay target.

- If it is in fluff mode, it aggregates transactions until its next patience timer expires, then broadcast them.

### _Why_

"Heavy" peers will naturally be chosen for relay targets, creating agreed-upon stem paths and fluff sinks, and leading to more opportunities for transaction aggregation.

### _Notes_

- In a well-connected network, one node may be the sink for all transactions during a given epoch, and thus see all unaggregated transactions during the epoch. It may thus be desirable to stick to random mode selection as in Dandelion++, so as to break up stem paths with randomly selected fluff sinks.

- If block hashes are used directly as the epoch ID, peers could select peer IDs with a large number of leading 1s, thus biasing relay target selection towards themselves. Thus, we take the double blake2b hash of the block header as the epoch ID.

- Since the epoch ID is known by all, a node could pick node IDs that had high relay weight during that epoch. To provent this, if a node connects to a new peer it should use a randomly assigned peer ID for the current epoch.

- Should this modification be called _Kademlion_, or _Objective-Dandelion_?

- Thanks to Antioch Peverell for the [clear description of Dandelion++](https://github.com/mimblewimble/grin/issues/2176) whose language I shamelessly copied.
