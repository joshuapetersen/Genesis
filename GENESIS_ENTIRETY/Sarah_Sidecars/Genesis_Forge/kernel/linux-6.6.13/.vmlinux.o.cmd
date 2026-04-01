savedcmd_vmlinux.o := ld -m elf_x86_64 -z noexecstack --no-warn-rwx-segments -r -o vmlinux.o  --whole-archive vmlinux.a --no-whole-archive --start-group lib/lib.a arch/x86/lib/lib.a --end-group  ; ./tools/objtool/objtool --hacks=jump_label --hacks=noinstr --hacks=skylake --ibt --orc --retpoline --rethunk --static-call --uaccess --prefix=16 --link vmlinux.o

vmlinux.o: $(wildcard ./tools/objtool/objtool)
