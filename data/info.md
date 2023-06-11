## from problem statement
* IIPIFFCPICICIICPIICIPPPICIIC -> health_check

## health_check:
* IIPIFFCPICFPPICIICCIICIPPPFIIC -> repair_guide/initial_0

## repair_guide/initial_0
* IIPIFFCPICFPPICIICCIICIPPPFIIC -> repair_guide/initial_0
* IIPIFFCPICFPPICIICCCIICIPPPCFIIC -> repair_guide/topics_1
* IIPIFFCPICPCIICICIICIPPPPIIC -> rotate_planet

## rotate_planet
* IIPIFFCPICFPPICIICCIICIPPPFIIC -> repair_guide/initial_0

## repair_guide/topics_1

### Transcript:
Repair guide navigation
Access repair guide topics by taking an existing repair guide prefix
and replacing the encoded integer with the encoding of the integer
of the desired repair guide page

Encoding of positive n-bit integers
b1...b_n

bi = C if the ith least-significant bit is set
bi = I otherwise

Catalog page index:  1337

Happy navigating

<blink>WARNING</blink>
REVERSAL may occur!
Don't change the length of integers, or TOTAL PROTONIC

### catalog_page_prefix
IIPIFFCPICFPPICIICC[] IICIPPP[] FIIC   => 0
IIPIFFCPICFPPICIICC[C]IICIPPP[C]FIIC   => 1
IIPIFFCPICFPPICIICC[CCCCCCCCCC]IICIPPP[FCCFFFCCFC]FIIC => 1337

## repair_guide/structure_of_genome_1729

Red | Green | Blue
        IFPICFPPCFIPP
marker: IFPICFPPCCC
between:
IFFCPICCFPICICFFFIIPIFFICIICIIPIFFICIICIIPIFFCPICCFPICICFPPICICIICIIPIPIICICIICCCCCICIICICIICIIPIIPIPCCICCICCICCIIIIIIIIIIIIIPIICIPCICIICIICCCCCICICICCIICIPIICIIPIFFCPICCFPICICFFCIICIICIPPCCPCPICCFPICICFPPICICIPPIICPCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCIPPPIPPCPIPPCICPIIPICPIPPICPIIC

Red:
where things happen
when it dies blue zone tells what to do next


Green:
starts at: IFPICFPPCFFPP
constant in size
feeds red zone
we can modify to repair the Funn

Blue Zone
starts after: IFPICFPPCFIPP

Example:
Activate gene location: 1234, gene len: 500
and pass integer 42
IIPIFFCPICCFPICICFPCICICIICIICIPPPCFCFCFCCCCCCCCCCCCCCCCCICIIC

and pass boolean false
IIPIFFCPICCFPICICFPCICICIICIICIPPPPIIC

activate adapter
IIPIFFCPICCFPICICFFFIIPIFFCPICCFPICICFFFIICIICIICIPPPCFCCFCFFCCFICCCFCFFFFFICIPPCPIIC

all:
IIPIFFCPICCFPICICFPCICICIICIICIPPPCFCFCFCCCCCCCCCCCCCCCCCICIICIIPIFFCPICCFPICICFPCICICIICIICIPPPPIICIIPIFFCPICCFPICICFFFIIPIFFCPICCFPICICFFFIICIICIICIPPPCFCCFCFFCCFICCCFCFFFFFICIPPCPIIC

1234 => ICIICICCIICP
42 => ICICICP

======================================================================
24bit per number
genes:
offset    len
000510    000018  AAA_geneTablePageNr
...
37870e    00372b  appletree