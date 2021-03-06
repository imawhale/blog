published: 2020-01-22T23:56:19-0800
tags:      [programming]
title:     Popcorn Time Should Become a Browser
+++

_I am not a lawyer. This is not legal advice._

Popcorn Time-style video streaming apps seem to be vulnerable to legal
action by rightsholders.

For example, the
[popcorntime.sh domain was recently suspended](https://torrentfreak.com/registrar-suspends-popcorn-time-domain-name-following-complaint-200121/),
and [the operator of a site which merely provided information about how to obtain and use Popcorn Time was sentenced to prison](https://torrentfreak.com/operator-of-popcorn-time-info-site-is-liable-for-piracy-supreme-court-rules-200115/).

Although given that Popcorn Time's servers do not themselves host
infringing content this may seem a bit unfair, it is simply the reality
of the world we live in.

It is interesting to note, however, that although web browsers can be
used in exactly the same way as Popcorn Time, namely searching for and
viewing copyrighted movies, the developers of web browsers have thus far
not faced successful legal challenges.

I believe that there are two major factors that differentiate Popcorn
Time from web browsers, and contribute to Popcorn Time's legal
vulnerability:

1. Upon opening a web browser, the user must supply the URL of the site
   they wish to visit. Compare this to Popcorn Time, where upon opening
   the application for the first time, the user is immediately presented
   with a selection of unlicensed Hollywood movies to download.

   This difference greatly weakens Popcorn Time's claims against
   contributory copyright infringement. The developers of Popcorn Time
   cannot credibly claim to be ignorant to the types of movies that
   their users are downloading, whereas a web browser developer can
   credibly claim that the choice of website to visit is always left to
   the end user.

1. Web browsers have substantial non-infringing uses. "Substantial
  non-infringing use" is jargon for a legal test used in the United
  States which protects the creator or purveyor of a piece of technology
  from liability for its use in infringement by users if that technology
  has "substantial non-infringing uses". For the canonical example, see
  [_Sony Corp. of America v. Universal City Studios, Inc._](https://en.wikipedia.org/wiki/Sony_Corp._of_America_v._Universal_City_Studios,_Inc.).

  Whereas a web browser provider can rightly claim that there are many
  websites,
  [and it's possible an infringing one might slip in, there would be no way of knowing](https://youtu.be/2JO3oJybBTw),
  the popcorn time developers cannot claim the same. There is a single,
  fixed Popcorn Time backend, run by the Popcorn Time team, serving the
  same search results to every user.

Fortunately, even given the above, I believe that Popcorn Time could
implement a small number of strategic changes that would allow it to
withstand future legal aggression:

- Add a URL bar, like a web browser's, to the top of the current GUI.
- Make the URL bar empty when opening the app for the first time, and
  display no default search results to the user.
- When the user enters a URL, use the existing Popcorn Time API protocol
  to contact the backend at that URL, and begin displaying search
  results from that backend to the user.
- If the app is closed and re-opened, keep the URL bar and the search
  results populated from the last run of the app.

These changes would be simple, have a minimal impact on the current
(quite excellent!) user experience, and leverage interaction flows, i.e.
a URL bar, that users already understand.

Additionally, the Popcorn Time team should create an official default
search backend which focuses exclusively on free, non-infringing,
user-created content.

These steps would, I believe, nicely protect the developers and
distributors of Popcorn Time from further legal challenges:

- Popcorn Time, when first opened, would display no search results by
  default, and would require the user to direct the app with the URL of
  a site that they would like to visit. This would avoid the current
  situation where a litigator sitting in front of a judge can open the
  Popcorn Time application, and it will by immediately display a choice
  selection of infringing content, which makes for a terrible first
  impression.

- Users of the Popcorn Time application could use it to access both
  infringing and non-infringing content, providing a nice argument that
  the application does in fact have _substantial non-infringing uses_.

- Infringement via the application would be entirely at the direction of
  the end-user, and not directly assisted by the default search results
  provided by the developers.

- Developers wouldn't be running a search back end with infringing
  content, which is the only component of the current Popcorn Time
  system that stores problematic metadata about infringing content.

- I suspect that the official free-content focused search backend would
  be successful and useful in its own right.

- Since the search backend could now be switched by end users, a very
  large number of third-party backends would likely appear, serving
  content that might not be otherwise available from the current Popcorn
  Time backend.

In short, Popcorn Time should become a browser, and then all those
lawyers and their handlers can go pound sand.
