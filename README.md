# "rna-ss-params", Library of RNA Secondary Structure Parameters
This library provides RNA secondary structure parameters.
Current available parameters are of [Turner 2004 model](https://rna.urmc.rochester.edu/NNDB/turner04/index.html) or [CONTRAfold v2.02 model](http://contra.stanford.edu/contrafold/).
Turner 2004 model provides experimentally estimated structure parameters based on nearest neighbor thermodynamics.
In contrast, CONTRAfold v2.02 model provides secondary structure parameters computationally optimized by maximizing the log likelihood of training secondary structures.
My implementation of CONTRAfold v2.02 model is just the copy of the CONTRAfold source code, pre-trained by CONTRAfold developers decade ago.
I will supply the CONTRAfold v2.02 model trained by myself using the databases of recent reference RNA secondary structures (e.g., [Rfam](http://rfam.xfam.org/) and [bpRNA-1m](http://bprna.cgrb.oregonstate.edu/)).

# Author
[Heartsh](https://github.com/heartsh)

# License
Copyright (c) 2017 Heartsh  
Licensed under [the MIT license](http://opensource.org/licenses/MIT).
