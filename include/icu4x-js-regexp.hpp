#ifndef ICU4X_JS_REGEXP_HPP
#define ICU4X_JS_REGEXP_HPP

#include <memory>

#include "icu4x-js-regexp.h"

namespace icu4x {

struct ICU4XUnisetDeleter {
  void operator()(ICU4XUniset* u) const noexcept { icu4x_uniset_destroy(u); }
};

class Uniset {
private:
  Uniset(ICU4XUniset* uniset) : inner_(uniset) {}

public:
  Uniset(const char* prop_name, const char* prop_value = nullptr)
    : Uniset(icu4x_uniset_create_for_property(prop_name, prop_value)) {}

  bool exists() const { return !!inner_; }

  size_t getRangeCount() const {
    return icu4x_uniset_get_range_count(inner_.get());
  }

  uint32_t getRangeStart(size_t index) const {
    return icu4x_uniset_get_range_start(inner_.get(), index);
  }
  uint32_t getRangeEnd(size_t index) const {
    return icu4x_uniset_get_range_end(inner_.get(), index);
  }

private:
  std::unique_ptr<ICU4XUniset, ICU4XUnisetDeleter> inner_;
};

}  // namespace icu4x

#endif  // ICU4X_UNISET_HPP
