# get


You need to set the user to 1000 so that it can write to the mounted volume

```
docker build -t stocks-get:local . 
docker run \
  -v "$PWD:/app" \
  -w /app \
  -e POLYGON_API_KEY \
  stocks-get:local \
  --ticker AMZN \
  --from 2023-01-01 --to 2023-12-16
```