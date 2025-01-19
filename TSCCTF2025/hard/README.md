＃ hard-
| Creater: ShallowFeather

這題主要是考在 FlareOn11 第七題的相似題，並且把它簡單化

參賽者會拿到一個執行黨以及封包

在逆向分析時應該能夠很簡單發現他在執行流加密
透過搜尋字串可以找到 Expand 32-byte-k
有此字串代表說一定是 salsa20 或是 chacha20
知道這件事後可以到網路上找 code 並通過比較代碼知道是 chacha20

執行流程會是

執行檔 -> 送出某字串到 -> server
server 去做某種運算後回傳檔案
server -> 回傳檔案

那字串會是 shallowfeather 在經過 chacha20 加密後送出的
既然知道了是 chacha20 那接下來就是要找出他的 key 跟 nonce
通過分析可以發現他的 key 是固定的
但是 nonce 的最後三個 bytes 是隨機的
因此其實到這邊就可以知道題目具體要幹嘛了

就是分析出封包中的資料 在哪個隨機數下會能夠解密回來 shallowfeather
不過這邊我塞了一個梗，就是不同於正常的 chacha20 的 Expand 32-byte-k
他的 E 會是大寫，因此沒辦法利用 pycrypto 庫直接解出來，如果觀察的出來就可以直接用外部庫去跑，看不出來可能只能用 Qiling 去模擬跑程式進行解密。
解密出來後，利用同個 key 跟 nonce 去解後面的封包就能拿到一個執行檔
會是 ARM 的執行檔，但這邊沒有要為難參賽者的意思，所以就直接執行起來，或是丟 Decompiler 就能看出在做 xor 操作，直接解出 flag。