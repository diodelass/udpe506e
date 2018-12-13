# udpe506e  

### Say what?
- Sends one UDP packet containing 508 `e` - the maximum safe UDP payload size - to every address on 
the internet.
- eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
- Uses port 101 (ASCII ordinal of `e`)
- I recommend you don't run it
- AGPL
- Comes with a 32-bit x86 linux binary, because I forgot to add that to the `.gitignore`
- R.I.P [e30e/e98e](https://linuxwit.ch/blog/2018/12/e98e/)

### How do I build it?
Please don't.  
(Use rustc)

### It seems slow. How long should it take to run?
Sending packets to all of IPv4 should take about 6 hours with a fast computer on a gigabit connection.  
Sending them to all of IPv6 (which begins immediately after IPv4 is completed) will take at least 10
septillion years with the same setup. Make sure to run it on a VPS provided by a company that will be 
around a while, like Amazon or Google.  

### How many e will this program send out in total?
If allowed to run until it stops, it will have transmitted precisely 
172,863,442,395,836,739,439,394,300,577,520,094,806,016 `e`, or roughly 1.7 trillion trillion quadrillion 
of them. one teraterapeta-e, if you will.  
The actual number of `e` received by the destinations will of course be far less, because UDP does not 
make any guarantees about delivery. Also, the vast majority of IPv6 addresses are unused. One could
sit here and look through the statistics to find a number of `e` expected to ultimately reach their 
destination, but that sounds like a lot of work, and given the scale of the amount of data sent by this
program if it is allowed to finish, it seems unlikely that current statistics about the internet would
even be relevant anyway.

### Will you help me make a distributed version of this that I can run on a botnet I control?
No.

### Isn't this basically a DDoS attack? You shouldn't be hosting evil software here!
What? No.  
This is the opposite of DDoS attack. An effective DDoS involves many computers all sending data to a 
small number of target computers, overwhelming the target with a large volume of requests. This program
sends a single packet to each address on the internet, meaning that each server will receive about one
packet. Since most servers probably won't expect it, their software will just filter it out, and continue
on as normal. It will be like a single-port port scan, except even less invasive, because most servers
will not have anything listening on UDP port 101 (HOSTNAME uses port 101, but I believe this is TCP only),
and the packet will simply be rejected without a reply.  
Meanwhile, your computer - the one running this program - will have one of its processor threads tied up
for up to 10 septillion years, during which it will also be transmitting a huge amount of data that might
very well saturate its network upload bandwidth. I suppose you could think of this as a "DoS attack" of 
sorts, but it's not distributed, and the only victim is yourself.
