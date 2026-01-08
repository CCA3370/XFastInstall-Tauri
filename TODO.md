我要用ai做一个X-Plane 12插件安装器。使用Python+PySide6+PyOneDark，实现各种功能时尽量用PySide6自带模块，如果没有的话再用别的库，这样能最大程度保证跨平台兼容性。
预期效果：直接把 X-Plane 的机模压缩包或文件夹（任意目录包含*.acf文件）、地景压缩包或文件夹（任意目录包含library.txt或*.dsf）、插件压缩包或文件夹（任意目录包含*.xpl）、导航数据压缩包或文件夹（任意目录包含cycle.json，并且要读取cycle.json的'name'键值以确定具体安装位置）拖到软件图标上方，或先打开软件再拖到软件内部时，将该文件解压（按需）并安装到X-Plane对应位置。要支持多种常见压缩格式的解压。

具体安装方式：
1.机模：把存在*.acf文件的目录解压放入 机模目录；如果压缩包中*.acf直接存在于第一级，那就把压缩包名作为文件夹名解压到 机模目录；如果拖入的是一个文件夹，那就省去解压流程，其他不变。
例：
    A330.zip/
        A330-rr/
            A330-200/
                ...
                A330.acf
                A330_xp11.acf
                ...
那么就要把A330-200文件夹解压放入 机模目录，注意不要重复识别。
2.地景：把存在library.txt文件的文件夹解压放入 地景目录；把存在*.dsf的上上级目录放入 地景目录。
例：
    RCMT.zip/
        RCMT Airport/
            Earth nav data/
                +20+110/
                    +26+119.dsf
                +20+120/
                    +26+120.dsf
那就把RCMT Airport文件夹放入 地景目录，注意不要重复识别。
例：
    ZU-LIB.zip/
        ZULIB/
            ...
            library.txt
            ...
那就把ZULIB文件夹放入 地景目录。
3.插件：把包含*.xpl（不含win/lin/mac.xpl）文件的上级文件夹放入 插件目录；把包含win/lin/mac.xpl的目录（若该目录名叫 32 或 64 ，那就取该目录的上级目录）放入 插件目录。
例：
    Plugin.zip/
        BetterSky/
            win_x64/
                BetterSky.xpl
            32/
                win.xpl
            mac.xpl
            lin.xpl
那就把BetterSky文件夹放入 插件目录，注意不要重复识别。
4.导航数据：查找cycle.json文件并读取，获取其中的"name"字段，如果包含"X-Plane 12"或"X-Plane 11"，那就把这个json所在的目录中的所有内容放入 导航数据目录；如果"name"字段中包含"X-Plane GNS430"，那就把这个json文件的上级文件夹中的文件和文件夹放入 导航数据目录 中的 GNS430 文件夹；如果两个字段都不包含，那就弹出提醒，不安装。
例：
    xp12_native.zip/
        ...
        cycle.json（"name"包含"X-Plane 12"或"X-Plane 11"）
        ...
那就把与cycle.json同目录的所有文件和文件夹放入 导航数据目录。
例：
    xp12_custom.zip/
        navdata/
            ...
            cycle.json（"name"包含"X-Plane GNS430"）
            ...
那就把与navdata目录同目录的所有文件夹和文件放入 导航数据目录中的GNS430目录。

软件处理过程中，要做好安全处理、错误处理、权限处理。如果目标文件已存在（用标志文件对比，*.acf,*.xpl,library.txt 等），那就弹窗提示用户并取消这个项目的安装。cycle.json已存在没关系，只要读取其中的"cycle"字段给用户做对比，让用户确认是否覆盖安装就行。
当一个标志文件的完整目录存在于另一个标志文件的目录中时，需要标记为同一个项目，不要重复解压。
如：
ABC/DEF/GHI.xpl
ABC/win.xpl
是同一个项目
ABC/DEF/GHI.xpl
ABC/DEF/win.xpl
也是同一个项目

一个压缩包里可能有多个类型的项目，要能正确识别。