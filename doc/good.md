# 添加货物

https://DOMAIN/good/addGood

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    id: String,
    name: String,
    size: u32,
    user_name: String
}
```

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 获取所有货物

https://DOMAIN/good/getGoodList

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

## 返回参数

```
{
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8
}
```

# 删除对应货物

https://DOMAIN/good/delGood

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    good_id: String
}
```

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 更新货物

https://DOMAIN/good/updateGood

`post`

## 请求方式

Content-Type: application/json

## 请求参数

`authorization:{{TOKEN}}`

```
{
    id: String,
    name: String,
    size: u32,
    user_name: String
}
```

## 返回参数

```
{
    errmsg: String,
    errcode: i8
}
```

# 获取所有货物

https://DOMAIN/good/getGoodList

`post`

## 请求参数

`authorization:{{TOKEN}}`

```
{
    good_name: String,
}
```

## 返回参数

```
{
    good_list: Vec<Good>,
    errmsg: String,
    errcode: i8
}
```
