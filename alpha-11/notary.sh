#! /bin/bash

docker build -t notary:alpha-11 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-11 notary:alpha-11
