# Settings for aarch64
$(info $$TARGET_ARCH is [${TARGET_ARCH}])
ifeq ($(TARGET_ARCH),aarch64)
  TARGET_TOOLCHAIN_PREFIX := aarch64-linux-gnu-
  CXXFLAGS += \
    -march=armv8-a \
    -funsafe-math-optimizations \
    -ftree-vectorize \
    -fPIC

  CCFLAGS += \
    -march=armv8-a \
    -funsafe-math-optimizations \
    -ftree-vectorize \
    -fPIC

  LDFLAGS := \
    -Wl,--no-export-dynamic \
    -Wl,--exclude-libs,ALL \
    -Wl,--gc-sections \
    -Wl,--as-needed

  LIBS := \
    -lstdc++ \
    -lpthread \
    -lm \
    -ldl

endif