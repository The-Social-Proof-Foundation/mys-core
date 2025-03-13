#!/bin/bash
# Script to build the Move packages for mys-framework

set -e

FRAMEWORK_DIR="$(pwd)"
PACKAGES_DIR="${FRAMEWORK_DIR}/packages"
OUTPUT_DIR="${FRAMEWORK_DIR}/packages_compiled"

# Clean existing compiled packages
rm -rf "${OUTPUT_DIR}" 
mkdir -p "${OUTPUT_DIR}"

echo "Building packages in ${PACKAGES_DIR} and storing in ${OUTPUT_DIR}..."

# Compile and run the Rust script
rustc -o build_packages_bin build_packages.rs \
  --extern anyhow="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libanyhow*.rlib" | head -n 1)" \
  --extern move_binary_format="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libmove_binary_format*.rlib" | head -n 1)" \
  --extern move_compiler="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libmove_compiler*.rlib" | head -n 1)" \
  --extern move_package="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libmove_package*.rlib" | head -n 1)" \
  --extern mys_move_build="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libmys_move_build*.rlib" | head -n 1)" \
  --extern bcs="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libbcs*.rlib" | head -n 1)" \
  --extern regex="$(find /Users/brandonshaw/Desktop/mys-core-5/target -name "libregex*.rlib" | head -n 1)"

./build_packages_bin

echo "Packages built successfully!" 