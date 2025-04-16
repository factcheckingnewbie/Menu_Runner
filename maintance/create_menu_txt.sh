#/usr/bn/env bash

for profile in ~/Stuff/Settings/firefox/*; do 
    while IFS= read -r word; do 
        [ ! -z ${word} ] && 
            profile_part="$(echo ${profile} | cut -d/ -f7 | cut -d. -f2)" && 
            echo  "\"${word}  ${profile_part}\" \"./target/debug/Menu_Runner_system '${word}' firefox  ${profile}\"" || 
            echo ""; done < ./words.txt;  done
