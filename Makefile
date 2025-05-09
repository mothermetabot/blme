# Stop on first error and echo each command
.SHELLFLAGS  = -eu -o pipefail -c
.SILENT:

# === Edit these to match your crate ===
PLUGIN_NAME := blme        # must match [package].name in Cargo.toml
PROFILE     := release     # Cargo profile (debug or release)

# Paths (relative to this Makefile)
TARGET_DIR  := target/$(PROFILE)
LUA_DIR     := lua

# Filenames for the built library
# Linux: libblme.so
# macOS: libblme.dylib
# Windows: blme.dll
OS := $(shell uname)
ifeq ($(OS),Darwin)
  SRC_LIB := $(TARGET_DIR)/lib$(PLUGIN_NAME).dylib
else ifeq ($(OS),Linux)
  SRC_LIB := $(TARGET_DIR)/lib$(PLUGIN_NAME).so
else
  # fallback for Windows under MSYS/WSL etc.
  SRC_LIB := $(TARGET_DIR)/$(PLUGIN_NAME).dll
endif
DST_LIB := $(LUA_DIR)/$(PLUGIN_NAME)$(notdir $(SRC_LIB) | sed 's/^.*\(.\{4\}\)/\1/')  # preserves extension

# Default target: build, copy, clean
.PHONY: all
all: copy clean

# 1) Build in release mode
.PHONY: build
build:
	@echo "[1] cargo build --release"
	cargo build --release

# 2) Copy the built library into lua/
.PHONY: copy
copy: build
	@echo "[2] copying $(SRC_LIB) ¿ $(DST_LIB)"
	@mkdir -p $(LUA_DIR)
	@cp $(SRC_LIB) $(DST_LIB)

# 3) Cleanup the profile folder
.PHONY: clean
clean:
	@echo "[3] removing $(TARGET_DIR)"
	@rm -rf $(TARGET_DIR)
.SHELLFLAGS  = -eu -o pipefail -c
.SILENT:

# === Edit these to match your crate ===
PLUGIN_NAME := blme        # must match [package].name in Cargo.toml
PROFILE     := release     # Cargo profile (debug or release)

# Paths (relative to this Makefile)
TARGET_DIR  := target/$(PROFILE)
LUA_DIR     := lua

# Filenames for the built library
# Linux: libblme.so
# macOS: libblme.dylib
# Windows: blme.dll
OS := $(shell uname)
ifeq ($(OS),Darwin)
  SRC_LIB := $(TARGET_DIR)/lib$(PLUGIN_NAME).dylib
else ifeq ($(OS),Linux)
  SRC_LIB := $(TARGET_DIR)/lib$(PLUGIN_NAME).so
else
  # fallback for Windows under MSYS/WSL etc.
  SRC_LIB := $(TARGET_DIR)/$(PLUGIN_NAME).dll
endif
DST_LIB := $(LUA_DIR)/$(PLUGIN_NAME)$(notdir $(SRC_LIB) | sed 's/^.*\(.\{4\}\)/\1/')  # preserves extension

# Default target: build, copy, clean
.PHONY: all
all: copy clean

# 1) Build in release mode
.PHONY: build
build:
	@echo "[1] cargo build --release"
	cargo build --release

# 2) Copy the built library into lua/
.PHONY: copy
copy: build
	@echo "[2] copying $(SRC_LIB) ¿ $(DST_LIB)"
	@mkdir -p $(LUA_DIR)
	@cp $(SRC_LIB) $(DST_LIB)

# 3) Cleanup the profile folder
.PHONY: clean
clean:
	@echo "[3] removing $(TARGET_DIR)"
	@rm -rf $(TARGET_DIR)
