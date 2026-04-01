savedcmd_drivers/char/agp/built-in.a := rm -f drivers/char/agp/built-in.a;  printf "drivers/char/agp/%s " intel-gtt.o | xargs ar cDPrST drivers/char/agp/built-in.a
