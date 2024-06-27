

#include <iostream>
#include <string>

#include <Windows.h>


#define IOCTL_TEST(code) CTL_CODE(FILE_DEVICE_UNKNOWN, code, METHOD_BUFFERED, FILE_ANY_ACCESS)


class IOCTL {
	
public:
	IOCTL() {}
	~IOCTL() {}

	struct Buffer {
		char flag[33];
	};
	int DriverCheck(int check, std::string flag) {
		HANDLE Device = CreateFileA("\\\\.\\SF_Driver",
			GENERIC_READ | GENERIC_WRITE, 0,
			NULL, OPEN_EXISTING,
			FILE_ATTRIBUTE_NORMAL, NULL);

		if (Device == INVALID_HANDLE_VALUE) {
			std::cout << "Can't Link Driver" << '\n';
			return 0;
		}
		else {
			if (flag.size() != 32) {
				std::cout << "Wrong Flag Length\n" << '\n';
				return 0;
			}
			Buffer input;
			strcpy_s(input.flag, flag.c_str());
			#define Test CTL_CODE(FILE_DEVICE_UNKNOWN, check, METHOD_BUFFERED, FILE_ANY_ACCESS)
			DWORD output = 0, ref_len = 0;
			DeviceIoControl(Device, Test, &input, sizeof(input), &output, sizeof(output), &ref_len, 0);
			//DeviceIoControl(Device, checker, &input, sizeof(input), &output, sizeof(output), &ref_len, 0);

			CloseHandle(Device);
			//std::cout << output << '\n';
			return output;
		}
	}
};

int main() {


	IOCTL checker;
	std::string input;
	std::getline(std::cin, input);
	int cnt = 0;
	int err = 0;
	srand(time(NULL));
	bool check[41] = {};

	for (int i = 0; i < 40; i++) {
		int random_num = std::rand();
		random_num = random_num % 40 + 1;
		while (check[random_num] == 1) {
			random_num = std::rand();
			random_num = random_num % 40 + 1;
		}
		check[random_num] = 1;
		//std::cout << random_num << '\n';
		bool tmp = checker.DriverCheck(random_num, input);
		cnt += tmp;
		err += !tmp;
		if (err > 5) break;
		//std::cout << checker.DriverCheck(0x999, input) << '\n';
	}
	if (cnt == 35) {
		std::cout << "This is the flag\n";
	}
	else {
		std::cout << "This can't be flag\n";
	}
}