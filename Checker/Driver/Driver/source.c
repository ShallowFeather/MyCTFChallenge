#include <ntifs.h>
#include <windef.h>
// 控制器
#define Test1 CTL_CODE(FILE_DEVICE_UNKNOWN, 1, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test2 CTL_CODE(FILE_DEVICE_UNKNOWN, 2, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test3 CTL_CODE(FILE_DEVICE_UNKNOWN, 3, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test4 CTL_CODE(FILE_DEVICE_UNKNOWN, 4, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test5 CTL_CODE(FILE_DEVICE_UNKNOWN, 5, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test6 CTL_CODE(FILE_DEVICE_UNKNOWN, 6, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test7 CTL_CODE(FILE_DEVICE_UNKNOWN, 7, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test8 CTL_CODE(FILE_DEVICE_UNKNOWN, 8, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test9 CTL_CODE(FILE_DEVICE_UNKNOWN, 9, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test10 CTL_CODE(FILE_DEVICE_UNKNOWN, 10, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test11 CTL_CODE(FILE_DEVICE_UNKNOWN, 11, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test12 CTL_CODE(FILE_DEVICE_UNKNOWN, 12, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test13 CTL_CODE(FILE_DEVICE_UNKNOWN, 13, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test14 CTL_CODE(FILE_DEVICE_UNKNOWN, 14, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test15 CTL_CODE(FILE_DEVICE_UNKNOWN, 15, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test16 CTL_CODE(FILE_DEVICE_UNKNOWN, 16, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test17 CTL_CODE(FILE_DEVICE_UNKNOWN, 17, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test18 CTL_CODE(FILE_DEVICE_UNKNOWN, 18, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test19 CTL_CODE(FILE_DEVICE_UNKNOWN, 19, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test20 CTL_CODE(FILE_DEVICE_UNKNOWN, 20, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test21 CTL_CODE(FILE_DEVICE_UNKNOWN, 21, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test22 CTL_CODE(FILE_DEVICE_UNKNOWN, 22, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test23 CTL_CODE(FILE_DEVICE_UNKNOWN, 23, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test24 CTL_CODE(FILE_DEVICE_UNKNOWN, 24, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test25 CTL_CODE(FILE_DEVICE_UNKNOWN, 25, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test26 CTL_CODE(FILE_DEVICE_UNKNOWN, 26, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test27 CTL_CODE(FILE_DEVICE_UNKNOWN, 27, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test28 CTL_CODE(FILE_DEVICE_UNKNOWN, 28, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test29 CTL_CODE(FILE_DEVICE_UNKNOWN, 29, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test30 CTL_CODE(FILE_DEVICE_UNKNOWN, 30, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test31 CTL_CODE(FILE_DEVICE_UNKNOWN, 31, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test32 CTL_CODE(FILE_DEVICE_UNKNOWN, 32, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test33 CTL_CODE(FILE_DEVICE_UNKNOWN, 33, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test34 CTL_CODE(FILE_DEVICE_UNKNOWN, 34, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test35 CTL_CODE(FILE_DEVICE_UNKNOWN, 35, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test36 CTL_CODE(FILE_DEVICE_UNKNOWN, 36, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test37 CTL_CODE(FILE_DEVICE_UNKNOWN, 37, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test38 CTL_CODE(FILE_DEVICE_UNKNOWN, 38, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test39 CTL_CODE(FILE_DEVICE_UNKNOWN, 39, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define Test40 CTL_CODE(FILE_DEVICE_UNKNOWN, 40, METHOD_BUFFERED, FILE_ANY_ACCESS)

VOID UnDriver(PDRIVER_OBJECT pDriver)
{
	PDEVICE_OBJECT pDev;

		UNICODE_STRING SymLinkName;
		pDev = pDriver->DeviceObject;
	IoDeleteDevice(pDev);
		RtlInitUnicodeString(&SymLinkName, L"\\??\\SF_Driver");
		IoDeleteSymbolicLink(&SymLinkName); 
		DbgPrint("Unload\n");
}
NTSTATUS CreateDriverObject(IN PDRIVER_OBJECT pDriver)
{
	NTSTATUS Status;
	PDEVICE_OBJECT pDevObj;
	UNICODE_STRING DriverName;
	UNICODE_STRING SymLinkName;
	RtlInitUnicodeString(&DriverName, L"\\Device\\SF_Driver");
	Status = IoCreateDevice(pDriver, 0, &DriverName, FILE_DEVICE_UNKNOWN, 0,
		TRUE, &pDevObj);
	pDevObj->Flags |= DO_BUFFERED_IO;
	RtlInitUnicodeString(&SymLinkName, L"\\??\\SF_Driver");
	Status = IoCreateSymbolicLink(&SymLinkName, &DriverName);
	return STATUS_SUCCESS;
}
NTSTATUS DispatchCreate(PDEVICE_OBJECT pDevObj, PIRP pIrp)
{
	pIrp->IoStatus.Status = STATUS_SUCCESS; 
	IoCompleteRequest(pIrp, IO_NO_INCREMENT); 
	return STATUS_SUCCESS; 
}

NTSTATUS DispatchClose(PDEVICE_OBJECT pDevObj, PIRP pIrp)
{
	pIrp->IoStatus.Status = STATUS_SUCCESS;
	IoCompleteRequest(pIrp, IO_NO_INCREMENT); 
	return STATUS_SUCCESS; 
}
NTSTATUS DispatchIoctl(PDEVICE_OBJECT pDevObj, PIRP pIrp)
{
	NTSTATUS status = STATUS_INVALID_DEVICE_REQUEST;
	PIO_STACK_LOCATION pIrpStack;
	ULONG uIoControlCode;
	PVOID pIoBuffer;
	ULONG uInSize;
	ULONG uOutSize;
	pIrpStack = IoGetCurrentIrpStackLocation(pIrp);
	uIoControlCode = pIrpStack->Parameters.DeviceIoControl.IoControlCode;
	pIoBuffer = pIrp->AssociatedIrp.SystemBuffer;
	uInSize = pIrpStack->Parameters.DeviceIoControl.InputBufferLength;
	uOutSize = pIrpStack->Parameters.DeviceIoControl.OutputBufferLength;
	char flag[33];
	DWORD dw = 0;
	memcpy(&flag, pIoBuffer, sizeof(flag));
	switch (uIoControlCode)
	{
    case Test1:
    {
        if (flag[28] + flag[14] + flag[20] + flag[5] + flag[26] == 331) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test2:
    {
        if (flag[18] + flag[12] + flag[19] + flag[30] + flag[14] == 395) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test3:
    {
        if (flag[25] + flag[4] + flag[2] + flag[19] + flag[3] == 442
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test4:
    {
        if (flag[11] + flag[28] + flag[2] + flag[12] + flag[4] == 449
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test5:
    { // FAKE
        if (flag[16] + flag[22] + flag[15] + flag[11] + flag[3] == 670
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test6:
    {
        if (flag[13] + flag[31] + flag[24] + flag[21] + flag[15] == 473
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test7:
    {
        if (flag[2] + flag[29] + flag[5] + flag[4] + flag[28] == 420
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test8:
    {
        if (flag[20] + flag[12] + flag[14] + flag[18] + flag[4] == 427
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test9:
    {
        if (flag[3] + flag[23] + flag[27] + flag[7] + flag[12] == 417
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test10:
    {
        if (flag[3] + flag[5] + flag[1] + flag[6] + flag[28] == 401
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test11:
    {
        if (flag[11] + flag[27] + flag[2] + flag[7] + flag[21] == 351
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test12:
    {
        if (flag[17] + flag[4] + flag[31] + flag[29] + flag[5] == 471
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test13:
    {
        if (flag[1] + flag[27] + flag[21] + flag[22] + flag[3] == 341
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test14:
    {
        if (flag[13] + flag[9] + flag[22] + flag[25] + flag[0] == 400
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test15:
    {
        if (flag[21] + flag[24] + flag[25] + flag[3] + flag[15] == 379
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test16:
    {
        if (flag[23] + flag[16] + flag[25] + flag[8] + flag[24] == 421
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test17:
    { // FAKE
        if (flag[19] + flag[31] + flag[24] + flag[12] + flag[27] == 312
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test18:
    {
        if (flag[16] + flag[20] + flag[1] + flag[19] + flag[5] == 431
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test19:
    {
        if (flag[29] + flag[20] + flag[30] + flag[12] + flag[15] == 392
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test20:
    { // FAKE
        if (flag[26] + flag[4] + flag[23] + flag[15] + flag[1] == 324
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test21:
    {
        if (flag[16] + flag[20] + flag[30] + flag[4] + flag[0] == 400
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test22:
    {
        if (flag[25] + flag[12] + flag[31] + flag[5] + flag[13] == 494
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test23:
    {
        if (flag[8] + flag[17] + flag[26] + flag[5] + flag[30] == 345
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test24:
    {
        if (flag[3] + flag[8] + flag[25] + flag[0] + flag[12] == 378
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test25:
    {
        if (flag[9] + flag[11] + flag[20] + flag[10] + flag[18] == 380
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test26:
    {//FAKE
        if (flag[0] + flag[4] + flag[31] + flag[2] + flag[31] == 499
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test27:
    {
        if (flag[3] + flag[30] + flag[6] + flag[11] + flag[18] == 338
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test28:
    {
        if (flag[31] + flag[0] + flag[3] + flag[9] + flag[4] == 442
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test29:
    {
        if (flag[20] + flag[18] + flag[27] + flag[14] + flag[2] == 355
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test30:
    {//FAKE
        if (flag[5] + flag[4] + flag[0] + flag[2] + flag[3] == 500
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test31:
    {
        if (flag[28] + flag[6] + flag[27] + flag[8] + flag[2] == 430
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test32:
    {
        if (flag[20] + flag[16] + flag[30] + flag[3] + flag[22] == 350
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test33:
    {
        if (flag[8] + flag[3] + flag[7] + flag[23] + flag[21] == 342
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test34:
    {
        if (flag[16] + flag[25] + flag[28] + flag[30] + flag[21] == 345
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test35:
    {
        if (flag[16] + flag[7] + flag[12] + flag[20] + flag[19] == 474) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test36:
    {
        if (flag[3] + flag[14] + flag[22] + flag[2] + flag[21] == 318
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test37:
    {
        if (flag[24] + flag[7] + flag[31] + flag[10] + flag[27] == 499
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test38:
    {
        if (flag[26] + flag[4] + flag[21] + flag[11] + flag[3] == 324
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test39:
    {
        if (flag[19] + flag[31] + flag[25] + flag[13] + flag[27] == 487
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }

    case Test40:
    {
        if (flag[16] + flag[22] + flag[5] + flag[12] + flag[3] == 432
            ) {
            dw++;
        }
        memcpy(pIoBuffer, &dw, sizeof(DWORD));
        status = STATUS_SUCCESS;
         break;
    }
		pIrp->IoStatus.Status = status;
		pIrp->IoStatus.Information = uOutSize;
		IoCompleteRequest(pIrp, IO_NO_INCREMENT);
		return status;
	}
	if (status == STATUS_SUCCESS)
		pIrp->IoStatus.Information = uOutSize;
	else
		pIrp->IoStatus.Information = 0;
	pIrp->IoStatus.Status = status;
	IoCompleteRequest(pIrp, IO_NO_INCREMENT);
	return status;
}

NTSTATUS DriverEntry(PDRIVER_OBJECT pDriver, PUNICODE_STRING RegistryPath)
{
	CreateDriverObject(pDriver);
	pDriver->DriverUnload = UnDriver;
	pDriver->MajorFunction[IRP_MJ_CREATE] = DispatchCreate;
	pDriver->MajorFunction[IRP_MJ_CLOSE] = DispatchClose;
	pDriver->MajorFunction[IRP_MJ_DEVICE_CONTROL] = DispatchIoctl;
	DbgPrint("By: ShallowFeather");
	return STATUS_SUCCESS;
}