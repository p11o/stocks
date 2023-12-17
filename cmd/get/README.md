# get


```
docker build -t stocks-get:local . 
docker run \
  -v "$PWD:/app" \
  -w /app \
  -e POLYGON_API_KEY \
  -u 1000 \
  stocks-get:local \
  --ticker AMZN \
  --from 2023-01-01 --to 2023-12-16
```