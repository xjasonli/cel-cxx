#!/bin/sh

export MSYS2_ARG_CONV_EXCL=*
bazel build --verbose_failures --sandbox_debug --config=msvc --platforms=//:x86_64-pc-windows-msvc //:cel

