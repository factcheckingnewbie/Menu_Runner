#!/bin/bash
# filepath: /workspaces/CTMenu_Runner/maintance/create_menu_yaml.sh

# Create YAML file header
echo "# Menu configuration - Firefox profiles" > ../../configs/menu_config.yaml

# Loop through all Firefox profiles
for profile in ~/Stuff/Settings/firefox/*; do
    # Extract label from profile path
    label="$(echo ${profile} | cut -d/ -f7 | cut -d. -f2)"
    
    # Add this profile entry to YAML
    echo "- label: \"${label}\"" >> ../../configs/menu_config.yaml
    echo "  actions:" >> ../../configs/menu_config.yaml
    
    # Process actions from words.txt
    while IFS= read -r action; do
        if [ ! -z "${action}" ]; then
            echo "    - ${action}" >> ../../configs/menu_config.yaml
        fi
    done < ./words.txt
    
    # Add command
    echo "  command: \"./target/debug/Menu_Runner_system ACTION firefox ${profile}\"" >> ../../configs/menu_config.yaml
    echo "" >> ../../configs/menu_config.yaml
done

echo "Menu configuration saved to ../../configs/menu_config.yaml"
