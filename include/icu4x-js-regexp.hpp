#ifndef ICU4X_JS_REGEXP_HPP
#define ICU4X_JS_REGEXP_HPP

#include <memory>

#include "icu4x-js-regexp.h"

namespace icu4x {

struct ICU4XUnisetDeleter {
  void operator()(ICU4XUniset* u) const noexcept { icu4x_free_unicode_set(u); }
};

class Uniset {
 private:
  Uniset(ICU4XUniset* uniset) : inner(uniset) {}

 public:
  Uniset(const char* prop_name, const char* prop_value = nullptr)
    : Uniset(icu4x_get_unicode_set_for_property(prop_name, prop_value)) {}

 private:
  std::unique_ptr<ICU4XUniset, ICU4XUnisetDeleter> inner;
};

}  // namespace icu4x

#endif  // ICU4X_UNISET_HPP
