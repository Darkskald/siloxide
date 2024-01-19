echo $SILA_BASE_PATH
if [ -z "$SILA_BASE_PATH" ]; then
  echo "SILA_BASE_PATH not set, exiting..."
  exit 1
fi

if ! command -v xsltproc &> /dev/null; then
    echo "xsltproc is not installed."
    exit 1
fi

# Loop through each XML file in the input directory
for file in "feature_definitions"/*.xml; do
    # Extract the filename without the directory
    filename=$(basename "$file")
    # Change the file extension from .xml to .proto
    proto_filename="${filename%.xml}.proto"
    # Run xsltproc on the file and output to the out_dir
    xsltproc "$SILA_BASE_PATH/xslt/fdl2proto.xsl" "$file" > "proto/$proto_filename"
done
