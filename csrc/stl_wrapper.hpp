#include <memory>
#include <string>
#include <vector>

namespace rust {

struct alignas(alignof(std::vector<std::unique_ptr<void>>)) dummy_vector {
  uint8_t payload[sizeof(std::vector<std::unique_ptr<void>>)];
};

struct alignas(alignof(std::vector<bool>)) vector_of_bool {
  uint8_t payload[sizeof(std::vector<bool>)];
};

struct alignas(alignof(std::unique_ptr<void>)) unique_ptr_of_void {
  uint8_t payload[sizeof(std::unique_ptr<void>)];
};

} // namespace rust
