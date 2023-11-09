# Optimizations
all runs using minimax for both sides
and depth of 4

each section shows
what changes were made
running average time per turn

pref.data separate folder

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
get valid used size(10)
get all used size (50)
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

## Iterators for getting Moves
average time: 0.04778242111206055
average time: 0.035840511322021484
average time: 0.05446171760559082
average time: 0.0454520583152771
average time: 0.05700221061706543
average time: 0.05101327101389567
average time: 0.05771064758300781
average time: 0.05168628692626953
average time: 0.0663758913675944
average time: 0.06042571067810058
average time: 0.05675695159218528
average time: 0.05257262786229452
average time: 0.05094111882723295
average time: 0.047427432877676826
average time: 0.045239830017089845
[ perf record: Woken up 48 times to write data ]
[ perf record: Captured and wrote 12.534 MB perf.data (787 samples) ]

## Iterator for moves with depth = 5 and 
average time: 0.8673126697540283
average time: 0.5690828561782837
average time: 0.9427290757497152
average time: 0.7884541153907776
average time: 1.0051043033599854
average time: 0.8897046645482382
average time: 1.9050840650285994
average time: 1.6775836944580078
average time: 2.0471408896976047
average time: 1.8629112005233766
average time: 1.712591366334395