// 安装docker
sudo yum-config-manager \
    --add-repo \
    http://mirrors.aliyun.com/docker-ce/linux/centos/docker-ce.repo

// docker 运行mongodb
docker run \
-p 17017:27017 \
--name=mongodb \
-v /docker/mongo:/data \
-e MONGO_INITDB_ROOT_USERNAME="wzw" -e MONGO_INITDB_ROOT_PASSWORD="Yexin@520.com" -d  mongo

docker run --name meilisearch -d \
    -p 7700:7700 \
    -e MEILI_MASTER_KEY="MASTER_KEY"\
    -e MEILI_NO_ANALYTICS=true\
    -v /docker/meili_data:/meili_data \
    getmeili/meilisearch:latest \
    meilisearch --env="production"

db.createUser(
     {
       user: "killifish",
       pwd: "Wzw20010827",
       roles:[{role:"dbOwner",db:"killifish"}]
     }
  )

db.createUser(
     {
       user: "logistics",
       pwd: "Wzw20010827",
       roles:[{role:"dbOwner",db:"logistics"}]
     }
  )
