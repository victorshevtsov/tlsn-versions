#! /bin/bash

docker build -t notary:alpha-9 ./

docker run --init -it --rm -p 7047:7047 --name notary-alpha-9 notary:alpha-9
