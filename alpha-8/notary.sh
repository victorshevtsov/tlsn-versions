#! /bin/bash

docker build -t notary:alpha-8 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-8 notary:alpha-8
