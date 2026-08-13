#include <stdio.h>
#include <string.h>

typedef struct {
	unsigned char *data;
	size_t size;
} ssh_string_struct;

extern ssh_string_struct *ssh_string_new(size_t);
extern int ssh_string_fill(ssh_string_struct *, void *, size_t);
extern ssh_string_struct *ssh_string_from_char(char *);
extern ssh_string_struct *ssh_string_from_data(void *, size_t);
extern size_t ssh_string_len(ssh_string_struct *);
extern char *ssh_string_get_char(ssh_string_struct *);
extern char *ssh_string_to_char(ssh_string_struct *);
extern void ssh_string_free_char(char *);

int main() {
	ssh_string_struct *s1 = ssh_string_new(6);
	printf("%p\n%s\n%d\n", s1, s1->data, s1->size);
	int rc = ssh_string_fill(s1, "hello1", 6);
	if (rc != 0) return 1;
	ssh_string_struct *s2 = ssh_string_from_char("hello2");
	printf("%p\n%s\n%d\n", s2, s2->data, s2->size);
	ssh_string_struct *s3 = ssh_string_from_data("hello3", 6);
	printf("%p\n%s\n%d\n", s3, s3->data, s3->size);
	size_t len = ssh_string_len(s3);
	printf("%ld\n", len);
	char *cstr = ssh_string_get_char(s2);
	printf("ctsr: %s\n", cstr);
	char *cstr2 = ssh_string_to_char(s1);
	printf("cstr2: %s\n", cstr2);
	ssh_string_free_char(cstr2);
	printf("cstr2: %s\n", cstr2); // Did this as  test to see if cstr2 is free
	return 0;
}

