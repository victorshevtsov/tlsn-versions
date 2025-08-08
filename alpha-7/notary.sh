#! /bin/bash

docker build -t notary:alpha-7 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-7 notary:alpha-7
