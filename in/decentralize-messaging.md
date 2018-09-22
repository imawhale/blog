published: 2020-01-29T20:00:50-0800
+++
Twitter is [funding a team](https://twitter.com/jack/status/1204766078468911106) to decentralize social media. I think they should start with messaging.

TL;DR
-----

### Why

- Messaging is a subset of social media.
- Users are tired of the wild proliferation of messaging apps.
- Social networks are all different, making them hard targets for standardization; messaging apps are all the same, making them easy targets for standardization and interoperability.
- Messaging is easier to scale.

### How

- Build on the matrix protocol.
- Extend email.
- Or maybe both.

Why
---

### messaging is a subset of social media

Messaging is a key feature of all social media platforms; decentralizing messaging is a prerequisite to decentralizing social media. And, even though it is a subset of social media, it still captures many of social media's central challenges: spam, moderation, and, most importantly, identity.

### users are tired of the wild proliferation of messaging apps

The average user is more negatively impacted by the proliferation of messaging apps than they are by the proliferation of social networks.

I have heard many people express irritation that they must use an inordinate number of messaging apps to communicate.

I haven't heard the same complaints regarding the proliferation of social media platforms. My best as to why this is the case is that the need to communicate one-on-one with an individual has no substitute. If I'm meeting Bob for lunch, being able to message anyone but Bob is completely useless. So, I must use whichever app Bob uses, or get Bob to use whatever app I use, and, if we don't have a common messaging app, one of us will need to install a new one, furthering the proliferation problem.

### social network distinctiveness

It's tempting to think that decentralizing social media should be begin with the features that we most readily associate with social media platforms, like posts, news feeds, and liking. However, the implementations of these features differ, and different social media platforms provide very different experiences.

This makes these features of social media a hard target for standardization. However, the messaging features of these platforms are all nearly identical, making messaging an easy target for standardization.

The goal is to decentralize social media as a whole, not just messaging, but messaging can be used as an initial step, with further extensions tackling the various individual features of social media platforms.

### messaging is easier to scale

At the upper end, tens of millions of users may engage with an individual tweet, whereas group messages are limited to no more than fifty users. This makes messaging a more attractive initial target for decentralization, since it avoids needing to hit massive scale on day one.

### a clear target

So, to summarize all the above, decentralized messaging has an obvious target feature set to aim for, a strong benefit to end users that will drive adoption, puts off hard scaling challenges, and can be extended in the future to all the features we associate with social media.

How
---

I see two promising and complementary paths to decentralized messaging: building on the new and shiny [Matrix protocol](https://matrix.org), and gradually extending email.

The Matrix protocol has the huge advantage of being modern. It already supports many, if not most, of the features of modern messaging apps.

Email, on the other hand, is ancient, weird, and has no support for modern luxuries like end-to-end encryption or user-presence notifications. However, email is extremely widely adopted, and a broadly supported and popular "Email 2.0" might receive a great deal of support.

Although it would be a challenging process to navigate, email could be extended with features until it reaches parity with modern messaging apps, and further extended to support the features of existing social media networks. These extensions can be added in backwards compatible ways that allow mail agents to negotiate upgrades, falling back to plain email when extensions are not available.

These extensions could include:

- delivery receipts
- optional read receipts
- user presence information
- binary serialization for efficiency and extensibility
- end-to-end encryption
- WebRTC signalling for negotiating VOIP and video chat
- signed introduction tokens to reduce spam
- a standard extension mechanism

A hybrid approach might be to design an email-to-matrix protocol upgrade negotiation mechanism, allowing legacy email addresses to seamlessly upgrade to a modern messaging protocol.

Whatever the approach, building a decentralized messaging protocol and integrating it into twitter would serve as an excellent foot in the door for decentralizing the entire social media landscape.
