#! /bin/bash

docker build -t notary:alpha-10 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-10 notary:alpha-10
