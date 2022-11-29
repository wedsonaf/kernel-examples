obj-m := thread.o

KDIR := /lib/modules/$(shell uname -r)/build

all:
	make -C $(KDIR) M=$(shell pwd) LLVM=1 modules

clean:
	make -C $(KDIR) M=$(shell pwd) LLVM=1 clean
