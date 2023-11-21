# tjson - Json To Table

## Features
Polls http address for json data and creates a table with the chosen fields. 

## Usage

    Usage: tjson [OPTIONS] --source <SOURCE>
    
    Options:
      -p, --pointers <POINTERS>
      -s, --source <SOURCE>
      -h, --help                 Print help
      -V, --version              Print version

## Example

Json structure: 

    {
      "status": {
        "current_thread": "FG4iFxT",
        "done": 1067347099,
        "is_clean": false,
        "load": 0.19821994153580336,
        "state": "SUCCESS"
      }
    }

Point out parts of the json top poll and show.

    ./tjson --source http://localhost:8080/data.json \
        -p /status/current_thread \
        -p /status/done \
        -p /status/load \
        -p /status/state \
        -p /status/is_clean

![image](https://github.com/cannibalcow/tjson/assets/6787042/74cc78df-796f-4fb4-8702-70222bf4eb4c)


### Maybe features
    [ ] More configurations options for arguments
    [ ] History. Show previous results
    [ ] Color coding changed values
    [ ] Toggle timestamps on items in list
    [ ] If object pointed out all fields besides from arrays and sub-objects will be a column in the table
    
