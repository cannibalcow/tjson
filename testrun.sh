cargo run -- --source http://localhost:8080/data.json \
  -p /status/current_thread \
  -p /status/done \
  -p /status/load \
  -p /status/state \
  -p /status/is_clean
