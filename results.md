# Optimizations
what changes were made
running average time per turn
pref.data in folder

## No Change
    Finished release [optimized + debuginfo] target(s) in 1.29s
average time: 0.2232825756072998
average time: 0.13446366786956787
average time: 0.19766545295715332
average time: 0.16034746170043945
average time: 0.19766020774841309
average time: 0.17421948909759521
average time: 0.2020261287689209
average time: 0.179607093334198
average time: 0.2330626646677653
average time: 0.21125929355621337
average time: 0.19696543433449484
average time: 0.18180441856384277
average time: 0.17569314516507661
average time: 0.1633458478110177
average time: 0.1555483341217041
[ perf record: Woken up 152 times to write data ]
[ perf record: Captured and wrote 38.294 MB perf.data (2411 samples) ]


## Preallocate
get valid size(10)
get all size (50)
average time: 0.10877585411071777
average time: 0.06750965118408203
average time: 0.10060962041219075
average time: 0.08209502696990967
average time: 0.10175275802612305
average time: 0.0899731715520223
average time: 0.10450458526611328
average time: 0.09302085638046265
average time: 0.12053105566236708
average time: 0.10941817760467529
average time: 0.10239982604980469
average time: 0.09458484252293904
average time: 0.09168909146235539
average time: 0.08526958738054548
average time: 0.08138302167256674
[ perf record: Woken up 81 times to write data ]
[ perf record: Captured and wrote 20.481 MB perf.data (1288 samples) ]
writing flamegraph to "flamegraph.svg"
