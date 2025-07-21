# cache-analysis
Created to replicate studies from this paper: https://dl.acm.org/doi/pdf/10.1145/230908.230913

This project is not done on a distributed file system, but rather just the local filesystem.

The caches are implemented in the same way, except for their eviction policy.
I included the following policies:
* Random - self explanatory!
* LRU - least recently used
* FIFO - first in, first out
* Frequency - last frequently used is evicted

To perform this experiment, we will be caching reads from files. The test procedure will be as follows:
* Create some number of randomly generated text files
* Create an order in which they will be accessed, according to a Zipf distribution
* Start a timer, and access the files in the given order
* Stop the test and report results!
* (clean up text files)

Files will be stored in /var/tmp/cache-test/

### NOTE:

> The LRU cache uses an O(n) deletion, resulting in worse performance. This should be adjusted in future work