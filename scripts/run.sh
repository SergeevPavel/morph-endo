#!/bin/bash

L1="_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..FromIterator$LT$T$GT$$GT$::from_iter::hff62afd4ffeea829"
L2="_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$alloc..vec..spec_from_iter..SpecFromIter$LT$T$C$I$GT$$GT$::from_iter::hb742b9b374dbaf4f"
echo $L1 | rustfilt
echo $L2 | rustfilt