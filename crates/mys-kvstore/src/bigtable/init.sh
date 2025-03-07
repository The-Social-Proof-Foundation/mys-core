# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
INSTANCE_ID=${1:-mys}
command=(
  cbt
  -instance
  "$INSTANCE_ID"
)
if [[ -n $BIGTABLE_EMULATOR_HOST ]]; then
  command+=(-project emulator)
fi

for table in objects transactions checkpoints checkpoints_by_digest watermark; do
  (
    set -x
    "${command[@]}" createtable $table
    "${command[@]}" createfamily $table mys
    "${command[@]}" setgcpolicy $table mys maxversions=1
  )
done
"${command[@]}" setgcpolicy watermark mys maxage=2d
