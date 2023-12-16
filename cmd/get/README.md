# get


You need to set the user to 1000 so that it can write to the mounted volume

```
docker run \
  -v "$PWD:/home/curl_user" \
  -e POLYGON_API_KEY \
  -u 1000 \
  stock-get:local \
  --ticker AMZN \
  --from 2023-01-01 --to 2023-12-16
```