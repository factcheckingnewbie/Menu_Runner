#!/usr/bin/env bash

# systemd-run --user --scope --unit=firefox-test firefox --profile ~/firefoxprofiles/wj547bg.Media

func="${1}"
app="${2}"
profile="${3}"


eol="/"
if [[ "${profile: -1}" == "$eol" ]]; then
  str="${profile%$eol}"
  profile="$str"
fi

profile_part="${profile##*/}"

echo "profile-%&eol: ${profile%$eol}"
echo "Profile part: ${profile_part}"
echo "Profile: ${profile}"

function app_start() {
# systemd-run --user --scope --slice=user."$USER".slice --unit="app.$app"-"$profile_part" "$app" --profile "$profile"
   systemd-run --user --scope --slice=user."${USER}".app.${app}.slice --unit=app.${app}-"${profile_part}" "${app}" --profile "${profile}"
}

function app_status() {
    systemctl --user status app.${app}-${profile_part}.scope
}

function app_full_syspath() {
    s="$(app_status | grep CGroup | cut -d: -f2 | cut -d' ' -f2)"
    echo "/sys/fs/cgroup/${s}"
}

function app_freeze()  {
    s="$(app_full_syspath)/cgroup.freeze"
    echo  1 > ${s} 
}

function app_unfreeze()  {
    s="$(app_full_syspath)/cgroup.freeze"
echo  0 > ${s} 
}

function app_kill()  {
    s="$(app_full_syspath)/cgroup.kill"
echo  1 > ${s} 
}

function show_error() {
    echo "${1}" >&2
}

main() {
    if [[ x${1}x == x"start"x ]]; then
        app_start &
        app_full_syspath
    elif [[ x${1}x == x"status"x ]]; then
        app_status
    elif [[ x${1}x == x"syspath"x ]]; then
        app_full_syspath
     elif [[ x${1}x == x"freeze"x ]]; then
        app_freeze
    elif [[ x${1}x == x"unfreeze"x ]]; then
        app_unfreeze
    elif [[ x${1}x == x"kill"x ]]; then
        app_kill
     else
        show_error "Unknown function: ${1}"
    fi
}

# exit
main "$@"

