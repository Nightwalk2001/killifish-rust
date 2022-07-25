### 安装docker

```shell
sudo yum-config-manager \
--add-repo \
https://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo
```

### docker 运行mongodb

ps: 如果服务器没有安全风险，可以不用指定用户名和密码的两句

```shell
docker run \
-p 17017:27017 \
--name=mongodb \
-v /docker/mongo:/data \
-e MONGO_INITDB_ROOT_USERNAME="这里填用户名" \
-e MONGO_INITDB_ROOT_PASSWORD="这里填密码" \
-d  mongo
```

### docker 运行搜索引擎

```shell
docker run --name meilisearch -d \
-p 7700:7700 \
-e MEILI_MASTER_KEY="MASTER_KEY"\
-e MEILI_NO_ANALYTICS=true\
-v /docker/meili_data:/meili_data \
getmeili/meilisearch:latest \
meilisearch --env="production"
```

### docker 运行MQTT服务器

```shell
docker run -d --name emqx \
-p 1883:1883 -p 8083:8083 
 emqx/emqx:latest
```

### 安装并启动nginx

```shell
 yum install nginx -y
 sudo systemctl enable nginx
 sudo systemctl start nginx
```

### 配置nginx
