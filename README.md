# 自动小猿口算

<center>

**程序猿也是猿，端口也是口，算法也是算。**

</center>

<hr>

## 实现原理

1. 获取SCRCPY窗口截图。
1. 截取题目部分。
1. OCR识别文本。
1. 处理题目。
1. 在SCRCPY窗口上填写答案。

## 快速开始

需要下载下列文件：

- 从[发行](https://github.com/MoRanYue/LittleMonkeyCalculation-rs/releases)页下载`blew_up_monkey_calc.exe`
- [SCRCPY](https://github.com/Genymobile/scrcpy/releases)
- [文本检测模型](https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten)，将它放置于`blew_up_monkey_calc.exe`所在目录的`models`目录下。
- [文本识别模型](https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten)，将它放置于`blew_up_monkey_calc.exe`所在目录的`models`目录下。

接下来，将手机连接至电脑，开启SCRCPY，通过终端启动`blew_up_monkey_calc.exe`。假设工作目录为其所在的目录，则按照如下形式传入参数：
注意，所有坐标均从SCRCPY窗口左上角为原点，而非屏幕左上角。

```
.\blew_up_monkey_calc.exe <SCRCPY窗口标题> <检测区域矩形>
```

检测区域矩形（单位：像素）的形式如下：

```
<矩形左上角X>,<矩形左上角Y>,<矩形宽度>,<矩形高度>
```

若一切正常，本项目将自动开始进行图像识别。在小猿口算中开始匹配，在题目界面，将自动推进题目。

控制台将输出每次文本识别的结果，与题目作答结果。