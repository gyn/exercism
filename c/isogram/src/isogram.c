#include <stdint.h>
#include <ctype.h>
#include "isogram.h"

int is_isogram(const char* text)
{
	uint32_t mask = 0;

	const char *p = text;
	while(*p) {
		char c = *p;

		if (isalpha(c)) {
			uint32_t v = 1u << (c - 'a');

			if ((v & mask) == v) {
				return 0;
			}

			mask |= v;
		}

		p++;
	}

	return 1;
}