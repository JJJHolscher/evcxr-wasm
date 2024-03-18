#!/bin/sh

if [ "$1" = "post-render" ]; then
    if [ -d webpage/evcxr_pkg ]; then
        rm webpage/evcxr_pkg/*/.gitignore
        mkdir -p webpage/files
        mv webpage/evcxr_pkg webpage/files/
    fi

elif [ "$1" = "preview" ]; then
    quarto preview evcxr_jupyter_tour.ipynb

fi


