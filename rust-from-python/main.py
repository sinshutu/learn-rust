#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from ctypes import cdll

lib = cdll.LoadLibrary("./target/release/libembed.so")

lib.rust_fn()

print("done!")
