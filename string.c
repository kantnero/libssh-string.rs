#include <stdio.h>
#include <string.h>

typedef struct {
	unsigned char *data;
	size_t size;
} ssh_string_struct;

extern ssh_string_struct *ssh_string_new(size_t);
extern int ssh_string_fill(ssh_string_struct *, void *, size_t);


int main() {
	ssh_string_struct *s1 = ssh_string_new(5);
	printf("%p\n%p\n%d\n", s1, s1->data, s1->size);
	int rc = ssh_string_fill(s1, "hello", 5);
	if (rc != 0) return 1;
	printf("%zu\n", strlen(s1->data));
}

