#!/bin/zsh

SUM=0
ITERATIONS=20

PAYLOAD='
{
  "center":
    {
      "a":-0.8,
      "b":0.0
    },
    "colors":256,
    "complex_width":3.1,
    "height": 720,
    "max_iterations":10000,
    "name":"basic",
    "width": 1280,
    "x_tiles":10,
    "y_tiles":10,
    "zoom":0.7
}'

FILENAME="m2_air_1280x720_10_000.csv"


echo "duration_rust_single_threaded; rust_multi_threaded; rust_rayon ; java_single_threaded; java_multi_threaded; java_multi_threaded_virtual" > $FILENAME

for i in {1..${ITERATIONS}}
do
  curl -X  POST 'http://localhost:3000/api/singlethreaded'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  DURATION_RUST_SINGLE_THREADDED=$(cat image.json | jq -r ".duration_ms")

  
  curl -X  POST 'http://localhost:3000/api/multithreaded'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  DURATION_RUST_MULTI_THREADDED=$(cat image.json | jq -r ".duration_ms")

    curl -X  POST 'http://localhost:3000/api/multithreadedopt1'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  DURATION_RUST_MULTI_THREADDED_OPT1=$(cat image.json | jq -r ".duration_ms")

  curl -X  POST 'http://localhost:3000/api/multithreadedopt2'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  DURATION_RUST_MULTI_THREADDED_OPT2=$(cat image.json | jq -r ".duration_ms")



  # curl -X  POST 'http://localhost:4000/api/singlethreaded'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  # DURATION_JAVA_SINGLE=$(cat image.json | jq -r ".duration_ms")


  # curl -X  POST 'http://localhost:4000/api/multithreaded'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  # DURATION_JAVA_MULTI=$(cat image.json | jq -r ".duration_ms")


  # curl -X  POST 'http://localhost:4000/api/multithreaded/virtual'  -H  "content-type: application/json"      -d $PAYLOAD    | jq     > image.json
  # DURATION_JAVA_MULTI_VIRTUAL=$(cat image.json | jq -r ".duration_ms")


  echo "${DURATION_RUST_SINGLE_THREADDED}; ${DURATION_RUST_MULTI_THREADDED}; ${DURATION_RUST_MULTI_THREADDED_OPT1};  ${DURATION_RUST_MULTI_THREADDED_OPT2};   " >> $FILENAME


done

