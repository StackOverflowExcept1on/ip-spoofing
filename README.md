### ip-spoofing

[![Build Status](https://github.com/StackOverflowExcept1on/ip-spoofing/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/ip-spoofing/actions)

Library to send fake IPv4 headers & UDP/TCP-SYN packets to perform L3/L4 attacks

In short, this library allows you to spoof your IP address on the network. For a better understanding, it is recommended
to read the article from cloudflare:
[The real cause of large DDoS - IP Spoofing](https://blog.cloudflare.com/the-root-cause-of-large-ddos-ip-spoofing/)

It can be done on the L3 (network layer) of the [OSI model](https://en.wikipedia.org/wiki/OSI_model#Layer_architecture)

Today, not all ISPs check the integrity of IPv4 headers.
Therefore, in a real network, there are 2 options for spoofing IP addresses:

1. network level IP spoofing

   e.g. you have a server with the address `195.174.232.102`, and the provider owns the IP
   range `195.174.224.0 - 195.174.239.255`, this means that you can use any address from the range

2. unlimited IP spoofing

   this allows you to spoof any ip address, you can pretend you own the address `8.8.8.8` (Google Public DNS)

The only limitation of spoofing is that you can send packets, but you cannot receive a response from the server.

You can check if this library works on your local network.
To attack real networks, you need a specific provider that allows one of 2 spoofing options.

### Useful links

- [Internet Protocol version 4](https://en.wikipedia.org/wiki/Internet_Protocol_version_4) wikipedia article describing
  the IPv4 header

- [rickettm/SendIP](https://github.com/rickettm/SendIP) repository provides command line tool to allow sending arbitrary
  IP packets

  useful code of the SendIP project written in C:
    - [creating raw sockets](https://github.com/rickettm/SendIP/blob/aad12a001157489ab9053c8665e09aec24a2ff6d/sendip.c#L143)
    - [IPv4 header structure](https://github.com/rickettm/SendIP/blob/aad12a001157489ab9053c8665e09aec24a2ff6d/ipv4.h)
    - [IPv4 header checksum](https://github.com/rickettm/SendIP/blob/aad12a001157489ab9053c8665e09aec24a2ff6d/csum.c)
