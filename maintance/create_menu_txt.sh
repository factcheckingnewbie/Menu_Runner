#/usr/bn/env bash

for w in ~/Stuff/Settings/firefox/*; do 
    p="${w}"; 
    while IFS= read -r line; do [ ! -z ${line} ] &&  echo  "\\\"\\\" \"./core/menu_runner '${line}' firefox  ${p}\"" || echo ""; done < ./words.txt;  done
