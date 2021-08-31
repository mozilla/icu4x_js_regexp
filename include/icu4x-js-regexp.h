// TODO: license

#ifndef ICU4X_JS_REGEXP_H
#define ICU4X_JS_REGEXP_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

typedef struct ICU4XUniset ICU4XUniset;

ICU4XUniset* icu4x_uniset_create_for_property(const char* prop_name,
					    const char* prop_value);
ICU4XUniset* icu4x_uniset_complemented(ICU4XUniset* set);

size_t icu4x_uniset_get_range_count(const ICU4XUniset* set);
uint32_t icu4x_uniset_get_range_start(const ICU4XUniset* set, size_t index);
uint32_t icu4x_uniset_get_range_end(const ICU4XUniset* set, size_t index);

void icu4x_uniset_destroy(ICU4XUniset* set);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_UNISET_H
