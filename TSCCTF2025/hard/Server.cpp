#include "chacha20.hpp"

#include <iostream>
#include <cstring>
#include <vector>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <fstream>
using namespace std;

#define PORT 8080

int main() {
    int server_fd, client_socket;
    struct sockaddr_in address;
    int opt = 1;
    int addrlen = sizeof(address);
    char buffer[1024] = {0};

    // 創建 socket
    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("Socket failed");
        exit(EXIT_FAILURE);
    }

    // 設置選項，避免使用不支持的 SO_REUSEPORT
    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0) {
        perror("Setsockopt failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    // 設定地址和端口
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = inet_addr("127.0.0.1"); // 修改為您的伺服器 IP 地址
    address.sin_port = htons(PORT);

    // 綁定 socket
    if (bind(server_fd, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("Bind failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    // 監聽
    if (listen(server_fd, 3) < 0) {
        perror("Listen failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }


    // 接受連接
    if ((client_socket = accept(server_fd, (struct sockaddr *)&address, (socklen_t *)&addrlen)) < 0) {
        perror("Accept failed");
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    
    srand(static_cast<unsigned>(time(0)));

    // 生成兩個隨機數
    int num1 = rand() % 0xFF; // 第一個隨機數
    int num2 = rand() % 0xFF; // 第二個隨機數
    int num3 = rand() % 0xFF;

    // ChaCha20 設置
    uint8_t key[32] = {0x73, 0x68, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x66, 0x65, 0x61, 0x74, 0x68, 0x65, 0x72, 0xc8, 0x76, 0x3, 0x73, 0x74, 0x61, 0x72, 0x62, 0x75, 0x72, 0x73, 0x74, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d};
    uint8_t nonce[8] = {0x68, 0x61, 0x68, 0x61, 0x68, num1, num2, num3};
    Chacha20 chacha(key, nonce);

    // 原始資料
    char arr[14] = {'a', 'a', 'e', 'e', 'f', 'h', 'h', 'l', 'l', 'o', 'r', 's', 't', 'w'};
    int haha[30] = {
        11, 0, 5, 1, 6, 1, 5, 2, 11, 2,
        7, 3, 8, 3, 8, 4, 9, 5, 13, 6,
        8, 7, 11, 9, 12, 10, 13, 11, 13, 12
    };

    // 調整資料順序
    for (int i = 0; i < 30; i += 2) {
        char t = arr[haha[i]];
        arr[haha[i]] = arr[haha[i + 1]];
        arr[haha[i + 1]] = t;
    }

    // 構建加密輸入
    vector<uint8_t> input(arr, arr + 14);

    // 加密資料
    chacha.crypt(input.data(), input.size());


    // 發送加密後的資料
    if (send(client_socket, input.data(), input.size(), 0) < 0) {
        perror("Send failed");
        close(client_socket);
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    // 接收數字字串
    char len_buffer[16] = {0}; // 假設輸入的長度不超過15位數字
    int received = recv(client_socket, len_buffer, sizeof(len_buffer) - 1, 0);
    if (received < 0) {
        perror("Failed to receive length");
        close(client_socket);
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    len_buffer[received] = '\n'; // 確保字串結束
    int data_length = atoi(len_buffer); // 將字串轉換為數字

    // 接收指定長度的資料
    vector<uint8_t> data_buffer(data_length);
    received = recv(client_socket, data_buffer.data(), data_length, 0);
    if (received < 0) {
        perror("Failed to receive data");
        close(client_socket);
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    // 確保接收到正確長度的資料
    if (received != data_length) {
        
        for (int i = 0; i < data_length; ++i) {
            cout << hex << static_cast<int>(data_buffer[i] & 0xFF) << " ";
        }
        close(client_socket);
        close(server_fd);
        exit(EXIT_FAILURE);
    }

    cout << "Received data: ";
    for (int i = 0; i < data_length; ++i) {
        cout << hex << static_cast<int>(data_buffer[i] & 0xFF) << " ";
    }
    cout << endl;

    // 將資料寫入檔案
    

    const char* output_file = "flag";
    ofstream outfile(output_file, ios::binary);
    if (!outfile.is_open()) {
        cerr << "Failed to open file for writing: " << output_file << endl;
        close(client_socket);
        close(server_fd);
        exit(EXIT_FAILURE);
    }
    Chacha20 chacha1(key, nonce);

    chacha1.crypt(data_buffer.data(), data_buffer.size());
    vector<char> vec;
    for(int i = 0; i < data_buffer.size(); i++) {
        vec.push_back(data_buffer[i]);
    }
    outfile.write(vec.data(), data_buffer.size());
    outfile.close();
    

    // 關閉 socket
    close(client_socket);
    close(server_fd);

    return 0;
}
