#!/bin/bash

# Start JSON array
echo "["

# Track if we need a comma separator
first_entry=true

# Loop through all Firefox profiles
for profile in ~/Stuff/Settings/firefox/*; do
    # Extract label from profile path
    label="$(echo ${profile} | cut -d/ -f7 | cut -d. -f2)"
    
    # If this is not the first entry, add a comma
    if [ "$first_entry" = false ]; then
        echo ","
    else
        first_entry=false
    fi
    
    # Create JSON object for this menu item
    echo "  {"
    echo "    \"label\": \"${label}\","
    echo "    \"actions\": ["
    
    # Process actions from words.txt
    first_action=true
    while IFS= read -r action; do
        if [ ! -z "${action}" ]; then
            if [ "$first_action" = false ]; then
                echo ","
            else
                first_action=false
            fi
            echo "      \"${action}\""
        fi
    done < ./words.txt
    
    echo "    ],"
    echo "    \"command\": \"./target/debug/Menu_Runner_system ACTION firefox ${profile}\""
    echo "  }"
done

# End JSON array
echo "]"