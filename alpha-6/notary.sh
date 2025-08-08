#! /bin/bash

docker build -t notary:alpha-6 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-6 notary:alpha-6
