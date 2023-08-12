#!/bin/sh

source ./scripts/util.sh

# command="echo one"
# command+=' {"two     three":"four"}'
# # command+=" {\"two    three\":\"four\"}"

# #execute_wrapper "$command"
# execute_array "$command"




# define the array
# mycmd=(echo one {"two    three":"four"}})

mycmd=(echo one)
mycmd+=('{"two    three":"four"}')

# # expand the array, run the command
# # "${mycmd[@]}"

execute_array "${mycmd[@]}"