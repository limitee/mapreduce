CC = gcc

#output dir
OUTDIR = ./build

VPATH = .

OBJPATH = /data/pgsql

#the module name
MODULE_NAME = myfun.so

TARGET = $(OUTDIR)/$(MODULE_NAME)

#可以定义变量
objs = $(OUTDIR)/myfun.o $(OUTDIR)/my_sum.o

CPPFLAGS = -I/usr/local/pgsql/include/server -Wall -Wmissing-prototypes -Wpointer-arith -Wdeclaration-after-statement -Wendif-labels -Wmissing-format-attribute -Wformat-security -fno-strict-aliasing -fwrapv -Wno-unused-command-line-argument -O2

LINKFALGS = -bundle -flat_namespace -undefined suppress

all: $(OUTDIR) $(objs)
	$(CC) $(LINKFALGS) -o $(TARGET) $(objs)
	cp -r $(TARGET) $(OBJPATH)/$(MODULE_NAME)

$(OUTDIR):
	mkdir $@

$(objs): $(OUTDIR)/%.o: %.c
	$(CC) -c $(CPPFLAGS) $< -o $@

#声明clean是一个伪目标
.PHONY: clean
clean:
	-rm -rf $(OUTDIR)/*
