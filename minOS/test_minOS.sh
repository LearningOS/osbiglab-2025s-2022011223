#!/bin/sh

rm pflash.img
rm disk.img

make pflash_img
make disk_img

make run A=app/hello_world
