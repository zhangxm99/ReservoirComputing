# ReservoirComputing
Intro: This is a reasearch project of branch predictor. Because of the superacy of reservor computing in time-series problem. I plan to use that to make a new predicting paradigm in branch predictor.
Note:
1. the input encoding is time-related. After so many experiment, 0->01, 1->10 is the most efficient way. At first I thought just 0->0 1->0 is enough, but the effect was very bad,because if you have continuous 0,the reservoir will got no spike at all, which is equivalent to information loss. So whatever 0 or 1, you must got spike, the difference is the sequence.
2. when training the reading neuron, the study step can be longer if you want a faster convergence speed. I set 2 here.
