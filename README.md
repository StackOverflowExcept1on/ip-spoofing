### ip-spoofing

[![Build Status](https://github.com/StackOverflowExcept1on/ip-spoofing/workflows/CI/badge.svg)](https://github.com/StackOverflowExcept1on/ip-spoofing/actions)

Library to send fake IPv4 headers & UDP/TCP-SYN packets to perform L3/L4 attacks

In short, this library allows you to spoof your IP address on the network. For a better understanding, it is recommended
to read the article from cloudflare:
[The real cause of large DDoS - IP Spoofing](https://blog.cloudflare.com/the-root-cause-of-large-ddos-ip-spoofing/)

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
