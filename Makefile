# Makefile — build Rust plugin, copy .so into lua/, and clean up

# === Edit these to match your crate ===
PLUGIN_NAME := blme
PROFILE     := release            

# Paths (relative to this Makefile)
TARGET_DIR  := target/$(PROFILE)  # where Cargo emits artifacts
LUA_DIR     := lua                # where Lua will look for myplugin.so

# The compiled library filenames
SRC_LIB     := $(TARGET_DIR)/lib$(PLUGIN_NAME).so
DST_LIB     := $(LUA_DIR)/$(PLUGIN_NAME).so

# Default target: build, copy, then cleanup
.PHONY: all
all: build copy clean

# 1) Build in release mode
.PHONY: build
build:
	cargo build --release

# 2) Copy the .so into lua/
.PHONY: copy
copy: build
	@mkdir -p $(LUA_DIR)
	@cp $(SRC_LIB) $(DST_LIB)
	@echo "Copied $(SRC_LIB) → $(DST_LIB)"

# 3) Remove the profile folder under target/
.PHONY: clean
clean:
	@rm -rf $(TARGET_DIR)
	@echo "Removed $(TARGET_DIR)"
