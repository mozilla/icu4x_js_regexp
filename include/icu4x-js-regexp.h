// TODO: license

#ifndef ICU4X_JS_REGEXP_H
#define ICU4X_JS_REGEXP_H

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XUniset ICU4XUniset;

ICU4XUniset* icu4x_get_unicode_set_for_property(const char* prop_name,
						const char* prop_value);

void icu4x_free_unicode_set(ICU4XUniset*);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_UNISET_H
