#!/bin/bash
#
tag=$(git ls-remote --tags --sort=committerdate|tail -1|awk -F / '{print $3}')

minor=$(echo $tag|cut -d. -f3)
minor=$((minor+1))
new_tag=$(echo $tag|sed -e s"|\.*\.*\.[0-9]$|.$minor|")
echo "##vso[task.setvariable variable=next_tag_version]$new_tag"
