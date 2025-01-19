#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <sstream>
#include <functional>
#include <thread>
#include <chrono>
#include <ctime>
// for Mac
// Flag: TSC{Bobo_milk-tea_black-carbon}
std::size_t calculateFileHash(const std::string& filePath) {
    std::ifstream file(filePath, std::ios::binary);
    if (!file) {
        std::cerr << "Error: Unable to open file " << filePath << std::endl;
        return 0;
    }

    std::ostringstream oss;
    oss << file.rdbuf(); // 將文件內容讀取到字符串流中
    std::string fileContent = oss.str();

    // 使用 std::hash 計算雜湊值
    return (std::hash<std::string>()(fileContent));
}

int main() {
    // 圖片列表
    std::vector<std::string> images;
    
    images.push_back("meow1.jpeg");
    images.push_back("meow2.jpeg");
    images.push_back("mouse.jpeg");


    // 初始化隨機數生成器
    std::srand(std::time(nullptr));

    while (std::rand() != 0) {
        // 隨機選擇一張圖片
        int randomIndex = std::rand() % images.size();
        std::string selectedImage = images[randomIndex];

        // 計算文件內容的雜湊值
        

        // 構建命令 (Windows 使用 `start`，Linux/macOS 使用 `xdg-open` 或 `open`)
        std::string command = "open " + selectedImage; // 如果是 macOS, 改為 "open " + selectedImage;

        // 開啟圖片
        std::system(command.c_str());

        // 等待檢查圖片是否已被關閉 (簡單延遲避免過多開啟)
        std::this_thread::sleep_for(std::chrono::seconds(2));

        // 顯示提示以便讓使用者中止
    }
    std::vector<char> flag;
    std::size_t hashValue = calculateFileHash(images[0]);
    if (hashValue != 0) {
        int arr[] = {-1843360995, -1843360998, -1843361014, -1843360974, -1843361013, -1843360986, -1843360981, -1843360986, -1843361002 };
        for(auto i: arr) 
            flag.push_back(i ^ hashValue);

    }

    hashValue = calculateFileHash(images[1]);
    if (hashValue != 0) {
        int arr[] = { -1450939332, -1450939336, -1450939331, -1450939334, -1450939268, -1450939355, -1450939340, -1450939344, -1450939378 };
        for(auto i: arr) {
            flag.push_back(i ^ hashValue);
        }
    }

    hashValue = calculateFileHash(images[2]);
    if (hashValue != 0) {
        int arr[] = { -1693850940, -1693850934, -1693850937, -1693850939, -1693850931, -1693850997, -1693850939, -1693850937, -1693850924, -1693850940, -1693850935, -1693850936, -1693850917 };
        for(auto i: arr) {
            flag.push_back(i ^ hashValue);
        }
    }
    std::cout << "flag is:";
    for(auto i : flag) {
        std::cout << i;
    }
    std::cout << '\n';
    return 0;
}
