#include "Enclave_u.h"
#include <errno.h>

typedef struct ms_printf_helloworld1_t {
	int ms_retval;
} ms_printf_helloworld1_t;

typedef struct ms_ocall_print_string_t {
	const char* ms_str;
} ms_ocall_print_string_t;

typedef struct ms_ocall_print_string1_t {
	int ms_retval;
	const char* ms_str;
} ms_ocall_print_string1_t;

static sgx_status_t SGX_CDECL Enclave_ocall_print_string(void* pms)
{
	ms_ocall_print_string_t* ms = SGX_CAST(ms_ocall_print_string_t*, pms);
	ocall_print_string(ms->ms_str);

	return SGX_SUCCESS;
}

static sgx_status_t SGX_CDECL Enclave_ocall_print_string1(void* pms)
{
	ms_ocall_print_string1_t* ms = SGX_CAST(ms_ocall_print_string1_t*, pms);
	ms->ms_retval = ocall_print_string1(ms->ms_str);

	return SGX_SUCCESS;
}

static const struct {
	size_t nr_ocall;
	void * table[2];
} ocall_table_Enclave = {
	2,
	{
		(void*)Enclave_ocall_print_string,
		(void*)Enclave_ocall_print_string1,
	}
};
sgx_status_t printf_helloworld(sgx_enclave_id_t eid)
{
	sgx_status_t status;
	status = sgx_ecall(eid, 0, &ocall_table_Enclave, NULL);
	return status;
}

sgx_status_t printf_helloworld1(sgx_enclave_id_t eid, int* retval)
{
	sgx_status_t status;
	ms_printf_helloworld1_t ms;
	status = sgx_ecall(eid, 1, &ocall_table_Enclave, &ms);
	if (status == SGX_SUCCESS && retval) *retval = ms.ms_retval;
	return status;
}

