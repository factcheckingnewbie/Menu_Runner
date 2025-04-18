#/usr/bn/env bash


for profile in ~/Stuff/Settings/firefox/*; do
    label="$(echo ${profile} | cut -d/ -f7| cut -d. -f2)"
    echo "Label: \"${label}\""
    
    action_string=""
    while IFS= read -r action; do 
        [ ! -z ${action} ] && 
         action_string="${action_string} \"${action}\"";
    done < ./words.txt
    echo " Actions:${action_string}":   
    echo " Command: \"./target/debug/Menu_Runner_system '<Action>' firefox ${profile}\""
    echo "" 
done
