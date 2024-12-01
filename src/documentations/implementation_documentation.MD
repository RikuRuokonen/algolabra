## Implementation Document


### General

General structure of program is following:
- main.rs includes all the main functions that relate to operation of the application, for example file-reading/saving and orchestrating running of functions around main-functionality.
- math.rs includes all math related functions and is most tested application(100%)
- primes.rs relates helper functions for dealing with primes and generating them.

### Time-complexity 

Time-complexity is a tad bit harder to define due to usage of Miller-Rabin. Worst case running-time is in-theory infinite since there is possible for it to never terminate. However, that is about zero-probability in practicality.
Miller Rabin's know complexity with inputs (b and k) is O(k log3 n), which defines the time-complexity here.

### Use of LLMS

ChatGPT was used to explain concepts and validating understanding of RSA-flow. 

### References

https://en.wikipedia.org/wiki/RSA_(cryptosystem)

https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
