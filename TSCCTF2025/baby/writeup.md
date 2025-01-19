# Meoware

| creater: ShallowFeather

當中為會一直開圖片的執行檔
當丟進ida 後可以看到他的迴圈是由 rand() 是否等於 0 決定的
並且跳出 rand() 迴圈後就會爆出 flag
因此只需 patch 掉 讓他不要執行迴圈即可
![alt text](image.png)
![alt text](image-1.png)