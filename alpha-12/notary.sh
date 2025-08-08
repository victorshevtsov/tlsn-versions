#! /bin/bash

docker build -t notary:alpha-12 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-12 notary:alpha-12
