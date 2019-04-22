</br>

<img align="left" width="100" height="100" src="logo.png">

# The Omni Project

**_Establishing a global scholarly commons_**

</br></br></br>

If you're looking to contribute, discuss, criticise or share ideas check out the [**project boards**](https://github.com/OmniProject/omni/projects) or talk with collaborators in our [**Holochain Mattermost channel**](https://chat.holochain.org/appsup/channels/app-omni).

## Installation & Usage

Please see the [releases](https://github.com/OmniProject/omni/releases) or follow the instructions below for using the most up-to-date version (although we don't guarantee that it will be stable, so if you just want a working demo use the latest release).

First, install [Holochain](https://holochain.org) using the [`v0.0.8-alpha` release](https://github.com/holochain/holochain-rust/releases/tag/v0.0.8-alpha). Be sure to follow the [quick start guide](https://developer.holochain.org/start.html) for help installing pre-requisites (you don't have to `make` if you use a binary release from the above link).

After installing the Holochain pre-requisites and Holochain itself, clone or download this repo and then run the command

`npm run dev-install`

from the project root. This will install the required node packages for both front-end development and back-end testing.

After you've run the above command, use:

`npm run dev-start`

This will start up an Omni instance with the Holochain engine, as well as a Webpack dev server for the UI which updates whenever you make changes.

# Introduction

The Omni Project is helping to bring about the next generation of scholarly communication. The current scholarly publishing system is broken—new knowledge that benefits everyone is locked behind expensive paywalls and certain individuals and companies gain disproportionate levels of power and influence, allowing them to shape the system to work for them and not for the research ecosystem as a whole. The Omni Project is making it easy to do scholarly publishing which is fair and benefits everyone.

**Omni** is the first Holochain app to come out of The Omni Project. 

List of projects:
* [Omni](https://github.com/OmniProject/omni) (distributed scholarly network for direct scholarly communication, treated as main access point for all other Omni Project applications)
* [Omni-Journal-Builder](https://github.com/OmniProject/omni-journal-builder) (setup wizard for new Open Access Omni journals)
* Omni-Data-Commons (a protocol for bridging and storing large scientific datasets with sharding)
* Omni-Knowledge-Commons (distributed graph of knowledge from all Omni journals)
* Fog (distributed computing network for general computation, powered by mutual credit)
* *TBD* (free reference manager)

# Problems

![Source: Twitter account @pedromics](https://pbs.twimg.com/media/DoMNn4rXsAAc_Mu.jpg =250x250)

A lot of useful - but 'unpublishable' - work goes unpublished (e.g. null results, replication studies, full data sets, code, mistakes worth learning from). This work should not go to waste. Scholars should find it easy to upload these findings to a shared digital space, and should be acknowledged for carrying out an essential part of the scientific process.

The reputation of researchers in the current model is based on an overly reductive indicator of value, the impact factor. Number of reads and citations of articles is [not enough to accurately represent](http://backreaction.blogspot.com/2017/03/academia-is-fucked-up-so-why-isnt.html) the value researchers contribute to the knowledge commons. As scholarly communications is further digitised, we see an opportunity to make visible the **many kinds** of contributions people make to the knowledge commons and publicly recognise them for these efforts.

Although the validity of research is a prerequisite for getting an article through peer-review, how it is treated is binary — i.e. approved or rejected. Any claim is constantly being questioned, as any model of reality is only an assertation of an individual or group at a particular point in time, and rarely something that all parties will agree on forever. In the current system, such conclusions become truth due to the decision of a small number of editors and anonymous peer-reviewers, and not by the scholarly community collectively. Omni makes all assertations accessible and allows any member of the scholarly community to review a piece of work, if they so wish - however, their identity will be known. We also think that getting an article published in one Omni journal shouldn't exclude it from getting published in other Omni journals.

Distinguishing _quality_ from _impact_ would allow researchers to be assessed on _process_, and not only on the results of their research (where, arguably, luck can play a big part). We think if researchers develop good hypotheses and follow methodologies accurately, then they deserve credit regardless of the outcome. We see this as a way to encourage replications and the sharing of null results, as well as other 'less interesting' content which is often claimed to be the cornerstone of good research but is rarely seen. Indicators of validity and quality could identify researchers who follow _good process_, helpful where a history of success may not be indicative of future success. 

We've been interested in the developments of social networking platforms and how strongly they can shape human behaviour, for good or for worse. In Facebook, Twitter and YouTube, forums can quickly devolve into echo-chambers and polarised communities breeding outrage. To make it worse, content is often micromanaged by the platform owners to maximise user attention and to serve targeted advertising. On the other hand, platforms like Wikipedia, Reddit, Stack Overflow and Quora are capable of aggregating high quality content and facilitating insightful and productive discussions. This shows the power digital architectures and community spaces can have on individual and collective behaviour, and much could be learned from these to help the scholarly commons design a platform that prioritises relevant content in unique ways, and maximises collective intelligence.

# Omni: A fully accessible digital space for sharing and discussing new knowledge

Omni is the first of a suite of Holochain apps which will help the scholarly commons to flourish. Omni is an **open source** and **distributed** social network platform enabling scholars to communicate and share data directly with each other, with no third party intermediating these value exchanges.

Data is stored in a Holochain _Distributed Hash Table (DHT)_, which is a distributed database not hosted by any central party, but is supported by all users of Omni using their currently unused spare computing resources—**no servers**. As long as some users are running the Omni application, no one can shut it down.

You can learn more about Holochain [here](https://www.notion.so/Holochain-Reading-List-352388be758f4356a6da1fbb7962f87c).
