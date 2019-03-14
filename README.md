</br>

<img align="left" width="100" height="100" src="logo.png">

# The Omni Project

**_Establishing a global scholarly commons_**

</br></br></br>

### Developers and fellow scholars

If you're looking to contribute, discuss, criticise or share ideas check out the [**project boards**](https://github.com/OmniProject/omni/projects) or talk with collaborators in our [**Holochain Mattermost channel**](https://chat.holochain.org/appsup/channels/app-omni).

## Installation & Usage

Please see the [releases](https://github.com/OmniProject/omni/releases) or follow the instructions below for using the most up-to-date version (although we don't guarantee that it will be stable, so if you just want a working demo use the latest release).

First, install [Holochain](https://holochain.org) using the [`v0.0.6-alpha` release](https://github.com/holochain/holochain-rust/releases/tag/v0.0.6-alpha). Be sure to follow the [quick start guide](https://developer.holochain.org/start.html) for help installing pre-requisites (you don't have to `make` if you use a binary release from the above link).

After installing the Holochain pre-requisites and Holochain itself, clone or download this repo and then run the command

`npm run dev-install`

from the project root. This will install the required node packages for both front-end development and back-end testing.

After you've run the above command, use:

`npm run dev-start`

This will start up an Omni instance with the Holochain engine, as well as a Webpack dev server for the UI which updates whenever you make changes.

# Introduction

The Omni Project is helping to bring about the next generation of scholarly communications. The current scholarly publishing system is broken—new knowledge that benefits everyone is locked behind expensive paywalls and certain individuals and companies gain disproportionate levels of power and influence, allowing them to shape the system to work for them and not for the research ecosystem as a whole. More researchers and institutions are willing to fork the bill to allow anyone to read their research, but this can't possibly be sustainable. Simply put, scholars, institutions and commoners **are getting screwed**, and we're only just beginning to wake up to this fact—publishers write the rules of the game.

# Motivations

The journal publication model made sense for the last few hundreds of years, but has rapidly become outdated within the new digital context. Although many journals have gone paperless, the _process_ of scholarly communications has not changed—publishing and journal issuing has merely been rendered in a digital format, and publishers have not made much use of the many powerful digital features we have today.

## False scarcity

The publishing system relies on false scarcity to reap the profits of the priceless work of authors. Not only is this unfair for researchers but prevents knowledge being shared more broadly. Open Access journals is a good start to help make research literally accessible, but there is still a lot of potential to improve on it and in a way that is beneficial for the research ecosystem as a whole. A lot of useful work goes unpublished (e.g. null results, replication studies, full data sets, code, mistakes worth learning from). With no straightforward way to share this knowledge around, nor any incentive system to support this stuff, a lot of potential value is binned.

## Perverse prestige system

The reputation of researchers in the current model is based on an overly reductive indicator of value, the impact factor. Number of reads and citations of articles is [not enough to accurately represent](http://backreaction.blogspot.com/2017/03/academia-is-fucked-up-so-why-isnt.html) the value researchers contribute to the knowledge commons. As scholarly communications is further digitised, we see an opportunity to make visible the **many kinds** of contributions people make to the knowledge commons and publicly recognise them for these efforts. We need [multiple currencies](http://www.artbrock.com/blog/designing-social-flows-chapter-6-designing-incentives) which promote good science and support high-quality information flows, not a scarcity-based, competitive model centered around pumping out papers or developing the catchiest title/abstract.

## Impact vs. quality

Although the validity of research is a prerequisite for getting an article through peer-review, how it is treated is binary — i.e. approved or rejected. Any claim is constantly being questioned, as any model of reality is only an assertation of an individual or group at a particular point in time, and rarely something that all parties will agree on forever. In the current system, such conclusions become truth due to the decision of a small number of editors and anonymous peer-reviewers, and not by the scholarly community collectively. We're definitely not saying that all scholars are experts in every speciality and all ought to review every article, but that any member of the scholarly community _should be able to express_ whether something is valid from their epistemological position—a statement at a particular point in time and always amendable.

We also think distinguishing _quality_ from _impact_ would allow researchers to be assessed on _process_, not only on the results of their research (where, arguably, luck can play a big part). We think if researchers develop good hypotheses and follow methodologies accurately, then they deserve credit regardless of the outcome. We see this as a way to encourage replications and the sharing of null results, as well as other 'less interesting' content which is often claimed to be the cornerstone of good research but is rarely seen. Indicators of validity and quality could identify researchers who follow _good process_, helpful where a history of success may not be indicative of future success.

## Stuck in static

Articles are still mostly treated as statically rendered documents with the figures and tables mashed obtrusively within the text. I often find myself scrolling up and down pages of text to find the figure being referenced, and then losing track of what I was just reading. This is an outdated way of viewing content, but many modern web platforms have come a long way to displaying content effectively. Reading articles can be much better with a little bit of UX design.

Digital storytelling allows for non-traditional forms of communication, which, if applied to scholarly communications, could greatly increase the comprehensibility and accessibility of research, as well as enabling large and dynamic artifacts to be displayed and interacted with nonlinearly.

## Collective intelligence

We've been interested in the developments of social networking platforms and how strongly they can shape human behaviour, for good or for worse. In Facebook, Twitter and YouTube, forums can quickly devolve into echo-chambers and polarised communities breeding outrage. To make it worse, content is often micromanaged by the platform owners to maximise user attention and to serve targeted advertising. On the other hand, platforms like Wikipedia, Reddit, Stack Overflow and Quora are capable of aggregating high quality content and facilitating insightful and productive discussions. This shows the power digital architectures and community spaces can have on individual and collective behaviour, and much could be learned from these to help the scholarly commons design a platform that prioritises relevant content to users and maximises collective intelligence.

## Centralised power (Strengthening the Scholarly Commons)

The intellectual pursuit of discovery always produces tremendous value to society, yet we have let ourselves become [enclosed](https://en.wikipedia.org/wiki/Enclosure) by the publishers who tell us they are absolutely critical and we have no choice but to go through them. Research ought to be an [act of commoning](http://wealthofthecommons.org/essay/introduction-commons-transformative-vision), something that is done for the sake of collective learning, not to make a profit for publishers or enable scholars to beat their rivals, [where studies suggest that quality of research output doesn't necessarily scale with large amounts of funding within a small number of elite researchers](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0065263). Not even [Open Access scholars have avoided enclosure](https://www.triple-c.at/index.php/tripleC/article/view/525), with [Hindawi, an Open Access publisher, having a higher profit margin (52%) than Elsevier (36%) in 2010](http://threader.ecs.soton.ac.uk/lists/boaiforum/2769.html). It's time for an alternative that resists enclosure and promotes what scholars actually care about: good science, for a start. Although editing and typesetting is important, the system can't only work for publishers, which is why we're taking a stand _to put researchers, institutions and citizens first_.

# Omni: A fully accessible digital space for sharing and discussing new knowledge

Omni is an **open source** and **distributed** platform enabling scholars to share their research with all relevant audiences, immediately. There is no publisher or publishing in Omni, yet all processes of editing, peer-review, discussion and dissemination of new knowledge are performed by participants of the Omni commons. All valuable contributions are tracked, where some tracked contributions are simply useful pieces of information for all to see, while other recognitions of contributions are directly redeemable for other forms of value.

Data is stored in a Holochain _Distributed Hash Table (DHT)_, which is a distributed database not hosted by any central party, but is supported by all users of Omni using their currently unused spare computing resources—**no servers**. As long as some users are running the Omni application, no one can shut it down.

Omni is not a journal, but is actually a _protocol_ which supports authors who wish to present their research to and receive feedback from the academic community, and allow anyone around the world to access their research for free. With an open peer-review system (both high-profile and pseudonymous), and quality assurance protocols decided on by the Omni community, we see the potential for academic research and scholarly communications to improve dramatically.

We aren't the only ones in the open source community working on new tools for scholarly researchers, and neither are we the only ones aiming to decentralise the publishing system—namely, the [Aletheia](https://github.com/aletheia-foundation/aletheia-whitepaper/blob/master/WHITE-PAPER.md) application built on Ethereum—but we see building Omni on Holochain [as the best way](https://github.com/holochain/holochain-proto/wiki/FAQ) to pull these efforts together into an efficient and scalable distributed application, uninhibited by the high costs and limitations of blockchain, whilst [also benefitting from an ecosystem of interoperable microservices](https://medium.com/holochain/holochain-reinventing-applications-d2ac1e4f25ef) such as [HoloChat](https://github.com/holochain/holochat-rust)'s distributed messaging protocols, [Humm Publisher](https://humm.earth/)'s blogging and collaborative writing tools, or even value-network / governance protocols developed by the [Commons Engine](https://commonsengine.org/). Omni, for example, is [aiming to bring peer-review](https://f1000research.com/articles/6-1151/v3) and community forum apps into the Holochain ecosystem, which we're sure will be useful for many other things. We encourage others working on similar open-source applications to also consider breaking their apps into Holochain microservices, so we can all directly benefit from each others' ideas and innovations.

Holochain also enables the community to administrate the Omni application themselves (much more simply than Ethereum DAOs), making it the way they want it—not just how we think it should be—through a process known as [mutual sovereignty](http://ceptr.org/projects/sovereign) (i.e. do people like the way it works, or would they rather boycott it, tweak it, and make it work better for everybody?). Again, all of this can be achieved at much lower costs than an Ethereum version of Omni would. Holochain is environmentally sustainable as no computing power is wasted on busywork. Holochain is cryptographically secure and tamper-proof, and Holochain applications can't 'go down' as long as at least one computer is running the application—the Omni archive of research will never disappear. Even if 50% of users suddenly dropped off the Omni network, all of the data would still be available.

You can learn more about Holochain [here](https://www.notion.so/Holochain-Reading-List-352388be758f4356a6da1fbb7962f87c).
