### home_cloud

基于rust语言构建的一个CMS

#### 规范约定：
* 1、蛇形命名法（Snake Case）

    文件名：例如：hello_world.rs、main_xxx.rs

    变量名：例如：zhangsan_name

    函数名：例如：func_name()


* 2、大驼峰命名法（Camel Case）

    结构体：例如：struct ExampleStruct { name: String}

    enum类型：例如：enum IpAddress {IPV4(u8,u8,u8,u8)}


* 3、其它

    关联常量：全部大写，例如：NAME、AGE

    连接符：Cargo默认把连接符“-”转换成下划线“_”

    语句：跟C，Java语言等一样，每行语句结束都要添加;

    PS：Rust也不建议以“-rs”或“_rs”为后缀来命名包名，如果以此来命名，会强制性的将此后缀去掉。


#### 遇到的疑难问题

* 下载文件乱码

  1）先对文件名进行urlencode，
  
  2）然后加上*=utf-8，参考：https://www.iefans.net/xiazai-wenjian-http-bianma-content-disposition/