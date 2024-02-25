# Blog

这是一个简易的 blog 服务。

## 前期调研

- [https://komisans.cc/](https://komisans.cc/)
- [https://blog.debuginn.cn/](https://blog.debuginn.cn/)
- [https://imzlp.com/](https://imzlp.com/)
- [https://www.devas.life/](https://www.devas.life/)

## 如何使用

将写好的 blog（例如 a.md） 存入 `back-end/blog` 目录下即可，服务启动后，可通过访问 `http://${host}/blog/a.md` 查看对应的 blog。

### 前置条件

支持使用 Docker 部署服务，需要提前安装：

- [Docker](https://www.docker.com/)
- [docker-compose](https://docs.docker.com/compose/)

### 启动服务

``` shell
docker-compose up -d
```

### 销毁服务

停止服务并且删除相关镜像：

``` shell
docker-compose down --rmi all
```

## TODO

1. 后端服务解析 HTTP 请求支持多线程处理；
2. 后端服务支持日志。