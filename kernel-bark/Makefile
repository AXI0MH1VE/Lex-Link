# BARK - Binary Authority Regulatory Kernel
# Makefile for building LSM module
#
# [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]

obj-m := bark.o
bark-objs := src/bark_main.o src/bark_signature.o src/bark_entropy.o src/bark_hooks.o

KERNEL_VERSION ?= $(shell uname -r)
KERNEL_DIR ?= /lib/modules/$(KERNEL_VERSION)/build
PWD := $(shell pwd)

# Compiler flags
ccflags-y := -I$(PWD)/include -Wall -Werror

# Build targets
all: modules

modules:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) modules

clean:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) clean
	rm -f Module.symvers modules.order

install: modules
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) modules_install
	depmod -a

uninstall:
	rm -f /lib/modules/$(KERNEL_VERSION)/extra/bark.ko
	depmod -a

load:
	insmod bark.ko

unload:
	rmmod bark

reload: unload load

# Development targets
check:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) C=1 modules

sparse:
	$(MAKE) -C $(KERNEL_DIR) M=$(PWD) C=2 modules

# Documentation
.PHONY: all modules clean install uninstall load unload reload check sparse

