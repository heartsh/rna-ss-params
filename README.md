# Library of RNA Secondary Structure Parameters
This library provides tuned computational parameters for RNA secondary structures.
Current available parameters are of [the Turner 2004 model](https://rna.urmc.rochester.edu/NNDB/turner04/index.html) or [the CONTRAfold v2.02 model](http://contra.stanford.edu/contrafold/).
The Turner 2004 model provides experimentally estimated structure parameters based on nearest neighbor thermodynamics.
In contrast, the CONTRAfold v2.02 model provides secondary structure parameters computationally optimized by maximizing the log likelihood of training secondary structures.
My implementation of the CONTRAfold v2.02 model is just the copy of the CONTRAfold source code, pre-trained by CONTRAfold developers decade ago.
The comparison between the Turner 2004 model and the CONTRAfold v2.02 model is as below:
|:-:|

| Model | Turner | CONTRAfold |
| --: | --: | --: |
| Model simpleness | Complex | Simple |
| Parameter optimization | Wet & dry | Dry only |
| Are RNA loop structures captured? | Yes | Yes |
| Are noncanonical base-pairings allowed? | No | Yes (but, they are disabled in this repository) |

# Author
[Heartsh](https://github.com/heartsh)

# License
Copyright (c) 2017 Heartsh  
Licensed under [the MIT license](http://opensource.org/licenses/MIT).
