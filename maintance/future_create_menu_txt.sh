#/usr/bn/env bash

for profile in ~/Stuff/Settings/firefox/*; do
    label="$(echo ${profile} | cut -d/ -f7| cut -d. -f2)"
    words=""
    echo "Label=${label}"
    while IFS= read -r action; do 
        [ ! -z ${action} ] && 
         words="${words} action=${action}";
    done < ./words.txt;
    echo "${words}"    
    echo  "command=./core/menu_runner <action> firefox  ${profile}" 
done
