#!/bin/bash

BLOCKTYPE=$1
SRC_FILE=$2
DEST_FILE=$3

echo -e "\\n\\n\`$SRC_FILE\`" >> $DEST_FILE
# Append the clipboard contents to the target file, surrounded by backticks
echo -e "\\n\`\`\`${BLOCKTYPE}\\n$(pbpaste)\\n\`\`\`\\n" >> $DEST_FILE
