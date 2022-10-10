# login
https://DOMAIN/user/login 

`post`

## 请求方式

Content-Type: application/json

## 请求参数

```
{
    username: String,
    password: String
}
```

## 返回参数

```
{
    token: String,
    errmsg: String,
    errcode: i8
}
```